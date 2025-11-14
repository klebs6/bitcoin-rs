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

#[cfg(test)]
mod in_addr_accessors_spec {
    use super::*;

    #[traced_test]
    fn get_in_addr_success_and_failures() {
        // Success
        let v4 = NetAddrBuilder::default()
            .addr(PreVector::from(&[1u8, 2, 3, 4][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();

        let mut out4: InAddr = unsafe { core::mem::zeroed() };
        let ok = v4.get_in_addr(&mut out4 as *mut InAddr);
        assert!(ok);

        let bytes = unsafe {
            core::slice::from_raw_parts(&out4 as *const _ as *const u8, ADDR_IPV4_SIZE)
        };
        assert_eq!(bytes, &[1u8, 2, 3, 4]);

        // Null pointer on IPv4 -> false
        assert!(!v4.get_in_addr(core::ptr::null_mut()));

        // Non‑IPv4 -> false
        let v6 = NetAddrBuilder::default()
            .addr(PreVector::from(&[0u8; ADDR_IPV6_SIZE][..]))
            .net(Network::NET_IPV6)
            .scope_id(0u32)
            .build()
            .unwrap();
        let mut dummy4: InAddr = unsafe { core::mem::zeroed() };
        assert!(!v6.get_in_addr(&mut dummy4 as *mut InAddr));
    }

    #[traced_test]
    fn get_in_6addr_success_and_failures() {
        // Success
        let mut bytes = [0u8; ADDR_IPV6_SIZE];
        bytes[15] = 1; // ::1
        let v6 = NetAddrBuilder::default()
            .addr(PreVector::from(&bytes[..]))
            .net(Network::NET_IPV6)
            .scope_id(0u32)
            .build()
            .unwrap();

        let mut out6: In6Addr = unsafe { core::mem::zeroed() };
        let ok = v6.get_in_6addr(&mut out6 as *mut In6Addr);
        assert!(ok);

        let out_bytes = unsafe {
            core::slice::from_raw_parts(&out6 as *const _ as *const u8, ADDR_IPV6_SIZE)
        };
        assert_eq!(out_bytes, &bytes);

        // Null pointer on IPv6 -> false
        assert!(!v6.get_in_6addr(core::ptr::null_mut()));

        // Non‑IPv6 -> false
        let v4 = NetAddrBuilder::default()
            .addr(PreVector::from(&[9u8, 9, 9, 9][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        let mut dummy6: In6Addr = unsafe { core::mem::zeroed() };
        assert!(!v4.get_in_6addr(&mut dummy6 as *mut In6Addr));
    }
}
