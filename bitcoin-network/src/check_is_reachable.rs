// ---------------- [ File: bitcoin-network/src/check_is_reachable.rs ]
crate::ix!();

pub trait CheckIsReachable {

    fn is_reachable(&self) -> bool;
}

impl CheckIsReachable for Network {
    /**
      | @return
      | 
      | true if the network is reachable, false
      | otherwise
      |
      */
    fn is_reachable(&self) -> bool {
        
        todo!();
        /*
        LOCK(cs_mapLocalHost);
        return !vfLimited[net];
        */
    }
}

impl CheckIsReachable for NetAddr {

    /**
      | @return
      | 
      | true if the address is in a reachable
      | network, false otherwise
      |
      */
    fn is_reachable(&self) -> bool {
        self.get_network().is_reachable()
    }
}

#[cfg(test)]
mod reachability_trait_conformance_spec {
    use super::*;

    // Compile‑time conformance check: these functions will fail to compile if
    // Network or NetAddr stop implementing the trait.
    fn _assert_impl_check_is_reachable<T: CheckIsReachable>() {}
    #[traced_test]
    fn trait_is_implemented_for_network_and_netaddr() {
        _assert_impl_check_is_reachable::<Network>();
        _assert_impl_check_is_reachable::<NetAddr>();
        info!("CheckIsReachable is implemented for Network and NetAddr (compile‑time assertion)");
        assert!(true);
    }
}
