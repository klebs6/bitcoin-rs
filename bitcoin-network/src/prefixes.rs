// ---------------- [ File: bitcoin-network/src/prefixes.rs ]
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

#[cfg(test)]
mod embedded_prefix_values_spec {
    use super::*;

    #[traced_test]
    fn ipv4_in_ipv6_prefix_value_and_length() {
        assert_eq!(IPV4_IN_IPV6_PREFIX.len(), 12);
        assert_eq!(
                IPV4_IN_IPV6_PREFIX,
                [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF]
                );
        info!("IPv4‑in‑IPv6 prefix matches expected RFC mapping");
    }

    #[traced_test]
    fn torv2_and_internal_prefixes_are_unique_local() {
        // Both should fall under fc00::/7
        debug!("Ensuring TORv2/INTERNAL prefixes are within fc00::/7");
        assert_eq!(TORV2_IN_IPV6_PREFIX[0] & 0xFE, 0xFC);
        assert_eq!(INTERNAL_IN_IPV6_PREFIX[0] & 0xFE, 0xFC);
        assert_eq!(TORV2_IN_IPV6_PREFIX.len(), 6);
        assert_eq!(INTERNAL_IN_IPV6_PREFIX.len(), 6);
        debug!("TORv2 and INTERNAL prefixes are within ULA space (fc00::/7)");
    }

    #[traced_test]
    fn prefixes_are_distinct_and_non_overlapping() {
        // Ensure we don't accidentally collide on leading segments
        assert_ne!(&TORV2_IN_IPV6_PREFIX[..], &INTERNAL_IN_IPV6_PREFIX[..]);
        assert_ne!(
                &IPV4_IN_IPV6_PREFIX[..6],
                &TORV2_IN_IPV6_PREFIX[..]
                );
        assert_ne!(
                &IPV4_IN_IPV6_PREFIX[..6],
                &INTERNAL_IN_IPV6_PREFIX[..]
                );
        info!("All embedded prefixes are distinct");
    }

    #[traced_test]
    fn ipv4_in_ipv6_prefix_shape() {
        info!(len = IPV4_IN_IPV6_PREFIX.len(), "Checking IPv4‑in‑IPv6 prefix shape");
        assert_eq!(IPV4_IN_IPV6_PREFIX.len(), 12);
        assert!(IPV4_IN_IPV6_PREFIX[..10].iter().all(|&b| b == 0));
        assert_eq!(IPV4_IN_IPV6_PREFIX[10], 0xFF);
        assert_eq!(IPV4_IN_IPV6_PREFIX[11], 0xFF);
    }
}
