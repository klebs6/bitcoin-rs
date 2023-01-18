crate::ix!();
    
/**
  | Track confirm delays up to 12 blocks
  | for short horizon
  |
  */
pub const SHORT_BLOCK_PERIODS: u32 = 12;
pub const SHORT_SCALE:         u32 = 1;

/**
  | Track confirm delays up to 48 blocks
  | for medium horizon
  |
  */
pub const MED_BLOCK_PERIODS: u32 = 24;
pub const MED_SCALE:         u32 = 2;

/**
  | Track confirm delays up to 1008 blocks
  | for long horizon
  |
  */
pub const LONG_BLOCK_PERIODS: u32 = 42;
pub const LONG_SCALE:         u32 = 24;

/**
  | Historical estimates that are older
  | than this aren't valid
  |
  */
pub const OLDEST_ESTIMATE_HISTORY: u32 = 6 * 1008;

/**
  | Decay of .962 is a half-life of 18 blocks
  | or about 3 hours
  |
  */
pub const SHORT_DECAY: f64 = 0.962;

/**
  | Decay of .9952 is a half-life of 144 blocks
  | or about 1 day
  |
  */
pub const MED_DECAY: f64 = 0.9952;

/**
  | Decay of .99931 is a half-life of 1008
  | blocks or about 1 week
  |
  */
pub const LONG_DECAY: f64 = 0.99931;

/**
  | Require greater than 60% of X feerate
  | transactions to be confirmed within
  | Y/2 blocks
  |
  */
pub const HALF_SUCCESS_PCT: f64 = 0.6;

/**
  | Require greater than 85% of X feerate
  | transactions to be confirmed within
  | Y blocks
  |
  */
pub const SUCCESS_PCT: f64 = 0.85;

/**
  | Require greater than 95% of X feerate
  | transactions to be confirmed within
  | 2 * Y blocks
  |
  */
pub const DOUBLE_SUCCESS_PCT: f64 = 0.95;

/**
  | Require an avg of 0.1 tx in the combined
  | feerate bucket per block to have stat
  | significance
  |
  */
pub const SUFFICIENT_FEETXS: f64 = 0.1;

/**
  | Require an avg of 0.5 tx when using short
  | decay since there are fewer blocks considered
  |
  */
pub const SUFFICIENT_TXS_SHORT: f64 = 0.5;

/**
  | Minimum and Maximum values for tracking
  | feerates
  | 
  | The MIN_BUCKET_FEERATE should just
  | be set to the lowest reasonable feerate
  | we might ever want to track. Historically
  | this has been 1000 since it was inheriting
  | DEFAULT_MIN_RELAY_TX_FEE and changing
  | it is disruptive as it invalidates old
  | estimates files. So leave it at 1000
  | unless it becomes necessary to lower
  | it, and then lower it substantially.
  |
  */
pub const MIN_BUCKET_FEERATE: f64 = 1000.0;
pub const MAX_BUCKET_FEERATE: f64 = 1.0e7;

/**
  | Spacing of FeeRate buckets
  | 
  | We have to lump transactions into buckets
  | based on feerate, but we want to be able
  | to give accurate estimates over a large
  | range of potential feerates
  | 
  | Therefore it makes sense to exponentially
  | space the buckets
  |
  */
pub const FEE_SPACING: f64 = 1.05;

