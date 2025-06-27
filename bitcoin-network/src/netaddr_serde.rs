crate::ix!();

impl NetAddr {

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
    pub fn serialize_v1array(&self, arr: &mut [u8; NET_ADDR_V1_SERIALIZATION_SIZE])  {
        
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
            memset(arr, 0x0, NET_ADDR_V1_SERIALIZATION_SIZE);
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
            uint8_t serialized[NET_ADDR_V1_SERIALIZATION_SIZE];

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
    pub fn unserialize_v1array(&mut self, arr: &mut [u8; NET_ADDR_V1_SERIALIZATION_SIZE])  {
        
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
            uint8_t serialized[NET_ADDR_V1_SERIALIZATION_SIZE];

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

            if (address_size > BIP155_MAX_ADDRV2_SIZE) {
                throw std::ios_base::failure(strprintf(
                    "Address too long: %u > %u", address_size, BIP155_MAX_ADDRV2_SIZE));
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
}
