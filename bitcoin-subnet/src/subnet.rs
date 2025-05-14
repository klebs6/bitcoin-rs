// ---------------- [ File: bitcoin-subnet/src/subnet.rs ]
crate::ix!();

///--------------------
#[derive(Hash,Clone)]
pub struct SubNet {

    /**
      | Network (base) address
      |
      */
    network: NetAddr,

    /**
      | Netmask, in network byte order
      |
      */
    netmask: [u8; 16],

    /**
      | Is this value valid? (only used to signal
      | parse errors)
      |
      */
    valid:   bool,
}

impl SubNet {

    /**
      | Construct from a given network start
      | and number of bits (CIDR mask).
      | 
      | -----------
      | @param[in] addr
      | 
      | Network start. Must be IPv4 or IPv6,
      | otherwise an invalid subnet is created.
      | ----------
      | @param[in] mask
      | 
      | CIDR mask, must be in [0, 32] for IPv4
      | addresses and in [0, 128] for
      | 
      | IPv6 addresses. Otherwise an invalid
      | subnet is created.
      |
      */
    pub fn new_from_net_addr(
        addr: &NetAddr,
        mask: Option<u8>) -> Self {
    
        todo!();
        /*
        : sub_net(),

            valid = (addr.IsIPv4() && mask <= ADDR_IPV4_SIZE * 8) ||
                (addr.IsIPv6() && mask <= ADDR_IPV6_SIZE * 8);
        if (!valid) {
            return;
        }

        assert(mask <= sizeof(netmask) * 8);

        network = addr;

        uint8_t n = mask;
        for (size_t i = 0; i < network.m_addr.size(); ++i) {
            const uint8_t bits = n < 8 ? n : 8;
            netmask[i] = (uint8_t)((uint8_t)0xFF << (8 - bits)); // Set first bits.
            network.m_addr[i] &= netmask[i]; // Normalize network according to netmask.
            n -= bits;
        }
        */
    }
    
    /**
      | Construct from a given network start
      | and mask.
      | 
      | -----------
      | @param[in] addr
      | 
      | Network start. Must be IPv4 or IPv6,
      | otherwise an invalid subnet is created.
      | ----------
      | @param[in] mask
      | 
      | Network mask, must be of the same type
      | as `addr` and not contain 0-bits followed
      | by 1-bits. Otherwise an invalid subnet
      | is created.
      |
      */
    pub fn new_from_net_addr_and_mask(
        addr: &NetAddr,
        mask: &NetAddr) -> Self {
    
        todo!();
        /*
        : sub_net(),

            valid = (addr.IsIPv4() || addr.IsIPv6()) && addr.m_net == mask.m_net;
        if (!valid) {
            return;
        }
        // Check if `mask` contains 1-bits after 0-bits (which is an invalid netmask).
        bool zeros_found = false;
        for (auto b : mask.m_addr) {
            const int num_bits = NetmaskBits(b);
            if (num_bits == -1 || (zeros_found && num_bits != 0)) {
                valid = false;
                return;
            }
            if (num_bits < 8) {
                zeros_found = true;
            }
        }

        assert(mask.m_addr.size() <= sizeof(netmask));

        memcpy(netmask, mask.m_addr.data(), mask.m_addr.size());

        network = addr;

        // Normalize network according to netmask
        for (size_t x = 0; x < network.m_addr.size(); ++x) {
            network.m_addr[x] &= netmask[x];
        }
        */
    }
    
    /**
      | @return
      | 
      | True if this subnet is valid, the specified
      | address is valid, and the specified
      | address belongs in this subnet.
      |
      */
    pub fn match_(&self, addr: &NetAddr) -> bool {
        
        todo!();
        /*
            if (!valid || !addr.IsValid() || network.m_net != addr.m_net)
            return false;

        switch (network.m_net) {
        case NET_IPV4:
        case NET_IPV6:
            break;
        case NET_ONION:
        case NET_I2P:
        case NET_CJDNS:
        case NET_INTERNAL:
            return addr == network;
        case NET_UNROUTABLE:
        case NET_MAX:
            return false;
        }

        assert(network.m_addr.size() == addr.m_addr.size());
        for (size_t x = 0; x < addr.m_addr.size(); ++x) {
            if ((addr.m_addr[x] & netmask[x]) != network.m_addr[x]) {
                return false;
            }
        }
        return true;
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            std::string suffix;

        switch (network.m_net) {
        case NET_IPV4:
        case NET_IPV6: {
            assert(network.m_addr.size() <= sizeof(netmask));

            uint8_t cidr = 0;

            for (size_t i = 0; i < network.m_addr.size(); ++i) {
                if (netmask[i] == 0x00) {
                    break;
                }
                cidr += NetmaskBits(netmask[i]);
            }

            suffix = strprintf("/%u", cidr);
            break;
        }
        case NET_ONION:
        case NET_I2P:
        case NET_CJDNS:
        case NET_INTERNAL:
        case NET_UNROUTABLE:
        case NET_MAX:
            break;
        }

        return network.ToString() + suffix;
        */
    }
    
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return valid;
        */
    }
    
    pub fn sanity_check(&self) -> bool {
        
        todo!();
        /*
            switch (network.m_net) {
        case NET_IPV4:
        case NET_IPV6:
            break;
        case NET_ONION:
        case NET_I2P:
        case NET_CJDNS:
            return true;
        case NET_INTERNAL:
        case NET_UNROUTABLE:
        case NET_MAX:
            return false;
        }

        for (size_t x = 0; x < network.m_addr.size(); ++x) {
            if (network.m_addr[x] & ~netmask[x]) return false;
        }

        return true;
        */
    }
}

///---------------------
impl PartialEq<SubNet> for SubNet {
    
    #[inline] fn eq(&self, other: &SubNet) -> bool {
        todo!();
        /*
            return a.valid == b.valid && a.network == b.network && !memcmp(a.netmask, b.netmask, 16);
        */
    }
}

impl Eq for SubNet {}

impl Ord for SubNet {
    
    #[inline] fn cmp(&self, other: &SubNet) -> Ordering {
        todo!();
        /*
            return (a.network < b.network || (a.network == b.network && memcmp(a.netmask, b.netmask, 16) < 0));
        */
    }
}

impl PartialOrd<SubNet> for SubNet {
    #[inline] fn partial_cmp(&self, other: &SubNet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&NetAddr> for SubNet {

    /**
      | Construct a single-host subnet.
      | 
      | -----------
      | @param[in] addr
      | 
      | The sole address to be contained in the
      | subnet, can also be non-IPv[46].
      |
      */
    fn from(addr: &NetAddr) -> Self {
    
        todo!();
        /*
        : sub_net(),

            switch (addr.m_net) {
        case NET_IPV4:
        case NET_IPV6:
            valid = true;
            assert(addr.m_addr.size() <= sizeof(netmask));
            memset(netmask, 0xFF, addr.m_addr.size());
            break;
        case NET_ONION:
        case NET_I2P:
        case NET_CJDNS:
            valid = true;
            break;
        case NET_INTERNAL:
        case NET_UNROUTABLE:
        case NET_MAX:
            return;
        }

        network = addr;
        */
    }
}

impl Default for SubNet {

    /**
      | Construct an invalid subnet (empty,
      | `Match()` always returns false).
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : valid(false),

            memset(netmask, 0, sizeof(netmask));
        */
    }
}
