// ---------------- [ File: bitcoin-network/src/bip155.rs ]
crate::ix!();

/**
  | BIP155 network ids recognized by this
  | software.
  |
  */
#[repr(u8)]
pub enum BIP155Network {
    IPV4  = 1,
    IPV6  = 2,
    TORV2 = 3,
    TORV3 = 4,
    I2P   = 5,
    CJDNS = 6,
}

/**
  | Size of CNetAddr when serialized as
  | ADDRv1 (pre-BIP155) (in bytes).
  |
  */
pub const NET_ADDR_V1_SERIALIZATION_SIZE: usize = ADDR_IPV6_SIZE;

/**
  | Maximum size of an address as defined
  | in BIP155 (in bytes).
  | 
  | This is only the size of the address,
  | not the entire CNetAddr object when
  | serialized.
  |
  */
pub const BIP155_MAX_ADDRV2_SIZE: usize = 512;

impl NetAddr {

    /// Get the BIP155 network id of this address.
    /// 
    /// **Panics** if called for an `NET_INTERNAL` object (the caller must ensure that case never
    /// happens – exactly like the original C++ code). @return
    /// 
    /// BIP155 network id, except TORV2 which is no longer supported.
    ///
    #[inline]
    pub fn get_bip155network(&self) -> BIP155Network {
        trace!(target: "netaddr", net = ?self.net(), "Deriving BIP155 network id");
        match *self.net() {
            Network::NET_IPV4     => BIP155Network::IPV4,
            Network::NET_IPV6     => BIP155Network::IPV6,
            Network::NET_ONION    => BIP155Network::TORV3,
            Network::NET_I2P      => BIP155Network::I2P,
            Network::NET_CJDNS    => BIP155Network::CJDNS,
            Network::NET_INTERNAL | Network::NET_UNROUTABLE | Network::NET_MAX => {
                panic!("get_bip155network() called for invalid network variant")
            }
        }
    }
    
    /// Validate `(id,size)` and set `self.net` accordingly.
    /// 
    /// Returns `true` for a *known* and *well‑sized* founding network id (1‥6).  
    ///
    /// Returns `false` for an *unknown* id (from the future).  
    ///
    /// **Panics** on founding ids with a mismatching size – faithful to the C++
    /// behaviour.
    ///
    pub fn set_net_from_bip155network(
        &mut self,
        possible_bip155_net: u8,
        address_size: usize,
    ) -> bool {
        debug!(
            target: "netaddr",
            id = possible_bip155_net,
            size = address_size,
            "Mapping BIP155 id to internal Network"
        );
        match possible_bip155_net {
            x if x == BIP155Network::IPV4 as u8 => {
                if address_size == ADDR_IPV4_SIZE {
                    self.set_net(Network::NET_IPV4);
                    true
                } else {
                    panic!(
                        "BIP155 IPv4 address with length {} (expected {})",
                        address_size, ADDR_IPV4_SIZE
                    );
                }
            }
            x if x == BIP155Network::IPV6 as u8 => {
                if address_size == ADDR_IPV6_SIZE {
                    self.set_net(Network::NET_IPV6);
                    true
                } else {
                    panic!(
                        "BIP155 IPv6 address with length {} (expected {})",
                        address_size, ADDR_IPV6_SIZE
                    );
                }
            }
            x if x == BIP155Network::TORV3 as u8 => {
                if address_size == ADDR_TORV3_SIZE {
                    self.set_net(Network::NET_ONION);
                    true
                } else {
                    panic!(
                        "BIP155 TORv3 address with length {} (expected {})",
                        address_size, ADDR_TORV3_SIZE
                    );
                }
            }
            x if x == BIP155Network::I2P as u8 => {
                if address_size == ADDR_I2P_SIZE {
                    self.set_net(Network::NET_I2P);
                    true
                } else {
                    panic!(
                        "BIP155 I2P address with length {} (expected {})",
                        address_size, ADDR_I2P_SIZE
                    );
                }
            }
            x if x == BIP155Network::CJDNS as u8 => {
                if address_size == ADDR_CJDNS_SIZE {
                    self.set_net(Network::NET_CJDNS);
                    true
                } else {
                    panic!(
                        "BIP155 CJDNS address with length {} (expected {})",
                        address_size, ADDR_CJDNS_SIZE
                    );
                }
            }
            // Unknown / future network id – silently ignore.
            //
            // Don't throw on addresses with unknown network ids (maybe from the future).
            // Instead silently drop them and have the unserialization code consume
            // subsequent ones which may be known to us.
            _ => false,
        }
    }
}

#[cfg(test)]
mod bip155_tests {
    use super::*;

    #[traced_test]
    fn mapping_round_trip() {
        let mut addr = NetAddr::default();

        // IPv4
        assert!(addr.set_net_from_bip155network(BIP155Network::IPV4 as u8, ADDR_IPV4_SIZE));
        assert_eq!(addr.get_bip155network() as u8, BIP155Network::IPV4 as u8);

        // IPv6
        assert!(addr.set_net_from_bip155network(BIP155Network::IPV6 as u8, ADDR_IPV6_SIZE));
        assert_eq!(addr.get_bip155network() as u8, BIP155Network::IPV6 as u8);

        // Tor v3
        assert!(addr.set_net_from_bip155network(BIP155Network::TORV3 as u8, ADDR_TORV3_SIZE));
        assert_eq!(addr.get_bip155network() as u8, BIP155Network::TORV3 as u8);
    }

    #[test]
    #[should_panic(expected = "IPv4 address with length")]
    fn wrong_size_panics() {
        info!("Verifying that founding BIP155 id with wrong length panics (Core‑compatible)");
        let mut addr = NetAddr::default();
        // Founding id with wrong length must panic.
        addr.set_net_from_bip155network(BIP155Network::IPV4 as u8, 15);
    }

    #[traced_test]
    fn unknown_id_is_ignored() {
        let mut addr = NetAddr::default();
        assert!(!addr.set_net_from_bip155network(250, 7)); // bogus future id
    }

    use std::panic;

    #[traced_test]
    fn founding_ids_round_trip_all_networks() {
        let mut a = NetAddr::default();

        let cases = [
            (BIP155Network::IPV4 as u8,  ADDR_IPV4_SIZE,  Network::NET_IPV4),
            (BIP155Network::IPV6 as u8,  ADDR_IPV6_SIZE,  Network::NET_IPV6),
            (BIP155Network::TORV3 as u8, ADDR_TORV3_SIZE, Network::NET_ONION),
            (BIP155Network::I2P as u8,   ADDR_I2P_SIZE,   Network::NET_I2P),
            (BIP155Network::CJDNS as u8, ADDR_CJDNS_SIZE, Network::NET_CJDNS),
        ];

        for (id, size, expected_net) in cases {
            info!(id, size, ?expected_net, "Applying founding BIP155 id");
            assert!(a.set_net_from_bip155network(id, size), "Expected founding id {id} with size {size} to be accepted");
            assert_eq!(*a.net(), expected_net, "Network mismatch after setting founding id {id}");
            assert_eq!(a.get_bip155network() as u8, id, "Round‑trip BIP155 id mismatch for {id}");
        }
    }

    #[traced_test]
    fn founding_id_wrong_length_panics_with_context() {
        let mut a = NetAddr::default();
        let cases: &[(u8, usize, &str)] = &[
            (BIP155Network::IPV4 as u8,  ADDR_IPV4_SIZE  - 1, "IPv4"),
            (BIP155Network::IPV6 as u8,  ADDR_IPV6_SIZE  - 1, "IPv6"),
            (BIP155Network::TORV3 as u8, ADDR_TORV3_SIZE - 1, "TORv3"),
            (BIP155Network::I2P  as u8,  ADDR_I2P_SIZE   - 1, "I2P"),
            (BIP155Network::CJDNS as u8, ADDR_CJDNS_SIZE - 1, "CJDNS"),
        ];

        for (id, bad_len, label) in cases {
            debug!(id = *id, bad_len = *bad_len, what = *label, "Expecting panic on length mismatch for founding id");
            let res = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                let _ = a.set_net_from_bip155network(*id, *bad_len);
            }));
            assert!(res.is_err(), "Expected panic for founding id {} with wrong length {}", label, bad_len);
        }
    }

    #[traced_test]
    fn unknown_future_id_is_ignored_and_returns_false() {
        let mut a = NetAddr::default();
        let future_id = 250u8;
        info!(future_id, "Submitting unknown/future BIP155 id");
        assert!(!a.set_net_from_bip155network(future_id, 7));
    }
}

#[cfg(test)]
mod bip155_id_mapping_spec {
    use super::*;
    use std::panic;

    fn make_ipv4() -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&[1, 2, 3, 4][..]))
            .net(Network::NET_IPV4)
            .scope_id(0_u32)
            .build()
            .unwrap()
    }

    fn make_ipv6() -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&[0u8; ADDR_IPV6_SIZE][..]))
            .net(Network::NET_IPV6)
            .scope_id(0_u32)
            .build()
            .unwrap()
    }

    fn make_onion() -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&[0x55u8; ADDR_TORV3_SIZE][..]))
            .net(Network::NET_ONION)
            .scope_id(0_u32)
            .build()
            .unwrap()
    }

    fn make_i2p() -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&[0x11u8; ADDR_I2P_SIZE][..]))
            .net(Network::NET_I2P)
            .scope_id(0_u32)
            .build()
            .unwrap()
    }

    fn make_cjdns() -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&[0xFCu8; ADDR_CJDNS_SIZE][..]))
            .net(Network::NET_CJDNS)
            .scope_id(0_u32)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn roundtrip_known_network_ids() {
        let mut a = NetAddr::default();

        assert!(a.set_net_from_bip155network(BIP155Network::IPV4 as u8, ADDR_IPV4_SIZE));
        assert_eq!(a.get_bip155network() as u8, BIP155Network::IPV4 as u8);

        assert!(a.set_net_from_bip155network(BIP155Network::IPV6 as u8, ADDR_IPV6_SIZE));
        assert_eq!(a.get_bip155network() as u8, BIP155Network::IPV6 as u8);

        assert!(a.set_net_from_bip155network(BIP155Network::TORV3 as u8, ADDR_TORV3_SIZE));
        assert_eq!(a.get_bip155network() as u8, BIP155Network::TORV3 as u8);

        assert!(a.set_net_from_bip155network(BIP155Network::I2P as u8, ADDR_I2P_SIZE));
        assert_eq!(a.get_bip155network() as u8, BIP155Network::I2P as u8);

        assert!(a.set_net_from_bip155network(BIP155Network::CJDNS as u8, ADDR_CJDNS_SIZE));
        assert_eq!(a.get_bip155network() as u8, BIP155Network::CJDNS as u8);
    }

    #[traced_test]
    fn founding_id_wrong_length_panics_for_each() {
        let mut addr = NetAddr::default();

        let cases: &[(u8, usize, &str)] = &[
            (BIP155Network::IPV4 as u8, ADDR_IPV4_SIZE - 1, "IPv4"),
            (BIP155Network::IPV6 as u8, ADDR_IPV6_SIZE - 1, "IPv6"),
            (BIP155Network::TORV3 as u8, ADDR_TORV3_SIZE - 1, "TORv3"),
            (BIP155Network::I2P  as u8, ADDR_I2P_SIZE  - 1, "I2P"),
            (BIP155Network::CJDNS as u8, ADDR_CJDNS_SIZE - 1, "CJDNS"),
        ];

        for (id, bad_len, label) in cases {
            let res = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                let _ = addr.set_net_from_bip155network(*id, *bad_len);
            }));
            assert!(res.is_err(), "Expected panic for {} wrong size", label);
        }
    }

    #[traced_test]
    fn future_id_is_ignored_and_returns_false() {
        let mut addr = NetAddr::default();
        let future_id = 250u8;
        assert!(!addr.set_net_from_bip155network(future_id, 7));
    }

    #[traced_test]
    fn get_bip155network_panics_for_invalid_variants() {
        let mut a = make_ipv4();
        *a.net_mut() = Network::NET_INTERNAL;
        let panic1 = std::panic::catch_unwind(|| {
            let _ = a.get_bip155network();
        });
        assert!(panic1.is_err(), "expected panic for NET_INTERNAL");

        let mut b = make_ipv6();
        *b.net_mut() = Network::NET_UNROUTABLE;
        let panic2 = std::panic::catch_unwind(|| {
            let _ = b.get_bip155network();
        });
        assert!(panic2.is_err(), "expected panic for NET_UNROUTABLE");

        let mut c = make_onion();
        *c.net_mut() = Network::NET_MAX;
        let panic3 = std::panic::catch_unwind(|| {
            let _ = c.get_bip155network();
        });
        assert!(panic3.is_err(), "expected panic for NET_MAX");
    }

    #[traced_test]
    fn get_bip155network_values_match_network_variants() {
        assert_eq!(make_ipv4().get_bip155network() as u8, BIP155Network::IPV4 as u8);
        assert_eq!(make_ipv6().get_bip155network() as u8, BIP155Network::IPV6 as u8);
        assert_eq!(make_onion().get_bip155network() as u8, BIP155Network::TORV3 as u8);
        assert_eq!(make_i2p().get_bip155network() as u8, BIP155Network::I2P as u8);
        assert_eq!(make_cjdns().get_bip155network() as u8, BIP155Network::CJDNS as u8);
    }
}
