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
}
