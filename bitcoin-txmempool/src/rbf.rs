// ---------------- [ File: bitcoin-txmempool/src/rbf.rs ]
crate::ix!();

/**
  | Determine whether an unconfirmed transaction
  | is signaling opt-in to RBF according
  | to BIP 125
  | 
  | This involves checking sequence numbers
  | of the transaction, as well as the sequence
  | numbers of all in-mempool ancestors.
  | 
  | -----------
  | @param tx
  | 
  | The unconfirmed transaction
  | ----------
  | @param pool
  | 
  | The mempool, which may contain the tx
  | 
  | -----------
  | @return
  | 
  | The rbf state
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(pool.cs)]
pub fn is_rbf_opt_in(
        tx:   &Transaction,
        pool: &TxMemPool) -> RBFTransactionState {
    
    todo!();
        /*
            AssertLockHeld(pool.cs);

        CTxMemPool::setEntries ancestors;

        // First check the transaction itself.
        if (SignalsOptInRBF(tx)) {
            return RBFTransactionState::REPLACEABLE_BIP125;
        }

        // If this transaction is not in our mempool, then we can't be sure
        // we will know about all its inputs.
        if (!pool.exists(GenTxId::Txid(tx.GetHash()))) {
            return RBFTransactionState::UNKNOWN;
        }

        // If all the inputs have nSequence >= maxint-1, it still might be
        // signaled for RBF if any unconfirmed parents have signaled.
        uint64_t noLimit = std::numeric_limits<uint64_t>::max();
        std::string dummy;
        CTxMemPoolEntry entry = *pool.mapTx.find(tx.GetHash());
        pool.CalculateMemPoolAncestors(entry, ancestors, noLimit, noLimit, noLimit, noLimit, dummy, false);

        for (CTxMemPool::txiter it : ancestors) {
            if (SignalsOptInRBF(it->GetTx())) {
                return RBFTransactionState::REPLACEABLE_BIP125;
            }
        }
        return RBFTransactionState::FINAL;
        */
}

/**
  | Get all descendants of iters_conflicting.
  | Also enforce BIP125 Rule #5, "The number
  | of original transactions to be replaced
  | and their descendant transactions
  | which will be evicted from the mempool
  | must not exceed a total of 100 transactions."
  | Quit as early as possible. There cannot
  | be more than MAX_BIP125_REPLACEMENT_CANDIDATES
  | potential entries.
  | 
  | -----------
  | @param[in] iters_conflicting
  | 
  | The set of iterators to mempool entries.
  | ----------
  | @param[out] all_conflicts
  | 
  | Populated with all the mempool entries
  | that would be replaced, which includes
  | descendants of iters_conflicting.
  | Not cleared at the start; any existing
  | mempool entries will remain in the set.
  | 
  | -----------
  | @return
  | 
  | an error message if Rule #5 is broken,
  | otherwise a std::nullopt.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(pool.cs)]
pub fn get_entries_for_conflicts(
        tx:                &Transaction,
        pool:              &mut TxMemPool,
        iters_conflicting: &TxMemPoolSetEntries,
        all_conflicts:     &mut TxMemPoolSetEntries) -> Option<String> {
    
    todo!();
        /*
            AssertLockHeld(pool.cs);
        const uint256 txid = tx.GetHash();
        uint64_t nConflictingCount = 0;
        for (const auto& mi : iters_conflicting) {
            nConflictingCount += mi->GetCountWithDescendants();
            // BIP125 Rule #5: don't consider replacing more than MAX_BIP125_REPLACEMENT_CANDIDATES
            // entries from the mempool. This potentially overestimates the number of actual
            // descendants (i.e. if multiple conflicts share a descendant, it will be counted multiple
            // times), but we just want to be conservative to avoid doing too much work.
            if (nConflictingCount > MAX_BIP125_REPLACEMENT_CANDIDATES) {
                return strprintf("rejecting replacement %s; too many potential replacements (%d > %d)\n",
                                 txid.ToString(),
                                 nConflictingCount,
                                 MAX_BIP125_REPLACEMENT_CANDIDATES);
            }
        }
        // Calculate the set of all transactions that would have to be evicted.
        for (CTxMemPool::txiter it : iters_conflicting) {
            pool.CalculateDescendants(it, all_conflicts);
        }
        return std::nullopt;
        */
}

/**
  | BIP125 Rule #2: "The replacement transaction
  | may only include an unconfirmed input if that
  | input was included in one of the original
  | transactions."
  | 
  | ----------- 
  | @return
  | 
  | error message if Rule #2 is broken, otherwise
  | std::nullopt.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(pool.cs)]
pub fn has_no_new_unconfirmed(
        tx:                &Transaction,
        pool:              &TxMemPool,
        iters_conflicting: &TxMemPoolSetEntries) -> Option<String> {
    
    todo!();
        /*
            AssertLockHeld(pool.cs);
        std::set<uint256> parents_of_conflicts;
        for (const auto& mi : iters_conflicting) {
            for (const CTxIn& txin : mi->GetTx().vin) {
                parents_of_conflicts.insert(txin.prevout.hash);
            }
        }

        for (unsigned int j = 0; j < tx.vin.size(); j++) {
            // BIP125 Rule #2: We don't want to accept replacements that require low feerate junk to be
            // mined first.  Ideally we'd keep track of the ancestor feerates and make the decision
            // based on that, but for now requiring all new inputs to be confirmed works.
            //
            // Note that if you relax this to make RBF a little more useful, this may break the
            // CalculateMempoolAncestors RBF relaxation which subtracts the conflict count/size from the
            // descendant limit.
            if (!parents_of_conflicts.count(tx.vin[j].prevout.hash)) {
                // Rather than check the UTXO set - potentially expensive - it's cheaper to just check
                // if the new input refers to a tx that's in the mempool.
                if (pool.exists(GenTxId::Txid(tx.vin[j].prevout.hash))) {
                    return strprintf("replacement %s adds unconfirmed input, idx %d",
                                     tx.GetHash().ToString(), j);
                }
            }
        }
        return std::nullopt;
        */
}

/**
  | Check the intersection between two
  | sets of transactions (a set of mempool
  | entries and a set of txids) to make sure
  | they are disjoint.
  | 
  | -----------
  | @param[in] ancestors
  | 
  | Set of mempool entries corresponding
  | to ancestors of the replacement transactions.
  | ----------
  | @param[in] direct_conflicts
  | 
  | Set of txids corresponding to the mempool
  | conflicts (candidates to be replaced).
  | ----------
  | @param[in] txid
  | 
  | Transaction ID, included in the error
  | message if violation occurs.
  | 
  | -----------
  | @return
  | 
  | error message if the sets intersect,
  | std::nullopt if they are disjoint.
  |
  */
pub fn entries_and_txids_disjoint(
        ancestors:        &TxMemPoolSetEntries,
        direct_conflicts: &HashSet<u256>,
        txid:             &u256) -> Option<String> {
    
    todo!();
        /*
            for (CTxMemPool::txiter ancestorIt : ancestors) {
            const uint256& hashAncestor = ancestorIt->GetTx().GetHash();
            if (direct_conflicts.count(hashAncestor)) {
                return strprintf("%s spends conflicting transaction %s",
                                 txid.ToString(),
                                 hashAncestor.ToString());
            }
        }
        return std::nullopt;
        */
}

/**
  | Check that the feerate of the replacement
  | transaction(s) is higher than the feerate
  | of each of the transactions in iters_conflicting.
  | 
  | -----------
  | @param[in] iters_conflicting
  | 
  | The set of mempool entries.
  | 
  | -----------
  | @return
  | 
  | error message if fees insufficient,
  | otherwise std::nullopt.
  |
  */
pub fn pays_more_than_conflicts(
        iters_conflicting:   &TxMemPoolSetEntries,
        replacement_feerate: FeeRate,
        txid:                &u256) -> Option<String> {
    
    todo!();
        /*
            for (const auto& mi : iters_conflicting) {
            // Don't allow the replacement to reduce the feerate of the mempool.
            //
            // We usually don't want to accept replacements with lower feerates than what they replaced
            // as that would lower the feerate of the next block. Requiring that the feerate always be
            // increased is also an easy-to-reason about way to prevent DoS attacks via replacements.
            //
            // We only consider the feerates of transactions being directly replaced, not their indirect
            // descendants. While that does mean high feerate children are ignored when deciding whether
            // or not to replace, we do require the replacement to pay more overall fees too, mitigating
            // most cases.
            CFeeRate original_feerate(mi->GetModifiedFee(), mi->GetTxSize());
            if (replacement_feerate <= original_feerate) {
                return strprintf("rejecting replacement %s; new feerate %s <= old feerate %s",
                                 txid.ToString(),
                                 replacement_feerate.ToString(),
                                 original_feerate.ToString());
            }
        }
        return std::nullopt;
        */
}
