use wasmuri_events::*;

pub mod store;
pub use store::*;

pub fn create() -> KeyStore {
    KeyStore::create()
}

pub fn start(){
    let app_instance = crate::get_instance();

    let key_store = &app_instance.input_manager.key_store;

    KEY_DOWN_HANDLER.add_listener(key_store.get_weak_ref());
    KEY_UP_HANDLER.add_listener(key_store.get_weak_ref());
}