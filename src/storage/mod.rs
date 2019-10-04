use crate::levels::LevelPack;

mod play;

pub use play::*;

/// Lists the names of all available level packs.
/// 
/// Until this is implemented properly, it returns the vector containing only "Default"
pub fn get_level_pack_names() -> Vec<String> {
    vec!["Default".to_string()]
}

pub fn get_pack(_pack_name: &str) -> LevelPack {
    panic!("I still need to implement this...");
}