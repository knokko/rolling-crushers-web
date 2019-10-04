use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use wasmuri_container::ContainerManager;
use wasmuri_text::{
    Font,
    FontDetails
};

use web_sys::{
    HtmlCanvasElement,
    window
};

pub mod input;
pub mod progress;
pub mod levels;
pub mod menu;
pub mod storage;
pub mod collission;

#[wasm_bindgen]
pub fn start_wasm(){

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let container_manager_cell = ContainerManager::start(window().expect("Should have window").document().expect("Should have document")
    .get_element_by_id("wasm-canvas").expect("Should have element with id 'wasm-canvas'").dyn_into::<HtmlCanvasElement>()
    .expect("Element with id 'wasm-canvas' should be a canvas"), None);

    let input_manager = input::create();
    let default_font;

    {
        let container_manager = container_manager_cell.borrow();

        let mut text_renderer = container_manager.get_text_renderer().borrow_mut();
        default_font = text_renderer.add_font(FontDetails::from_str("", "Arial"));
    }

    let application = Application {

        container_manager: container_manager_cell,
        fonts: Fonts {
            default: default_font
        },
        input_manager
    };

    // Static mutables are unsafe, even though this will be single-threaded
    unsafe {
        INSTANCE = Some(application);
    }

    let application = get_instance();
    let mut container_manager = application.container_manager.borrow_mut();
    
    let main_menu = menu::main::create_main_menu();

    container_manager.set_container_cell(main_menu);

    input::start();
}

pub struct Application {

    container_manager: Rc<RefCell<ContainerManager>>,
    fonts: Fonts,
    input_manager: input::InputManager
}

pub struct Fonts {

    default: Rc<Font>
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

pub fn get_default_font<'a>() -> &'a Rc<Font> {
    let instance = get_instance();
    &instance.fonts.default
}