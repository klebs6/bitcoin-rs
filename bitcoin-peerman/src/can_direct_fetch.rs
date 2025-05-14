// ---------------- [ File: bitcoin-peerman/src/can_direct_fetch.rs ]
crate::ix!();

pub trait CanDirectFetch {

    fn can_direct_fetch(self: Arc<Self>) -> bool;
}
    
impl CanDirectFetch for PeerManager {

    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn can_direct_fetch(self: Arc<Self>) -> bool {

        let block_time 
        = self.chainman.get().active_chain().tip().as_ref().unwrap().get_block_time();

        let n_pow_target_spacing 
        = self.chainparams.get_consensus().n_pow_target_spacing;
        
        block_time > get_adjusted_time() - n_pow_target_spacing * 20
    }
}
