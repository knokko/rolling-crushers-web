use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub mod store;
pub use store::*;

pub fn create() -> KeyStore {
    KeyStore::create()
}

pub fn start(){
    let window = window().unwrap();

    let press = Closure::wrap(Box::new(move |event: KeyboardEvent| {

        // It's a good thing javascript events are singlethreaded
        let app_instance = crate::get_mut_instance();
        app_instance.input_manager.key_store.set_pressed(event.key());
    }) as Box<dyn FnMut(_)>);

    window.add_event_listener_with_callback("keydown", press.as_ref().unchecked_ref()).unwrap();

    press.forget();

    let release = Closure::wrap(Box::new(move |event: KeyboardEvent| {

        // It's a good thing javascript events are singlethreaded
        let app_instance = crate::get_mut_instance();
        app_instance.input_manager.key_store.set_released(event.key());
    }) as Box<dyn FnMut(_)>);

    window.add_event_listener_with_callback("keyup", release.as_ref().unchecked_ref()).unwrap();

    release.forget();
}