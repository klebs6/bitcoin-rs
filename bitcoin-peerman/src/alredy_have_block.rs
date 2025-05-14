// ---------------- [ File: bitcoin-peerman/src/alredy_have_block.rs ]
crate::ix!();

pub trait AlreadyHaveBlock {

    fn already_have_block(self: Arc<Self>, block_hash: &u256) -> bool;
}

impl AlreadyHaveBlock for PeerManager {

    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn already_have_block(self: Arc<Self>, block_hash: &u256) -> bool {
        
        self.chainman.get()
            .inner
            .blockman
            .lookup_block_index(block_hash)
            .is_some()
    }
}
