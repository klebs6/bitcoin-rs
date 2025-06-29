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

    /**
      | Get the BIP155 network id of this address.
      | 
      | Must not be called for IsInternal()
      | objects.
      | 
      | 
      | -----------
      | @return
      | 
      | BIP155 network id, except TORV2 which
      | is no longer supported.
      |
      */
    pub fn get_bip155network(&self) -> BIP155Network {
        
        todo!();
        /*
            switch (m_net) {
        case NET_IPV4:
            return BIP155Network::IPV4;
        case NET_IPV6:
            return BIP155Network::IPV6;
        case NET_ONION:
            return BIP155Network::TORV3;
        case NET_I2P:
            return BIP155Network::I2P;
        case NET_CJDNS:
            return BIP155Network::CJDNS;
        case NET_INTERNAL:   // should have been handled before calling this function
        case NET_UNROUTABLE: // m_net is never and should not be set to NET_UNROUTABLE
        case NET_MAX:        // m_net is never and should not be set to NET_MAX
            assert(false);
        } // no default case, so the compiler can warn about missing cases

        assert(false);
        */
    }
    
    /**
      | Set `m_net` from the provided BIP155
      | network id and size after validation.
      | 
      | -----------
      | @return
      | 
      | true the network was recognized, is
      | valid and `m_net` was set
      | ----------
      | @return
      | 
      | false not recognised (from future?)
      | and should be silently ignored @throws
      | std::ios_base::failure if the network
      | is one of the BIP155 founding networks
      | (id 1..6) with wrong address size.
      |
      */
    pub fn set_net_from_bip155network(&mut self, 
        possible_bip155_net: u8,
        address_size:        usize) -> bool {
        
        todo!();
        /*
            switch (possible_bip155_net) {
        case BIP155Network::IPV4:
            if (address_size == ADDR_IPV4_SIZE) {
                m_net = NET_IPV4;
                return true;
            }
            throw std::ios_base::failure(
                strprintf("BIP155 IPv4 address with length %u (should be %u)", address_size,
                          ADDR_IPV4_SIZE));
        case BIP155Network::IPV6:
            if (address_size == ADDR_IPV6_SIZE) {
                m_net = NET_IPV6;
                return true;
            }
            throw std::ios_base::failure(
                strprintf("BIP155 IPv6 address with length %u (should be %u)", address_size,
                          ADDR_IPV6_SIZE));
        case BIP155Network::TORV3:
            if (address_size == ADDR_TORV3_SIZE) {
                m_net = NET_ONION;
                return true;
            }
            throw std::ios_base::failure(
                strprintf("BIP155 TORv3 address with length %u (should be %u)", address_size,
                          ADDR_TORV3_SIZE));
        case BIP155Network::I2P:
            if (address_size == ADDR_I2P_SIZE) {
                m_net = NET_I2P;
                return true;
            }
            throw std::ios_base::failure(
                strprintf("BIP155 I2P address with length %u (should be %u)", address_size,
                          ADDR_I2P_SIZE));
        case BIP155Network::CJDNS:
            if (address_size == ADDR_CJDNS_SIZE) {
                m_net = NET_CJDNS;
                return true;
            }
            throw std::ios_base::failure(
                strprintf("BIP155 CJDNS address with length %u (should be %u)", address_size,
                          ADDR_CJDNS_SIZE));
        }

        // Don't throw on addresses with unknown network ids (maybe from the future).
        // Instead silently drop them and have the unserialization code consume
        // subsequent ones which may be known to us.
        return false;
        */
    }
}
