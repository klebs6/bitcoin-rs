// ---------------- [ File: bitcoin-network/src/sam3_1.rs ]
crate::ix!();

/**
  | SAM 3.1 and earlier do not support specifying
  | ports and force the port to 0.
  |
  */
pub const I2P_SAM31_PORT: u16 = 0;
