// ---------------- [ File: bitcoin-peerman/src/remove_block_request.rs ]
crate::ix!();

pub trait RemoveBlockRequest {

    /**
      | Remove this block from our tracked requested
      | blocks. Called if:
      | 
      | - the block has been received from a peer
      | 
      | - the request for the block has timed
      | out
      |
      */
    fn remove_block_request(&self, hash: &u256);
}

impl RemoveBlockRequest for PeerManager {

    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn remove_block_request(
        &self,
        hash: &u256)
    {
        self.inner.lock().remove_block_request(hash)
    }
}

impl PeerManagerInner {

    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    pub fn remove_block_request(&self, hash: &u256)  {

        let mut map_blocks_in_flight = self.map_blocks_in_flight.lock();

        if let Some(mut it) = map_blocks_in_flight.get_mut(hash) {

            let (node_id,list_it) = it;

            let created_state: Amo<NodeState> = create_state(*node_id);

            assert!(created_state.is_some());

            let mut state = created_state.get_mut();

            let mut bif_it = state.blocks_in_flight.iter().peekable();

            if **bif_it.peek().unwrap() == list_it.peek().unwrap().1 {

                // First block on the queue was
                // received, update the start download
                // time for the next one
                state.downloading_since 
                    = max(state.downloading_since, get_datetime());
            }

            let eraseme = list_it.peek().unwrap().0.clone();

            state.blocks_in_flight.remove(eraseme);

            state.n_blocks_in_flight.fetch_sub(1,atomic::Ordering::Relaxed);

            if state.n_blocks_in_flight.load(atomic::Ordering::Relaxed) == 0 {

                // Last validated block on the
                // queue was received.
                self.peers_downloading_from.fetch_sub(1,atomic::Ordering::Relaxed);
            }

            state.stalling_since = OffsetDateTime::from_unix_timestamp(0).ok();

            map_blocks_in_flight.remove(hash);

        } else {

            // Block was not requested
            return;
        }
    }
}
