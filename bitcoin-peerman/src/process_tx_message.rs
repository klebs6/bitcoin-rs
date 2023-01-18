crate::ix!();

impl PeerManager {

    pub fn process_tx_message(
        self:               Arc<Self>, 
        peer:               Amo<Peer>,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        //  Stop processing the transaction early
        //  if
        //
        //  1) We are in blocks only mode and peer
        //  has no relay permission
        //
        //  2) This peer is a block-relay-only
        //  peer
        if (self.ignore_incoming_txs && !pfrom.has_permission(NetPermissionFlags::Relay)) 
        || !pfrom.has_tx_relay() {

            log_print!(
                LogFlags::NET, 
                "transaction sent in violation of protocol peer=%d\n", 
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();

            return;
        }

        let mut ptx = TransactionRef::none();

        recv.stream_into(&mut ptx);

        let tx = ptx.get();

        let txid:  &u256 = tx.get_hash();
        let wtxid: &u256 = tx.get_witness_hash();

        let mut guard_main    = CS_MAIN.lock();
        let mut guard_orphans = G_CS_ORPHANS.lock();

        let nodestate: Amo<NodeState> = create_state(pfrom.get_id());

        let hash: &u256 = match nodestate.get().wtxid_relay.load(atomic::Ordering::Relaxed) {
            true   => wtxid,
            false  => txid
        };

        pfrom.add_known_tx(hash);

        if nodestate.get().wtxid_relay.load(atomic::Ordering::Relaxed) 
        && txid != wtxid {

            // Insert txid into
            // filterInventoryKnown, even for
            // wtxidrelay peers. This prevents
            // re-adding of unconfirmed parents to
            // the recently_announced filter, when
            // a child tx is requested. See
            // ProcessGetData().
            pfrom.add_known_tx(txid);
        }

        self.inner.lock().txrequest.lock().received_response(pfrom.get_id(), txid);

        if tx.has_witness() {
            self.inner.lock().txrequest.lock().received_response(pfrom.get_id(), wtxid);
        }

        // We do the AlreadyHaveTx() check using
        // wtxid, rather than txid - in the
        // absence of witness malleation, this is
        // strictly better, because the recent
        // rejects filter may contain the wtxid
        // but rarely contains the txid of
        // a segwit transaction that has been
        // rejected. In the presence of witness
        // malleation, it's possible that by only
        // doing the check with wtxid, we could
        // overlook a transaction which was
        // confirmed with a different witness, or
        // exists in our mempool with a different
        // witness, but this has limited downside:
        // mempool validation does its own lookup
        // of whether we have the txid already;
        // and an adversary can already relay us
        // old transactions (older than our
        // recency filter) if trying to DoS us,
        // without any need for witness
        // malleation.
        if self.clone().already_have_tx(&GenTxId::wtxid(wtxid)) {

            if pfrom.has_permission(NetPermissionFlags::ForceRelay) {

                // Always relay transactions
                // received from peers with
                // forcerelay permission, even
                // if they were already in the
                // mempool, allowing the node
                // to function as a gateway
                // for nodes hidden behind it.
                if !self.mempool.get().exists(&GenTxId::txid(tx.get_hash())) {

                    log_printf!(
                        "Not relaying non-mempool transaction %s from forcerelay peer=%d\n", 
                        tx.get_hash().to_string(), 
                        pfrom.get_id()
                    );

                } else {

                    log_printf!(
                        "Force relaying tx %s from peer=%d\n", 
                        tx.get_hash().to_string(), 
                        pfrom.get_id()
                    );

                    self.clone().relay_transaction(tx.get_hash(), tx.get_witness_hash());
                }
            }

            return;
        }

        let result: MempoolAcceptResult = accept_to_memory_pool(
            self.chainman.get().active_chainstate(),
            self.mempool.clone(),
            ptx.clone(),
            /* bypass_limits */ false,
            None
        );

        let state: &TxValidationState = &result.state;

        if result.result_type == MempoolAcceptResultType::VALID {

            let chainman = self.chainman.get();

            let active_chainstate = chainman.active_chainstate();

            let mempool = self.mempool.get();

            let chain_height = active_chainstate.height().unwrap();

            mempool.check(
                active_chainstate.coins_tip(), 
                (chain_height + 1).try_into().unwrap()
            );

            // As this version of the
            // transaction was acceptable, we
            // can forget about any requests
            // for it.
            {
                let mut inner = self.inner.lock();

                inner.txrequest.lock().forget_tx_hash(tx.get_hash());
                inner.txrequest.lock().forget_tx_hash(tx.get_witness_hash());
            }

            self.clone().relay_transaction(
                tx.get_hash(), 
                tx.get_witness_hash()
            );

            self.orphanage.clone().add_children_to_work_set(
                &tx, 
                &mut peer.get_mut().orphan_work_set
            );

            pfrom.set_n_last_tx_time(Some(get_datetime()));

            log_print!(
                LogFlags::MEMPOOL, 
                "AcceptToMemoryPool: peer=%d: accepted %s (poolsz %u txn, %u kB)\n", 
                pfrom.get_id(), 
                tx.get_hash().to_string(), 
                self.mempool.len(), 
                self.mempool.dynamic_memory_usage() / 1000
            );

            for removed_tx in result.replaced_transactions.as_ref().unwrap().iter() {
                self.clone().add_to_compact_extra_transactions(removed_tx);
            }

            // Recursively process any orphan
            // transactions that depended on
            // this one
            self.clone()
                .process_orphan_tx(&mut peer.get_mut().orphan_work_set);

        } else {

            if state.get_result() == TxValidationResult::TX_MISSING_INPUTS {

                // It may be the case that the
                // orphans parents have all
                // been rejected
                let mut rejected_parents: bool = false;

                // Deduplicate parent txids,
                // so that we don't have to
                // loop over the same parent
                // txid more than once down
                // below.
                let mut unique_parents: Vec<u256> = vec![];

                unique_parents.reserve(tx.vin.len());

                for txin in tx.vin.iter() {
                    // We start with all
                    // parents, and then
                    // remove duplicates
                    // below.
                    unique_parents.push(txin.prevout.hash.clone());
                }

                unique_parents.sort();

                unique_parents.dedup();

                for parent_txid in unique_parents.iter() {
                    if self.inner.lock().recent_rejects.contains_key(parent_txid.as_slice()) {
                        rejected_parents = true;
                        break;
                    }
                }

                if !rejected_parents {

                    let current_time = get_datetime();

                    for parent_txid in unique_parents.iter() {

                        // Here, we only have
                        // the txid (and not
                        // wtxid) of the
                        // inputs, so we only
                        // request in txid
                        // mode, even for
                        // wtxidrelay peers.
                        //
                        // Eventually we
                        // should replace this
                        // with an improved
                        // protocol for
                        // getting all
                        // unconfirmed
                        // parents.
                        let gtxid = GenTxId::txid(parent_txid);

                        pfrom.add_known_tx(parent_txid);

                        if !self.clone().already_have_tx(&gtxid) {

                            self.clone()
                                .add_tx_announcement(
                                    pfrom, 
                                    &gtxid, 
                                    current_time
                                );
                        }
                    }

                    if self.orphanage.clone().add_tx(&ptx, pfrom.get_id()) {
                        self.clone().add_to_compact_extra_transactions(&ptx);
                    }

                    let mut inner = self.inner.lock();

                    // Once added to the
                    // orphan pool, a tx is
                    // considered AlreadyHave,
                    // and we shouldn't
                    // request it anymore.
                    inner.txrequest.lock().forget_tx_hash(tx.get_hash());
                    inner.txrequest.lock().forget_tx_hash(tx.get_witness_hash());

                    // DoS prevention: do not
                    // allow m_orphanage to
                    // grow unbounded (see
                    // CVE-2012-3789)
                    let n_max_orphan_tx: u32 = max(
                        0 as i64,
                        G_ARGS.lock().get_int_arg("-maxorphantx", DEFAULT_MAX_ORPHAN_TRANSACTIONS.into())
                    ) as u32;

                    let mut n_evicted: u32 = self.orphanage.clone().limit_orphans(n_max_orphan_tx);

                    if n_evicted > 0 {

                        log_print!(
                            LogFlags::MEMPOOL, 
                            "orphanage overflow, removed %u tx\n", 
                            n_evicted
                        );
                    }

                } else {

                    log_print!(
                        LogFlags::MEMPOOL, 
                        "not keeping orphan with rejected parents %s\n", 
                        tx.get_hash().to_string()
                    );

                    let mut inner = self.inner.lock();

                    // We will continue to
                    // reject this tx since it
                    // has rejected parents so
                    // avoid re-requesting it
                    // from other peers.
                    //
                    // Here we add both the
                    // txid and the wtxid, as
                    // we know that regardless
                    // of what witness is
                    // provided, we will not
                    // accept this, so we
                    // don't need to allow for
                    // redownload of this txid
                    // from any of our
                    // non-wtxidrelay peers.
                    inner.recent_rejects.insert_key(tx.get_hash().as_slice());
                    inner.recent_rejects.insert_key(tx.get_witness_hash().as_slice());

                    inner.txrequest.lock().forget_tx_hash(tx.get_hash());
                    inner.txrequest.lock().forget_tx_hash(tx.get_witness_hash());
                }

            } else {

                if state.get_result() != TxValidationResult::TX_WITNESS_STRIPPED {

                    // We can add the wtxid of
                    // this transaction to our
                    // reject filter.
                    //
                    // Do not add txids of
                    // witness transactions or
                    // witness-stripped
                    // transactions to the
                    // filter, as they can
                    // have been malleated;
                    // adding such txids to
                    // the reject filter would
                    // potentially interfere
                    // with relay of valid
                    // transactions from peers
                    // that do not support
                    // wtxid-based relay. See
                    // https://github.com/bitcoin/bitcoin/issues/8279
                    // for details.
                    //
                    // We can remove this
                    // restriction (and always
                    // add wtxids to the
                    // filter even for witness
                    // stripped transactions)
                    // once wtxid-based relay
                    // is broadly deployed.
                    //
                    // See also comments in
                    // https://github.com/bitcoin/bitcoin/pull/18044#discussion_r443419034
                    // for concerns around
                    // weakening security of
                    // unupgraded nodes if we
                    // start doing this too
                    // early.
                    self.inner.lock().recent_rejects.insert_key(tx.get_witness_hash().as_slice());

                    self.inner.lock().txrequest.lock().forget_tx_hash(tx.get_witness_hash());

                    // If the transaction
                    // failed for
                    // TX_INPUTS_NOT_STANDARD,
                    // then we know that the
                    // witness was irrelevant
                    // to the policy failure,
                    // since this check
                    // depends only on the
                    // txid (the scriptPubKey
                    // being spent is covered
                    // by the txid).
                    //
                    // Add the txid to the
                    // reject filter to
                    // prevent repeated
                    // processing of this
                    // transaction in the
                    // event that child
                    // transactions are later
                    // received (resulting in
                    // parent-fetching by txid
                    // via the orphan-handling
                    // logic).
                    if state.get_result() == TxValidationResult::TX_INPUTS_NOT_STANDARD 
                    && tx.get_witness_hash() != tx.get_hash() {

                        self.inner.lock().recent_rejects.insert_key(tx.get_hash().as_slice());

                        self.inner.lock().txrequest.lock().forget_tx_hash(tx.get_hash());
                    }

                    if recursive_dynamic_usage(&ptx) < 100000 {
                        self.clone().add_to_compact_extra_transactions(&ptx);
                    }
                }
            }
        }

        // If a tx has been detected by
        // m_recent_rejects, we will have reached
        // this point and the tx will have been
        // ignored. Because we haven't run the tx
        // through AcceptToMemoryPool, we won't
        // have computed a DoS score for it or
        // determined exactly why we consider it
        // invalid.
        //
        // This means we won't penalize any peer
        // subsequently relaying a DoSy tx (even
        // if we penalized the first peer who gave
        // it to us) because we have to account
        // for m_recent_rejects showing false
        // positives. In other words, we shouldn't
        // penalize a peer if we aren't *sure*
        // they submitted a DoSy tx.
        //
        // Note that m_recent_rejects doesn't just
        // record DoSy or invalid transactions,
        // but any tx not accepted by the mempool,
        // which may be due to node policy
        // (vs. consensus). So we can't blanket
        // penalize a peer simply for relaying
        // a tx that our m_recent_rejects has
        // caught, regardless of false positives.
        if state.is_invalid() {

            log_print!(
                LogFlags::MEMPOOLREJ, 
                "%s from peer=%d was not accepted: %s\n", 
                tx.get_hash().to_string(), 
                pfrom.get_id(), 
                state.to_string()
            );

            self.clone().maybe_punish_node_for_tx(pfrom.get_id(), state, None);
        }
    }
}
