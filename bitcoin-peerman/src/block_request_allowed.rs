crate::ix!();

pub trait BlockRequestAllowed {

    fn block_request_allowed(self: Arc<Self>, pindex: Option<Arc<BlockIndex>>) -> bool;
}

impl BlockRequestAllowed for PeerManager {

    /**
      | To prevent fingerprinting attacks,
      | only send blocks/headers outside of
      | the active chain if they are no more than
      | a month older (both in time, and in best
      | equivalent proof of work) than the best
      | header chain we know about and we fully-validated
      | them at some point.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn block_request_allowed(self: Arc<Self>, pindex: Option<Arc<BlockIndex>>) -> bool {

        assert_lock_held!(CS_MAIN);

        if self.chainman.get().active_chain().contains(pindex.clone()) {
            return true;
        }

        let proof_equiv_time: Duration = get_block_proof_equivalent_time( 
            PINDEX_BEST_HEADER.lock().clone(), 
            pindex.clone(), 
            PINDEX_BEST_HEADER.lock().clone(), 
            &self.chainparams.get_consensus()
        );

        pindex.as_ref().unwrap().is_valid(Some(BlockStatus::BLOCK_VALID_SCRIPTS)) 
        && PINDEX_BEST_HEADER.lock().is_some()
        && {

            let mut best_block_time: i64 = PINDEX_BEST_HEADER.lock().as_ref().unwrap().get_block_time();
            let this_block_time: i64 = pindex.as_ref().unwrap().get_block_time();

            let best_dt = OffsetDateTime::from_unix_timestamp(best_block_time).unwrap();
            let this_dt = OffsetDateTime::from_unix_timestamp(this_block_time).unwrap();

            let diff: Duration = best_dt - this_dt;

            diff < STALE_RELAY_AGE_LIMIT
        }
        && ( proof_equiv_time < STALE_RELAY_AGE_LIMIT)
    }
}

