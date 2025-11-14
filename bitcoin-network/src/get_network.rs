// ---------------- [ File: bitcoin-network/src/get_network.rs ]
crate::ix!();

impl NetAddr {
    /// “Reachability network” for outbound selection.
    #[inline]
    pub fn get_network(&self) -> Network {
        if self.is_internal() {
            return Network::NET_INTERNAL;
        }
        if !self.is_routable() {
            return Network::NET_UNROUTABLE;
        }
        *self.net()
    }
}

#[cfg(test)]
mod get_network_tests {
    use super::*;

    #[traced_test]
    fn routable_vs_unroutable() {
        let pub_v4 = NetAddrBuilder::default()
            .addr(PreVector::from(vec![8, 8, 8, 8].as_slice()))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_eq!(pub_v4.get_network(), Network::NET_IPV4);

        let priv_v4 = NetAddrBuilder::default()
            .addr(PreVector::from(vec![10, 0, 0, 1].as_slice()))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_eq!(priv_v4.get_network(), Network::NET_UNROUTABLE);
    }

    #[traced_test]
    fn internal_beats_routability_and_default_is_unroutable() {
        // Default constructed (:: placeholder) is not routable -> NET_UNROUTABLE
        let a = NetAddr::default();
        let n = a.get_network();
        info!(?n, "Default NetAddr get_network()");
        assert_eq!(n, Network::NET_UNROUTABLE);

        // Internal always maps to NET_INTERNAL regardless of byte content
        let mut b = NetAddr::default();
        assert!(b.set_internal("dnsseed.example"));
        let n2 = b.get_network();
        info!(?n2, "Internal NetAddr get_network()");
        assert_eq!(n2, Network::NET_INTERNAL);
    }
}
