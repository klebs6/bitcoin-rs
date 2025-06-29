// ---------------- [ File: bitcoin-network/src/set_internal.rs ]
crate::ix!();

impl NetAddr {

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
