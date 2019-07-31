mod keyboard;

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
    
    key_store: keyboard::KeyStore
}