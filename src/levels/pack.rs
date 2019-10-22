use super::Level;

pub struct LevelPack {

    name: String,

    levels: Vec<Level>
}

impl LevelPack {

    pub fn get_standard() -> LevelPack {
        panic!("Still have to implement this x)");
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_levels(&self) -> &Vec<Level> {
        &self.levels
    }
}

pub struct LevelPackBuilder {

    pub name: String,

    pub levels: Vec<Level>
}

impl LevelPackBuilder {

    pub fn new_empty(name: &str) -> LevelPackBuilder {
        LevelPackBuilder {
            name: name.to_string(),
            levels: Vec::new()
        }
    }
}