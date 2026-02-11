use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Summary {
    pub level_counts: HashMap<String, usize>,
}

impl Summary {
    pub fn increment(&mut self, level: &str) {
        *self.level_counts.entry(level.to_string()).or_insert(0) += 1;
    }
}
