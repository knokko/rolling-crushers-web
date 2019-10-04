use crate::levels::Level;

pub struct ProgressEntry {

    level_completion_table: Vec<bool>,
    completed_amount: u32
}

impl ProgressEntry {

    pub fn has_completed(&self, level: Level) -> bool {
        self.level_completion_table[level.get_index()]
    }

    pub fn get_completed_amount(&self) -> u32 {
        self.completed_amount
    }
}