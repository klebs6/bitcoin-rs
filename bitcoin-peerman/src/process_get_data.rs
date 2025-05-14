// ---------------- [ File: bitcoin-peerman/src/process_get_data.rs ]
crate::ix!();

pub trait ProcessGetData {

    fn process_get_data(
        self:               Arc<Self>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        peer:               Amo<Peer>,
        interrupt_msg_proc: &AtomicBool);
}

impl ProcessGetData for PeerManager {

    #[EXCLUSIVE_LOCKS_REQUIRED(peer.m_getdata_requests_mutex)]
    #[LOCKS_EXCLUDED(CS_MAIN)]
    fn process_get_data(
        self:               Arc<Self>,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        peer:               Amo<Peer>,
        interrupt_msg_proc: &AtomicBool)  {

        assert_lock_not_held!(CS_MAIN);

        let gpeer = peer.get();

        let mut guard = gpeer.getdata_requests.lock();

        let mut it = guard.iter().enumerate().peekable();

        let mut not_found: Vec<Inv> = vec![];

        let msg_maker: NetMsgMaker = NetMsgMaker::new(pfrom.get_common_version());

        let now = get_datetime();

        //  Get last mempool request time
        let mempool_req: Option<OffsetDateTime> = match pfrom.has_tx_relay() {

            true   => 
                pfrom.get_tx_relay()
                    .last_mempool_req
                    .load(atomic::Ordering::Relaxed),

            false  => None,
        };
        
        // Process as many TX items from the front
        // of the getdata queue as possible, since
        // they're common and it's efficient to
        // batch process them.
        while it.peek().is_some() && it.peek().unwrap().1.is_gen_tx_msg() {

            if interrupt_msg_proc.load(atomic::Ordering::Relaxed) {
                return;
            }

            // The send buffer provides
            // backpressure. If there's no space
            // in the buffer, pause processing
            // until the next call.
            if pfrom.send_paused() {
                break;
            }

            let inv: &Inv = it.peek().unwrap().1;

            it.next();

            if !pfrom.has_tx_relay() {

                // Ignore GETDATA requests for
                // transactions from blocks-only
                // peers.
                continue;
            }

            let tx: TransactionRef 
            = self.clone().find_tx_for_get_data(
                &***pfrom,
                &(inv.clone()).into(),
                mempool_req,
                now
            );

            if tx.is_some() {

                // WTX and WITNESS_TX imply we serialize with witness
                let n_send_flags: i32 = (match inv.is_msg_tx() {
                    true   => SERIALIZE_TRANSACTION_NO_WITNESS,
                    false  => 0
                });

                self.connman.get_mut().push_message(
                    &mut *pfrom, 
                    msg_maker.make_with_flags(
                        n_send_flags, 
                        NetMsgType::TX, 
                        &[
                            &tx.clone()
                        ]
                    )
                );

                self.mempool
                    .get_mut()
                    .remove_unbroadcast_tx(tx.get().get_hash(), None);

                // As we're going to send tx,
                // make sure its unconfirmed
                // parents are made requestable.
                let mut parent_ids_to_add: Vec<u256> = vec![];

                {
                    let mempool = self.mempool.get();

                    let mut guard = mempool.cs.lock();

                    let txiter = mempool.get_iter(
                        tx.get().get_hash()
                    );

                    if txiter.is_some() {

                        let parents: &TxMemPoolEntryParents 
                        = txiter.as_ref().unwrap().get_mem_pool_parents_const::<&TxMemPoolEntryParents>();

                        parent_ids_to_add.reserve(parents.len());

                        for parent in parents.iter() {

                            if parent.get().get_time() > now - UNCONDITIONAL_RELAY_DELAY {

                                let parent    = parent.get();
                                let parent_tx = parent.get_tx();

                                let parent_tx_guard = parent_tx.get();

                                let parent_hash = parent_tx_guard.get_hash();

                                parent_ids_to_add.push(parent_hash.clone());
                            }
                        }
                    }
                }

                for parent_txid in parent_ids_to_add.iter() {

                    // Relaying a transaction with
                    // a recent but unconfirmed
                    // parent.
                    if {

                        let tx_relay = pfrom.get_tx_relay();

                        let mut guard = tx_relay.cs_tx_inventory.lock();

                        !pfrom.get_tx_relay()
                            .cs_tx_inventory.lock()
                            .filter_inventory_known
                            .contains_key(parent_txid.as_slice())
                    } 
                    {
                        let mut guard = CS_MAIN.lock();

                        create_state(pfrom.get_id())
                            .get_mut()
                            .recently_announced_invs
                            .insert_key(parent_txid.as_slice());
                    }
                }

            } else {
                not_found.push(inv.clone());
            }
        }

        // Only process one BLOCK item per call,
        // since they're uncommon and can be
        // expensive to process.
        if it.peek().is_some() 
        && !pfrom.send_paused() {

            let inv: &Inv = it.peek().unwrap().1;

            it.next();

            if inv.is_gen_blk_msg() {

                self.clone().process_get_block_data(
                    &mut pfrom, 
                    peer.clone(), 
                    inv
                );
            }

            //  else: If the first item on the queue is an unknown type, we erase it
            //  and continue processing the queue on the next call.
        }

        {
            let idx = it.peek().unwrap().0;
            gpeer.getdata_requests.lock().drain(0..idx);
        }

        if !not_found.is_empty() {

            //  Let the peer know that we didn't
            //  find what it asked for, so it
            //  doesn't have to wait around
            //  forever.
            //
            //  SPV clients care about this
            //  message: it's needed when they are
            //  recursively walking the
            //  dependencies of relevant
            //  unconfirmed transactions. SPV
            //  clients want to do that because
            //  they want to know about (and store
            //  and rebroadcast and risk analyze)
            //  the dependencies of transactions
            //  relevant to them, without having
            //  to download the entire memory
            //  pool.
            //
            //  Also, other nodes can use these
            //  messages to automatically request
            //  a transaction from some other peer
            //  that annnounced it, and stop
            //  waiting for us to respond.
            //
            //  In normal operation, we often send
            //  NOTFOUND messages for parents of
            //  transactions that we relay; if
            //  a peer is missing a parent, they
            //  may assume we have them and
            //  request the parents from us.
            self.connman.get_mut().push_message(
                &mut *pfrom, 
                msg_maker.make(
                    NetMsgType::NOTFOUND, 
                    &[
                        &not_found
                    ]
                )
            );
        }
    }
}
