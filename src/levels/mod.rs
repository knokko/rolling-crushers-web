mod pack;
mod content;
mod builder;

pub use pack::*;
pub use content::*;
pub use builder::*;

#[derive(Clone)]
pub struct Level {

    index: usize,
    name: String,
    required_points: u32,

    content_key: LevelContentKey
}

impl Level {

    pub fn new(index: usize, name: String, required_points: u32, content_key: LevelContentKey) -> Level {
        Level {
            index,
            name,
            required_points,
            content_key
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_required_points(&self) -> u32 {
        self.required_points
    }

    pub fn load_content(&self) -> LevelContent {
        panic!("Still need to implement this...")
    }
}