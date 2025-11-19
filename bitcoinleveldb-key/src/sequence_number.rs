// ---------------- [ File: bitcoinleveldb-key/src/sequence_number.rs ]
crate::ix!();

pub type SequenceNumber = u64;

/**
  | We leave eight bits empty at the bottom
  | so a type and sequence# can be packed
  | together into 64-bits.
  |
  */
pub const MAX_SEQUENCE_NUMBER: SequenceNumber = ((0x1 << 56) - 1);
