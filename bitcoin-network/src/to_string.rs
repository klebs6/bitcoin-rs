// ---------------- [ File: bitcoin-network/src/to_string.rs ]
crate::ix!();

impl NetAddr {

    pub fn to_stringip(&self) -> String {
        trace!(target: "netaddr", net = ?self.get_net_class(), "Formatting address as string (IP-style)");
        match *self.net() {
            Network::NET_IPV4 => {
                assert_eq!(self.addr().len(), ADDR_IPV4_SIZE, "IPv4 address must be 4 bytes");
                ipv4_to_string(self.addr().as_slice())
            }
            Network::NET_IPV6 => {
                assert_eq!(self.addr().len(), ADDR_IPV6_SIZE, "IPv6 address must be 16 bytes");
                ipv6_to_string(self.addr().as_slice(), *self.scope_id())
            }
            Network::NET_ONION => {
                assert_eq!(self.addr().len(), ADDR_TORV3_SIZE, "TORv3 address must be 32 bytes");
                onion_to_string(self.addr().as_slice())
            }
            Network::NET_I2P => {
                assert_eq!(self.addr().len(), ADDR_I2P_SIZE, "I2P address must be 32 bytes");
                // Do not pad with '=' (encoding helper uses no padding when second arg is None).
                let mut s = encode_base32(self.addr().as_slice(), Some(false));
                s.push_str(".b32.i2p");
                s
            }
            Network::NET_CJDNS => {
                assert_eq!(self.addr().len(), ADDR_CJDNS_SIZE, "CJDNS address must be 16 bytes");
                ipv6_to_string(self.addr().as_slice(), 0)
            }
            Network::NET_INTERNAL => {
                assert_eq!(self.addr().len(), ADDR_INTERNAL_SIZE, "INTERNAL address must be 10 bytes");
                let mut s = encode_base32(self.addr().as_slice(), Some(false));
                s.push_str(".internal");
                s
            }
            Network::NET_UNROUTABLE | Network::NET_MAX => {
                panic!("m_net is never and should not be set to NET_UNROUTABLE/NET_MAX");
            }
        }
    }
    
    pub fn to_string(&self) -> String {
        trace!(target: "netaddr", "Converting NetAddr to string");
        self.to_stringip()
    }
}

#[cfg(test)]
mod display_formatting_spec {
    use super::*;

    #[traced_test]
    fn to_string_ip_for_all_networks() {
        // IPv4
        let v4 = NetAddrBuilder::default()
            .addr(PreVector::from(&[192u8,168,1,10][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_eq!(v4.to_string(), "192.168.1.10");

        // IPv6
        let mut bytes = [0u8; 16];
        bytes[0] = 0xFE;
        bytes[1] = 0x80;
        bytes[15] = 0x34;
        let v6 = NetAddrBuilder::default()
            .addr(PreVector::from(&bytes[..]))
            .net(Network::NET_IPV6)
            .scope_id(7u32)
            .build()
            .unwrap();
        assert!(v6.to_string().starts_with("fe80::34%7"));

        // TORv3
        let pk = [0x77u8; ADDR_TORV3_SIZE];
        let onion = NetAddrBuilder::default()
            .addr(PreVector::from(&pk[..]))
            .net(Network::NET_ONION)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_eq!(onion.to_string(), onion_to_string(&pk));

        // I2P
        let raw = [0x11u8; ADDR_I2P_SIZE];
        let i2p = NetAddrBuilder::default()
            .addr(PreVector::from(&raw[..]))
            .net(Network::NET_I2P)
            .scope_id(0u32)
            .build()
            .unwrap();
        let s = i2p.to_string();
        assert!(s.ends_with(".b32.i2p"));
        let base = &s[..s.len() - ".b32.i2p".len()];
        let expected_b32 = encode_base32(&raw, Some(false));
        info!(base_len = base.len(), expected_len = expected_b32.len(), "Comparing I2P base32 text");
        assert_eq!(base, expected_b32);

        // CJDNS
        let mut c = [0u8; 16];
        c[0] = 0xFC;
        let cjdns = NetAddrBuilder::default()
            .addr(PreVector::from(&c[..]))
            .net(Network::NET_CJDNS)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert!(cjdns.to_string().starts_with("fc00:"));

        // INTERNAL
        let mut internal = NetAddr::default();
        assert!(internal.set_internal("seed.x.example"));
        let si = internal.to_string();
        assert!(si.ends_with(".internal"));
        let base = &si[..si.len() - ".internal".len()];
        let expected_internal_b32 = encode_base32(internal.addr().as_slice(), Some(false));
        debug!(got = base, expected = expected_internal_b32.as_str(), "Comparing INTERNAL base32 text");
        assert_eq!(base, expected_internal_b32);
    }
}
