// ---------------- [ File: bitcoinnode-interface/src/addr_local.rs ]
crate::ix!();

pub struct NodeAddrLocal {

    /**
      | Our address, as reported by the peer
      |
      */
    pub addr_local: Service,
}

impl Default for NodeAddrLocal {

    fn default() -> Self {
        Self {
            addr_local: Service::default(),
        }
    }
}
