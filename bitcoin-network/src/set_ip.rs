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
