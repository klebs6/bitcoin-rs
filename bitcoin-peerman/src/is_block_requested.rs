// ---------------- [ File: bitcoin-peerman/src/is_block_requested.rs ]
crate::ix!();

pub trait IsBlockRequested {

    /**
      | Have we requested this block from a peer
      |
      */
    fn is_block_requested(&self, hash: &u256) -> bool;
}

impl IsBlockRequested for PeerManagerInner {

    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn is_block_requested(&self, hash: &u256) -> bool {
        
        let guard = self.map_blocks_in_flight.lock();

        guard.get(hash).is_some()
    }
}

impl IsBlockRequested for PeerManager {

    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn is_block_requested(&self, hash: &u256) -> bool {
        self.inner.lock().is_block_requested(hash)
    }
}
