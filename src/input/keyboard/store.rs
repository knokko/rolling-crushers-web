use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use wasmuri_events::*;

pub struct KeyStore {
    internal: Rc<RefCell<InternalKeyStore>>
}

impl KeyStore {

    pub fn create() -> KeyStore {
        KeyStore {
            internal: Rc::new(RefCell::new(InternalKeyStore::create()))
        }
    }

    pub(super) fn get_weak_ref(&self) -> Weak<RefCell<InternalKeyStore>> {
        Rc::downgrade(&self.internal)
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        let internal = self.internal.borrow_mut();
        internal.is_pressed(key)
    }
}

pub(super) struct InternalKeyStore {

    pressed_keys: Vec<bool>
}

impl InternalKeyStore {

    fn create() -> InternalKeyStore {
        InternalKeyStore {
            pressed_keys: vec![false; AMOUNT]
        }
    }

    fn set_key_state(&mut self, key_string: String, new_state: bool){
        let maybe_key = get_key_code(key_string.clone());
        if maybe_key.is_some() {
            let key = maybe_key.unwrap();
            self.pressed_keys[key.index] = new_state;
        }
    }

    fn set_pressed(&mut self, key_string: String){
        self.set_key_state(key_string, true);
    }

    fn set_released(&mut self, key_string: String){
        self.set_key_state(key_string, false);
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed_keys[key.index]
    }
}

impl Listener<KeyDownEvent> for InternalKeyStore {

    fn process(&mut self, event: &KeyDownEvent){
        self.set_pressed(event.key_event.key());
    }
}

impl Listener<KeyUpEvent> for InternalKeyStore {

    fn process(&mut self, event: &KeyUpEvent){
        self.set_released(event.key_event.key());
    }
}

pub struct Key {
    index: usize
}

const fn key(index: usize) -> Key {
    Key {
        index
    }
}

pub const KEY_LEFT: Key = key(0);
pub const KEY_RIGHT: Key = key(1);
pub const KEY_UP: Key = key(2);
pub const KEY_DOWN: Key = key(3);

const AMOUNT: usize = 2;

fn get_key_code(key: String) -> Option<Key> {
    println!("Asked for code of key {}", key);
    if key == "a" {
        return Some(KEY_LEFT);
    } else if key == "right" {
        return Some(KEY_RIGHT);
    } else if key == "up" {
        return Some(KEY_UP);
    } else if key == "down" {
        return Some(KEY_DOWN);
    }

    println!("The key wasn't found ;(");

    None
}