use wasm_bindgen::prelude::*;

mod input;
mod render;

#[wasm_bindgen]
extern {
    // Might need this later
}

#[wasm_bindgen]
pub fn start_wasm(){

    let input_manager = input::create();
    let render_manager = render::create();

    let application = Application {
        input_manager,
        render_manager
    };

    // Static mutables are unsafe, even though this will be single-threaded
    unsafe {
        INSTANCE = Some(application);
    }

    input::start();
    render::start();
}

pub struct Application {

    input_manager: input::InputManager,
    render_manager: render::RenderManager

}

static mut INSTANCE: Option<Application> = None;

pub fn get_mut_instance() -> &'static mut Application {
    unsafe {
        match &mut INSTANCE {
            Some(success) => success,
            None => panic!("Application instance has not yet been set")
        }
    }
}

pub fn get_instance() -> &'static Application {
    unsafe {
        match &INSTANCE {
            Some(success) => success,
            None => panic!("Application instance has not yet been set")
        }
    }
}