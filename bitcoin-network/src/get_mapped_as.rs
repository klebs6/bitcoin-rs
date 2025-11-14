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

        // Use the asmap interpreter from bitcoin-asmap.
        let mapped_as: u32 = bitcoin_asmap::interpret(asmap.as_slice(), &ip_bits);
        debug!(target: "netaddr", mapped_as, "Mapped AS computed");
        mapped_as
    }
}


#[cfg(test)]
mod mapped_as_observation_spec {
    use super::*;

    #[traced_test]
    fn empty_asmap_or_non_ip_returns_zero() {
        // IPv4 with empty asmap -> 0
        let v4 = NetAddrBuilder::default()
            .addr(PreVector::from(&[8u8, 8, 8, 8][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_eq!(v4.get_mappedas(&vec![]), 0);

        // Non‑IPv4/IPv6 with non‑empty asmap still returns 0
        let onion = NetAddrBuilder::default()
            .addr(PreVector::from(&[0x42u8; ADDR_TORV3_SIZE][..]))
            .net(Network::NET_ONION)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_eq!(onion.get_mappedas(&vec![true, false, true]), 0);
    }
}
