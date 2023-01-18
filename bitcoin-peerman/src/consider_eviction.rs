crate::ix!();

pub trait ConsiderEviction {

    fn consider_eviction(self: Arc<Self>, 
        pto:             Amo<Box<dyn NodeInterface>>,
        time_in_seconds: OffsetDateTime);
}

impl ConsiderEviction for PeerManager {

    /**
      | Consider evicting an outbound peer
      | based on the amount of time they've been
      | behind our tip
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn consider_eviction(self: Arc<Self>, 
        pto:             Amo<Box<dyn NodeInterface>>,
        time_in_seconds: OffsetDateTime)  {

        assert_lock_held!(CS_MAIN);

        let mut create_state = create_state(pto.get().get_id());

        let mut state = create_state.get_mut();

        let msg_maker: NetMsgMaker = NetMsgMaker::new(pto.get().get_common_version());

        if !state.chain_sync.protect 
        && pto.get().is_outbound_or_block_relay_conn() 
        && state.sync_started.load(atomic::Ordering::Relaxed) {

            //  This is an outbound peer subject to disconnection if they don't
            //  announce a block with as much work as the current tip within
            //  CHAIN_SYNC_TIMEOUT + HEADERS_RESPONSE_TIME seconds (note: if
            //  their chain has more work than ours, we should sync to it,
            //  unless it's invalid, in which case we should find that out and
            //  disconnect from them elsewhere).
            if state.pindex_best_known_block.is_some() 
            && state.pindex_best_known_block.as_ref().unwrap().n_chain_work 
            >= self.chainman.get().active_chain().tip().as_ref().unwrap().n_chain_work {

                if state.chain_sync.timeout.is_some() {
                    state.chain_sync.timeout         = None;
                    state.chain_sync.work_header     = None;
                    state.chain_sync.sent_getheaders = false;
                }

            } else {

                if state.chain_sync.timeout.is_none()
                || (
                    state.chain_sync.work_header.is_some() 
                    && state.pindex_best_known_block.is_some() 
                    && state.pindex_best_known_block.as_ref().unwrap().n_chain_work 
                    >= state.chain_sync.work_header.as_ref().unwrap().n_chain_work
                ) {
                    // Our best block known by
                    // this peer is behind our
                    // tip, and we're either
                    // noticing that for the first
                    // time, OR this peer was able
                    // to catch up to some earlier
                    // point where we checked
                    // against our tip.
                    //
                    // Either way, set a new
                    // timeout based on current
                    // tip.
                    state.chain_sync.timeout         = Some(time_in_seconds + CHAIN_SYNC_TIMEOUT);
                    state.chain_sync.work_header     = self.chainman.get().active_chain().tip();
                    state.chain_sync.sent_getheaders = false;

                } else {

                    if state.chain_sync.timeout.is_some() 
                    && Some(time_in_seconds) > state.chain_sync.timeout {

                        // No evidence yet that
                        // our peer has synced to
                        // a chain with work equal
                        // to that of our tip,
                        // when we first detected
                        // it was behind. Send
                        // a single getheaders
                        // message to give the
                        // peer a chance to update
                        // us.
                        if state.chain_sync.sent_getheaders {

                            //  They've run out of time to catch up!
                            log_printf!(
                                "Disconnecting outbound peer %d for old chain, best known block = %s\n",
                                pto.get_id(),
                                match state.pindex_best_known_block.is_some() {
                                    true   => (*state.pindex_best_known_block()).get_block_hash().to_string(),
                                    false  => "<none>"
                                }
                            );

                            pto.get_mut().mark_for_disconnect();

                        } else {

                            assert!(state.chain_sync.work_header.is_some());

                            log_print!(
                                LogFlags::NET, 
                                "sending getheaders to outbound peer=%d to verify chain work (current best known block:%s, benchmark blockhash: %s)\n", 
                                pto.get_id(), 
                                match state.pindex_best_known_block.is_some() {
                                    true   => (*state.pindex_best_known_block()).get_block_hash().to_string(),
                                    false  => "<none>"
                                }, 
                                (*state.chain_sync().work_header()).get_block_hash().to_string()
                            );

                            let msg = {

                                let chainman     = self.chainman.get();
                                let active_chain = chainman.active_chain();
                                let work_header  = state.chain_sync.work_header.as_ref().unwrap();
                                let locator      = active_chain.get_locator(work_header.pprev.clone());

                                msg_maker.make(
                                    NetMsgType::GETHEADERS, 

                                    &[
                                        &locator, 
                                        &u256::default()
                                    ]
                                )
                            };

                            self.connman
                                .get_mut()
                                .push_message(
                                    &mut pto.get_mut(), 

                                    msg
                                );

                            state.chain_sync.sent_getheaders = true;

                            //  2 minutes
                            pub const HEADERS_RESPONSE_TIME: Duration = Duration::minutes(2);

                            // Bump the timeout to allow a response, which could clear the timeout
                            // (if the response shows the peer has synced), reset the timeout (if
                            // the peer syncs to the required work but not to our tip), or result
                            // in disconnect (if we advance to the timeout and pindexBestKnownBlock
                            // has not sufficiently progressed)
                            state.chain_sync.timeout = Some(time_in_seconds + HEADERS_RESPONSE_TIME);
                        }
                    }
                }
            }
        }
    }
}
