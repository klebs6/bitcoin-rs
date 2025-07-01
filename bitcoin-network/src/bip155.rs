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
        let mut addr = NetAddr::default();
        // Founding id with wrong length must panic.
        addr.set_net_from_bip155network(BIP155Network::IPV4 as u8, 15);
    }

    #[traced_test]
    fn unknown_id_is_ignored() {
        let mut addr = NetAddr::default();
        assert!(!addr.set_net_from_bip155network(250, 7)); // bogus future id
    }
}
