crate::ix!();

impl NetAddr {

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
}
