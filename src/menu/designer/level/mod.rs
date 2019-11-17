use crate::levels::LevelContent;

use std::cell::RefCell;
use std::rc::Rc;

use super::*;

use wasmuri_container::*;
use wasmuri_container::layer::*;

pub fn create_level_edit(level: LevelContent) -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));

    FlatContainer::celled(layer)
}