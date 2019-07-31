pub struct KeyStore {

    pressed_keys: Vec<bool>
}

impl KeyStore {

    pub fn create() -> KeyStore {
        KeyStore {
            pressed_keys: vec![false; AMOUNT]
        }
    }

    fn set_key_state(&mut self, key_string: String, new_state: bool){
        let maybe_key = get_key_code(key_string);
        if maybe_key.is_some() {
            let key = maybe_key.unwrap();
            self.pressed_keys[key.index] = new_state;
        }
    }

    pub fn set_pressed(&mut self, key_string: String){
        self.set_key_state(key_string, true);
    }

    pub fn set_released(&mut self, key_string: String){
        self.set_key_state(key_string, false);
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed_keys[key.index]
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
    if key == "Left" {
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