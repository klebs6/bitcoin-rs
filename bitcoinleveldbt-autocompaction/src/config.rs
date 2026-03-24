// ---------------- [ File: bitcoinleveldbt-autocompaction/src/config.rs ]
crate::ix!();

pub const VALUE_SIZE: i32 = 200 * 1024;
pub const TOTAL_SIZE: i32 = 100 * 1024 * 1024;
pub const COUNT:      i32 = TOTAL_SIZE / VALUE_SIZE;
