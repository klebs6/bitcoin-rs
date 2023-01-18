crate::ix!();

pub trait ProcessOrphanTx {

    fn process_orphan_tx(self: Arc<Self>, orphan_work_set: &mut HashSet<u256>);
}

impl ProcessOrphanTx for PeerManager {

    /**
      | Reconsider orphan transactions after
      | a parent has been accepted to the mempool.
      | 
      | -----------
      | @param[in,out] orphan_work_set
      | 
      | The set of orphan transactions to reconsider.
      | Generally only one orphan will be reconsidered
      | on each call of this function. This set
      | may be added to if accepting an orphan
      | causes its children to be reconsidered.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN, G_CS_ORPHANS)]
    fn process_orphan_tx(self: Arc<Self>, orphan_work_set: &mut HashSet<u256>)  {

        assert_lock_held!(CS_MAIN);
        assert_lock_held!(G_CS_ORPHANS);

        while !orphan_work_set.is_empty() {

            let mut iter = orphan_work_set.iter().peekable();

            let orphan_hash: u256 = (*iter.peek().unwrap()).clone();

            orphan_work_set.remove(&orphan_hash);

            let (porphan_tx,from_peer) = self.orphanage.get_tx(&orphan_hash);

            let porphan_tx = self.orphanage.get_tx(&orphan_hash).0;

            if porphan_tx.is_none() {
                continue;
            }

            let result: MempoolAcceptResult = accept_to_memory_pool(
                self.chainman.get().active_chainstate(),
                self.mempool.clone(),
                porphan_tx.clone(),
                /* bypass_limits */ false,
                None
            );

            let state: &TxValidationState = &result.state;

            if result.result_type == MempoolAcceptResultType::VALID {

                log_print!(
                    LogFlags::MEMPOOL, 
                    "   accepted orphan tx %s\n", 
                    orphan_hash.to_string()
                );

                self.clone().relay_transaction(
                    &orphan_hash, 
                    porphan_tx.get().get_witness_hash()
                );

                self.orphanage.add_children_to_work_set(&porphan_tx.get(), orphan_work_set);

                self.orphanage.clone().erase_tx(&orphan_hash);

                for removed_tx in result.replaced_transactions.as_ref().unwrap().iter() {
                    self.clone().add_to_compact_extra_transactions(removed_tx);
                }

                break;

            } else {

                if state.get_result() != TxValidationResult::TX_MISSING_INPUTS {

                    if state.is_invalid() {

                        log_print!(
                            LogFlags::MEMPOOL, 
                            "   invalid orphan tx %s from peer=%d. %s\n", 
                            orphan_hash.to_string(), 
                            from_peer, 
                            state.to_string()
                        );

                        //  Maybe punish peer that gave us an invalid orphan tx
                        self.clone().maybe_punish_node_for_tx(from_peer,state,None);
                    }

                    // Has inputs but not accepted
                    // to mempool
                    //
                    // Probably non-standard or
                    // insufficient fee
                    log_print!(LogFlags::MEMPOOL,"   removed orphan tx %s\n",orphan_hash.to_string());

                    if state.get_result() != TxValidationResult::TX_WITNESS_STRIPPED {

                        //  We can add the wtxid of this transaction to our reject filter.
                        //  Do not add txids of witness transactions or witness-stripped
                        //  transactions to the filter, as they can have been malleated;
                        //  adding such txids to the reject filter would potentially
                        //  interfere with relay of valid transactions from peers that
                        //  do not support wtxid-based relay. See
                        //  https://github.com/bitcoin/bitcoin/issues/8279 for details.
                        //  We can remove this restriction (and always add wtxids to
                        //  the filter even for witness stripped transactions) once
                        //  wtxid-based relay is broadly deployed.
                        //  See also comments in https://github.com/bitcoin/bitcoin/pull/18044#discussion_r443419034
                        //  for concerns around weakening security of unupgraded nodes
                        //  if we start doing this too early.
                        self.inner.lock().recent_rejects.insert_key(
                            porphan_tx.get().get_witness_hash().as_slice()
                        );

                        //  If the transaction
                        //  failed for
                        //  TX_INPUTS_NOT_STANDARD,
                        //  then we know that the
                        //  witness was irrelevant
                        //  to the policy failure,
                        //  since this check
                        //  depends only on the
                        //  txid (the scriptPubKey
                        //  being spent is covered
                        //  by the txid).
                        //
                        //  Add the txid to the
                        //  reject filter to
                        //  prevent repeated
                        //  processing of this
                        //  transaction in the
                        //  event that child
                        //  transactions are later
                        //  received (resulting in
                        //  parent-fetching by
                        //  txid via the
                        //  orphan-handling
                        //  logic).
                        if state.get_result() == TxValidationResult::TX_INPUTS_NOT_STANDARD 
                        && porphan_tx.get().get_witness_hash() != porphan_tx.get().get_hash() 
                        {
                            // We only add the
                            // txid if it differs
                            // from the wtxid, to
                            // avoid wasting
                            // entries in the
                            // rolling bloom
                            // filter.
                            self.inner.lock().recent_rejects.insert_key(
                                porphan_tx.get().get_hash().as_slice()
                            );
                        }
                    }

                    self.orphanage.clone().erase_tx(&orphan_hash);

                    break;
                }
            }
        }

        let chainman = self.chainman.get();

        let active_chainstate = chainman.active_chainstate();

        let chain_height = active_chainstate.height().unwrap();

        let coins_tip = active_chainstate.coins_tip();

        let active_chainstate_height_p1 = chain_height + 1;

        self.mempool.get().check(
            coins_tip, 
            active_chainstate_height_p1.try_into().unwrap() 
        );
    }
}
