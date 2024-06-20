crate::ix!();
    
pub trait ProcessGetCfCheckPt {

    fn process_get_cf_check_pt(self: Arc<Self>, 
        peer: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        recv: &mut DataStream);
}

impl ProcessGetCfCheckPt for PeerManager {

    /**
      | Handle a getcfcheckpt request.
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
    fn process_get_cf_check_pt(self: Arc<Self>, 
        mut peer: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        recv:     &mut DataStream)  {

        let mut filter_type_ser = u8::default();
        let mut stop_hash = u256::default();

        recv.stream_into(&mut filter_type_ser);
        recv.stream_into(&mut stop_hash);

        let filter_type = BlockFilterType::from(filter_type_ser);
        
        let mut stop_index: Option<Arc<BlockIndex>> = None;

        let mut filter_index: Amo<BlockFilterIndex> = Amo::<BlockFilterIndex>::none();

        if !self.clone().prepare_block_filter_request(
            &mut peer,
            filter_type,
            /*start_height=*/ 0,
            &stop_hash,
            /*max_height_diff=*/ u32::MAX,
            &mut stop_index,
            &mut filter_index) 
        {
            return;
        }

        let mut headers: Vec::<u256> 
        = Vec::<u256>::with_capacity(
            usize::try_from(stop_index.as_ref().unwrap().n_height / CFCHECKPT_INTERVAL).unwrap()
        );

        //  Populate headers.
        let mut block_index: Option<Arc<BlockIndex>> = stop_index.clone();

        for i in (0..=headers.len() - 1).rev() {

            let height: i32 = ((i + 1) * usize::try_from(CFCHECKPT_INTERVAL).unwrap()).try_into().unwrap();

            block_index = match block_index {
                Some(index) => index.get_ancestor(height),
                None        => None,
            };

            if !filter_index.get_mut().lookup_filter_header(
                block_index.clone(), 
                &mut headers[i]
            ) {

                log_print!(
                    LogFlags::NET, 
                    "Failed to find block filter header in index: filter_type=%s, block_hash=%s\n", 
                    block_filter_type_name(filter_type), 
                    (*block_index).get_block_hash().to_string()
                );

                return;
            }
        }

        let msg: SerializedNetMsg = 
        NetMsgMaker::new(peer.get_common_version())
            .make(
                NetMsgType::CFCHECKPT, 
                &[
                    &filter_type_ser, 
                    &stop_index.as_ref().unwrap().get_block_hash(), 
                    &headers
                ]
            );

        self.connman.get_mut().push_message(&mut *peer, msg /* move */);
    }
}
