use std::collections::HashMap;

#[derive(Default)]
pub struct Summary {
    pub level_counts: HashMap<String, usize>,
}

impl Summary {
    pub fn merge(&mut self, local: HashMap<String, usize>) {
        for (level, count) in local {
            *self.level_counts.entry(level).or_insert(0) += count;
        }
    }
}
