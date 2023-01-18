crate::ix!();

/**
  | @note
  | 
  | When adjusting this, update rpcnet:setban's
  | help ("24h")
  |
  */
pub const DEFAULT_MISBEHAVING_BANTIME: u32 = 60 * 60 * 24; // Default 24-hour ban

/**
  | How often to dump banned addresses/subnets
  | to disk.
  |
  */
pub const DUMP_BANS_INTERVAL: Duration = Duration::minutes(15);
