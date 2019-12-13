use crate::levels::LevelBuilder;

use std::cell::RefCell;
use std::rc::Rc;

use super::*;

use wasmuri_container::*;
use wasmuri_container::layer::*;

pub fn create_level_edit(level_copy: LevelBuilder) -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));

    // TODO Add parameter for modification

    FlatContainer::celled(layer)
}