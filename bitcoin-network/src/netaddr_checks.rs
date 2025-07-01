// ---------------- [ File: bitcoin-network/src/netaddr_checks.rs ]
crate::ix!();

/// Read a big‑endian u32 from 4 bytes (assumes slice length ≥ 4).
#[inline]
fn read_be32(src: &[u8]) -> u32 {
    ((src[0] as u32) << 24)
        | ((src[1] as u32) << 16)
        | ((src[2] as u32) << 8)
        | (src[3] as u32)
}

impl NetAddr {

    /**
      | Whether this address should be relayed
      | to other peers even if we can't reach
      | it ourselves.
      */
    #[inline]
    pub fn is_relayable(&self) -> bool {
        self.is_ipv4() || self.is_ipv6() || self.is_tor() || self.isi2p()
    }

    /**
      | INADDR_ANY equivalent (`0.0.0.0` or `::`)
      */
    pub fn is_bind_any(&self) -> bool {
        if !(self.is_ipv4() || self.is_ipv6()) {
            return false;
        }
        self.addr().iter().all(|b| *b == 0)
    }
    
    /**
      | IPv4 mapped address (::FFFF:0:0/96, 0.0.0.0/0)
      */
    #[inline]
    pub fn is_ipv4(&self) -> bool {
        trace!(
            target: "netaddr",
            net = ?self.net(),
            "Checking IPv4 network classification"
        );
        *self.net() == Network::NET_IPV4
    }
    
    /**
      | IPv6 address (not mapped IPv4, not Tor)
      */
    #[inline]
    pub fn is_ipv6(&self) -> bool {
        trace!(
            target: "netaddr",
            net = ?self.net(),
            "Checking IPv6 network classification"
        );
        *self.net() == Network::NET_IPV6
    }

    /**
      | IPv6 Hurricane Electric - https://he.net
      | (2001:0470::/36)
      |
      */
    #[inline]
    pub fn is_he_net(&self) -> bool {
        self.is_ipv6() && has_prefix(self.addr(), &[0x20, 0x01, 0x04, 0x70])
    }

    /**
      | Check whether this object represents
      | a TOR address. see `NetAddr::set_special`
      */
    #[inline]
    pub fn is_tor(&self) -> bool {
        trace!(
            target: "netaddr",
            net = ?self.net(),
            "Checking Tor network classification"
        );
        *self.net() == Network::NET_ONION
    }

    /**
      | True for IPv4 127/8, IPv4 0/8 and IPv6 ::1
      */
    pub fn is_local(&self) -> bool {

        // IPv4 loopback (127.0.0.0/8 or 0.0.0.0/8)
        if self.is_ipv4() {
            let a = self.addr();
            return a[0] == 127 || a[0] == 0;
        }

        // IPv6 loopback (::1/128)
        if self.is_ipv6() {
            static LOOPBACK_V6: [u8; 16] =
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
            return **self.addr() == LOOPBACK_V6;
        }
        false
    }

    /**
      | @note
      | 
      | A valid address may or may not be publicly
      | routable on the global internet. As
      | in, the set of valid addresses is a superset
      | of the set of publicly routable addresses.
      | @see CNetAddr::IsRoutable()
      | 
      | -----------
      | @return
      | 
      | Whether or not this network address
      | is a valid address that @a could be used
      | to refer to an actual host.
      |
      */
    pub fn is_valid(&self) -> bool {
        // ::/128 (all‑zero IPv6) is invalid
        if self.is_ipv6() && self.addr().iter().all(|b| *b == 0) {
            return false;
        }

        // CJDNS must start with 0xFC
        if self.iscjdns() && self.addr()[0] != 0xFC {
            return false;
        }

        // Documentation IPv6
        if self.isrfc3849() {
            return false;
        }

        if self.is_internal() {
            return false;
        }

        if self.is_ipv4() {
            let v = read_be32(self.addr());
            const INADDR_ANY: u32 = 0x0000_0000;
            const INADDR_NONE: u32 = 0xFFFF_FFFF;
            if v == INADDR_ANY || v == INADDR_NONE {
                return false;
            }
        }

        true
    }

    /**
      | @note
      | 
      | A routable address is always valid.
      | As in, the set of routable addresses
      | is a subset of the set of valid addresses.
      | @see CNetAddr::IsValid()
      | 
      | -----------
      | @return
      | 
      | Whether or not this network address
      | is publicly routable on the global internet.
      |
      */
    pub fn is_routable(&self) -> bool {
        self.is_valid()
            && !(self.isrfc1918()
                || self.isrfc2544()
                || self.isrfc3927()
                || self.isrfc4862()
                || self.isrfc6598()
                || self.isrfc5737()
                || self.isrfc4193()
                || self.isrfc4843()
                || self.isrfc7343()
                || self.is_local()
                || self.is_internal())
    }

    /**
      | @return
      | 
      | Whether or not this is a dummy address
      | that represents a name. @see CNetAddr::SetInternal(const
      | std::string &)
      |
      */
    #[inline]
    pub fn is_internal(&self) -> bool {
        *self.net() == Network::NET_INTERNAL
    }
    
    /**
      | Check if the current object can be serialized
      | in pre-ADDRv2/BIP155 format.
      |
      */
    pub fn is_addr_v1compatible(&self) -> bool {

        if matches!(*self.net(), Network::NET_MAX | Network::NET_UNROUTABLE ) {
            assert!(false, "m_net is never and should not be set to NET_UNROUTABLE");
        }

        matches!(
            *self.net(),
            Network::NET_IPV4 | Network::NET_IPV6 | Network::NET_INTERNAL
        )
    }

    /**
      | IPv4 private networks (10.0.0.0/8,
      | 192.168.0.0/16, 172.16.0.0/12)
      |
      */
    #[inline]
    pub fn isrfc1918(&self) -> bool {
        self.is_ipv4()
            && matches!(
                (self.addr()[0], self.addr()[1]),
                (10, _) | (192, 168) | (172, 16..=31)
            )
    }
    
    /**
      | IPv4 inter-network communications
      | (198.18.0.0/15)
      |
      */
    #[inline]
    pub fn isrfc2544(&self) -> bool {
        self.is_ipv4() && self.addr()[0] == 198 && matches!(self.addr()[1], 18 | 19)
    }
    
    /**
      | IPv4 autoconfig (169.254.0.0/16)
      |
      */
    #[inline]
    pub fn isrfc3927(&self) -> bool {
        self.is_ipv4() && has_prefix(self.addr(), &[169, 254])
    }
    
    /**
      | IPv4 ISP-level NAT (100.64.0.0/10)
      |
      */
    #[inline]
    pub fn isrfc6598(&self) -> bool {
        self.is_ipv4() && self.addr()[0] == 100 && (64..=127).contains(&self.addr()[1])
    }
    
    /**
      | IPv4 documentation addresses (192.0.2.0/24,
      | 198.51.100.0/24, 203.0.113.0/24)
      |
      */
    #[inline]
    pub fn isrfc5737(&self) -> bool {
        self.is_ipv4()
            && (has_prefix(self.addr(), &[192, 0, 2])
                || has_prefix(self.addr(), &[198, 51, 100])
                || has_prefix(self.addr(), &[203, 0, 113]))
    }
    
    /**
      | IPv6 documentation address (2001:0DB8::/32)
      |
      */
    #[inline]
    pub fn isrfc3849(&self) -> bool {
        self.is_ipv6() && has_prefix(self.addr(), &[0x20, 0x01, 0x0D, 0xB8])
    }
    
    /**
      | IPv6 6to4 tunnelling (2002::/16)
      |
      */
    #[inline]
    pub fn isrfc3964(&self) -> bool {
        self.is_ipv6() && has_prefix(self.addr(), &[0x20, 0x02])
    }

    /**
      | IPv6 well-known prefix for IPv4-embedded
      | address (64:FF9B::/96)
      |
      */
    #[inline]
    pub fn isrfc6052(&self) -> bool {
        self.is_ipv6()
            && has_prefix(
                self.addr(),
                &[0x00, 0x64, 0xFF, 0x9B, 0, 0, 0, 0, 0, 0, 0, 0],
            )
    }

    /**
      | IPv6 Teredo tunnelling (2001::/32)
      |
      */
    #[inline]
    pub fn isrfc4380(&self) -> bool {
        self.is_ipv6() && has_prefix(self.addr(), &[0x20, 0x01, 0x00, 0x00])
    }
    
    /**
      | IPv6 autoconfig (FE80::/64)
      |
      */
    #[inline]
    pub fn isrfc4862(&self) -> bool {
        self.is_ipv6() && has_prefix(self.addr(), &[0xFE, 0x80, 0, 0, 0, 0, 0, 0])
    }
    
    /**
      | IPv6 unique local (FC00::/7)
      |
      */
    #[inline]
    pub fn isrfc4193(&self) -> bool {
        self.is_ipv6() && (self.addr()[0] & 0xFE) == 0xFC
    }
    
    /**
      | IPv6 IPv4-translated address (::FFFF:0:0:0/96)
      | (actually defined in RFC2765)
      |
      */
    #[inline]
    pub fn isrfc6145(&self) -> bool {
        self.is_ipv6()
            && has_prefix(
                self.addr(),
                &[0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 0x00, 0x00],
            )
    }
    
    /**
      | IPv6 ORCHID (deprecated) (2001:10::/28)
      |
      */
    #[inline]
    pub fn isrfc4843(&self) -> bool {
        self.is_ipv6()
            && has_prefix(self.addr(), &[0x20, 0x01, 0x00])
            && (self.addr()[3] & 0xF0) == 0x10
    }
    
    /**
      | IPv6 ORCHIDv2 (2001:20::/28)
      |
      */
    #[inline]
    pub fn isrfc7343(&self) -> bool {
        self.is_ipv6()
            && has_prefix(self.addr(), &[0x20, 0x01, 0x00])
            && (self.addr()[3] & 0xF0) == 0x20
    }

    /**
      | Check whether this object represents
      | an I2P address.
      |
      */
    #[inline]
    pub fn isi2p(&self) -> bool {
        trace!(
            target: "netaddr",
            net = ?self.net(),
            "Checking I2P network classification"
        );
        *self.net() == Network::NET_I2P
    }

    /**
      | Check whether this object represents
      | a CJDNS address.
      |
      */
    #[inline]
    pub fn iscjdns(&self) -> bool {
        trace!(
            target: "netaddr",
            net = ?self.net(),
            "Checking CJDNS network classification"
        );
        *self.net() == Network::NET_CJDNS
    }
}

#[cfg(test)]
mod netaddr_classification_tests {
    use super::*;

    #[traced_test]
    fn ipv4_and_ipv6_detection_works() {
        let mut addr4 = NetAddr::default();
        addr4.set_net(Network::NET_IPV4);
        assert!(addr4.is_ipv4(), "Expected NET_IPV4 to be detected as IPv4");
        assert!(!addr4.is_ipv6(), "Expected NET_IPV4 not to be detected as IPv6");

        let mut addr6 = NetAddr::default();
        addr6.set_net(Network::NET_IPV6);
        assert!(addr6.is_ipv6(), "Expected NET_IPV6 to be detected as IPv6");
        assert!(!addr6.is_ipv4(), "Expected NET_IPV6 not to be detected as IPv4");
    }

    #[traced_test]
    fn tor_i2p_cjdns_detection() {
        let mut addr_tor = NetAddr::default();
        addr_tor.set_net(Network::NET_ONION);
        assert!(addr_tor.is_tor(), "Expected NET_ONION to be detected as Tor");

        let mut addr_i2p = NetAddr::default();
        addr_i2p.set_net(Network::NET_I2P);
        assert!(addr_i2p.isi2p(), "Expected NET_I2P to be detected as I2P");

        let mut addr_cjdns = NetAddr::default();
        addr_cjdns.set_net(Network::NET_CJDNS);
        assert!(addr_cjdns.iscjdns(), "Expected NET_CJDNS to be detected as CJDNS");
    }

    fn make_ipv4(a: [u8; 4]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(a.to_vec().as_slice()))
            .net(Network::NET_IPV4)
            .scope_id(0_u32)
            .build()
            .unwrap()
    }

    fn make_ipv6(a: [u8; 16]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(a.to_vec().as_slice()))
            .net(Network::NET_IPV6)
            .scope_id(0_u32)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn relayable_logic() {
        let v4 = make_ipv4([1, 2, 3, 4]);
        assert!(v4.is_relayable());

        let tor = NetAddrBuilder::default()
            .addr(PreVector::with_capacity(ADDR_TORV3_SIZE))
            .net(Network::NET_ONION)
            .scope_id(0_u32)
            .build()
            .unwrap();

        assert!(tor.is_relayable());
    }

    #[traced_test]
    fn bind_any_ipv4_ipv6() {
        let any_v4 = make_ipv4([0, 0, 0, 0]);
        assert!(any_v4.is_bind_any());

        let any_v6 = make_ipv6([0; 16]);
        assert!(any_v6.is_bind_any());

        let non_any_v4 = make_ipv4([1, 0, 0, 0]);
        assert!(!non_any_v4.is_bind_any());
    }

    #[traced_test]
    fn he_net_detection() {
        let mut addr = make_ipv6([0; 16]);
        addr.addr_mut()[..4].copy_from_slice(&[0x20, 0x01, 0x04, 0x70]);
        assert!(addr.is_he_net());
    }

    #[traced_test]
    fn rfc1918_and_routable() {
        // 10.1.1.1 -> RFC1918
        let rfc1918 = make_ipv4([10, 1, 1, 1]);
        assert!(rfc1918.isrfc1918());
        assert!(!rfc1918.is_routable());

        // 8.8.8.8 -> public Google DNS
        let google = make_ipv4([8, 8, 8, 8]);
        assert!(!google.isrfc1918());
        assert!(google.is_routable());
    }

    #[traced_test]
    fn validity_checks() {
        // Unspecified IPv6 ::/128 is invalid
        let unspecified_v6 = make_ipv6([0; 16]);
        assert!(!unspecified_v6.is_valid());

        // Valid IPv4
        let good_v4 = make_ipv4([123, 45, 67, 89]);
        assert!(good_v4.is_valid());
    }
}
