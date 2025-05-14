// ---------------- [ File: bitcoin-peerman/src/maybe_send_addr.rs ]
crate::ix!();

pub trait MaybeSendAddr {

    fn maybe_send_addr(self: Arc<Self>, 
        node:         Amo<Box<dyn NodeInterface>>,
        peer:         &mut Peer,
        current_time: OffsetDateTime /* micros */);
}

impl MaybeSendAddr for PeerManager {

    /**
      | Send `addr` messages on a regular schedule.
      |
      */
    fn maybe_send_addr(self: Arc<Self>, 
        node:         Amo<Box<dyn NodeInterface>>,
        peer:         &mut Peer,
        current_time: OffsetDateTime /* micros */)  {

        // Nothing to do for non-address-relay
        // peers
        if !peer.addr_relay_enabled.load(atomic::Ordering::Relaxed) {
            return;
        }

        let mut guard = peer.addr_send_times_mutex.lock();

        // Periodically advertise our local
        // address to the peer.
        if *LISTEN 
        && !self.chainman.get().active_chainstate().is_initial_block_download() 
        && guard.next_local_addr_send < Some(current_time) {

            // If we've sent before, clear the
            // bloom filter for the peer, so that
            // our self-announcement will actually
            // go out.
            //
            // This might be unnecessary if the
            // bloom filter has already rolled
            // over since our last
            // self-announcement, but there is
            // only a small bandwidth cost that we
            // can incur by doing this (which
            // happens once a day on average).
            if guard.next_local_addr_send != None {
                peer.addr_known.as_mut().unwrap().reset();
            }

            let local_addr: Option::<Address> = get_local_addr_for_peer(
                node.clone()
            );

            if let Some(local_addr) = local_addr {

                let mut insecure_rand = FastRandomContext::default();

                peer.push_address(
                    &local_addr, 
                    &mut insecure_rand
                );
            }

            guard.next_local_addr_send = Some(poisson_next_send(current_time,AVG_LOCAL_ADDRESS_BROADCAST_INTERVAL));
        }

        // We sent an `addr` message to this peer
        // recently. Nothing more to do.
        //
        if Some(current_time) <= guard.next_addr_send {
            return;
        }

        guard.next_addr_send = Some(poisson_next_send(current_time,AVG_ADDRESS_BROADCAST_INTERVAL));

        let mut guard = peer.addrs_to_send.lock();

        if !assume!(guard.len() <= MAX_ADDR_TO_SEND) {

            // Should be impossible since we
            // always check len before adding to
            // m_addrs_to_send. Recover by
            // trimming the vector.
            //
            guard.resize(MAX_ADDR_TO_SEND, Default::default());
        }

        // Remove addr records that the peer
        // already knows about, and add new addrs
        // to the m_addr_known filter on the same
        // pass.
        //
        let mut addr_already_known = |addr: &Address| {

            let ret: bool 
                = peer
                .addr_known
                .as_ref()
                .unwrap()
                .contains_key(
                    &addr.get_key()
                );

            if !ret {
                peer.addr_known.as_mut().unwrap().insert_key(&addr.get_key());
            }

            return ret;
        };

        guard.retain(|item| 

            !addr_already_known(item)
        );

        // No addr messages to send
        if guard.is_empty() {
            return;
        }

        let mut msg_type: Option<String> = None;

        let mut make_flags: i32 = 0;

        if peer.wants_addrv2.load(atomic::Ordering::Relaxed) {
            msg_type = Some(NetMsgType::ADDRV2.into());
            make_flags = ADDRV2_FORMAT;
        } else {
            msg_type = Some(NetMsgType::ADDR.into());
            make_flags = 0;
        }

        let msg = {

            let common_version = node.get().get_common_version();

            let msg_maker      = NetMsgMaker::new(common_version);

            msg_maker.make_with_flags(
                make_flags, 
                &msg_type.unwrap(), 
                &[ &guard.clone() ]
            )
        };

        self.connman.get_mut().push_message(
            &mut node.get_mut(), 
            msg
        );

        guard.clear();

        // we only send the big addr message once
        if guard.capacity() > 40 {
            guard.shrink_to_fit();
        }
    }
}
