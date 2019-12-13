pub mod component;

use crate::collission::Point;
use component::LevelComponent;

#[derive(Clone)]
pub struct LevelContentKey {

    storage_key: String
}

impl LevelContentKey {

    pub fn new(storage_key: String) -> LevelContentKey {
        LevelContentKey {
            storage_key
        }
    }
}

pub struct LevelContent {

    pub start_point: Point,

    pub components: Vec<Box<dyn LevelComponent>>
}

impl LevelContent {

    pub fn new_empty() -> LevelContent {
        LevelContent {
            start_point: Point::new(0, 0, 0),
            components: Vec::new()
        }
    }
}

impl Clone for LevelContent {

    fn clone(&self) -> LevelContent {
        let mut components = Vec::with_capacity(self.components.len());
        for component in &self.components {
            components.push(component.boxed_clone());
        }

        LevelContent {
            start_point: self.start_point,
            components
        }
    }
}