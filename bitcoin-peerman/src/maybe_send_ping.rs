crate::ix!();

pub trait MaybeSendPing {

    fn maybe_send_ping(self: Arc<Self>, 
        node_to: Amo<Box<dyn NodeInterface>>,
        peer:    Amo<Peer>,
        now:     OffsetDateTime /* micros */);
}

impl MaybeSendPing for PeerManager {

    /**
      | Send a ping message every PING_INTERVAL
      | or if requested via RPC. May mark the
      | peer to be disconnected if a ping has
      | timed out.
      | 
      | We use mockable time for ping timeouts,
      | so setmocktime may cause pings to time
      | out.
      |
      */
    fn maybe_send_ping(self: Arc<Self>, 
        node_to: Amo<Box<dyn NodeInterface>>,
        peer:    Amo<Peer>,
        now:     OffsetDateTime /* micros */)  {

        if self.connman.get()
            .should_run_inactivity_checks(
                &mut node_to.get_mut(), 
                now
            ) 
        && peer.get().ping_nonce_sent.load(atomic::Ordering::Relaxed) != 0 
        && now > peer.get().ping_start.load(atomic::Ordering::Relaxed) + TIMEOUT_INTERVAL 
        {
            // The ping timeout is using
            // mocktime. To disable the check
            // during testing, increase
            // -peertimeout.
            log_print!(
                LogFlags::NET,
                "ping timeout: %fs peer=%d\n",0.000001 * count_microseconds(now - peer.get().ping_start.load()),
                peer.get().id
            );

            node_to.get().mark_for_disconnect();

            return;
        }

        let msg_maker: NetMsgMaker = NetMsgMaker::new(node_to.get().get_common_version());

        let mut ping_send: bool = false;

        if peer.get().ping_queued.load(atomic::Ordering::Relaxed) {

            // RPC ping request by user
            ping_send = true;
        }

        if peer.get().ping_nonce_sent.load(atomic::Ordering::Relaxed) == 0 
        && now > peer.get().ping_start.load(atomic::Ordering::Relaxed) + PING_INTERVAL 
        {
            // Ping automatically sent as
            // a latency probe & keepalive.
            ping_send = true;
        }
        
        if ping_send {

            let mut nonce: [u8; 8] = [0; 8];

            while u64::from_be_bytes(nonce) == 0 {

                let size: i32 = size_of_val(&nonce).try_into().unwrap();

                get_rand_bytes(&mut nonce, size);
            }

            peer.get().ping_queued.store(false, atomic::Ordering::Relaxed);
            peer.get().ping_start.store(now, atomic::Ordering::Relaxed);

            if node_to.get().get_common_version() > BIP0031_VERSION {

                peer.get().ping_nonce_sent.store(u64::from_be_bytes(nonce), atomic::Ordering::Relaxed);

                self.connman.get_mut().push_message(
                    &mut node_to.get_mut(), 
                    msg_maker.make(NetMsgType::PING, &[&nonce])
                );

            } else {

                // Peer is too old to support ping
                // command with nonce, pong will
                // never arrive.
                peer.get().ping_nonce_sent.store(0, atomic::Ordering::Relaxed);

                let dummy: Option<i32> = None;

                self.connman.get_mut().push_message(
                    &mut node_to.get_mut(), 
                    msg_maker.make(NetMsgType::PING, &[&dummy])
                );
            }
        }
    }
}
