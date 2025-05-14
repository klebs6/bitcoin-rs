// ---------------- [ File: bitcoin-peerman/src/process_message.rs ]
crate::ix!();

pub fn read_block_from_disk_with_flat_file_pos(
        block:            &mut Block,
        pos:              &FlatFilePos,
        consensus_params: &ChainConsensusParams) -> bool {
    
    todo!();
        /*
            block.SetNull();

        // Open history file to read
        CAutoFile filein(OpenBlockFile(pos, true), SER_DISK, CLIENT_VERSION);
        if (filein.IsNull()) {
            return error("ReadBlockFromDisk: OpenBlockFile failed for %s", pos.ToString());
        }

        // Read block
        try {
            filein >> block;
        } catch (const std::exception& e) {
            return error("%s: Deserialize or I/O error - %s at %s", __func__, e.what(), pos.ToString());
        }

        // Check the header
        if (!CheckProofOfWork(block.GetHash(), block.nBits, consensusParams)) {
            return error("ReadBlockFromDisk: Errors in block header at %s", pos.ToString());
        }

        // Signet only: check block solution
        if (consensusParams.signet_blocks && !CheckSignetBlockSolution(block, consensusParams)) {
            return error("ReadBlockFromDisk: Errors in block solution at %s", pos.ToString());
        }

        return true;
        */
}

pub fn read_block_from_disk_with_blockindex(
        block:            &mut Block,
        pindex:           Arc<BlockIndex>,
        consensus_params: &ChainConsensusParams) -> bool {
    
    todo!();
        /*
            const FlatFilePos block_pos{
    [&]() { LOCK(cs_main);  return pindex->GetBlockPos() }()
    };

        if (!ReadBlockFromDisk(block, block_pos, consensusParams)) {
            return false;
        }
        if (block.GetHash() != pindex->GetBlockHash()) {
            return error("ReadBlockFromDisk(CBlock&, CBlockIndex*): GetHash() doesn't match index for %s at %s",
                         pindex->ToString(), block_pos.ToString());
        }
        return true;
        */
}

pub trait ProcessMessage {

    /**
      | Process a single message from a peer.
      | Public for fuzz testing
      |
      */
    fn process_message(
        self:               Arc<Self>, 
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool);

}

impl ProcessMessage for PeerManager {

    fn process_message(self: Arc<Self>, 
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        log_print!(
            LogFlags::NET, 
            "received: %s (%u bytes) peer=%d\n", 
            sanitize_string(msg_type), 
            recv.len(), 
            pfrom.get_id()
        );

        let peer: Amo<Peer> = self.get_peer_ref(pfrom.get_id());

        if peer.is_none() {
            return;
        }
        
        if msg_type == NetMsgType::VERSION {
            self.process_version_message(
                &mut peer.getopt_mut(),
                pfrom,
                msg_type,
                recv,
                time_received,
                interrupt_msg_proc
            );
            return;
        }

        if pfrom.n_version() == 0 {
            // Must have a version message before
            // anything else
            log_print!(
                LogFlags::NET,
                "non-version message before version handshake. Message \"%s\" from peer=%d\n",
                sanitize_string(msg_type),
                pfrom.get_id()
            );
            return;
        }

        // At this point, the outgoing message
        // serialization version can't change.
        let msg_maker: NetMsgMaker = NetMsgMaker::new(pfrom.get_common_version());

        if msg_type == NetMsgType::VERACK {

            self.process_verack_message(
                &peer.getopt(),
                msg_maker,
                pfrom,
                msg_type,
                recv,
                time_received,
                interrupt_msg_proc
            );
            return;
        }

        if msg_type == NetMsgType::SENDHEADERS {

            self.process_sendheaders_message(
                &peer.getopt(),
                pfrom,
                msg_type,
                recv,
                time_received,
                interrupt_msg_proc
            );
            return;
        }

        if msg_type == NetMsgType::SENDCMPCT {

            self.process_sendcmpct_message(
                &peer.getopt(),
                pfrom,
                msg_type,
                recv,
                time_received,
                interrupt_msg_proc
            );

            return;
        }

        //  BIP339 defines feature negotiation of
        //  wtxidrelay, which must happen between
        //  VERSION and VERACK to avoid relay
        //  problems from switching after
        //  a connection is up.
        if msg_type == NetMsgType::WTXIDRELAY {

            self.process_wtxidrelay_message(
                &peer.getopt(),
                pfrom,
                msg_type,
                recv,
                time_received,
                interrupt_msg_proc
            );

            return;
        }

        // BIP155 defines feature negotiation of
        // addrv2 and sendaddrv2, which must
        // happen between VERSION and VERACK.
        if msg_type == NetMsgType::SENDADDRV2 {

            self.process_sendaddrv2_message(
                &peer.getopt(),
                pfrom,
                msg_type,
                recv,
                time_received,
                interrupt_msg_proc
            );

            return;
        }

        if !pfrom.is_successfully_connected() {

            log_print!(
                LogFlags::NET, 
                "Unsupported message \"%s\" prior to verack from peer=%d\n", 
                sanitize_string(msg_type), 
                pfrom.get_id()
            );

            return;
        }

        match msg_type {

            NetMsgType::ADDR | NetMsgType::ADDRV2  => {

                self.process_addr_message(
                    &mut peer.getopt_mut(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::INV => {

                self.process_inv_message(
                    &peer.getopt(),
                    &msg_maker,
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::GETDATA => {

                self.process_getdata_message(
                    peer.clone(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::GETBLOCKS => {

                self.process_getblocks_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::GETBLOCKTXN => {

                self.process_getblockstxn_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::GETHEADERS => {

                self.process_getheaders_message(
                    &peer.getopt(),
                    &msg_maker,
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::TX => {

                self.process_tx_message(
                    peer.clone(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::CMPCTBLOCK => {

                self.process_cmpctblock_message(
                    &peer.getopt(),
                    &msg_maker,
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::BLOCKTXN => {

                self.process_blocktxn_message(
                    &peer.getopt(),
                    &msg_maker,
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::HEADERS => {

                self.process_headers_message(
                    &peer.getopt(),
                    msg_maker,
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::BLOCK => {

                self.process_block_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::GETADDR => {

                self.process_getaddr_message(
                    &mut peer.getopt_mut(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::MEMPOOL => {

                self.process_mempool_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::PING => {

                self.process_ping_message(
                    &peer.getopt(),
                    &msg_maker,
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::PONG => {

                self.process_pong_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::FILTERLOAD => {

                self.process_filterload_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::FILTERADD => {

                self.process_filteradd_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::FILTERCLEAR => {

                self.process_filterclear_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::FEEFILTER => {

                self.process_feefilter_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            NetMsgType::GETCFILTERS => {

                self.process_get_filters(pfrom, recv);

                return;
            }

            NetMsgType::GETCFHEADERS => {

                self.process_get_cf_headers(pfrom, recv);

                return;
            }

            NetMsgType::GETCFCHECKPT => {

                self.process_get_cf_check_pt(
                    pfrom, 
                    recv
                );

                return;
            }

            NetMsgType::NOTFOUND => {

                self.process_notfound_message(
                    &peer.getopt(),
                    pfrom,
                    msg_type,
                    recv,
                    time_received,
                    interrupt_msg_proc
                );

                return;
            }

            _ => {}
        }


        // Ignore unknown commands for
        // extensibility
        log_print!(
            LogFlags::NET,
            "Unknown command \"%s\" from peer=%d\n",
            sanitize_string(msg_type),
            pfrom.get_id()
        );
    }
}
