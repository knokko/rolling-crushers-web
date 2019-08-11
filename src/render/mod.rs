use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext;

use wasmuri_text::{
    TextRenderer,
    FontID,
    FontDetails,
    TextModel
};
use wasmuri_core::util::color::Color;

use std::rc::Rc;

use crate::input::*;

const FONT_DETAILS: FontDetails = FontDetails::new("bold", "Arial");

/// Call init() before using the obtained instance
/// This currently happens in the start() function of this module
pub fn create() -> RenderManager<'static> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("wasm-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let gl = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    gl.clear_color(1.0, 0.0, 0.8, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    let rc_gl = Rc::new(gl);
    let rc_clone = Rc::clone(&rc_gl);

    let text_renderer = TextRenderer::new_empty(rc_clone, 1);

    RenderManager {
        gl: rc_gl,
        text_renderer,
        text_model: None,
        font_id: None
    }
}

pub fn start() {

    let app_instance = crate::get_mut_instance();
    app_instance.render_manager.init();

    update_canvas_size();
    listen_resize();

    // Actually the first frame
    request_next_frame();
}

fn listen_resize(){
    let window = window().unwrap();

    let resize_closure = Closure::wrap(Box::new(move || {
        update_canvas_size();
    }) as Box<dyn FnMut()>);

    window
        .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
        .unwrap();

    resize_closure.forget();
}

fn update_canvas_size(){
    let inner_window = web_sys::window().unwrap();
    let canvas = inner_window.document().unwrap().get_element_by_id("wasm-canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
    let width = inner_window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = inner_window.inner_height().unwrap().as_f64().unwrap() as u32;
    canvas.set_width(width);
    canvas.set_height(height);
    let gl = canvas.get_context("webgl").unwrap().unwrap().dyn_into::<WebGlRenderingContext>().unwrap();
    gl.viewport(0, 0, width as i32, height as i32);
}

fn request_next_frame(){
    let window = window().unwrap();

    let render_closure = Closure::wrap(Box::new(move || {
        render();
        request_next_frame();
    }) as Box<dyn FnMut()>);

    window
        .request_animation_frame(render_closure.as_ref().unchecked_ref())
        .unwrap();

    render_closure.forget();
}

fn render() {
    let app_instance = crate::get_mut_instance();
    let render_manager = &mut app_instance.render_manager;
    let gl = &render_manager.gl;
    if app_instance
        .input_manager
        .get_key_store()
        .is_pressed(KEY_LEFT)
    {
        gl.clear_color(1.0, 0.0, 1.0, 1.0);
    } else {
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
    }

    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    let text_renderer = &mut render_manager.text_renderer;
    text_renderer.start_rendering();
    let font = text_renderer.get_font_by_id(render_manager.font_id.expect("Should have font id by now"));
    let text_model = render_manager.text_model.as_ref().unwrap();

    font.render_text_model(&text_model, -1.0, 0.7, 0.3, Color::from_rgb(200, 0, 200), Color::from_rgba(40, 200, 200, 255), Color::from_rgba(200, 200, 0, 255));
}

pub struct RenderManager<'a> {
    gl: Rc<WebGlRenderingContext>,
    text_renderer: TextRenderer<'a>,
    text_model: Option<TextModel<'a>>,
    font_id: Option<FontID>
}

impl<'a> RenderManager<'a> {

    pub fn init(&'a mut self){
        let font = self.text_renderer.add_font(FONT_DETAILS);
        self.text_model = Some(font.create_text_model("HÁllo World!"));
        self.font_id = Some(font.get_id());
    }
}
