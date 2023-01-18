crate::ix!();

/**
  | Network address.
  |
  */
#[derive(Debug,Serialize,Deserialize,Clone,Hash)]
pub struct NetAddr {

    /**
      | Raw representation of the network address.
      | 
      | In network byte order (big endian) for
      | IPv4 and IPv6.
      |
      */
    pub addr:     PreVector<u8,ADDR_IPV6_SIZE>,

    /**
      | Network to which this address belongs.
      |
      */
    pub net:      Network,

    /**
      | Scope id if scoped/link-local IPV6
      | address.
      | 
      | See https://tools.ietf.org/html/rfc4007
      |
      */
    pub scope_id: u32,
}

impl Default for NetAddr {

    /**
      | Construct an unspecified IPv6 network
      | address (::/128).
      | 
      | -----------
      | @note
      | 
      | This address is considered invalid
      | by
      | 
      | CNetAddr::IsValid()
      |
      */
    fn default() -> Self {
        Self {
            addr:     PreVector::with_capacity(ADDR_IPV6_SIZE),
            net:      Network::NET_IPV6,
            scope_id: 0,
        }
    }
}

pub mod net_addr {

    use super::*;

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
    pub const V1_SERIALIZATION_SIZE: usize = ADDR_IPV6_SIZE;

    /**
      | Maximum size of an address as defined
      | in BIP155 (in bytes).
      | 
      | This is only the size of the address,
      | not the entire CNetAddr object when
      | serialized.
      |
      */
    pub const MAX_ADDRV2_SIZE: usize = 512;
}

//-------------------------------------------[.cpp/bitcoin/src/netaddress.cpp]
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
      | Serialize to a stream.
      |
      */
    pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            if (s.GetVersion() & ADDRV2_FORMAT) {
                SerializeV2Stream(s);
            } else {
                SerializeV1Stream(s);
            }
        */
    }

    /**
      | Unserialize from a stream.
      |
      */
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            if (s.GetVersion() & ADDRV2_FORMAT) {
                UnserializeV2Stream(s);
            } else {
                UnserializeV1Stream(s);
            }
        */
    }
    
    /**
      | Serialize in pre-ADDRv2/BIP155 format
      | to an array.
      |
      */
    pub fn serialize_v1array(&self, arr: &mut [u8; net_addr::V1_SERIALIZATION_SIZE])  {
        
        todo!();
        /*
            size_t prefix_size;

            switch (m_net) {
            case NET_IPV6:
                assert(m_addr.size() == sizeof(arr));
                memcpy(arr, m_addr.data(), m_addr.size());
                return;
            case NET_IPV4:
                prefix_size = sizeof(IPV4_IN_IPV6_PREFIX);
                assert(prefix_size + m_addr.size() == sizeof(arr));
                memcpy(arr, IPV4_IN_IPV6_PREFIX.data(), prefix_size);
                memcpy(arr + prefix_size, m_addr.data(), m_addr.size());
                return;
            case NET_INTERNAL:
                prefix_size = sizeof(INTERNAL_IN_IPV6_PREFIX);
                assert(prefix_size + m_addr.size() == sizeof(arr));
                memcpy(arr, INTERNAL_IN_IPV6_PREFIX.data(), prefix_size);
                memcpy(arr + prefix_size, m_addr.data(), m_addr.size());
                return;
            case NET_ONION:
            case NET_I2P:
            case NET_CJDNS:
                break;
            case NET_UNROUTABLE:
            case NET_MAX:
                assert(false);
            } // no default case, so the compiler can warn about missing cases

            // Serialize ONION, I2P and CJDNS as all-zeros.
            memset(arr, 0x0, V1_SERIALIZATION_SIZE);
        */
    }

    /**
      | Serialize in pre-ADDRv2/BIP155 format
      | to a stream.
      |
      */
    pub fn serialize_v1stream<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            uint8_t serialized[V1_SERIALIZATION_SIZE];

            SerializeV1Array(serialized);

            s << serialized;
        */
    }

    /**
      | Serialize as ADDRv2 / BIP155.
      |
      */
    pub fn serialize_v2stream<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            if (IsInternal()) {
                // Serialize NET_INTERNAL as embedded in IPv6. We need to
                // serialize such addresses from addrman.
                s << static_cast<uint8_t>(BIP155Network::IPV6);
                s << COMPACTSIZE(ADDR_IPV6_SIZE);
                SerializeV1Stream(s);
                return;
            }

            s << static_cast<uint8_t>(GetBIP155Network());
            s << m_addr;
        */
    }

    /**
      | Unserialize from a pre-ADDRv2/BIP155
      | format from an array.
      |
      */
    pub fn unserialize_v1array(&mut self, arr: &mut [u8; net_addr::V1_SERIALIZATION_SIZE])  {
        
        todo!();
        /*
            // Use SetLegacyIPv6() so that m_net is set correctly. For example
            // ::FFFF:0102:0304 should be set as m_net=NET_IPV4 (1.2.3.4).
            SetLegacyIPv6(arr);
        */
    }

    /**
      | Unserialize from a pre-ADDRv2/BIP155
      | format from a stream.
      |
      */
    pub fn unserialize_v1stream<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            uint8_t serialized[V1_SERIALIZATION_SIZE];

            s >> serialized;

            UnserializeV1Array(serialized);
        */
    }

    /**
      | Unserialize from a ADDRv2 / BIP155 format.
      |
      */
    pub fn unserialize_v2stream<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            uint8_t bip155_net;
            s >> bip155_net;

            size_t address_size;
            s >> COMPACTSIZE(address_size);

            if (address_size > MAX_ADDRV2_SIZE) {
                throw std::ios_base::failure(strprintf(
                    "Address too long: %u > %u", address_size, MAX_ADDRV2_SIZE));
            }

            m_scope_id = 0;

            if (SetNetFromBIP155Network(bip155_net, address_size)) {
                m_addr.resize(address_size);
                s >> MakeSpan(m_addr);

                if (m_net != NET_IPV6) {
                    return;
                }

                // Do some special checks on IPv6 addresses.

                // Recognize NET_INTERNAL embedded in IPv6, such addresses are not
                // gossiped but could be coming from addrman, when unserializing from
                // disk.
                if (HasPrefix(m_addr, INTERNAL_IN_IPV6_PREFIX)) {
                    m_net = NET_INTERNAL;
                    memmove(m_addr.data(), m_addr.data() + INTERNAL_IN_IPV6_PREFIX.size(),
                            ADDR_INTERNAL_SIZE);
                    m_addr.resize(ADDR_INTERNAL_SIZE);
                    return;
                }

                if (!HasPrefix(m_addr, IPV4_IN_IPV6_PREFIX) &&
                    !HasPrefix(m_addr, TORV2_IN_IPV6_PREFIX)) {
                    return;
                }

                // IPv4 and TORv2 are not supposed to be embedded in IPv6 (like in V1
                // encoding). Unserialize as !IsValid(), thus ignoring them.
            } else {
                // If we receive an unknown BIP155 network id (from the future?) then
                // ignore the address - unserialize as !IsValid().
                s.ignore(address_size);
            }

            // Mimic a default-constructed CNetAddr object which is !IsValid() and thus
            // will not be gossiped, but continue reading next addresses from the stream.
            m_net = NET_IPV6;
            m_addr.assign(ADDR_IPV6_SIZE, 0x0);
        */
    }

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
    pub fn get_bip155network(&self) -> net_addr::BIP155Network {
        
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

    pub fn setip(&mut self, ip_in: &NetAddr)  {
        
        todo!();
        /*
            // Size check.
        switch (ipIn.m_net) {
        case NET_IPV4:
            assert(ipIn.m_addr.size() == ADDR_IPV4_SIZE);
            break;
        case NET_IPV6:
            assert(ipIn.m_addr.size() == ADDR_IPV6_SIZE);
            break;
        case NET_ONION:
            assert(ipIn.m_addr.size() == ADDR_TORV3_SIZE);
            break;
        case NET_I2P:
            assert(ipIn.m_addr.size() == ADDR_I2P_SIZE);
            break;
        case NET_CJDNS:
            assert(ipIn.m_addr.size() == ADDR_CJDNS_SIZE);
            break;
        case NET_INTERNAL:
            assert(ipIn.m_addr.size() == ADDR_INTERNAL_SIZE);
            break;
        case NET_UNROUTABLE:
        case NET_MAX:
            assert(false);
        } // no default case, so the compiler can warn about missing cases

        m_net = ipIn.m_net;
        m_addr = ipIn.m_addr;
        */
    }
    
    /**
      | Set from a legacy IPv6 address.
      | 
      | Legacy IPv6 address may be a normal IPv6
      | address, or another address (e.g. IPv4)
      | disguised as IPv6.
      | 
      | This encoding is used in the legacy `addr`
      | encoding.
      |
      */
    pub fn set_legacy_ipv6(&mut self, ipv6: &[u8])  {
        
        todo!();
        /*
            assert(ipv6.size() == ADDR_IPV6_SIZE);

        size_t skip{0};

        if (HasPrefix(ipv6, IPV4_IN_IPV6_PREFIX)) {
            // IPv4-in-IPv6
            m_net = NET_IPV4;
            skip = sizeof(IPV4_IN_IPV6_PREFIX);
        } else if (HasPrefix(ipv6, TORV2_IN_IPV6_PREFIX)) {
            // TORv2-in-IPv6 (unsupported). Unserialize as !IsValid(), thus ignoring them.
            // Mimic a default-constructed CNetAddr object which is !IsValid() and thus
            // will not be gossiped, but continue reading next addresses from the stream.
            m_net = NET_IPV6;
            m_addr.assign(ADDR_IPV6_SIZE, 0x0);
            return;
        } else if (HasPrefix(ipv6, INTERNAL_IN_IPV6_PREFIX)) {
            // Internal-in-IPv6
            m_net = NET_INTERNAL;
            skip = sizeof(INTERNAL_IN_IPV6_PREFIX);
        } else {
            // IPv6
            m_net = NET_IPV6;
        }

        m_addr.assign(ipv6.begin() + skip, ipv6.end());
        */
    }

    /**
      | Create an "internal" address that represents
      | a name or FQDN. AddrMan uses these fake
      | addresses to keep track of which DNS
      | seeds were used.
      | 
      | 
      | -----------
      | @return
      | 
      | Whether or not the operation was successful.
      | @see NET_INTERNAL, INTERNAL_IN_IPV6_PREFIX,
      | CNetAddr::IsInternal(), CNetAddr::IsRFC4193()
      |
      */
    pub fn set_internal(&mut self, name: &str) -> bool {
        
        todo!();
        /*
            if (name.empty()) {
            return false;
        }
        m_net = NET_INTERNAL;
        unsigned char hash[32] = {};
        CSHA256().Write((const unsigned char*)name.data(), name.size()).Finalize(hash);
        m_addr.assign(hash, hash + ADDR_INTERNAL_SIZE);
        return true;
        */
    }
}

impl PartialEq<NetAddr> for NetAddr {
    
    #[inline] fn eq(&self, other: &NetAddr) -> bool {
        todo!();
        /*
            return a.m_net == b.m_net && a.m_addr == b.m_addr;
        */
    }
}

impl Eq for NetAddr {}

impl Ord for NetAddr {
    
    #[inline] fn cmp(&self, other: &NetAddr) -> Ordering {
        todo!();
        /*
            return std::tie(a.m_net, a.m_addr) < std::tie(b.m_net, b.m_addr);
        */
    }
}

impl PartialOrd<NetAddr> for NetAddr {
    #[inline] fn partial_cmp(&self, other: &NetAddr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&InAddr> for NetAddr {
    fn from(ipv_4addr: &InAddr) -> Self {
    
        todo!();
        /*


            m_net = NET_IPV4;
        const uint8_t* ptr = reinterpret_cast<const uint8_t*>(&ipv4Addr);
        m_addr.assign(ptr, ptr + ADDR_IPV4_SIZE);
        */
    }
}

impl CheckIsReachable for NetAddr {

    /**
      | @return
      | 
      | true if the address is in a reachable
      | network, false otherwise
      |
      */
    fn is_reachable(&self) -> bool {
        self.get_network().is_reachable()
    }
}
    
impl NetAddr {

    /**
      | Parse a Tor or I2P address and set this
      | object to it.
      | 
      | -----------
      | @param[in] addr
      | 
      | Address to parse, for example pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion
      | or ukeu3k5oycgaauneqgtnvselmt4yemvoilkln7jpvamvfx7dnkdq.b32.i2p.
      | 
      | -----------
      | @return
      | 
      | Whether the operation was successful.
      | @see CNetAddr::IsTor(), CNetAddr::IsI2P()
      |
      */
    pub fn set_special(&mut self, addr: &String) -> bool {
        
        todo!();
        /*
            if (!ValidAsCString(addr)) {
            return false;
        }

        if (SetTor(addr)) {
            return true;
        }

        if (SetI2P(addr)) {
            return true;
        }

        return false;
        */
    }
    
    /**
      | Parse a Tor address and set this object
      | to it.
      | 
      | -----------
      | @param[in] addr
      | 
      | Address to parse, must be a valid C string,
      | for example pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion.
      | 
      | -----------
      | @return
      | 
      | Whether the operation was successful.
      | @see CNetAddr::IsTor()
      |
      */
    pub fn set_tor(&mut self, addr: &String) -> bool {
        
        todo!();
        /*
            static const char* suffix{".onion"};
        static constexpr size_t suffix_len{6};

        if (addr.size() <= suffix_len || addr.substr(addr.size() - suffix_len) != suffix) {
            return false;
        }

        bool invalid;
        const auto& input = DecodeBase32(addr.substr(0, addr.size() - suffix_len).c_str(), &invalid);

        if (invalid) {
            return false;
        }

        if (input.size() == torv3::TOTAL_LEN) {
            Span<const uint8_t> input_pubkey{input.data(), ADDR_TORV3_SIZE};
            Span<const uint8_t> input_checksum{input.data() + ADDR_TORV3_SIZE, torv3::CHECKSUM_LEN};
            Span<const uint8_t> input_version{input.data() + ADDR_TORV3_SIZE + torv3::CHECKSUM_LEN, sizeof(torv3::VERSION)};

            if (input_version != torv3::VERSION) {
                return false;
            }

            uint8_t calculated_checksum[torv3::CHECKSUM_LEN];
            torv3::Checksum(input_pubkey, calculated_checksum);

            if (input_checksum != calculated_checksum) {
                return false;
            }

            m_net = NET_ONION;
            m_addr.assign(input_pubkey.begin(), input_pubkey.end());
            return true;
        }

        return false;
        */
    }
    
    /**
      | Parse an I2P address and set this object
      | to it.
      | 
      | -----------
      | @param[in] addr
      | 
      | Address to parse, must be a valid C string,
      | for example ukeu3k5oycgaauneqgtnvselmt4yemvoilkln7jpvamvfx7dnkdq.b32.i2p.
      | 
      | -----------
      | @return
      | 
      | Whether the operation was successful.
      | @see CNetAddr::IsI2P()
      |
      */
    pub fn seti2p(&mut self, addr: &String) -> bool {
        
        todo!();
        /*
            // I2P addresses that we support consist of 52 base32 characters + ".b32.i2p".
        static constexpr size_t b32_len{52};
        static const char* suffix{".b32.i2p"};
        static constexpr size_t suffix_len{8};

        if (addr.size() != b32_len + suffix_len || ToLower(addr.substr(b32_len)) != suffix) {
            return false;
        }

        // Remove the ".b32.i2p" suffix and pad to a multiple of 8 chars, so DecodeBase32()
        // can decode it.
        const std::string b32_padded = addr.substr(0, b32_len) + "====";

        bool invalid;
        const auto& address_bytes = DecodeBase32(b32_padded.c_str(), &invalid);

        if (invalid || address_bytes.size() != ADDR_I2P_SIZE) {
            return false;
        }

        m_net = NET_I2P;
        m_addr.assign(address_bytes.begin(), address_bytes.end());

        return true;
        */
    }
    
    pub fn new(
        ipv_6addr: &In6Addr,
        scope:     Option<u32>) -> Self {

        let scope: u32 = scope.unwrap_or(0);
    
        todo!();
        /*


            SetLegacyIPv6(Span<const uint8_t>(reinterpret_cast<const uint8_t*>(&ipv6Addr), sizeof(ipv6Addr)));
        m_scope_id = scope;
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
    
    pub fn get_network(&self) -> Network {
        
        todo!();
        /*
            if (IsInternal())
            return NET_INTERNAL;

        if (!IsRoutable())
            return NET_UNROUTABLE;

        return m_net;
        */
    }
    
    pub fn to_stringip(&self) -> String {
        
        todo!();
        /*
            switch (m_net) {
        case NET_IPV4:
            return IPv4ToString(m_addr);
        case NET_IPV6:
            return IPv6ToString(m_addr, m_scope_id);
        case NET_ONION:
            return OnionToString(m_addr);
        case NET_I2P:
            return EncodeBase32(m_addr, false /* don't pad with = */) + ".b32.i2p";
        case NET_CJDNS:
            return IPv6ToString(m_addr, 0);
        case NET_INTERNAL:
            return EncodeBase32(m_addr) + ".internal";
        case NET_UNROUTABLE: // m_net is never and should not be set to NET_UNROUTABLE
        case NET_MAX:        // m_net is never and should not be set to NET_MAX
            assert(false);
        } // no default case, so the compiler can warn about missing cases

        assert(false);
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return ToStringIP();
        */
    }

    /**
      | Try to get our IPv4 address.
      | 
      | -----------
      | @param[out] pipv4Addr
      | 
      | The in_addr struct to which to copy.
      | 
      | -----------
      | @return
      | 
      | Whether or not the operation was successful,
      | in particular, whether or not our address
      | was an IPv4 address. @see CNetAddr::IsIPv4()
      |
      */
    pub fn get_in_addr(&self, pipv_4addr: *mut InAddr) -> bool {
        
        todo!();
        /*
            if (!IsIPv4())
            return false;
        assert(sizeof(*pipv4Addr) == m_addr.size());
        memcpy(pipv4Addr, m_addr.data(), m_addr.size());
        return true;
        */
    }

    /**
      | Try to get our IPv6 address.
      | 
      | -----------
      | @param[out] pipv6Addr
      | 
      | The in6_addr struct to which to copy.
      | 
      | -----------
      | @return
      | 
      | Whether or not the operation was successful,
      | in particular, whether or not our address
      | was an IPv6 address. @see CNetAddr::IsIPv6()
      |
      */
    pub fn get_in_6addr(&self, pipv_6addr: *mut In6Addr) -> bool {
        
        todo!();
        /*
            if (!IsIPv6()) {
            return false;
        }
        assert(sizeof(*pipv6Addr) == m_addr.size());
        memcpy(pipv6Addr, m_addr.data(), m_addr.size());
        return true;
        */
    }
    
    /**
      | Whether this address has a linked IPv4
      | address (see GetLinkedIPv4()).
      |
      */
    pub fn has_linked_ipv4(&self) -> bool {
        
        todo!();
        /*
            return IsRoutable() && (IsIPv4() || IsRFC6145() || IsRFC6052() || IsRFC3964() || IsRFC4380());
        */
    }
    
    /**
      | For IPv4, mapped IPv4, SIIT translated
      | 
      | IPv4, Teredo, 6to4 tunneled addresses,
      | return the relevant IPv4 address as
      | a uint32.
      |
      */
    pub fn get_linked_ipv4(&self) -> u32 {
        
        todo!();
        /*
            if (IsIPv4()) {
            return ReadBE32(m_addr.data());
        } else if (IsRFC6052() || IsRFC6145()) {
            // mapped IPv4, SIIT translated IPv4: the IPv4 address is the last 4 bytes of the address
            return ReadBE32(MakeSpan(m_addr).last(ADDR_IPV4_SIZE).data());
        } else if (IsRFC3964()) {
            // 6to4 tunneled IPv4: the IPv4 address is in bytes 2-6
            return ReadBE32(MakeSpan(m_addr).subspan(2, ADDR_IPV4_SIZE).data());
        } else if (IsRFC4380()) {
            // Teredo tunneled IPv4: the IPv4 address is in the last 4 bytes of the address, but bitflipped
            return ~ReadBE32(MakeSpan(m_addr).last(ADDR_IPV4_SIZE).data());
        }
        assert(false);
        */
    }
    
    pub fn get_net_class(&self) -> Network {
        
        todo!();
        /*
            // Make sure that if we return NET_IPV6, then IsIPv6() is true. The callers expect that.

        // Check for "internal" first because such addresses are also !IsRoutable()
        // and we don't want to return NET_UNROUTABLE in that case.
        if (IsInternal()) {
            return NET_INTERNAL;
        }
        if (!IsRoutable()) {
            return NET_UNROUTABLE;
        }
        if (HasLinkedIPv4()) {
            return NET_IPV4;
        }
        return m_net;
        */
    }
    
    /**
      | The AS on the BGP path to the node we use
      | to diversify peers in AddrMan bucketing
      | based on the AS infrastructure.
      |
      | The ip->AS mapping depends on how asmap is
      | constructed.
      */
    pub fn get_mappedas(&self, asmap: &Vec<bool>) -> u32 {
        
        todo!();
        /*
            uint32_t net_class = GetNetClass();
        if (asmap.size() == 0 || (net_class != NET_IPV4 && net_class != NET_IPV6)) {
            return 0; // Indicates not found, safe because AS0 is reserved per RFC7607.
        }
        std::vector<bool> ip_bits(128);
        if (HasLinkedIPv4()) {
            // For lookup, treat as if it was just an IPv4 address (IPV4_IN_IPV6_PREFIX + IPv4 bits)
            for (int8_t byte_i = 0; byte_i < 12; ++byte_i) {
                for (uint8_t bit_i = 0; bit_i < 8; ++bit_i) {
                    ip_bits[byte_i * 8 + bit_i] = (IPV4_IN_IPV6_PREFIX[byte_i] >> (7 - bit_i)) & 1;
                }
            }
            uint32_t ipv4 = GetLinkedIPv4();
            for (int i = 0; i < 32; ++i) {
                ip_bits[96 + i] = (ipv4 >> (31 - i)) & 1;
            }
        } else {
            // Use all 128 bits of the IPv6 address otherwise
            assert(IsIPv6());
            for (int8_t byte_i = 0; byte_i < 16; ++byte_i) {
                uint8_t cur_byte = m_addr[byte_i];
                for (uint8_t bit_i = 0; bit_i < 8; ++bit_i) {
                    ip_bits[byte_i * 8 + bit_i] = (cur_byte >> (7 - bit_i)) & 1;
                }
            }
        }
        uint32_t mapped_as = Interpret(asmap, ip_bits);
        return mapped_as;
        */
    }

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
        
        todo!();
        /*
            std::vector<unsigned char> vchRet;
        uint32_t net_class = GetNetClass();
        // If non-empty asmap is supplied and the address is IPv4/IPv6,
        // return ASN to be used for bucketing.
        uint32_t asn = GetMappedAS(asmap);
        if (asn != 0) { // Either asmap was empty, or address has non-asmappable net class (e.g. TOR).
            vchRet.push_back(NET_IPV6); // IPv4 and IPv6 with same ASN should be in the same bucket
            for (int i = 0; i < 4; i++) {
                vchRet.push_back((asn >> (8 * i)) & 0xFF);
            }
            return vchRet;
        }

        vchRet.push_back(net_class);
        int nBits{0};

        if (IsLocal()) {
            // all local addresses belong to the same group
        } else if (IsInternal()) {
            // all internal-usage addresses get their own group
            nBits = ADDR_INTERNAL_SIZE * 8;
        } else if (!IsRoutable()) {
            // all other unroutable addresses belong to the same group
        } else if (HasLinkedIPv4()) {
            // IPv4 addresses (and mapped IPv4 addresses) use /16 groups
            uint32_t ipv4 = GetLinkedIPv4();
            vchRet.push_back((ipv4 >> 24) & 0xFF);
            vchRet.push_back((ipv4 >> 16) & 0xFF);
            return vchRet;
        } else if (IsTor() || IsI2P() || IsCJDNS()) {
            nBits = 4;
        } else if (IsHeNet()) {
            // for he.net, use /36 groups
            nBits = 36;
        } else {
            // for the rest of the IPv6 network, use /32 groups
            nBits = 32;
        }

        // Push our address onto vchRet.
        const size_t num_bytes = nBits / 8;
        vchRet.insert(vchRet.end(), m_addr.begin(), m_addr.begin() + num_bytes);
        nBits %= 8;
        // ...for the last byte, push nBits and for the rest of the byte push 1's
        if (nBits > 0) {
            assert(num_bytes < m_addr.size());
            vchRet.push_back(m_addr[num_bytes] | ((1 << (8 - nBits)) - 1));
        }

        return vchRet;
        */
    }
    
    pub fn get_addr_bytes(&self) -> Vec<u8> {
        
        todo!();
        /*
            if (IsAddrV1Compatible()) {
            uint8_t serialized[V1_SERIALIZATION_SIZE];
            SerializeV1Array(serialized);
            return {std::begin(serialized), std::end(serialized)};
        }
        return std::vector<unsigned char>(m_addr.begin(), m_addr.end());
        */
    }
    
    pub fn get_hash(&self) -> u64 {
        
        todo!();
        /*
            uint256 hash = Hash(m_addr);
        uint64_t nRet;
        memcpy(&nRet, &hash, sizeof(nRet));
        return nRet;
        */
    }

    /**
      | Calculates a metric for how reachable
      | (*this) is from a given partner
      |
      */
    pub fn get_reachability_from(&self, paddr_partner: *const NetAddr) -> i32 {
        
        todo!();
        /*
            enum Reachability {
            REACH_UNREACHABLE,
            REACH_DEFAULT,
            REACH_TEREDO,
            REACH_IPV6_WEAK,
            REACH_IPV4,
            REACH_IPV6_STRONG,
            REACH_PRIVATE
        };

        if (!IsRoutable() || IsInternal())
            return REACH_UNREACHABLE;

        int ourNet = GetExtNetwork(this);
        int theirNet = GetExtNetwork(paddrPartner);
        bool fTunnel = IsRFC3964() || IsRFC6052() || IsRFC6145();

        switch(theirNet) {
        case NET_IPV4:
            switch(ourNet) {
            default:       return REACH_DEFAULT;
            case NET_IPV4: return REACH_IPV4;
            }
        case NET_IPV6:
            switch(ourNet) {
            default:         return REACH_DEFAULT;
            case NET_TEREDO: return REACH_TEREDO;
            case NET_IPV4:   return REACH_IPV4;
            case NET_IPV6:   return fTunnel ? REACH_IPV6_WEAK : REACH_IPV6_STRONG; // only prefer giving our IPv6 address if it's not tunnelled
            }
        case NET_ONION:
            switch(ourNet) {
            default:         return REACH_DEFAULT;
            case NET_IPV4:   return REACH_IPV4; // Tor users can connect to IPv4 as well
            case NET_ONION:    return REACH_PRIVATE;
            }
        case NET_I2P:
            switch (ourNet) {
            case NET_I2P: return REACH_PRIVATE;
            default: return REACH_DEFAULT;
            }
        case NET_TEREDO:
            switch(ourNet) {
            default:          return REACH_DEFAULT;
            case NET_TEREDO:  return REACH_TEREDO;
            case NET_IPV6:    return REACH_IPV6_WEAK;
            case NET_IPV4:    return REACH_IPV4;
            }
        case NET_UNKNOWN:
        case NET_UNROUTABLE:
        default:
            switch(ourNet) {
            default:          return REACH_DEFAULT;
            case NET_TEREDO:  return REACH_TEREDO;
            case NET_IPV6:    return REACH_IPV6_WEAK;
            case NET_IPV4:    return REACH_IPV4;
            case NET_ONION:     return REACH_PRIVATE; // either from Tor, or don't care about our address
            }
        }
        */
    }
}
