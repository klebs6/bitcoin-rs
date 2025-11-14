// ---------------- [ File: bitcoin-network/src/sizes.rs ]
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

#[cfg(test)]
mod bip155_wrong_size_panics_test {
    use super::*;
    use std::panic;

    /// Founding id with wrong length must panic (Core‑compatible).
    #[traced_test]
    fn founding_id_wrong_length_panics() {
        let mut addr = NetAddr::default();
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            // IPv4 but with invalid length 15 should trigger panic.
            let _ = addr.set_net_from_bip155network(BIP155Network::IPV4 as u8, 15);
        }));
        assert!(result.is_err(), "expected panic on wrong IPv4 length");
    }
}
