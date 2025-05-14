// ---------------- [ File: bitcoin-peerman/src/updated_block_tip.rs ]
crate::ix!();

impl UpdatedBlockTip for PeerManager {

    /**
      | Update our best height and announce
      | any block hashes which weren't previously
      | in m_chainman.ActiveChain() to our
      | peers.
      |
      */
    fn updated_block_tip(&mut self, 
        pindex_new:       Option<Arc<BlockIndex>>,
        pindex_fork:      Option<Arc<BlockIndex>>,
        initial_download: bool)  {
        
        self.set_best_height(pindex_new.as_ref().unwrap().n_height);

        set_service_flags_ibd_cache(!initial_download);

        // Don't relay inventory during initial
        // block download.
        if initial_download {
            return;
        }

        // Find the hashes of all blocks that
        // weren't previously in the best chain.
        let mut hashes: Vec<u256> = vec![];

        let mut pindex_to_announce: Option<Arc<BlockIndex>> = pindex_new;

        while pindex_to_announce != pindex_fork {

            hashes.push(pindex_to_announce.as_ref().unwrap().get_block_hash());

            pindex_to_announce = pindex_to_announce.as_ref().unwrap().pprev.clone();

            if hashes.len() == MAX_BLOCKS_TO_ANNOUNCE.try_into().unwrap() {

                // Limit announcements in case of
                // a huge reorganization.
                //
                // Rely on the peer's
                // synchronization mechanism in
                // that case.
                break;
            }
        }

        {
            let mut peer_map = self.peer_map.get();

            for it in peer_map.iter() {

                let peer = it.1.get_mut();

                let mut guard = peer.block_inv_mutex.lock();

                for hash in hashes.iter().rev() {
                    peer.block_inv_mutex.lock().blocks_for_headers_relay.push(hash.clone());
                }
            }
        }

        self.connman.get().wake_message_handler();
    }
}
