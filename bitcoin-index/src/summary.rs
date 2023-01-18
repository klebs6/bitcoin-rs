crate::ix!();

pub struct IndexSummary {
    name:              String,
    synced:            bool,
    best_block_height: i32,
}

impl IndexSummary {

    pub fn new(name: &str) -> Self {
        Self {
            name:              name.to_string(),
            synced:            false,
            best_block_height: 0,
        }
    }
}
