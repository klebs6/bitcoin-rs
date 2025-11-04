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
