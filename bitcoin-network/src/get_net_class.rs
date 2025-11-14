// ---------------- [ File: bitcoin-network/src/get_net_class.rs ]
crate::ix!();

impl NetAddr {
    
    /// Categorise the address for AddrMan bucketing.
    #[inline]
    pub fn get_net_class(&self) -> Network {

        // Make sure that if we return NET_IPV6, then IsIPv6() is true. The
        // callers expect that.
        //
        // Check for "internal" first because such addresses are also
        // !IsRoutable() and we don't want to return NET_UNROUTABLE in that
        // case.
        //
        if self.is_internal() {
            return Network::NET_INTERNAL;
        }
        if !self.is_routable() {
            return Network::NET_UNROUTABLE;
        }
        if self.has_linked_ipv4() {
            return Network::NET_IPV4; // treat mapped/tunnelled as IPv4
        }
        *self.net()
    }
}

#[cfg(test)]
mod net_class_tests {
    use super::*;

    #[traced_test]
    fn internal_priority() {
        let mut a = NetAddr::default();
        a.set_net(Network::NET_INTERNAL);
        assert_eq!(a.get_net_class(), Network::NET_INTERNAL);
    }

    #[traced_test]
    fn mapped_ipv4_is_ipv4_class() {
        let mut bytes = [0u8; 16];
        bytes[..12].copy_from_slice(&[0x00, 0x64, 0xFF, 0x9B, 0, 0, 0, 0, 0, 0, 0, 0]);
        bytes[12..].copy_from_slice(&[1, 2, 3, 4]);
        let ip = NetAddrBuilder::default()
            .addr(PreVector::from(bytes.to_vec().as_slice()))
            .net(Network::NET_IPV6)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_eq!(ip.get_net_class(), Network::NET_IPV4);
    }
}

#[cfg(test)]
mod net_addr_core_spec {
    use super::*;

    #[traced_test]
    fn default_is_unspecified_ipv6() {
        let a = NetAddr::default();
        assert_eq!(*a.net(), Network::NET_IPV6);
        assert_eq!(a.addr().len(), 0, "Default uses empty addr payload (:: placeholder)");
    }

    #[traced_test]
    fn ordering_and_equality_are_consistent() {
        let v4a = NetAddrBuilder::default()
            .addr(PreVector::from(&[1u8,2,3,4][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        let v4b = NetAddrBuilder::default()
            .addr(PreVector::from(&[1u8,2,3,4][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_eq!(v4a, v4b);

        let v6 = NetAddrBuilder::default()
            .addr(PreVector::from(&[0u8;16][..]))
            .net(Network::NET_IPV6)
            .scope_id(0u32)
            .build()
            .unwrap();

        // Enum order puts NET_IPV4 (1) before NET_IPV6 (2)
        assert!(v4a < v6);
    }

    #[traced_test]
    fn construct_from_inaddr_and_in6addr() {
        // IPv4
        let mut raw4: InAddr = unsafe { core::mem::zeroed() };
        let dst = unsafe { core::slice::from_raw_parts_mut(&mut raw4 as *mut _ as *mut u8, 4) };
        dst.copy_from_slice(&[203, 0, 113, 6]);

        let a4 = NetAddr::from(&raw4);
        assert!(a4.is_ipv4());
        assert_eq!(a4.addr().as_slice(), &[203,0,113,6]);

        // IPv6
        let mut raw6: In6Addr = unsafe { core::mem::zeroed() };
        let dst6 = unsafe { core::slice::from_raw_parts_mut(&mut raw6 as *mut _ as *mut u8, 16) };
        let mut bytes = [0u8; 16];
        bytes[15] = 1;
        dst6.copy_from_slice(&bytes);

        let a6 = NetAddr::new(&raw6, Some(7));
        assert!(a6.is_ipv6());
        assert_eq!(*a6.scope_id(), 7);
        assert_eq!(a6.addr().as_slice(), &bytes);
    }

    #[traced_test]
    fn v1_compat_bytes_and_hash_behave_as_expected() {
        // IPv4 should serialize as IPv4‑in‑IPv6 in v1 array
        let v4 = NetAddrBuilder::default()
            .addr(PreVector::from(&[9u8,9,9,9][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        let bytes = v4.get_addr_bytes();
        assert_eq!(bytes.len(), NET_ADDR_V1_SERIALIZATION_SIZE);
        assert_eq!(&bytes[..IPV4_IN_IPV6_PREFIX.len()], &IPV4_IN_IPV6_PREFIX);
        assert_eq!(&bytes[IPV4_IN_IPV6_PREFIX.len()..], &[9u8,9,9,9]);

        // Hash stability and difference
        let v4b = v4.clone();
        assert_eq!(v4.get_hash(), v4b.get_hash());

        let v4c = NetAddrBuilder::default()
            .addr(PreVector::from(&[1u8,2,3,4][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_ne!(v4.get_hash(), v4c.get_hash());
    }
}
