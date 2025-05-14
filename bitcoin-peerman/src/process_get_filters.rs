// ---------------- [ File: bitcoin-peerman/src/process_get_filters.rs ]
crate::ix!();

pub trait ProcessGetFilters {
    fn process_get_filters(self: Arc<Self>, 
        peer: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        recv: &mut DataStream);
}

impl ProcessGetFilters for PeerManager {

    /**
      | Handle a filters request.
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
    fn process_get_filters(
        self:     Arc<Self>, 
        mut peer: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        recv:     &mut DataStream)  {

        let mut filter_type_ser: u8  = 0;
        let mut start_height:    u32 = 0;

        let mut stop_hash = u256::default();

        recv.stream_into(&mut filter_type_ser);
        recv.stream_into(&mut start_height);
        recv.stream_into(&mut stop_hash);

        let filter_type = BlockFilterType::from(filter_type_ser);

        let mut stop_index:   Option<Arc<BlockIndex>> = None;
        let mut filter_index: Amo<BlockFilterIndex>   = Amo::<BlockFilterIndex>::none();

        if !self.clone()
            .prepare_block_filter_request(
                peer,
                filter_type,
                start_height,
                &stop_hash,
                MAX_GETCFILTERS_SIZE,
                &mut stop_index,
                &mut filter_index) 
        {
            return;
        }

        let mut filters: Vec<BlockFilter> = vec![];

        if !filter_index.get().lookup_filter_range(
            start_height.try_into().unwrap(), 
            stop_index.clone(), 
            &mut filters) 
        {
            log_print!(
                LogFlags::NET, 
                "Failed to find block filter in index: filter_type=%s, start_height=%d, stop_hash=%s\n", 
                block_filter_type_name(filter_type), 
                start_height, 
                stop_hash.to_string()
            );

            return;
        }

        for filter in filters.iter() {

            let common_version = peer.get_common_version();

            let msg_maker = NetMsgMaker::new(common_version);

            let msg: SerializedNetMsg = msg_maker.make(NetMsgType::CFILTER, &[filter]);

            self.connman.get_mut().push_message(&mut *peer, msg /* move */);
        }
    }
}
