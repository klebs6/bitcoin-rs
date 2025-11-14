// ---------------- [ File: bitcoin-network/src/set_ip.rs ]
crate::ix!();

impl NetAddr {

    pub fn setip(&mut self, ip_in: &NetAddr) {
        trace!(target: "netaddr", src_net = ?ip_in.get_net_class(), "Setting IP from another NetAddr");
        // Size check.
        match *ip_in.net() {
            Network::NET_IPV4 => {
                assert_eq!(ip_in.addr().len(), ADDR_IPV4_SIZE, "Expected 4-byte IPv4 address");
            }
            Network::NET_IPV6 => {
                assert_eq!(ip_in.addr().len(), ADDR_IPV6_SIZE, "Expected 16-byte IPv6 address");
            }
            Network::NET_ONION => {
                assert_eq!(ip_in.addr().len(), ADDR_TORV3_SIZE, "Expected 32-byte TORv3 address");
            }
            Network::NET_I2P => {
                assert_eq!(ip_in.addr().len(), ADDR_I2P_SIZE, "Expected 32-byte I2P address");
            }
            Network::NET_CJDNS => {
                assert_eq!(ip_in.addr().len(), ADDR_CJDNS_SIZE, "Expected 16-byte CJDNS address");
            }
            Network::NET_INTERNAL => {
                assert_eq!(ip_in.addr().len(), ADDR_INTERNAL_SIZE, "Expected 10-byte INTERNAL address");
            }
            Network::NET_UNROUTABLE | Network::NET_MAX => {
                panic!("m_net is never and should not be set to NET_UNROUTABLE/NET_MAX");
            }
        }

        *self.net_mut() = *ip_in.net();
        *self.addr_mut() = PreVector::from(ip_in.addr().as_slice());
        debug!(target: "netaddr", new_net = ?self.get_net_class(), "IP set from source NetAddr");
    }
}


#[cfg(test)]
mod set_ip_clone_spec {
    use super::*;

    fn make(net: Network, bytes: &[u8]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(bytes))
            .net(net)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn set_ip_copies_bytes_and_network_for_supported_types() {
        let src_v4 = make(Network::NET_IPV4, &[100, 64, 0, 1]);
        let mut dst = NetAddr::default();
        dst.setip(&src_v4);
        assert_eq!(*dst.net(), Network::NET_IPV4);
        assert_eq!(dst.addr().as_slice(), src_v4.addr().as_slice());

        let src_onion = make(Network::NET_ONION, &[0x11; ADDR_TORV3_SIZE]);
        dst.setip(&src_onion);
        assert_eq!(*dst.net(), Network::NET_ONION);
        assert_eq!(dst.addr().as_slice(), src_onion.addr().as_slice());

        let src_i2p = make(Network::NET_I2P, &[0x22; ADDR_I2P_SIZE]);
        dst.setip(&src_i2p);
        assert_eq!(*dst.net(), Network::NET_I2P);
        assert_eq!(dst.addr().as_slice(), src_i2p.addr().as_slice());

        let src_cjdns = make(Network::NET_CJDNS, &[0xFC, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        dst.setip(&src_cjdns);
        assert_eq!(*dst.net(), Network::NET_CJDNS);
        assert_eq!(dst.addr().as_slice(), src_cjdns.addr().as_slice());

        let src_internal = make(Network::NET_INTERNAL, &[0xAA; ADDR_INTERNAL_SIZE]);
        dst.setip(&src_internal);
        assert!(dst.is_internal());
        assert_eq!(dst.addr().as_slice(), src_internal.addr().as_slice());
    }
}
