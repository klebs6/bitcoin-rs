// ---------------- [ File: bitcoin-network/src/get_reachability_from.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Calculates a metric for how reachable
      | (*this) is from a given partner
      |
      */
    pub fn get_reachability_from(&self, paddr_partner: *const NetAddr) -> i32 {

        trace!(
            target: "netaddr", 
            ours_net = ?self.get_net_class(), 
            "Computing reachability score from peer"
        );

        // Reachability categories (kept as integers to mirror the original control flow)
        const REACH_UNREACHABLE: i32 = 0;
        const REACH_DEFAULT:     i32 = 1;
        const REACH_TEREDO:      i32 = 2;
        const REACH_IPV6_WEAK:   i32 = 3;
        const REACH_IPV4:        i32 = 4;
        const REACH_IPV6_STRONG: i32 = 5;
        const REACH_PRIVATE:     i32 = 6;

        if !self.is_routable() || self.is_internal() {
            debug!(target: "netaddr", "Our address is not routable or is internal → unreachable");
            return REACH_UNREACHABLE;
        }

        let our_net  = get_ext_network(Some(self));

        let their_net = unsafe {
            if paddr_partner.is_null() {
                get_ext_network(None)
            } else {
                get_ext_network(Some(&*paddr_partner))
            }
        };

        let f_tunnel = self.isrfc3964() || self.isrfc6052() || self.isrfc6145();

        debug!(
            target: "netaddr", 
            our_net, 
            their_net, 
            f_tunnel, 
            "Extended networks and tunnel status"
        );

        match their_net {
            x if x == Network::NET_IPV4 as i32 => {
                match our_net {
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4,
                    _ => REACH_DEFAULT,
                }
            }
            x if x == Network::NET_IPV6 as i32 => {
                match our_net {
                    y if y == NET_TEREDO => REACH_TEREDO,
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4,
                    y if y == Network::NET_IPV6 as i32 => {
                        if f_tunnel { REACH_IPV6_WEAK } else { REACH_IPV6_STRONG }
                    }
                    _ => REACH_DEFAULT,
                }
            }
            x if x == Network::NET_ONION as i32 => {
                match our_net {
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4, // Tor users can connect to IPv4 as well
                    y if y == Network::NET_ONION as i32 => REACH_PRIVATE,
                    _ => REACH_DEFAULT,
                }
            }
            x if x == Network::NET_I2P as i32 => {
                match our_net {
                    y if y == Network::NET_I2P as i32 => REACH_PRIVATE,
                    _ => REACH_DEFAULT,
                }
            }
            x if x == NET_TEREDO => {
                match our_net {
                    y if y == NET_TEREDO => REACH_TEREDO,
                    y if y == Network::NET_IPV6 as i32 => REACH_IPV6_WEAK,
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4,
                    _ => REACH_DEFAULT,
                }
            }
            _ => {
                // NET_UNKNOWN or NET_UNROUTABLE or any other
                match our_net {
                    y if y == NET_TEREDO => REACH_TEREDO,
                    y if y == Network::NET_IPV6 as i32 => REACH_IPV6_WEAK,
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4,
                    y if y == Network::NET_ONION as i32 => REACH_PRIVATE, // either from Tor, or don't care about our address
                    _ => REACH_DEFAULT,
                }
            }
        }
    }
}

#[cfg(test)]
mod reachability_scoring_spec {
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

    fn v6_generic() -> NetAddr {
        // 2001:db8a:: (not special, not tunneled)
        let mut b = [0u8; 16];
        b[..4].copy_from_slice(&[0x20, 0x01, 0xDB, 0x8A]);
        v6(b)
    }

    fn v6_6to4(ipv4: [u8; 4]) -> NetAddr {
        // 6to4: 2002:vvvv:... where vvvv are IPv4 octets
        let mut b = [0u8; 16];
        b[0] = 0x20;
        b[1] = 0x02;
        b[2..6].copy_from_slice(&ipv4);
        v6(b)
    }

    fn v6_rfc6052(ipv4: [u8; 4]) -> NetAddr {
        // 64:ff9b::/96 + IPv4
        let mut b = [0u8; 16];
        b[..12].copy_from_slice(&[0x00, 0x64, 0xFF, 0x9B, 0, 0, 0, 0, 0, 0, 0, 0]);
        b[12..].copy_from_slice(&ipv4);
        v6(b)
    }

    fn teredo_stub() -> NetAddr {
        // RFC4380 Teredo: 2001::/32
        let mut b = [0u8; 16];
        b[..4].copy_from_slice(&[0x20, 0x01, 0x00, 0x00]);
        v6(b)
    }

    fn onion_pk(byte: u8) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&[byte; ADDR_TORV3_SIZE][..]))
            .net(Network::NET_ONION)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn unreachable_for_internal_and_unroutable() {
        let mut internal = NetAddr::default();
        assert!(internal.set_internal("seed.reach.example"));
        let partner = v4([8, 8, 8, 8]);

        let score_internal = internal.get_reachability_from(&partner as *const NetAddr);
        info!(score_internal, "Internal address must be unreachable");
        assert_eq!(score_internal, 0);

        // RFC1918 → unroutable
        let unroutable = v4([10, 0, 0, 1]);
        let score_unrout = unroutable.get_reachability_from(&partner as *const NetAddr);
        info!(score_unrout, "RFC1918 address must be unreachable");
        assert_eq!(score_unrout, 0);
    }

    #[traced_test]
    fn ipv4_to_ipv4_yields_ipv4_score() {
        let a = v4([1, 2, 3, 4]);
        let b = v4([8, 8, 8, 8]);
        let score = a.get_reachability_from(&b as *const NetAddr);
        info!(score, "IPv4 ↔ IPv4 score");
        assert_eq!(score, 4);
    }

    #[traced_test]
    fn ipv6_strong_between_native_ipv6() {
        let a = v6_generic();
        let b = v6_generic();
        let score = a.get_reachability_from(&b as *const NetAddr);
        info!(score, "Native IPv6 ↔ Native IPv6 (strong)");
        assert_eq!(score, 5);
    }

    #[traced_test]
    fn ipv6_weak_when_tunnelled() {
        let a = v6_6to4([1, 2, 3, 4]);
        let b = v6_generic();
        let score = a.get_reachability_from(&b as *const NetAddr);
        info!(score, "6to4 IPv6 → IPv6 (weak)");
        assert_eq!(score, 3);

        let c = v6_rfc6052([8, 8, 8, 8]);
        let score2 = c.get_reachability_from(&b as *const NetAddr);
        info!(score2, "RFC6052 IPv6 → IPv6 (weak)");
        assert_eq!(score2, 3);
    }

    #[traced_test]
    fn tor_private_between_onion_peers() {
        let a = onion_pk(0x11);
        let b = onion_pk(0x22);
        let score = a.get_reachability_from(&b as *const NetAddr);
        info!(score, "Tor ↔ Tor (private)");
        assert_eq!(score, 6);
    }

    #[traced_test]
    fn teredo_handling_covers_scenarios() {
        let teredo_peer = teredo_stub();

        let a = teredo_stub();
        let s1 = a.get_reachability_from(&teredo_peer as *const NetAddr);
        info!(s1, "Teredo ↔ Teredo");
        assert_eq!(s1, 2);

        let native_v6 = v6_generic();
        let s2 = native_v6.get_reachability_from(&teredo_peer as *const NetAddr);
        info!(s2, "Native IPv6 from Teredo peer");
        assert_eq!(s2, 3);

        let ipv4 = v4([9, 9, 9, 9]);
        let s3 = ipv4.get_reachability_from(&teredo_peer as *const NetAddr);
        info!(s3, "IPv4 from Teredo peer");
        assert_eq!(s3, 4);
    }

    #[traced_test]
    fn null_partner_maps_to_unknown_branch() {
        let a = onion_pk(0x33);
        let s = a.get_reachability_from(core::ptr::null());
        info!(s, "Null partner → NET_UNKNOWN branch; Tor result should be private");
        assert_eq!(s, 6);
    }

    #[traced_test]
    fn ipv6_from_ipv4_peer_is_default() {
        let a = v6_generic();
        let b = v4([8, 8, 8, 8]);
        let score = a.get_reachability_from(&b as *const NetAddr);
        info!(score, "IPv6 from IPv4 peer → default");
        assert_eq!(score, 1);
    }
}

#[cfg(test)]
mod reachability_matrix_spec {
    use super::*;

    const REACH_UNREACHABLE: i32 = 0;
    const REACH_DEFAULT:     i32 = 1;
    const REACH_TEREDO:      i32 = 2;
    const REACH_IPV6_WEAK:   i32 = 3;
    const REACH_IPV4:        i32 = 4;
    const REACH_IPV6_STRONG: i32 = 5;
    const REACH_PRIVATE:     i32 = 6;

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

    fn onion(bytes: [u8; ADDR_TORV3_SIZE]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&bytes[..]))
            .net(Network::NET_ONION)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    fn i2p(bytes: [u8; ADDR_I2P_SIZE]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&bytes[..]))
            .net(Network::NET_I2P)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    fn v6_plain_global() -> NetAddr {
        // 2001:4860::/32 (Google) – routable and not a tunnel/prefix special
        let mut b = [0u8; 16];
        b[0..4].copy_from_slice(&[0x20, 0x01, 0x48, 0x60]);
        v6(b)
    }

    fn v6_6to4() -> NetAddr {
        // 2002::/16
        let mut b = [0u8; 16];
        b[0..2].copy_from_slice(&[0x20, 0x02]);
        b[2..6].copy_from_slice(&[1, 2, 3, 4]); // embedded IPv4 for realism
        v6(b)
    }

    fn v6_6052() -> NetAddr {
        // 64:ff9b::/96 with some IPv4 tail
        let mut b = [0u8; 16];
        b[0..12].copy_from_slice(&[0x00, 0x64, 0xFF, 0x9B, 0, 0, 0, 0, 0, 0, 0, 0]);
        b[12..16].copy_from_slice(&[1, 2, 3, 4]);
        v6(b)
    }

    fn v6_teredo() -> NetAddr {
        // 2001::/32 Teredo – last 4 bytes are IPv4^0xFF for a canonical form
        let mut b = [0u8; 16];
        b[0..4].copy_from_slice(&[0x20, 0x01, 0x00, 0x00]);
        // Put NOT(1.2.3.4) into the tail: FE FD FC FB
        b[12..16].copy_from_slice(&[0xFE, 0xFD, 0xFC, 0xFB]);
        v6(b)
    }

    #[traced_test]
    fn unroutable_or_internal_are_unreachable() {
        let partner = v6_plain_global();

        let unroutable = v4([10, 0, 0, 1]); // RFC1918
        let score_unroutable = unroutable.get_reachability_from(&partner as *const NetAddr);
        info!(score = score_unroutable, "Unroutable private IPv4 must be unreachable");
        assert_eq!(score_unroutable, REACH_UNREACHABLE);

        let mut internal = NetAddr::default();
        assert!(internal.set_internal("seed.example"));
        let score_internal = internal.get_reachability_from(&partner as *const NetAddr);
        info!(score = score_internal, "Internal address must be unreachable");
        assert_eq!(score_internal, REACH_UNREACHABLE);
    }

    #[traced_test]
    fn ipv6_peer_reachability_grades() {
        let peer_v6 = v6_plain_global();

        let ours_v6 = v6_plain_global();
        let ours_v4 = v4([8, 8, 8, 8]);
        let ours_6to4 = v6_6to4();
        let ours_6052 = v6_6052();
        let ours_teredo = v6_teredo();

        let s_strong = ours_v6.get_reachability_from(&peer_v6 as *const NetAddr);
        let s_v4    = ours_v4.get_reachability_from(&peer_v6 as *const NetAddr);
        let s_weak1 = ours_6to4.get_reachability_from(&peer_v6 as *const NetAddr);
        let s_weak2 = ours_6052.get_reachability_from(&peer_v6 as *const NetAddr);
        let s_teredo= ours_teredo.get_reachability_from(&peer_v6 as *const NetAddr);

        debug!(s_strong, s_v4, s_weak1, s_weak2, s_teredo, "Scores against IPv6 peer");

        assert_eq!(s_strong, REACH_IPV6_STRONG);
        assert_eq!(s_v4,     REACH_IPV4);
        assert_eq!(s_weak1,  REACH_IPV6_WEAK);
        assert_eq!(s_weak2,  REACH_IPV6_WEAK);
        assert_eq!(s_teredo, REACH_TEREDO);
    }

    #[traced_test]
    fn onion_peer_behavior() {
        let peer_onion = onion([0x33; ADDR_TORV3_SIZE]);

        let ours_onion = onion([0x77; ADDR_TORV3_SIZE]);
        let ours_v4    = v4([8, 8, 4, 4]);
        let ours_v6    = v6_plain_global();

        let s_onion_onion = ours_onion.get_reachability_from(&peer_onion as *const NetAddr);
        let s_v4_onion    = ours_v4.get_reachability_from(&peer_onion as *const NetAddr);
        let s_v6_onion    = ours_v6.get_reachability_from(&peer_onion as *const NetAddr);

        info!(s_onion_onion, s_v4_onion, s_v6_onion, "Scores against TOR peer");

        assert_eq!(s_onion_onion, REACH_PRIVATE);
        assert_eq!(s_v4_onion,    REACH_IPV4);
        assert_eq!(s_v6_onion,    REACH_DEFAULT);
    }

    #[traced_test]
    fn i2p_peer_behavior() {
        let peer_i2p = i2p([0x22; ADDR_I2P_SIZE]);

        let ours_i2p = i2p([0x11; ADDR_I2P_SIZE]);
        let ours_v4  = v4([1, 1, 1, 1]);

        let s_i2p_i2p = ours_i2p.get_reachability_from(&peer_i2p as *const NetAddr);
        let s_v4_i2p  = ours_v4.get_reachability_from(&peer_i2p as *const NetAddr);

        debug!(s_i2p_i2p, s_v4_i2p, "Scores against I2P peer");

        assert_eq!(s_i2p_i2p, REACH_PRIVATE);
        assert_eq!(s_v4_i2p,  REACH_DEFAULT);
    }

    #[traced_test]
    fn unknown_partner_defaults_based_on_our_net() {
        let null_peer: *const NetAddr = core::ptr::null();

        let ours_v4     = v4([9, 9, 9, 9]);
        let ours_v6     = v6_plain_global();
        let ours_teredo = v6_teredo();
        let ours_onion  = onion([0x66; ADDR_TORV3_SIZE]);

        let s_v4     = ours_v4.get_reachability_from(null_peer);
        let s_v6     = ours_v6.get_reachability_from(null_peer);
        let s_teredo = ours_teredo.get_reachability_from(null_peer);
        let s_onion  = ours_onion.get_reachability_from(null_peer);

        info!(s_v4, s_v6, s_teredo, s_onion, "Scores against unknown/None peer");

        assert_eq!(s_v4,     REACH_IPV4);
        assert_eq!(s_v6,     REACH_IPV6_WEAK);
        assert_eq!(s_teredo, REACH_TEREDO);
        assert_eq!(s_onion,  REACH_PRIVATE);
    }

    #[traced_test]
    fn teredo_against_teredo_is_teredo() {
        let a = v6_teredo();
        let b = v6_teredo();
        let s = a.get_reachability_from(&b as *const NetAddr);
        debug!(s, "Teredo↔Teredo reachability");
        assert_eq!(s, REACH_TEREDO);
    }
}
