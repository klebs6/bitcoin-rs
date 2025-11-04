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
                let mut s = encode_base32(self.addr().as_slice(), None);
                s.push_str(".b32.i2p");
                s
            }
            Network::NET_CJDNS => {
                assert_eq!(self.addr().len(), ADDR_CJDNS_SIZE, "CJDNS address must be 16 bytes");
                ipv6_to_string(self.addr().as_slice(), 0)
            }
            Network::NET_INTERNAL => {
                assert_eq!(self.addr().len(), ADDR_INTERNAL_SIZE, "INTERNAL address must be 10 bytes");
                let mut s = encode_base32(self.addr().as_slice(), None);
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
