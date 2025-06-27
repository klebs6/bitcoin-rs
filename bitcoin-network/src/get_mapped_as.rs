crate::ix!();

impl NetAddr {

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
}
