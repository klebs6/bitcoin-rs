// ---------------- [ File: bitcoin-network/src/set_legacy_ipv6.rs ]
crate::ix!();

impl NetAddr {
    /**
      | Set from a legacy IPv6 address.
      | 
      | Legacy IPv6 address may be a normal IPv6
      | address, or another address (e.g. IPv4)
      | disguised as IPv6.
      | 
      | This encoding is used in the legacy `addr`
      | encoding.
      |
      */
    pub fn set_legacy_ipv6(&mut self, ipv6: &[u8]) {
        trace!(target: "netaddr", input_len = ipv6.len(), "Setting from legacy IPv6");

        assert_eq!(ipv6.len(), ADDR_IPV6_SIZE, "Legacy IPv6 input must be 16 bytes");

        let mut skip: usize = 0;

        if ipv6.starts_with(&IPV4_IN_IPV6_PREFIX) {
            // IPv4-in-IPv6
            *self.net_mut() = Network::NET_IPV4;
            skip = IPV4_IN_IPV6_PREFIX.len();
            debug!(target: "netaddr", "Detected IPv4-in-IPv6 mapping");
        } else if ipv6.starts_with(&TORV2_IN_IPV6_PREFIX) {
            // TORv2-in-IPv6 (unsupported). Unserialize as !IsValid(), thus ignoring them.
            // Mimic a default-constructed CNetAddr object which is !IsValid() and thus
            // will not be gossiped, but continue reading next addresses from the stream.
            *self.net_mut() = Network::NET_IPV6;
            *self.addr_mut() = PreVector::from(&[0u8; ADDR_IPV6_SIZE][..]);
            warn!(target: "netaddr", "Detected deprecated TORv2-in-IPv6; setting to invalid IPv6 (::)");
            return;
        } else if ipv6.starts_with(&INTERNAL_IN_IPV6_PREFIX) {
            // Internal-in-IPv6
            *self.net_mut() = Network::NET_INTERNAL;
            skip = INTERNAL_IN_IPV6_PREFIX.len();
            debug!(target: "netaddr", "Detected INTERNAL-in-IPv6 mapping");
        } else {
            // IPv6
            *self.net_mut() = Network::NET_IPV6;
            debug!(target: "netaddr", "Detected plain IPv6 address");
        }

        *self.addr_mut() = PreVector::from(&ipv6[skip..]);
        debug!(target: "netaddr", net = ?self.get_net_class(), addr_len = self.addr().len(), "Legacy IPv6 set complete");
    }
}

#[cfg(test)]
mod legacy_ipv6_decoder_spec {
    use super::*;

    #[traced_test]
    fn recognizes_ipv4_in_ipv6_and_internal_in_ipv6() {
        // IPv4‑in‑IPv6 (::ffff:1.2.3.4)
        let mut ipv6 = [0u8; 16];
        ipv6[..IPV4_IN_IPV6_PREFIX.len()].copy_from_slice(&IPV4_IN_IPV6_PREFIX);
        ipv6[12..].copy_from_slice(&[1,2,3,4]);

        let mut a = NetAddr::default();
        a.set_legacy_ipv6(&ipv6);
        assert!(a.is_ipv4());
        assert_eq!(a.addr().as_slice(), &[1,2,3,4]);

        // INTERNAL‑in‑IPv6
        let mut internal = [0u8; 16];
        internal[..INTERNAL_IN_IPV6_PREFIX.len()].copy_from_slice(&INTERNAL_IN_IPV6_PREFIX);
        internal[6..16].copy_from_slice(&[0xAB; 10]);

        let mut b = NetAddr::default();
        b.set_legacy_ipv6(&internal);
        assert!(b.is_internal());
        assert_eq!(b.addr().as_slice(), &[0xAB; 10]);
    }

    #[traced_test]
    fn torv2_in_ipv6_is_rejected_to_invalid_ipv6() {
        // TORv2‑in‑IPv6 (deprecated)
        let mut bogus = [0u8; 16];
        bogus[..TORV2_IN_IPV6_PREFIX.len()].copy_from_slice(&TORV2_IN_IPV6_PREFIX);

        let mut a = NetAddr::default();
        a.set_legacy_ipv6(&bogus);

        // Should be set to IPv6 with all‑zero bytes (::), which is !IsValid().
        assert!(a.is_ipv6());
        assert_eq!(a.addr().as_slice(), &[0u8; 16]);
    }
}
