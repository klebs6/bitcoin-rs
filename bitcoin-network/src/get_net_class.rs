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
