// ---------------- [ File: bitcoin-indexed-chain/src/mempool_accept.rs ]
crate::ix!();

pub struct MemPoolAccept {

    pool:                  Arc<Mutex<TxMemPool>>,
    view:                  CoinsViewCache,
    viewmempool:           CoinsViewMemPool,
    dummy:                 Box<dyn CoinsView>,
    active_chainstate:     Arc<Mutex<ChainState>>,

    /**
      | The package limits in effect at the time
      | of invocation.
      |
      */
    limit_ancestors:       usize,

    limit_ancestor_size:   usize,

    /**
      | These may be modified while evaluating
      | a transaction (eg to account for in-mempool
      | conflicts; see below).
      |
      */
    limit_descendants:     usize,

    limit_descendant_size: usize,
}

/**
  | We put the arguments we're handed into
  | a struct, so we can pass them around easier.
  |
  */
pub struct MemPoolAcceptATMPArgs {
    chainparams:              Arc<ChainParams>,
    accept_time:              i64,
    bypass_limits:            bool,

    /**
      | Return any outpoints which were not
      | previously present in the coins cache,
      | but were added as a result of validating
      | the tx for mempool acceptance.
      | 
      | This allows the caller to optionally
      | remove the cache additions if the associated
      | transaction ends up being rejected
      | by the mempool.
      |
      */
    coins_to_uncache:         Arc<Mutex<Vec<OutPoint>>>,

    test_accept:              bool,

    /**
      | Whether we allow transactions to replace
      | mempool transactions by BIP125 rules.
      | If false, any transaction spending
      | the same inputs as a transaction in the
      | mempool is considered a conflict.
      |
      */
    allow_bip125_replacement: bool, // default = { true }
}

/**
  | All the intermediate state that gets
  | passed between the various levels of
  | checking a given transaction.
  |
  */
pub struct MemPoolAcceptWorkspace {

    conflicts:             HashSet<u256>,
    all_conflicting:       TxMemPoolSetEntries,
    ancestors:             TxMemPoolSetEntries,
    entry:                 Option<TxMemPoolEntry>,
    replaced_transactions: LinkedList<TransactionRef>,
    base_fees:             Amount,
    modified_fees:         Amount,

    /**
      | Total modified fees of all transactions
      | being replaced.
      |
      */
    conflicting_fees:      Amount, // default = { 0 }

    /**
      | Total virtual size of all transactions
      | being replaced.
      |
      */
    conflicting_size:      usize, // default = { 0 }

    ptx:                   Arc<TransactionRef>,
    hash:                  Arc<u256>,
    state:                 TxValidationState,
}

impl From<&TransactionRef> for MemPoolAcceptWorkspace {

    fn from(ptx: &TransactionRef) -> Self {
    
        todo!();
        /*
            : m_ptx(ptx), m_hash(ptx->GetHash())
        */
    }
}

impl MemPoolAccept {

    pub fn new(
        mempool:           &mut TxMemPool,
        active_chainstate: &mut ChainState) -> Self {
    
        todo!();
        /*
            : m_pool(mempool), m_view(&m_dummy), m_viewmempool(&active_chainstate.CoinsTip(), m_pool), m_active_chainstate(active_chainstate),
            m_limit_ancestors(gArgs.GetIntArg("-limitancestorcount", DEFAULT_ANCESTOR_LIMIT)),
            m_limit_ancestor_size(gArgs.GetIntArg("-limitancestorsize", DEFAULT_ANCESTOR_SIZE_LIMIT)*1000),
            m_limit_descendants(gArgs.GetIntArg("-limitdescendantcount", DEFAULT_DESCENDANT_LIMIT)),
            m_limit_descendant_size(gArgs.GetIntArg("-limitdescendantsize", DEFAULT_DESCENDANT_SIZE_LIMIT)*1000)
        */
    }

    /**
      | Compare a package's feerate against
      | minimum allowed.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main, m_pool.cs)]
    pub fn check_fee_rate(&mut self, 
        package_size: usize,
        package_fee:  Amount,
        state:        &mut TxValidationState) -> bool {
        
        todo!();
        /*
            CAmount mempoolRejectFee = m_pool.GetMinFee(gArgs.GetIntArg("-maxmempool", DEFAULT_MAX_MEMPOOL_SIZE) * 1000000).GetFee(package_size);
            if (mempoolRejectFee > 0 && package_fee < mempoolRejectFee) {
                return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY, "mempool min fee not met", strprintf("%d < %d", package_fee, mempoolRejectFee));
            }

            if (package_fee < ::minRelayTxFee.GetFee(package_size)) {
                return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY, "min relay fee not met", strprintf("%d < %d", package_fee, ::minRelayTxFee.GetFee(package_size)));
            }
            return true;
        */
    }
    
    /**
      | Run the policy checks on a given
      | transaction, excluding any script checks.
      |
      | Looks up inputs, calculates feerate,
      | considers replacement, evaluates package
      | limits, etc. As this function can be
      | invoked for "free" by a peer, only tests
      | that are fast should be done here (to avoid
      | CPU DoS).
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main, m_pool.cs)]
    pub fn pre_checks(&mut self, 
        args: &mut MemPoolAcceptATMPArgs,
        ws:   &mut MemPoolAcceptWorkspace) -> bool {
        
        todo!();
        /*
            const CTransactionRef& ptx = ws.m_ptx;
        const CTransaction& tx = *ws.m_ptx;
        const uint256& hash = ws.m_hash;

        // Copy/alias what we need out of args
        const int64_t nAcceptTime = args.m_accept_time;
        const bool bypass_limits = args.m_bypass_limits;
        std::vector<OutPoint>& coins_to_uncache = args.m_coins_to_uncache;

        // Alias what we need out of ws
        TxValidationState& state = ws.m_state;
        std::set<uint256>& setConflicts = ws.m_conflicts;
        CTxMemPool::setEntries& allConflicting = ws.m_all_conflicting;
        CTxMemPool::setEntries& setAncestors = ws.m_ancestors;
        std::unique_ptr<CTxMemPoolEntry>& entry = ws.m_entry;
        CAmount& nModifiedFees = ws.m_modified_fees;
        CAmount& nConflictingFees = ws.m_conflicting_fees;
        size_t& nConflictingSize = ws.m_conflicting_size;

        if (!CheckTransaction(tx, state)) {
            return false; // state filled in by CheckTransaction
        }

        // Coinbase is only valid in a block, not as a loose transaction
        if (tx.IsCoinBase())
            return state.Invalid(TxValidationResult::TX_CONSENSUS, "coinbase");

        // Rather not work on nonstandard transactions (unless -testnet/-regtest)
        std::string reason;
        if (fRequireStandard && !IsStandardTx(tx, reason))
            return state.Invalid(TxValidationResult::TX_NOT_STANDARD, reason);

        // Do not work on transactions that are too small.
        // A transaction with 1 segwit input and 1 P2WPHK output has non-witness size of 82 bytes.
        // Transactions smaller than this are not relayed to mitigate CVE-2017-12842 by not relaying
        // 64-byte transactions.
        if (::GetSerializeSize(tx, PROTOCOL_VERSION | SERIALIZE_TRANSACTION_NO_WITNESS) < MIN_STANDARD_TX_NONWITNESS_SIZE)
            return state.Invalid(TxValidationResult::TX_NOT_STANDARD, "tx-size-small");

        // Only accept nLockTime-using transactions that can be mined in the next
        // block; we don't want our mempool filled up with transactions that can't
        // be mined yet.
        if (!CheckFinalTx(m_active_chainstate.m_chain.Tip(), tx, STANDARD_LOCKTIME_VERIFY_FLAGS))
            return state.Invalid(TxValidationResult::TX_PREMATURE_SPEND, "non-final");

        if (m_pool.exists(GenTxId::Wtxid(tx.GetWitnessHash()))) {
            // Exact transaction already exists in the mempool.
            return state.Invalid(TxValidationResult::TX_CONFLICT, "txn-already-in-mempool");
        } else if (m_pool.exists(GenTxId::Txid(tx.GetHash()))) {
            // Transaction with the same non-witness data but different witness (same txid, different
            // wtxid) already exists in the mempool.
            return state.Invalid(TxValidationResult::TX_CONFLICT, "txn-same-nonwitness-data-in-mempool");
        }

        // Check for conflicts with in-memory transactions
        for (const CTxIn &txin : tx.vin)
        {
            const CTransaction* ptxConflicting = m_pool.GetConflictTx(txin.prevout);
            if (ptxConflicting) {
                if (!args.m_allow_bip125_replacement) {
                    // Transaction conflicts with a mempool tx, but we're not allowing replacements.
                    return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY, "bip125-replacement-disallowed");
                }
                if (!setConflicts.count(ptxConflicting->GetHash()))
                {
                    // Transactions that don't explicitly signal replaceability are
                    // *not* replaceable with the current logic, even if one of their
                    // unconfirmed ancestors signals replaceability. This diverges
                    // from BIP125's inherited signaling description (see CVE-2021-31876).
                    // Applications relying on first-seen mempool behavior should
                    // check all unconfirmed ancestors; otherwise an opt-in ancestor
                    // might be replaced, causing removal of this descendant.
                    if (!SignalsOptInRBF(*ptxConflicting)) {
                        return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY, "txn-mempool-conflict");
                    }

                    setConflicts.insert(ptxConflicting->GetHash());
                }
            }
        }

        LockPoints lp;
        m_view.SetBackend(m_viewmempool);

        const CCoinsViewCache& coins_cache = m_active_chainstate.CoinsTip();
        // do all inputs exist?
        for (const CTxIn& txin : tx.vin) {
            if (!coins_cache.HaveCoinInCache(txin.prevout)) {
                coins_to_uncache.push_back(txin.prevout);
            }

            // Note: this call may add txin.prevout to the coins cache
            // (coins_cache.cacheCoins) by way of FetchCoin(). It should be removed
            // later (via coins_to_uncache) if this tx turns out to be invalid.
            if (!m_view.HaveCoin(txin.prevout)) {
                // Are inputs missing because we already have the tx?
                for (size_t out = 0; out < tx.vout.size(); out++) {
                    // Optimistically just do efficient check of cache for outputs
                    if (coins_cache.HaveCoinInCache(OutPoint(hash, out))) {
                        return state.Invalid(TxValidationResult::TX_CONFLICT, "txn-already-known");
                    }
                }
                // Otherwise assume this might be an orphan tx for which we just haven't seen parents yet
                return state.Invalid(TxValidationResult::TX_MISSING_INPUTS, "bad-txns-inputs-missingorspent");
            }
        }

        // This is const, but calls into the back end CoinsViews. The CCoinsViewDB at the bottom of the
        // hierarchy brings the best block into scope. See CCoinsViewDB::GetBestBlock().
        m_view.GetBestBlock();

        // we have all inputs cached now, so switch back to dummy (to protect
        // against bugs where we pull more inputs from disk that miss being added
        // to coins_to_uncache)
        m_view.SetBackend(m_dummy);

        // Only accept BIP68 sequence locked transactions that can be mined in the next
        // block; we don't want our mempool filled up with transactions that can't
        // be mined yet.
        // Pass in m_view which has all of the relevant inputs cached. Note that, since m_view's
        // backend was removed, it no longer pulls coins from the mempool.
        if (!CheckSequenceLocks(m_active_chainstate.m_chain.Tip(), m_view, tx, STANDARD_LOCKTIME_VERIFY_FLAGS, &lp))
            return state.Invalid(TxValidationResult::TX_PREMATURE_SPEND, "non-BIP68-final");

        if (!consensus::CheckTxInputs(tx, state, m_view, m_active_chainstate.m_blockman.GetSpendHeight(m_view), ws.m_base_fees)) {
            return false; // state filled in by CheckTxInputs
        }

        // Check for non-standard pay-to-script-hash in inputs
        const bool taproot_active = DeploymentActiveAfter(m_active_chainstate.m_chain.Tip(), args.m_chainparams.GetConsensus(), consensus::DEPLOYMENT_TAPROOT);
        if (fRequireStandard && !AreInputsStandard(tx, m_view, taproot_active)) {
            return state.Invalid(TxValidationResult::TX_INPUTS_NOT_STANDARD, "bad-txns-nonstandard-inputs");
        }

        // Check for non-standard witnesses.
        if (tx.HasWitness() && fRequireStandard && !IsWitnessStandard(tx, m_view))
            return state.Invalid(TxValidationResult::TX_WITNESS_MUTATED, "bad-witness-nonstandard");

        int64_t nSigOpsCost = GetTransactionSigOpCost(tx, m_view, STANDARD_SCRIPT_VERIFY_FLAGS);

        // nModifiedFees includes any fee deltas from PrioritiseTransaction
        nModifiedFees = ws.m_base_fees;
        m_pool.ApplyDelta(hash, nModifiedFees);

        // Keep track of transactions that spend a coinbase, which we re-scan
        // during reorgs to ensure COINBASE_MATURITY is still met.
        bool fSpendsCoinbase = false;
        for (const CTxIn &txin : tx.vin) {
            const Coin &coin = m_view.AccessCoin(txin.prevout);
            if (coin.IsCoinBase()) {
                fSpendsCoinbase = true;
                break;
            }
        }

        entry.reset(new CTxMemPoolEntry(ptx, ws.m_base_fees, nAcceptTime, m_active_chainstate.m_chain.Height(),
                fSpendsCoinbase, nSigOpsCost, lp));
        unsigned int nSize = entry->GetTxSize();

        if (nSigOpsCost > MAX_STANDARD_TX_SIGOPS_COST)
            return state.Invalid(TxValidationResult::TX_NOT_STANDARD, "bad-txns-too-many-sigops",
                    strprintf("%d", nSigOpsCost));

        // No transactions are allowed below minRelayTxFee except from disconnected
        // blocks
        if (!bypass_limits && !CheckFeeRate(nSize, nModifiedFees, state)) return false;

        const CTxMemPool::setEntries setIterConflicting = m_pool.GetIterSet(setConflicts);
        // Calculate in-mempool ancestors, up to a limit.
        if (setConflicts.size() == 1) {
            // In general, when we receive an RBF transaction with mempool conflicts, we want to know whether we
            // would meet the chain limits after the conflicts have been removed. However, there isn't a practical
            // way to do this short of calculating the ancestor and descendant sets with an overlay cache of
            // changed mempool entries. Due to both implementation and runtime complexity concerns, this isn't
            // very realistic, thus we only ensure a limited set of transactions are RBF'able despite mempool
            // conflicts here. Importantly, we need to ensure that some transactions which were accepted using
            // the below carve-out are able to be RBF'ed, without impacting the security the carve-out provides
            // for off-chain contract systems (see link in the comment below).
            //
            // Specifically, the subset of RBF transactions which we allow despite chain limits are those which
            // conflict directly with exactly one other transaction (but may evict children of said transaction),
            // and which are not adding any new mempool dependencies. Note that the "no new mempool dependencies"
            // check is accomplished later, so we don't bother doing anything about it here, but if BIP 125 is
            // amended, we may need to move that check to here instead of removing it wholesale.
            //
            // Such transactions are clearly not merging any existing packages, so we are only concerned with
            // ensuring that (a) no package is growing past the package size (not count) limits and (b) we are
            // not allowing something to effectively use the (below) carve-out spot when it shouldn't be allowed
            // to.
            //
            // To check these we first check if we meet the RBF criteria, above, and increment the descendant
            // limits by the direct conflict and its descendants (as these are recalculated in
            // CalculateMempoolAncestors by assuming the new transaction being added is a new descendant, with no
            // removals, of each parent's existing dependent set). The ancestor count limits are unmodified (as
            // the ancestor limits should be the same for both our new transaction and any conflicts).
            // We don't bother incrementing m_limit_descendants by the full removal count as that limit never comes
            // into force here (as we're only adding a single transaction).
            assert(setIterConflicting.size() == 1);
            CTxMemPool::txiter conflict = *setIterConflicting.begin();

            m_limit_descendants += 1;
            m_limit_descendant_size += conflict->GetSizeWithDescendants();
        }

        std::string errString;
        if (!m_pool.CalculateMemPoolAncestors(*entry, setAncestors, m_limit_ancestors, m_limit_ancestor_size, m_limit_descendants, m_limit_descendant_size, errString)) {
            setAncestors.clear();
            // If CalculateMemPoolAncestors fails second time, we want the original error string.
            std::string dummy_err_string;
            // Contracting/payment channels CPFP carve-out:
            // If the new transaction is relatively small (up to 40k weight)
            // and has at most one ancestor (ie ancestor limit of 2, including
            // the new transaction), allow it if its parent has exactly the
            // descendant limit descendants.
            //
            // This allows protocols which rely on distrusting counterparties
            // being able to broadcast descendants of an unconfirmed transaction
            // to be secure by simply only having two immediately-spendable
            // outputs - one for each counterparty. For more info on the uses for
            // this, see https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2018-November/016518.html
            if (nSize >  EXTRA_DESCENDANT_TX_SIZE_LIMIT ||
                    !m_pool.CalculateMemPoolAncestors(*entry, setAncestors, 2, m_limit_ancestor_size, m_limit_descendants + 1, m_limit_descendant_size + EXTRA_DESCENDANT_TX_SIZE_LIMIT, dummy_err_string)) {
                return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY, "too-long-mempool-chain", errString);
            }
        }

        // A transaction that spends outputs that would be replaced by it is invalid. Now
        // that we have the set of all ancestors we can detect this
        // pathological case by making sure setConflicts and setAncestors don't
        // intersect.
        if (const auto err_string{EntriesAndTxidsDisjoint(setAncestors, setConflicts, hash)}) {
            // We classify this as a consensus error because a transaction depending on something it
            // conflicts with would be inconsistent.
            return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-spends-conflicting-tx", *err_string);
        }

        if (!setConflicts.empty()) {
            CFeeRate newFeeRate(nModifiedFees, nSize);
            // It's possible that the replacement pays more fees than its direct conflicts but not more
            // than all conflicts (i.e. the direct conflicts have high-fee descendants). However, if the
            // replacement doesn't pay more fees than its direct conflicts, then we can be sure it's not
            // more economically rational to mine. Before we go digging through the mempool for all
            // transactions that would need to be removed (direct conflicts and all descendants), check
            // that the replacement transaction pays more than its direct conflicts.
            if (const auto err_string{PaysMoreThanConflicts(setIterConflicting, newFeeRate, hash)}) {
                return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY, "insufficient fee", *err_string);
            }

            // Calculate all conflicting entries and enforce BIP125 Rule #5.
            if (const auto err_string{GetEntriesForConflicts(tx, m_pool, setIterConflicting, allConflicting)}) {
                return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY,
                                     "too many potential replacements", *err_string);
            }
            // Enforce BIP125 Rule #2.
            if (const auto err_string{HasNoNewUnconfirmed(tx, m_pool, setIterConflicting)}) {
                return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY,
                                     "replacement-adds-unconfirmed", *err_string);
            }

            // Check if it's economically rational to mine this transaction rather than the ones it
            // replaces and pays for its own relay fees. Enforce BIP125 Rules #3 and #4.
            for (CTxMemPool::txiter it : allConflicting) {
                nConflictingFees += it->GetModifiedFee();
                nConflictingSize += it->GetTxSize();
            }
            if (const auto err_string{PaysForRBF(nConflictingFees, nModifiedFees, nSize, ::incrementalRelayFee, hash)}) {
                return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY, "insufficient fee", *err_string);
            }
        }
        return true;
        */
    }
    
    /**
      | Run the script checks using our policy
      | flags. As this can be slow, we should only
      | invoke this on transactions that have
      | otherwise passed policy checks.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main, m_pool.cs)]
    pub fn policy_script_checks(&mut self, 
        args:   &MemPoolAcceptATMPArgs,
        ws:     &mut MemPoolAcceptWorkspace,
        txdata: &mut PrecomputedTransactionData) -> bool {
        
        todo!();
        /*
            const CTransaction& tx = *ws.m_ptx;
        TxValidationState& state = ws.m_state;

        constexpr unsigned int scriptVerifyFlags = STANDARD_SCRIPT_VERIFY_FLAGS;

        // Check input scripts and signatures.
        // This is done last to help prevent CPU exhaustion denial-of-service attacks.
        if (!CheckInputScripts(tx, state, m_view, scriptVerifyFlags, true, false, txdata)) {
            // SCRIPT_VERIFY_CLEANSTACK requires SCRIPT_VERIFY_WITNESS, so we
            // need to turn both off, and compare against just turning off CLEANSTACK
            // to see if the failure is specifically due to witness validation.
            TxValidationState state_dummy; // Want reported failures to be from first CheckInputScripts
            if (!tx.HasWitness() && CheckInputScripts(tx, state_dummy, m_view, scriptVerifyFlags & ~(SCRIPT_VERIFY_WITNESS | SCRIPT_VERIFY_CLEANSTACK), true, false, txdata) &&
                    !CheckInputScripts(tx, state_dummy, m_view, scriptVerifyFlags & ~SCRIPT_VERIFY_CLEANSTACK, true, false, txdata)) {
                // Only the witness is missing, so the transaction itself may be fine.
                state.Invalid(TxValidationResult::TX_WITNESS_STRIPPED,
                        state.GetRejectReason(), state.GetDebugMessage());
            }
            return false; // state filled in by CheckInputScripts
        }

        return true;
        */
    }
    
    /**
      | Re-run the script checks, using consensus
      | flags, and try to cache the result in the
      | scriptcache. This should be done after
      | PolicyScriptChecks(). This requires that
      | all inputs either be in our utxo set or in
      | the mempool.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main, m_pool.cs)]
    pub fn consensus_script_checks(&mut self, 
        args:   &MemPoolAcceptATMPArgs,
        ws:     &mut MemPoolAcceptWorkspace,
        txdata: &mut PrecomputedTransactionData) -> bool {
        
        todo!();
        /*
            const CTransaction& tx = *ws.m_ptx;
        const uint256& hash = ws.m_hash;
        TxValidationState& state = ws.m_state;
        const CChainParams& chainparams = args.m_chainparams;

        // Check again against the current block tip's script verification
        // flags to cache our script execution flags. This is, of course,
        // useless if the next block has different script flags from the
        // previous one, but because the cache tracks script flags for us it
        // will auto-invalidate and we'll just have a few blocks of extra
        // misses on soft-fork activation.
        //
        // This is also useful in case of bugs in the standard flags that cause
        // transactions to pass as valid when they're actually invalid. For
        // instance the STRICTENC flag was incorrectly allowing certain
        // CHECKSIG NOT scripts to pass, even though they were invalid.
        //
        // There is a similar check in CreateNewBlock() to prevent creating
        // invalid blocks (using TestBlockValidity), however allowing such
        // transactions into the mempool can be exploited as a DoS attack.
        unsigned int currentBlockScriptVerifyFlags = GetBlockScriptFlags(m_active_chainstate.m_chain.Tip(), chainparams.GetConsensus());
        if (!CheckInputsFromMempoolAndCache(tx, state, m_view, m_pool, currentBlockScriptVerifyFlags, txdata, m_active_chainstate.CoinsTip())) {
            return error("%s: BUG! PLEASE REPORT THIS! CheckInputScripts failed against latest-block but not STANDARD flags %s, %s",
                    __func__, hash.ToString(), state.ToString());
        }

        return true;
        */
    }
    
    /**
      | Try to add the transaction to the mempool,
      | removing any conflicts first.
      |
      | Returns true if the transaction is in the
      | mempool after any size limiting is
      | performed, false otherwise.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main, m_pool.cs)]
    pub fn finalize(&mut self, 
        args: &MemPoolAcceptATMPArgs,
        ws:   &mut MemPoolAcceptWorkspace) -> bool {
        
        todo!();
        /*
            const CTransaction& tx = *ws.m_ptx;
        const uint256& hash = ws.m_hash;
        TxValidationState& state = ws.m_state;
        const bool bypass_limits = args.m_bypass_limits;

        CTxMemPool::setEntries& allConflicting = ws.m_all_conflicting;
        CTxMemPool::setEntries& setAncestors = ws.m_ancestors;
        const CAmount& nModifiedFees = ws.m_modified_fees;
        const CAmount& nConflictingFees = ws.m_conflicting_fees;
        const size_t& nConflictingSize = ws.m_conflicting_size;
        std::unique_ptr<CTxMemPoolEntry>& entry = ws.m_entry;

        // Remove conflicting transactions from the mempool
        for (CTxMemPool::txiter it : allConflicting)
        {
            LogPrint(BCLog::MEMPOOL, "replacing tx %s with %s for %s additional fees, %d delta bytes\n",
                    it->GetTx().GetHash().ToString(),
                    hash.ToString(),
                    FormatMoney(nModifiedFees - nConflictingFees),
                    (int)entry->GetTxSize() - (int)nConflictingSize);
            ws.m_replaced_transactions.push_back(it->GetSharedTx());
        }
        m_pool.RemoveStaged(allConflicting, false, MemPoolRemovalReason::REPLACED);

        // This transaction should only count for fee estimation if:
        // - it's not being re-added during a reorg which bypasses typical mempool fee limits
        // - the node is not behind
        // - the transaction is not dependent on any other transactions in the mempool
        bool validForFeeEstimation = !bypass_limits && IsCurrentForFeeEstimation(m_active_chainstate) && m_pool.HasNoInputsOf(tx);

        // Store transaction in memory
        m_pool.addUnchecked(*entry, setAncestors, validForFeeEstimation);

        // trim mempool and check if tx was trimmed
        if (!bypass_limits) {
            LimitMempoolSize(m_pool, m_active_chainstate.CoinsTip(), gArgs.GetIntArg("-maxmempool", DEFAULT_MAX_MEMPOOL_SIZE) * 1000000, hours{gArgs.GetIntArg("-mempoolexpiry", DEFAULT_MEMPOOL_EXPIRY)});
            if (!m_pool.exists(GenTxId::Txid(hash)))
                return state.Invalid(TxValidationResult::TX_MEMPOOL_POLICY, "mempool full");
        }
        return true;
        */
    }
    
    /**
      | Single transaction acceptance
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn accept_single_transaction(&mut self, 
        ptx:  &TransactionRef,
        args: &mut MemPoolAcceptATMPArgs) -> MempoolAcceptResult {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        LOCK(m_pool.cs); // mempool "read lock" (held through GetMainSignals().TransactionAddedToMempool())

        Workspace ws(ptx);

        if (!PreChecks(args, ws)) return MempoolAcceptResult::Failure(ws.m_state);

        // Only compute the precomputed transaction data if we need to verify
        // scripts (ie, other policy checks pass). We perform the inexpensive
        // checks first and avoid hashing and signature verification unless those
        // checks pass, to mitigate CPU exhaustion denial-of-service attacks.
        PrecomputedTransactionData txdata;

        if (!PolicyScriptChecks(args, ws, txdata)) return MempoolAcceptResult::Failure(ws.m_state);

        if (!ConsensusScriptChecks(args, ws, txdata)) return MempoolAcceptResult::Failure(ws.m_state);

        // Tx was accepted, but not added
        if (args.m_test_accept) {
            return MempoolAcceptResult::Success(std::move(ws.m_replaced_transactions), ws.m_base_fees);
        }

        if (!Finalize(args, ws)) return MempoolAcceptResult::Failure(ws.m_state);

        GetMainSignals().TransactionAddedToMempool(ptx, m_pool.GetAndIncrementSequence());

        return MempoolAcceptResult::Success(std::move(ws.m_replaced_transactions), ws.m_base_fees);
        */
    }
    
    /**
      | Multiple transaction acceptance.
      | Transactions may or may not be interdependent,
      | but must not conflict with each other.
      | Parents must come before children if
      | any dependencies exist.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn accept_multiple_transactions(&mut self, 
        txns: &Vec<TransactionRef>,
        args: &mut MemPoolAcceptATMPArgs) -> PackageMempoolAcceptResult {
        
        todo!();
        /*
            AssertLockHeld(cs_main);

        // These context-free package limits can be done before taking the mempool lock.
        PackageValidationState package_state;
        if (!CheckPackage(txns, package_state)) return PackageMempoolAcceptResult(package_state, {});

        std::vector<Workspace> workspaces{};
        workspaces.reserve(txns.size());
        std::transform(txns.cbegin(), txns.cend(), std::back_inserter(workspaces),
                       [](const auto& tx) { return Workspace(tx); });
        std::map<const uint256, const MempoolAcceptResult> results;

        LOCK(m_pool.cs);

        // Do all PreChecks first and fail fast to avoid running expensive script checks when unnecessary.
        for (Workspace& ws : workspaces) {
            if (!PreChecks(args, ws)) {
                package_state.Invalid(PackageValidationResult::PCKG_TX, "transaction failed");
                // Exit early to avoid doing pointless work. Update the failed tx result; the rest are unfinished.
                results.emplace(ws.m_ptx->GetWitnessHash(), MempoolAcceptResult::Failure(ws.m_state));
                return PackageMempoolAcceptResult(package_state, std::move(results));
            }
            // Make the coins created by this transaction available for subsequent transactions in the
            // package to spend. Since we already checked conflicts in the package and we don't allow
            // replacements, we don't need to track the coins spent. Note that this logic will need to be
            // updated if package replace-by-fee is allowed in the future.
            assert(!args.m_allow_bip125_replacement);
            m_viewmempool.PackageAddTransaction(ws.m_ptx);
        }

        // Apply package mempool ancestor/descendant limits. Skip if there is only one transaction,
        // because it's unnecessary. Also, CPFP carve out can increase the limit for individual
        // transactions, but this exemption is not extended to packages in CheckPackageLimits().
        std::string err_string;
        if (txns.size() > 1 &&
            !m_pool.CheckPackageLimits(txns, m_limit_ancestors, m_limit_ancestor_size, m_limit_descendants,
                                       m_limit_descendant_size, err_string)) {
            // All transactions must have individually passed mempool ancestor and descendant limits
            // inside of PreChecks(), so this is separate from an individual transaction error.
            package_state.Invalid(PackageValidationResult::PCKG_POLICY, "package-mempool-limits", err_string);
            return PackageMempoolAcceptResult(package_state, std::move(results));
        }

        for (Workspace& ws : workspaces) {
            PrecomputedTransactionData txdata;
            if (!PolicyScriptChecks(args, ws, txdata)) {
                // Exit early to avoid doing pointless work. Update the failed tx result; the rest are unfinished.
                package_state.Invalid(PackageValidationResult::PCKG_TX, "transaction failed");
                results.emplace(ws.m_ptx->GetWitnessHash(), MempoolAcceptResult::Failure(ws.m_state));
                return PackageMempoolAcceptResult(package_state, std::move(results));
            }
            if (args.m_test_accept) {
                // When test_accept=true, transactions that pass PolicyScriptChecks are valid because there are
                // no further mempool checks (passing PolicyScriptChecks implies passing ConsensusScriptChecks).
                results.emplace(ws.m_ptx->GetWitnessHash(),
                                MempoolAcceptResult::Success(std::move(ws.m_replaced_transactions), ws.m_base_fees));
            }
        }

        return PackageMempoolAcceptResult(package_state, std::move(results));
        */
    }
}
