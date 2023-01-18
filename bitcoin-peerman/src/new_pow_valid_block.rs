crate::ix!();

/**
  | Determine if a deployment is active
  | for this block
  |
  */
#[inline] pub fn deployment_active_at_with_buried_deployment(
        index:  Arc<BlockIndex>,
        params: &ChainConsensusParams,
        dep:    ConsensusBuriedDeployment) -> bool {
    
    todo!();
        /*
            assert(consensus::ValidDeployment(dep));
        return index.nHeight >= params.DeploymentHeight(dep);
        */
}

#[inline] pub fn deployment_active_at_with_deployment_pos(
        index:  &BlockIndex,
        params: &ChainConsensusParams,
        dep:    ConsensusDeploymentPos) -> bool {
    
    todo!();
        /*
            assert(consensus::ValidDeployment(dep));
        return DeploymentActiveAfter(index.pprev, params, dep);
        */
}

pub trait NewPowValidBlock {

    fn new_pow_valid_block(self: Arc<Self>, 
        pindex: Arc<BlockIndex>,
        pblock: Amo<Block>);
}

impl NewPowValidBlock for PeerManager {

    /**
      | Maintain state about the best-seen
      | block and fast-announce a compact block
      | to compatible peers.
      |
      */
    fn new_pow_valid_block(self: Arc<Self>, 
        pindex: Arc<BlockIndex>,
        pblock: Amo<Block>)  {
        
        let pcmpctblock: Amo<BlockHeaderAndShortTxIDs> 
        = Amo::<BlockHeaderAndShortTxIDs>::from(BlockHeaderAndShortTxIDs::new(pblock.clone(),true));

        let msg_maker: NetMsgMaker = NetMsgMaker::new(PROTOCOL_VERSION);

        let mut main_guard = CS_MAIN.lock();

        lazy_static!{
            static ref N_HIGHEST_FAST_ANNOUNCE: AtomicI32 = AtomicI32::new(0);
        }

        if pindex.n_height <= N_HIGHEST_FAST_ANNOUNCE.load(atomic::Ordering::Relaxed) {
            return;
        }

        N_HIGHEST_FAST_ANNOUNCE.store(pindex.n_height, atomic::Ordering::Relaxed);

        let witness_enabled: bool 
        = deployment_active_at_with_buried_deployment(
            pindex.clone(),
            &self.chainparams.get_consensus(),
            ConsensusBuriedDeployment::DEPLOYMENT_SEGWIT
        );

        let hash_block: u256 = pblock.get().get_hash();

        {
            let mut most_recent_guard = CS_MOST_RECENT_BLOCK.get();

            MOST_RECENT_BLOCK_HASH.replace(&hash_block);
            MOST_RECENT_BLOCK.replace(&pblock.get());
            MOST_RECENT_COMPACT_BLOCK.replace(&pcmpctblock.get());

            WITNESSES_PRESENT_IN_MOST_RECENT_COMPACT_BLOCK.store(witness_enabled, atomic::Ordering::Relaxed);
        }

        let peerman = self.clone();

        let maybe_send_header = move |pnode: Amo<Box<dyn NodeInterface>>| {

            let pindex = pindex.clone();

            // EXCLUSIVE_LOCKS_REQUIRED(::CS_MAIN)
            assert_lock_held!(CS_MAIN);

            let mut pnode = pnode.get_mut();

            let version_gate: bool = pnode.get_common_version() < INVALID_CB_NO_BAN_VERSION;
            let disconnect:   bool = pnode.marked_for_disconnect();

            // TODO: Avoid the repeated-serialization here
            if !version_gate && !disconnect {

                peerman.clone().process_block_availability(pnode.get_id());

                let created_state = create_state(pnode.get_id());

                let mut state = created_state.get_mut();

                // If the peer has, or we announced to
                // them the previous block already,
                // but we don't think they have this
                // one, go ahead and announce it
                if state.prefer_header_and_ids.load(atomic::Ordering::Relaxed) 
                && (!witness_enabled || state.wants_cmpct_witness.load(atomic::Ordering::Relaxed)) 
                && !peer_has_header_with_amo(&state,pindex.clone()) 
                && peer_has_header_with_arc(&state,pindex.clone().pprev.as_ref().unwrap().clone()) 
                {
                    log_print!(
                        LogFlags::NET, 
                        "%s sending header-and-ids %s to peer=%d\n", 
                        "PeerManager::NewPoWValidBlock", 
                        hash_block.to_string(), 
                        (*pnode).get_id()
                    );

                    peerman.connman.get_mut().push_message(
                        &mut pnode, 
                        msg_maker.make(
                            NetMsgType::CMPCTBLOCK, 
                            &[
                                &pcmpctblock
                            ]
                        )
                    );

                    state.pindex_best_header_sent = Some(pindex);
                }
            }
        };

        self.connman
            .get_mut()
            .for_each_node(&maybe_send_header);
    }
}
