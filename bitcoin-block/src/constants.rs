crate::ix!();

/**
  | Maximum amount of time that a block timestamp
  | is allowed to exceed the current network-adjusted
  | time before the block will be accepted.
  |
  */
pub const MAX_FUTURE_BLOCK_TIME: i64 = 2 * 60 * 60;

/**
  | Timestamp window used as a grace period
  | by code that compares external timestamps
  | (such as timestamps passed to RPCs,
  | or wallet key creation times) to block
  | timestamps. This should be set at least
  | as high as
  | 
  | MAX_FUTURE_BLOCK_TIME.
  |
  */
pub const TIMESTAMP_WINDOW: i64 = MAX_FUTURE_BLOCK_TIME;

/**
  | Maximum gap between node time and block
  | time used for the "Catching up..." mode
  | in GUI.
  | 
  | Ref: https://github.com/bitcoin/bitcoin/pull/1026
  |
  */
pub const MAX_BLOCK_TIME_GAP: i64 = 90 * 60;
