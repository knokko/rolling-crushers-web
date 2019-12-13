use web_sys::WebGlRenderingContext;

pub trait LevelComponent {

    fn boxed_clone(&self) -> Box<dyn LevelComponent>;

    fn render(&self, gl: &WebGlRenderingContext);

    fn get_render_bounds(&self);

    fn to_space_component(&self);
}