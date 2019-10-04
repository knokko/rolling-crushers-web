pub mod component;

use crate::collission::Point;
use component::LevelComponent;

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