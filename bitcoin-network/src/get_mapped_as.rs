// ---------------- [ File: bitcoin-network/src/get_mapped_as.rs ]
crate::ix!();

impl NetAddr {

    /**
      | The AS on the BGP path to the node we use
      | to diversify peers in AddrMan bucketing
      | based on the AS infrastructure.
      |
      | The ip->AS mapping depends on how asmap is
      | constructed.
      */
    pub fn get_mappedas(&self, asmap: &Vec<bool>) -> u32 {

        trace!(
            target: "netaddr", 
            asmap_len = asmap.len(), 
            net = ?self.get_net_class(), 
            "Deriving mapped AS number"
        );

        let net_class = self.get_net_class();

        if asmap.is_empty() || (net_class != Network::NET_IPV4 && net_class != Network::NET_IPV6) {
            // Indicates not found, safe because AS0 is reserved per RFC7607.
            debug!(target: "netaddr", "No asmap or non-IPv4/IPv6 address → returning AS0");
            return 0;
        }

        // Build 128-bit view of the IP per Bitcoin Core's Interpret() convention.
        let mut ip_bits = vec![false; 128];

        if self.has_linked_ipv4() {
            // Treat as if it was just an IPv4 address (IPV4_IN_IPV6_PREFIX + IPv4 bits)
            for (byte_i, b) in IPV4_IN_IPV6_PREFIX.iter().enumerate() {
                for bit_i in 0..8 {
                    ip_bits[byte_i * 8 + bit_i] = ((b >> (7 - bit_i)) & 1) == 1;
                }
            }
            let ipv4 = self.get_linked_ipv4();
            for i in 0..32 {
                ip_bits[96 + i] = ((ipv4 >> (31 - i)) & 1) == 1;
            }
        } else {
            // Use all 128 bits of the IPv6 address otherwise
            assert!(self.is_ipv6(), "Expected IPv6 address when no linked IPv4 is present");
            for byte_i in 0..16 {
                let cur = self.addr().as_slice()[byte_i];
                for bit_i in 0..8 {
                    ip_bits[byte_i * 8 + bit_i] = ((cur >> (7 - bit_i)) & 1) == 1;
                }
            }
        }

        let mapped_as: u32 = interpret(asmap, ip_bits);
        mapped_as
    }
}
