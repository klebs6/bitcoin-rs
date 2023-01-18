crate::ix!();

pub trait ProcessGetCfHeaders {

    fn process_get_cf_headers(self: Arc<Self>, 
        peer: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        recv: &mut DataStream);
}

impl ProcessGetCfHeaders for PeerManager {

    /**
      | Handle a cfheaders request.
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
      | @param[in] vRecv
      | 
      | The raw message received
      |
      */
    fn process_get_cf_headers(
        self:     Arc<Self>, 
        mut peer: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        recv:     &mut DataStream)  {

        let mut filter_type_ser = u8::default();
        let mut start_height    = u32::default();
        let mut stop_hash       = u256::default();

        recv.stream_into(&mut filter_type_ser);
        recv.stream_into(&mut start_height);
        recv.stream_into(&mut stop_hash);

        let filter_type = BlockFilterType::from(filter_type_ser);
        
        let mut stop_index:   Option<Arc<BlockIndex>> = None;
        let mut filter_index: Amo<BlockFilterIndex>   = amo_none();

        if !self.clone().prepare_block_filter_request(
            &mut peer,
            filter_type,
            start_height,
            &stop_hash,
            MAX_GETCFHEADERS_SIZE,
            &mut stop_index,
            &mut filter_index) 
        {
            return;
        }

        let mut prev_header = u256::default();

        if start_height > 0 {

            let prev_block: Option<Arc<BlockIndex>> 
            = stop_index
                .as_ref()
                .unwrap()
                .clone()
                .get_ancestor(start_height as i32 - 1);

            if !filter_index.get_mut().lookup_filter_header(prev_block, &mut prev_header) {

                log_print!(
                    LogFlags::NET, 
                    "Failed to find block filter header in index: filter_type=%s, block_hash=%s\n", 
                    block_filter_type_name(filter_type), 
                    (*prev_block).get_block_hash().to_string()
                );

                return;
            }
        }

        let mut filter_hashes: Vec<u256> = vec![];

        if !filter_index.get().lookup_filter_hash_range(
            start_height.try_into().unwrap(), 
            stop_index.clone(), 
            &mut filter_hashes) 
        {
            log_print!(
                LogFlags::NET, 
                "Failed to find block filter hashes in index: filter_type=%s, start_height=%d, stop_hash=%s\n", 
                block_filter_type_name(filter_type), 
                start_height, 
                stop_hash.to_string()
            );

            return;
        }

        let msg: SerializedNetMsg = 
        NetMsgMaker::new(peer.get_common_version())
            .make(
                NetMsgType::CFHEADERS, 
                &[
                    &filter_type_ser, 
                    &stop_index.as_ref().unwrap().get_block_hash(), 
                    &prev_header, 
                    &filter_hashes
                ]
            );

        self.connman.get_mut().push_message(&mut *peer, msg /* move */);
    }
}

