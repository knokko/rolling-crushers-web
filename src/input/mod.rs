mod keyboard;

pub use keyboard::*;

pub fn create() -> InputManager {
    let key_store = keyboard::create();

    InputManager {
        key_store
    }
}

pub fn start(){
    keyboard::start();
}

pub struct InputManager {
    
    key_store: KeyStore
}

impl InputManager {

    pub fn get_key_store(&self) -> &KeyStore {
        &self.key_store
    }
}