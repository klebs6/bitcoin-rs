crate::ix!();

impl NetAddr {

    /**
      | Whether this address should be relayed
      | to other peers even if we can't reach
      | it ourselves.
      |
      */
    pub fn is_relayable(&self) -> bool {
        
        todo!();
        /*
            return IsIPv4() || IsIPv6() || IsTor() || IsI2P();
        */
    }

    /**
      | INADDR_ANY equivalent
      |
      */
    pub fn is_bind_any(&self) -> bool {
        
        todo!();
        /*
            if (!IsIPv4() && !IsIPv6()) {
            return false;
        }
        return std::all_of(m_addr.begin(), m_addr.end(), [](uint8_t b) { return b == 0; });
        */
    }
    
    /**
      | IPv4 mapped address (::FFFF:0:0/96,
      | 0.0.0.0/0)
      |
      */
    pub fn is_ipv4(&self) -> bool {
        
        todo!();
        /*
            return m_net == NET_IPV4;
        */
    }
    
    /**
      | IPv6 address (not mapped IPv4, not Tor)
      |
      */
    pub fn is_ipv6(&self) -> bool {
        
        todo!();
        /*
            return m_net == NET_IPV6;
        */
    }

    /**
      | IPv6 Hurricane Electric - https://he.net
      | (2001:0470::/36)
      |
      */
    pub fn is_he_net(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() && HasPrefix(m_addr, std::array<uint8_t, 4>{0x20, 0x01, 0x04, 0x70});
        */
    }

    /**
      | Check whether this object represents
      | a TOR address. @see CNetAddr::SetSpecial(const
      | std::string &)
      |
      */
    pub fn is_tor(&self) -> bool {
        
        todo!();
        /*
            return m_net == NET_ONION;
        */
    }

    pub fn is_local(&self) -> bool {
        
        todo!();
        /*
            // IPv4 loopback (127.0.0.0/8 or 0.0.0.0/8)
        if (IsIPv4() && (m_addr[0] == 127 || m_addr[0] == 0)) {
            return true;
        }

        // IPv6 loopback (::1/128)
        static const unsigned char pchLocal[16] = {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1};
        if (IsIPv6() && memcmp(m_addr.data(), pchLocal, sizeof(pchLocal)) == 0) {
            return true;
        }

        return false;
        */
    }

    /**
      | @note
      | 
      | A valid address may or may not be publicly
      | routable on the global internet. As
      | in, the set of valid addresses is a superset
      | of the set of publicly routable addresses.
      | @see CNetAddr::IsRoutable()
      | 
      | -----------
      | @return
      | 
      | Whether or not this network address
      | is a valid address that @a could be used
      | to refer to an actual host.
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            // unspecified IPv6 address (::/128)
        unsigned char ipNone6[16] = {};
        if (IsIPv6() && memcmp(m_addr.data(), ipNone6, sizeof(ipNone6)) == 0) {
            return false;
        }

        // CJDNS addresses always start with 0xfc
        if (IsCJDNS() && (m_addr[0] != 0xFC)) {
            return false;
        }

        // documentation IPv6 address
        if (IsRFC3849())
            return false;

        if (IsInternal())
            return false;

        if (IsIPv4()) {
            const uint32_t addr = ReadBE32(m_addr.data());
            if (addr == INADDR_ANY || addr == INADDR_NONE) {
                return false;
            }
        }

        return true;
        */
    }

    /**
      | @note
      | 
      | A routable address is always valid.
      | As in, the set of routable addresses
      | is a subset of the set of valid addresses.
      | @see CNetAddr::IsValid()
      | 
      | -----------
      | @return
      | 
      | Whether or not this network address
      | is publicly routable on the global internet.
      |
      */
    pub fn is_routable(&self) -> bool {
        
        todo!();
        /*
            return IsValid() && !(IsRFC1918() || IsRFC2544() || IsRFC3927() || IsRFC4862() || IsRFC6598() || IsRFC5737() || IsRFC4193() || IsRFC4843() || IsRFC7343() || IsLocal() || IsInternal());
        */
    }

    /**
      | @return
      | 
      | Whether or not this is a dummy address
      | that represents a name. @see CNetAddr::SetInternal(const
      | std::string &)
      |
      */
    pub fn is_internal(&self) -> bool {
        
        todo!();
        /*
            return m_net == NET_INTERNAL;
        */
    }
    
    /**
      | Check if the current object can be serialized
      | in pre-ADDRv2/BIP155 format.
      |
      */
    pub fn is_addr_v1compatible(&self) -> bool {
        
        todo!();
        /*
            switch (m_net) {
        case NET_IPV4:
        case NET_IPV6:
        case NET_INTERNAL:
            return true;
        case NET_ONION:
        case NET_I2P:
        case NET_CJDNS:
            return false;
        case NET_UNROUTABLE: // m_net is never and should not be set to NET_UNROUTABLE
        case NET_MAX:        // m_net is never and should not be set to NET_MAX
            assert(false);
        } // no default case, so the compiler can warn about missing cases

        assert(false);
        */
    }

    /**
      | IPv4 private networks (10.0.0.0/8,
      | 192.168.0.0/16, 172.16.0.0/12)
      |
      */
    pub fn isrfc1918(&self) -> bool {
        
        todo!();
        /*
            return IsIPv4() && (
            m_addr[0] == 10 ||
            (m_addr[0] == 192 && m_addr[1] == 168) ||
            (m_addr[0] == 172 && m_addr[1] >= 16 && m_addr[1] <= 31));
        */
    }
    
    /**
      | IPv4 inter-network communications
      | (198.18.0.0/15)
      |
      */
    pub fn isrfc2544(&self) -> bool {
        
        todo!();
        /*
            return IsIPv4() && m_addr[0] == 198 && (m_addr[1] == 18 || m_addr[1] == 19);
        */
    }
    
    /**
      | IPv4 autoconfig (169.254.0.0/16)
      |
      */
    pub fn isrfc3927(&self) -> bool {
        
        todo!();
        /*
            return IsIPv4() && HasPrefix(m_addr, std::array<uint8_t, 2>{169, 254});
        */
    }
    
    /**
      | IPv4 ISP-level NAT (100.64.0.0/10)
      |
      */
    pub fn isrfc6598(&self) -> bool {
        
        todo!();
        /*
            return IsIPv4() && m_addr[0] == 100 && m_addr[1] >= 64 && m_addr[1] <= 127;
        */
    }
    
    /**
      | IPv4 documentation addresses (192.0.2.0/24,
      | 198.51.100.0/24, 203.0.113.0/24)
      |
      */
    pub fn isrfc5737(&self) -> bool {
        
        todo!();
        /*
            return IsIPv4() && (HasPrefix(m_addr, std::array<uint8_t, 3>{192, 0, 2}) ||
                            HasPrefix(m_addr, std::array<uint8_t, 3>{198, 51, 100}) ||
                            HasPrefix(m_addr, std::array<uint8_t, 3>{203, 0, 113}));
        */
    }
    
    /**
      | IPv6 documentation address (2001:0DB8::/32)
      |
      */
    pub fn isrfc3849(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() && HasPrefix(m_addr, std::array<uint8_t, 4>{0x20, 0x01, 0x0D, 0xB8});
        */
    }
    
    /**
      | IPv6 6to4 tunnelling (2002::/16)
      |
      */
    pub fn isrfc3964(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() && HasPrefix(m_addr, std::array<uint8_t, 2>{0x20, 0x02});
        */
    }
    
    /**
      | IPv6 well-known prefix for IPv4-embedded
      | address (64:FF9B::/96)
      |
      */
    pub fn isrfc6052(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() &&
               HasPrefix(m_addr, std::array<uint8_t, 12>{0x00, 0x64, 0xFF, 0x9B, 0x00, 0x00,
                                                         0x00, 0x00, 0x00, 0x00, 0x00, 0x00});
        */
    }
    
    /**
      | IPv6 Teredo tunnelling (2001::/32)
      |
      */
    pub fn isrfc4380(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() && HasPrefix(m_addr, std::array<uint8_t, 4>{0x20, 0x01, 0x00, 0x00});
        */
    }
    
    /**
      | IPv6 autoconfig (FE80::/64)
      |
      */
    pub fn isrfc4862(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() && HasPrefix(m_addr, std::array<uint8_t, 8>{0xFE, 0x80, 0x00, 0x00,
                                                                    0x00, 0x00, 0x00, 0x00});
        */
    }
    
    /**
      | IPv6 unique local (FC00::/7)
      |
      */
    pub fn isrfc4193(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() && (m_addr[0] & 0xFE) == 0xFC;
        */
    }
    
    /**
      | IPv6 IPv4-translated address (::FFFF:0:0:0/96)
      | (actually defined in RFC2765)
      |
      */
    pub fn isrfc6145(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() &&
               HasPrefix(m_addr, std::array<uint8_t, 12>{0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                                         0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00});
        */
    }
    
    /**
      | IPv6 ORCHID (deprecated) (2001:10::/28)
      |
      */
    pub fn isrfc4843(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() && HasPrefix(m_addr, std::array<uint8_t, 3>{0x20, 0x01, 0x00}) &&
               (m_addr[3] & 0xF0) == 0x10;
        */
    }
    
    /**
      | IPv6 ORCHIDv2 (2001:20::/28)
      |
      */
    pub fn isrfc7343(&self) -> bool {
        
        todo!();
        /*
            return IsIPv6() && HasPrefix(m_addr, std::array<uint8_t, 3>{0x20, 0x01, 0x00}) &&
               (m_addr[3] & 0xF0) == 0x20;
        */
    }

    /**
      | Check whether this object represents
      | an I2P address.
      |
      */
    pub fn isi2p(&self) -> bool {
        
        todo!();
        /*
            return m_net == NET_I2P;
        */
    }

    /**
      | Check whether this object represents
      | a CJDNS address.
      |
      */
    pub fn iscjdns(&self) -> bool {
        
        todo!();
        /*
            return m_net == NET_CJDNS;
        */
    }
}
