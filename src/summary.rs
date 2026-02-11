use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Summary {
    pub level_counts: HashMap<String, usize>,
}
