// ---------------- [ File: bitcoin-network/src/get_in_addr.rs ]
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
    #[inline]
    pub fn get_in_addr(&self, pipv_4addr: *mut InAddr) -> bool {
        trace!(target: "netaddr", "Attempting IPv4 extraction via get_in_addr");
        if !self.is_ipv4() {
            debug!(target: "netaddr", "get_in_addr called on non‑IPv4 NetAddr");
            return false;
        }
        if pipv_4addr.is_null() {
            error!(target: "netaddr", "Null pointer passed to get_in_addr");
            return false;
        }

        assert_eq!(
            self.addr().len(),
            ADDR_IPV4_SIZE,
            "IPv4 NetAddr must contain exactly four bytes"
        );

        // Safety: the caller guarantees that `pipv_4addr` points to valid, writable
        //         memory large enough for four bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.addr().as_ptr(), 
                pipv_4addr as *mut u8, 
                ADDR_IPV4_SIZE
            );
        }
        true
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
    #[inline]
    pub fn get_in_6addr(&self, pipv_6addr: *mut In6Addr) -> bool {
        trace!(target: "netaddr", "Attempting IPv6 extraction via get_in_6addr");
        if !self.is_ipv6() {
            debug!(target: "netaddr", "get_in_6addr called on non‑IPv6 NetAddr");
            return false;
        }
        if pipv_6addr.is_null() {
            error!(target: "netaddr", "Null pointer passed to get_in_6addr");
            return false;
        }

        assert_eq!(
            self.addr().len(),
            ADDR_IPV6_SIZE,
            "IPv6 NetAddr must contain exactly sixteen bytes"
        );

        // Safety: the caller guarantees that `pipv_6addr` points to valid, writable
        //         memory large enough for sixteen bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.addr().as_ptr(), 
                pipv_6addr as *mut u8, 
                ADDR_IPV6_SIZE
            );
        }

        true
    }
}
