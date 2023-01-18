crate::ix!();

pub trait MaybeSetPeerAsAnnouncingHeaderAndIds {

    fn maybe_set_peer_as_announcing_header_and_ids(self: Arc<Self>, nodeid: NodeId);
}

impl MaybeSetPeerAsAnnouncingHeaderAndIds for PeerManager {

    /**
      | When a peer sends us a valid block, instruct
      | it to announce blocks to us using CMPCTBLOCK
      | if possible by adding its nodeid to the
      | end of lNodesAnnouncingHeaderAndIDs,
      | and keeping that list under a certain
      | size by removing the first element if
      | necessary.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn maybe_set_peer_as_announcing_header_and_ids(self: Arc<Self>, nodeid: NodeId)  {

        assert_lock_held!(CS_MAIN);

        // Never request high-bandwidth mode from
        // peers if we're blocks-only. Our mempool
        // will not contain the transactions
        // necessary to reconstruct the compact
        // block.
        if self.ignore_incoming_txs {
            return;
        }

        let nodestate: Amo<NodeState> = create_state(nodeid);

        if nodestate.is_none() || !nodestate.get().supports_desired_cmpct_version.load(atomic::Ordering::Relaxed) {
            //  Never ask from peers who can't provide witnesses.
            return;
        }
        
        if nodestate.get().provides_header_and_ids.load(atomic::Ordering::Relaxed) {

            let mut num_outbound_hb_peers: i32 = 0;

            let inner = self.inner.lock();

            let mut it = inner.l_nodes_announcing_header_and_ids.iter().peekable();

            while it.peek().is_some() {

                if **it.peek().unwrap() == nodeid {

                    let idx: usize = (**it.peek().unwrap()).try_into().unwrap();

                    self.inner.lock().l_nodes_announcing_header_and_ids.remove(idx);

                    self.inner.lock().l_nodes_announcing_header_and_ids.push_back(nodeid);

                    return;
                }

                let state: Amo<NodeState> = create_state(**it.peek().unwrap());

                if state.is_some() && !state.get().is_inbound.load(atomic::Ordering::Relaxed) {
                    {
                        num_outbound_hb_peers += 1;
                        num_outbound_hb_peers
                    };
                }

                it.next();
            }

            if nodestate.get().is_inbound.load(atomic::Ordering::Relaxed) {

                // If we're adding an inbound HB
                // peer, make sure we're not
                // removing our last outbound HB
                // peer in the process.
                if self.inner.lock().l_nodes_announcing_header_and_ids.len() >= 3 
                && num_outbound_hb_peers == 1 {

                    let remove_node: Amo<NodeState> 
                    = create_state(
                        *self.inner.lock().l_nodes_announcing_header_and_ids.front().unwrap()
                    );

                    if remove_node.is_some() && !remove_node.get().is_inbound.load(atomic::Ordering::Relaxed) {

                        // Put the HB outbound
                        // peer in the second
                        // slot, so that it
                        // doesn't get removed.
                        self.inner.lock().l_nodes_announcing_header_and_ids.swap(0,1);
                    }
                }
            }

            let cself = self.clone();

            let mut maybe_push_message = move |pfrom: Amo<Box<dyn NodeInterface>>| {

                let cself = cself.clone();

                // EXCLUSIVE_LOCKS_REQUIRED(::CS_MAIN)
                assert_lock_held!(CS_MAIN);

                let mut n_cmpctblock_version: u64 = 2;

                if cself.inner.lock().l_nodes_announcing_header_and_ids.len() >= 3 {

                    let ccself = cself.clone();

                    let push_message = move |pnode_stop: Amo<Box<dyn NodeInterface>>| {

                        let ccself = ccself.clone();


                        let msg = {

                            let common_version = pnode_stop.get().get_common_version();
                            let msg_maker      = NetMsgMaker::new(common_version);

                            msg_maker.make(
                                NetMsgType::SENDCMPCT, 
                                &[
                                /*fAnnounceUsingCMPCTBLOCK=*/ &false, 
                                &n_cmpctblock_version
                                ]
                            )
                        };

                        ccself.connman.get_mut().push_message(
                            &mut pnode_stop.get_mut(), 
                            msg 
                        );

                        let mut node_stop = pnode_stop.get_mut();

                        // save BIP152
                        // bandwidth state: we
                        // select peer to be
                        // low-bandwidth
                        node_stop.set_bip152_highbandwidth_to(false);

                        return true;
                    };

                    // As per BIP152, we only
                    // get 3 of our peers to
                    // announce blocks using
                    // compact encodings.
                    cself.connman.get_mut().for_node( 
                        *cself.inner.lock().l_nodes_announcing_header_and_ids.front().unwrap(),
                        &push_message
                    );

                    cself.inner.lock().l_nodes_announcing_header_and_ids.pop_front();
                }

                let msg = {

                    let common_version = pfrom.get().get_common_version();
                    let msg_maker      = NetMsgMaker::new(common_version);

                    msg_maker 
                        .make(
                            NetMsgType::SENDCMPCT, 
                            &[
                            /*fAnnounceUsingCMPCTBLOCK=*/ &true, 
                            &n_cmpctblock_version
                            ]
                        )
                };

                cself.connman.get_mut().push_message(
                    &mut pfrom.get_mut(), 
                    msg
                );

                // save BIP152 bandwidth
                // state: we select peer to be
                // high-bandwidth
                pfrom.get_mut().set_bip152_highbandwidth_to(true);

                cself.inner.lock().l_nodes_announcing_header_and_ids.push_back(
                    pfrom.get().get_id()
                );

                return true;
            };

            self.connman
                .get_mut()
                .for_node_mut(
                    nodeid, 
                    &mut maybe_push_message
                );
        }
    }
}

