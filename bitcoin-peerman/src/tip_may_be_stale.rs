// ---------------- [ File: bitcoin-peerman/src/tip_may_be_stale.rs ]
crate::ix!();

pub trait TipMayBeStale {

    fn tip_may_be_stale(self: Arc<Self>) -> bool;
}

impl TipMayBeStale for PeerManager {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn tip_may_be_stale(self: Arc<Self>) -> bool {
        
        assert_lock_held!(CS_MAIN);

        let consensus_params: Arc<ChainConsensusParams> 
        = self.chainparams.get_consensus();

        if self.last_tip_update.load(atomic::Ordering::Relaxed).is_none() {
            self.last_tip_update.store(Some(get_datetime()), atomic::Ordering::Relaxed);
        }

        let last_tip_update = self.last_tip_update.load(atomic::Ordering::Relaxed);

        let inner = self.inner.lock();
        let mbif = inner.map_blocks_in_flight.lock();

        last_tip_update < OffsetDateTime::from_unix_timestamp(get_datetime().unix_timestamp() - consensus_params.n_pow_target_spacing * 3).ok()
        && mbif.is_empty()
    }
}
