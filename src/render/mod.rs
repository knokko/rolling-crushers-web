use web_sys::window;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext;
use web_sys::Event;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

fn maximize_canvas() -> WebGlRenderingContext {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("wasm-canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
    canvas.set_width(width);
    canvas.set_height(height);
    let gl = canvas.get_context("webgl").unwrap().unwrap().dyn_into::<WebGlRenderingContext>().unwrap();
    gl.viewport(0, 0, width as i32, height as i32);
    gl
}

pub fn create() -> RenderManager {
    let gl = maximize_canvas();

    RenderManager {
        gl
    }
}

pub fn start(){
    let window = window().unwrap();

    let on_resize = Closure::wrap(Box::new(move |_event: Event| {
        maximize_canvas();
    }) as Box<dyn FnMut(_)>);

    window.add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref()).unwrap();

    on_resize.forget();
}

pub struct RenderManager {

    gl: WebGlRenderingContext
}

impl RenderManager {

    
}