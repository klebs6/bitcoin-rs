crate::ix!();

/**
  | Prefix of an IPv6 address when it contains an
  | embedded IPv4 address.
  |
  | Used when (un)serializing addresses in ADDRv1
  | format (pre-BIP155).
  */
pub const IPV4_IN_IPV6_PREFIX: [u8; 12] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF
];

/**
  | Prefix of an IPv6 address when it contains an
  | embedded TORv2 address.
  |
  | Used when (un)serializing addresses in ADDRv1
  | format (pre-BIP155).
  |
  | Such dummy IPv6 addresses are guaranteed to
  | not be publicly routable as they fall under
  | RFC4193's fc00::/7 subnet allocated to
  | unique-local addresses.
  */
pub const TORV2_IN_IPV6_PREFIX: [u8; 6] = [
    0xFD, 0x87, 0xD8, 0x7E, 0xEB, 0x43];

/**
  | Prefix of an IPv6 address when it contains an
  | embedded "internal" address.
  |
  | Used when (un)serializing addresses in ADDRv1
  | format (pre-BIP155).
  |
  | The prefix comes from 0xFD
  | + SHA256("bitcoin")[0:5].
  |
  | Such dummy IPv6 addresses are guaranteed to
  | not be publicly routable as they fall under
  | RFC4193's fc00::/7 subnet allocated to
  | unique-local addresses.
  */
pub const INTERNAL_IN_IPV6_PREFIX: [u8; 6] = [
    0xFD, 0x6B, 0x88, 0xC0, 0x87, 0x24 // 0xFD + sha256("bitcoin")[0:5].
];
