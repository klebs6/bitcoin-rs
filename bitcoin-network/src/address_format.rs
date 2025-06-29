// ---------------- [ File: bitcoin-network/src/address_format.rs ]
crate::ix!();

/**
  | A flag that is ORed into the protocol
  | version to designate that addresses
  | should be serialized in (unserialized
  | from) v2 format (BIP155).
  | 
  | Make sure that this does not collide
  | with any of the values in `version.h`
  | or with `SERIALIZE_TRANSACTION_NO_WITNESS`.
  |
  */
pub const ADDRV2_FORMAT: i32 = 0x20000000;
