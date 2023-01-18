crate::ix!();

#[derive(PartialEq)]
pub struct TxStatsInfo {

    block_height: u32,
    bucket_index: u32,
}

impl Default for TxStatsInfo {
    
    fn default() -> Self {
        todo!();
        /*
        : block_height(0),
        : bucket_index(0),

        
        */
    }
}
