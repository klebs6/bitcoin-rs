crate::ix!();

impl NetAddr {
    
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
}
