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
        
        todo!();
        /*
            assert(ipv6.size() == ADDR_IPV6_SIZE);

        size_t skip{0};

        if (HasPrefix(ipv6, IPV4_IN_IPV6_PREFIX)) {
            // IPv4-in-IPv6
            m_net = NET_IPV4;
            skip = sizeof(IPV4_IN_IPV6_PREFIX);
        } else if (HasPrefix(ipv6, TORV2_IN_IPV6_PREFIX)) {
            // TORv2-in-IPv6 (unsupported). Unserialize as !IsValid(), thus ignoring them.
            // Mimic a default-constructed CNetAddr object which is !IsValid() and thus
            // will not be gossiped, but continue reading next addresses from the stream.
            m_net = NET_IPV6;
            m_addr.assign(ADDR_IPV6_SIZE, 0x0);
            return;
        } else if (HasPrefix(ipv6, INTERNAL_IN_IPV6_PREFIX)) {
            // Internal-in-IPv6
            m_net = NET_INTERNAL;
            skip = sizeof(INTERNAL_IN_IPV6_PREFIX);
        } else {
            // IPv6
            m_net = NET_IPV6;
        }

        m_addr.assign(ipv6.begin() + skip, ipv6.end());
        */
    }
}
