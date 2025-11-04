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
    pub fn set_legacy_ipv6(&mut self, ipv6: &[u8])  {

        trace!(target: "netaddr", input_len = ipv6.len(), "Setting from legacy IPv6");

        assert_eq!(ipv6.len(), ADDR_IPV6_SIZE, "Legacy IPv6 input must be 16 bytes");
        
        let mut skip: usize = 0;

        if has_prefix(ipv6, &IPV4_IN_IPV6_PREFIX) {
            // IPv4-in-IPv6
            *self.net_mut() = Network::NET_IPV4;
            skip = IPV4_IN_IPV6_PREFIX.len();
            debug!(target: "netaddr", "Detected IPv4-in-IPv6 mapping");
        } else if has_prefix(ipv6, &TORV2_IN_IPV6_PREFIX) {
            // TORv2-in-IPv6 (unsupported). Unserialize as !IsValid(), thus ignoring them.
            // Mimic a default-constructed CNetAddr object which is !IsValid() and thus
            // will not be gossiped, but continue reading next addresses from the stream.
            *self.net_mut() = Network::NET_IPV6;
            *self.addr_mut() = PreVector::from(&[0u8; ADDR_IPV6_SIZE][..]);
            warn!(target: "netaddr", "Detected deprecated TORv2-in-IPv6; setting to invalid IPv6 (::)");
            return;
        } else if has_prefix(ipv6, &INTERNAL_IN_IPV6_PREFIX) {
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
