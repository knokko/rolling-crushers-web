use web_sys::WebGlRenderingContext;

pub trait LevelComponent {

    fn render(&self, gl: &WebGlRenderingContext);

    fn get_render_bounds(&self);

    fn to_space_component(&self);
}