// ---------------- [ File: bitcoin-network/src/get_group.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Get the canonical identifier of our
      | network group
      | 
      | The groups are assigned in a way where
      | it should be costly for an attacker to
      | obtain addresses with many different
      | group identifiers, even if it is cheap
      | to obtain addresses with the same identifier.
      | 
      | -----------
      | @note
      | 
      | No two connections will be attempted
      | to addresses with the same network group.
      |
      */
    pub fn get_group(&self, asmap: &Vec<bool>) -> Vec<u8> {

        trace!(
            target: "netaddr", 
            net = ?self.get_net_class(), 
            "Computing address group identifier"
        );

        let mut vch_ret: Vec<u8> = Vec::with_capacity(1 + ADDR_IPV6_SIZE);

        let net_class: u8 = self.get_net_class() as u8;
        
        // If non-empty asmap is supplied and the address is IPv4/IPv6,
        // return ASN to be used for bucketing.
        let asn: u32 = self.get_mappedas(asmap);

        if asn != 0 {
            // Either asmap was empty, or address has non-asmappable net class (e.g. TOR).

            // IPv4 and IPv6 with same ASN should be in the same bucket
            vch_ret.push(Network::NET_IPV6 as u8);
            for i in 0..4 {
                vch_ret.push(((asn >> (8 * i)) & 0xFF) as u8);
            }
            debug!(target: "netaddr", asn, group = ?vch_ret, "AS-mapped grouping applied");
            return vch_ret;
        }

        vch_ret.push(net_class);
        let mut n_bits: usize = 0;

        if self.is_local() {
            // all local addresses belong to the same group
            debug!(target: "netaddr", "Local address → single-group by net class");
        } else if self.is_internal() {
            // all internal-usage addresses get their own group
            n_bits = ADDR_INTERNAL_SIZE * 8;
            debug!(target: "netaddr", bits = n_bits, "Internal address grouping");
        } else if !self.is_routable() {
            // all other unroutable addresses belong to the same group
            debug!(target: "netaddr", "Unroutable address → single-group by net class");
        } else if self.has_linked_ipv4() {
            // IPv4 addresses (and mapped IPv4 addresses) use /16 groups
            let ipv4 = self.get_linked_ipv4();
            vch_ret.push(((ipv4 >> 24) & 0xFF) as u8);
            vch_ret.push(((ipv4 >> 16) & 0xFF) as u8);
            debug!(target: "netaddr", octet1 = (ipv4 >> 24) & 0xFF, octet2 = (ipv4 >> 16) & 0xFF, "IPv4-/16 grouping");
            return vch_ret;
        } else if self.is_tor() || self.isi2p() || self.iscjdns() {
            n_bits = 4;
            debug!(target: "netaddr", bits = n_bits, "Overlay network grouping (/4)");
        } else if self.is_he_net() {
            // for he.net, use /36 groups
            n_bits = 36;
            debug!(target: "netaddr", bits = n_bits, "HE.NET IPv6 grouping (/36)");
        } else {
            // for the rest of the IPv6 network, use /32 groups
            n_bits = 32;
            debug!(target: "netaddr", bits = n_bits, "Generic IPv6 grouping (/32)");
        }

        // Push our address bytes according to n_bits.
        let num_bytes = n_bits / 8;
        let addr_slice = self.addr().as_slice();

        if num_bytes > 0 {
            let upto = num_bytes.min(addr_slice.len());
            vch_ret.extend_from_slice(&addr_slice[..upto]);
        }

        // ...for the last byte, push nBits and for the rest of the byte push 1's
        let rem_bits = n_bits % 8;
        if rem_bits > 0 {
            assert!(num_bytes < addr_slice.len(), "Address length shorter than expected for grouping");
            let last = addr_slice[num_bytes];
            // Safe because rem_bits ∈ 1..=7 here
            let mask: u8 = ((1u16 << (8 - rem_bits)) - 1) as u8;
            vch_ret.push(last | mask);
        }

        debug!(target: "netaddr", group = ?vch_ret, "Computed group identifier");
        vch_ret
    }
}

#[cfg(test)]
mod grouping_rules_spec {
    use super::*;

    fn v4(a: [u8; 4]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&a[..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    fn v6(bytes: [u8; 16]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&bytes[..]))
            .net(Network::NET_IPV6)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    fn onion_with(bytes: [u8; ADDR_TORV3_SIZE]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&bytes[..]))
            .net(Network::NET_ONION)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn local_and_unroutable_addresses_share_single_group() {
        let mut loopback_v4 = v4([127, 0, 0, 1]);
        let g1 = loopback_v4.get_group(&vec![]);
        assert_eq!(g1.len(), 1);
        assert_eq!(g1[0], Network::NET_UNROUTABLE as u8);

        let priv_v4 = v4([10, 0, 0, 1]);
        let g2 = priv_v4.get_group(&vec![]);
        assert_eq!(g2.len(), 1);
        assert_eq!(g2[0], Network::NET_UNROUTABLE as u8);
    }

    #[traced_test]
    fn internal_addresses_include_full_payload() {
        let mut a = NetAddr::default();
        assert!(a.set_internal("seed.bitcoin.example"));
        let g = a.get_group(&vec![]);
        assert_eq!(g[0], Network::NET_INTERNAL as u8);
        assert_eq!(g.len(), 1 + ADDR_INTERNAL_SIZE);
    }

    #[traced_test]
    fn ipv4_grouping_is_slash16() {
        let g = v4([8, 8, 8, 8]).get_group(&vec![]);
        assert_eq!(g, vec![Network::NET_IPV4 as u8, 8, 8]);
    }

    #[traced_test]
    fn overlay_networks_bucket_on_first_nibble() {
        let addr_bytes = [0xABu8; ADDR_TORV3_SIZE];
        let g = onion_with(addr_bytes).get_group(&vec![]);
        assert_eq!(g[0], Network::NET_ONION as u8);
        assert_eq!(g.len(), 2);
        assert_eq!(g[1] & 0x0F, 0x0F, "low nibble must be all ones");
    }

    #[traced_test]
    fn he_net_ipv6_grouping_is_slash36() {
        let mut bytes = [0u8; 16];
        bytes[..4].copy_from_slice(&[0x20, 0x01, 0x04, 0x70]); // 2001:0470::/36
        bytes[4] = 0xAA; // arbitrary
        let g = v6(bytes).get_group(&vec![]);
        assert_eq!(g[0], Network::NET_IPV6 as u8);
        assert_eq!(&g[1..5], &[0x20, 0x01, 0x04, 0x70]);
        assert_eq!(g[5] & 0x0F, 0x0F, "low nibble must be all ones");
    }

    #[traced_test]
    fn generic_ipv6_grouping_is_slash32() {
        let mut bytes = [0u8; 16];
        bytes[..4].copy_from_slice(&[0x20, 0x01, 0xDB, 0x8A]); // some routable /32 (not special)
        let g = v6(bytes).get_group(&vec![]);
        assert_eq!(g[0], Network::NET_IPV6 as u8);
        assert_eq!(g.len(), 1 + 4);
        assert_eq!(&g[1..], &[0x20, 0x01, 0xDB, 0x8A]);
    }
}
