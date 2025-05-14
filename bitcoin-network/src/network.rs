// ---------------- [ File: bitcoin-network/src/network.rs ]
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
