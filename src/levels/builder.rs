use super::*;

pub struct LevelBuilder {

    name: String,
    required_points: u32,

    content: LevelContent
}

impl LevelBuilder {

    pub fn new_empty() -> LevelBuilder {
        LevelBuilder {
            name: "New level".to_string(),
            required_points: 0,

            content: LevelContent::new_empty()
        }
    }

    pub fn from_level(level: Level) -> LevelBuilder {
        LevelBuilder {
            name: level.name.clone(),
            required_points: level.required_points,

            content: level.load_content()
        }
    }
}