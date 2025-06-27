crate::ix!();

/**
  | Size of IPv4 address (in bytes).
  |
  */
pub const ADDR_IPV4_SIZE: usize = 4;

/**
  | Size of IPv6 address (in bytes).
  |
  */
pub const ADDR_IPV6_SIZE: usize = 16;

/**
  | Size of TORv3 address (in bytes). This
  | is the length of just the address as used
  | in BIP155, without the checksum and
  | the version byte.
  |
  */
pub const ADDR_TORV3_SIZE: usize = 32;

/**
  | Size of I2P address (in bytes).
  |
  */
pub const ADDR_I2P_SIZE: usize = 32;

/**
  | Size of CJDNS address (in bytes).
  |
  */
pub const ADDR_CJDNS_SIZE: usize = 16;

/**
  | Size of "internal" (NET_INTERNAL)
  | address (in bytes).
  |
  */
pub const ADDR_INTERNAL_SIZE: usize = 10;
