crate::ix!();

pub struct QueuedBlockIterator {
    idx:  usize,
    item: Arc<QueuedBlock>,
}

impl Iterator for QueuedBlockIterator {

    type Item = QueuedBlock;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

pub type QueuedBlockIter = CppIter<QueuedBlockIterator>;


pub trait PrepareBlockFilterRequest {

    fn prepare_block_filter_request(
        self:            Arc<Self>,
        peer:            &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        filter_type:     BlockFilterType,
        start_height:    u32,
        stop_hash:       &u256,
        max_height_diff: u32,
        stop_index:      &mut Option<Arc<BlockIndex>>,
        filter_index:    &mut Amo<BlockFilterIndex>) -> bool;
}

impl PrepareBlockFilterRequest for PeerManager {

    /**
      | Validation logic for compact filters
      | request handling.
      | 
      | May disconnect from the peer in the case
      | of a bad request.
      | 
      | -----------
      | @param[in] peer
      | 
      | The peer that we received the request
      | from
      | ----------
      | @param[in] filter_type
      | 
      | The filter type the request is for. Must
      | be basic filters.
      | ----------
      | @param[in] start_height
      | 
      | The start height for the request
      | ----------
      | @param[in] stop_hash
      | 
      | The stop_hash for the request
      | ----------
      | @param[in] max_height_diff
      | 
      | The maximum number of items permitted
      | to request, as specified in BIP 157
      | ----------
      | @param[out] stop_index
      | 
      | The CBlockIndex for the stop_hash block,
      | if the request can be serviced.
      | ----------
      | @param[out] filter_index
      | 
      | The filter index, if the request can
      | be serviced.
      | 
      | -----------
      | @return
      | 
      | True if the request can be serviced.
      |
      */
    fn prepare_block_filter_request(
        self:            Arc<Self>,
        peer:            &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        filter_type:     BlockFilterType,
        start_height:    u32,
        stop_hash:       &u256,
        max_height_diff: u32,
        stop_index:      &mut Option<Arc<BlockIndex>>,
        filter_index:    &mut Amo<BlockFilterIndex>) -> bool {

        let supported_filter_type: bool = 
        filter_type == BlockFilterType::BASIC 
        && (peer.get_local_services() & ServiceFlags::NODE_COMPACT_FILTERS).bits() != 0;

        if !supported_filter_type {

            log_print!(
                LogFlags::NET, 
                "peer %d requested unsupported block filter type: %d\n", 
                peer.get_id(), 
                filter_type as u8
            );

            peer.mark_for_disconnect();

            return false;
        }

        {
            let mut guard = CS_MAIN.lock();

            *stop_index
                = self.chainman
                    .get()
                    .inner
                    .blockman.lookup_block_index(stop_hash);

            // Check that the stop block exists
            // and the peer would be allowed to
            // fetch it.
            if stop_index.is_none() 
            || !self.block_request_allowed(stop_index.clone()) {

                log_print!(
                    LogFlags::NET, 
                    "peer %d requested invalid block hash: %s\n", 
                    peer.get_id(), 
                    stop_hash.to_string()
                );

                peer.mark_for_disconnect();
                return false;
            }
        }

        let stop_height: u32 = stop_index.as_ref().unwrap().n_height.try_into().unwrap();

        if start_height > stop_height {

            log_print!(
                LogFlags::NET, 
                "peer %d sent invalid getcfilters/getcfheaders with start height %d and stop height %d\n", 
                peer.get_id(), 
                start_height, 
                stop_height
            );

            peer.mark_for_disconnect();

            return false;
        }

        if stop_height - start_height >= max_height_diff {

            log_print!(
                LogFlags::NET, 
                "peer %d requested too many cfilters/cfheaders: %d / %d\n", 
                peer.get_id(), 
                stop_height - start_height + 1, 
                max_height_diff
            );

            peer.mark_for_disconnect();

            return false;
        }

        *filter_index = get_block_filter_index(filter_type).clone();

        if filter_index.is_none() {

            log_print!(
                LogFlags::NET, 
                "Filter index for supported type %s not found\n", 
                block_filter_type_name(filter_type)
            );

            return false;
        }

        true
    }
}
