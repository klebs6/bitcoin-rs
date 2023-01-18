crate::ix!();

/**
  | Block and header tip information
  |
  */
pub struct BlockAndHeaderTipInfo
{
    block_height:          i32,
    block_time:            i64,
    header_height:         i32,
    header_time:           i64,
    verification_progress: f64,
}

/**
  | Block tip (could be a header or not, depends
  | on the subscribed signal).
  |
  */
pub struct BlockTip {
    block_height: i32,
    block_time:   i64,
    block_hash:   u256,
}

