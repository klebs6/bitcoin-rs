// ---------------- [ File: bitcoin-peerman/src/process_verack_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_verack_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        msg_maker:          NetMsgMaker,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        if pfrom.is_successfully_connected() {

            log_print!(
                LogFlags::NET, 
                "ignoring redundant verack message from peer=%d\n", 
                pfrom.get_id()
            );

            return;
        }

        if !pfrom.is_inbound_conn() {

            log_printf!(
                "New outbound peer connected: version: %d, blocks=%d, peer=%d%s (%s)\n", 
                pfrom.n_version.load(), 
                (*peer).starting_height, 
                pfrom.get_id(), 
                match LOG_IPS {
                    true   => strprintf(", peeraddr=%s",pfrom.addr.to_string()),
                    false  => ""
                }, 
                pfrom.connection_type_as_string()
            );
        }

        if pfrom.get_common_version() >= SENDHEADERS_VERSION {

            // Tell our peer we prefer to receive
            // headers rather than inv's
            //
            // We send this to non-NODE NETWORK
            // peers as well, because even
            // non-NODE NETWORK peers can announce
            // blocks (such as pruning nodes)
            self.connman.get_mut().push_message(
                &mut *pfrom, 
                msg_maker.make(NetMsgType::SENDHEADERS, &[])
            );
        }

        if pfrom.get_common_version() >= SHORT_IDS_BLOCKS_VERSION {

            // Tell our peer we are willing to
            // provide version 1 or 2 cmpctblocks
            //
            // However, we do not request new
            // block announcements using
            // cmpctblock messages.
            //
            // We send this to non-NODE NETWORK
            // peers as well, because they may
            // wish to request compact blocks from
            // us
            let announce_usingcmpctblock: bool = false;

            let mut n_cmpctblock_version: u64 = 2;

            self.connman.get_mut().push_message(
                &mut *pfrom, 
                msg_maker.make(
                    NetMsgType::SENDCMPCT, 
                    &[
                        &announce_usingcmpctblock, 
                        &n_cmpctblock_version
                    ]
                )
            );

            n_cmpctblock_version = 1;

            self.connman.get_mut().push_message(
                &mut *pfrom, 
                msg_maker.make(
                    NetMsgType::SENDCMPCT, 
                    &[
                        &announce_usingcmpctblock, 
                        &n_cmpctblock_version
                    ]
                )
            );
        }

        pfrom.set_successfully_connected(true);
    }
}
