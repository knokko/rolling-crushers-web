mod pack;
mod content;

pub use pack::*;
pub use content::*;

pub struct Level {

    index: usize,
    required_points: u32,

    content: LevelContent
}

impl Level {

    pub fn new(index: usize, required_points: u32, content: LevelContent) -> Level {
        Level {
            index,
            required_points,
            content
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_required_points(&self) -> u32 {
        self.required_points
    }

    pub fn get_content(&self) -> &LevelContent {
        &self.content
    }
}