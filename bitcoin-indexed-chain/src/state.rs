crate::ix!();

pub type RawReentrantMutex = Broken;

/**
  | ChainState stores and provides an
  | API to update our local knowledge of
  | the current best chain.
  | 
  | Eventually, the API here is targeted
  | at being exposed externally as a consumable
  | libconsensus library, so any functions
  | added must only call other class member
  | functions, pure functions in other
  | parts of the consensus library, callbacks
  | via the validation interface, or read/write-to-disk
  | functions (eventually this will also
  | be via callbacks).
  | 
  | Anything that is contingent on the current
  | tip of the chain is stored here, whereas
  | block information and metadata independent
  | of the current tip is kept in `BlockManager`.
  |
  */
pub struct ChainState {

    //TODO: #[GUARDED_BY(::cs_main)]
    pub inner:                       ChainStateInner,

    /**
      | Decreasing counter (used by subsequent
      | preciousblock calls).
      |
      */
    pub n_block_reverse_sequence_id: i32, // default = -1

    /**
      | chainwork for the last block that preciousblock
      | has been applied to.
      |
      */
    pub n_last_precious_chainwork:   ArithU256, // default = 0

    /**
      | the ChainState CriticalSection
      | 
      | A lock that must be held when modifying
      | this ChainState - held in ActivateBestChain()
      |
      */
    pub cs_chainstate:               RawReentrantMutex,

    /**
      | Whether this chainstate is undergoing
      | initial block download.
      | 
      | Mutable because we need to be able to
      | mark IsInitialBlockDownload() const,
      | which latches this for caching purposes.
      |
      */
    pub cached_finished_ibd:         AtomicBool, // default = { false }

    /**
      | Optional mempool that is kept in sync
      | with the chain. Only the active chainstate
      | has a mempool.
      |
      */
    pub mempool:                    Amo<Box<dyn ITxMemPool>>,

    pub params:                     Arc<ChainParams>,

    /**
      | Manages the UTXO set, which is a reflection
      | of the contents of `m_chain`.
      |
      */
    pub coins_views:                Box<CoinsViews>,

    /**
      | Reference to a BlockManager instance
      | which itself is shared across all
      | 
      | ChainState instances.
      |
      */
    pub blockman:                   Arc<Mutex<BlockManager>>,

    /**
      | The chainstate manager that owns this
      | chainstate. The reference is necessary
      | so that this instance can check whether
      | it is the active chainstate within deeply
      | nested method calls.
      |
      */
    pub chainman:                   Arc<Mutex<ChainstateManager>>,

    /**
      | The current chain of blockheaders we
      | consult and build on. @see Chain, CBlockIndex.
      |
      */
    pub chain:                      Chain,

    /**
      | The blockhash which is the base of the
      | snapshot this chainstate was created
      | from. std::nullopt if this chainstate
      | was not created from a snapshot.
      |
      */
    pub from_snapshot_blockhash:    Option<u256>,

    /**
      | The set of all CBlockIndex entries with
      | either BLOCK_VALID_TRANSACTIONS (for
      | itself and all ancestors) *or*
      | BLOCK_ASSUMED_VALID (if using background
      | chainstates) and as good as our current
      | tip or better. 
      |
      | Entries may be failed, though, and pruning
      | nodes may be missing the data for the
      | block.
      |
      */
    pub set_block_index_candidates: HashSet<*mut BlockIndex,BlockIndexWorkComparator>,

    /**
      | The cache size of the on-disk coins view.
      |
      */
    pub coinsdb_cache_size_bytes:   usize, // default = { 0 }

    /**
      | The cache size of the in-memory coins
      | view.
      |
      */
    pub coinstip_cache_size_bytes:  usize, // default = { 0 }
}

impl ChainStateInterface for ChainState {}

impl ChainHeight for ChainState {

    fn height(&self) -> Option<usize> {
        self.chain.height()
    }
}

impl CoinsTip for ChainState {

    /**
      | @return
      | 
      | A reference to the in-memory cache of
      | the UTXO set.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    fn coins_tip(&mut self) -> &mut CoinsViewCache {
        
        todo!();
        /*
            assert(m_coins_views->m_cacheview);
            return *m_coins_views->m_cacheview.get();
        */
    }

}

impl IsInitialBlockDownload for ChainState {

    /**
      | Check whether we are doing an initial
      | block download (synchronizing from
      | disk or network)
      |
      | NOTE: though this is marked const, we may
      | end up modifying `m_cached_finished_ibd`, which
      | is a performance-related implementation
      | detail. This function must be marked `const` so
      | that `CValidationInterface` clients (which are
      | given a `const ChainState*`) can call it.
      |
      */
    fn is_initial_block_download(&self) -> bool {
        
        todo!();
        /*
            // Optimization: pre-test latch before taking the lock.
        if (m_cached_finished_ibd.load(std::memory_order_relaxed))
            return false;

        LOCK(cs_main);
        if (m_cached_finished_ibd.load(std::memory_order_relaxed))
            return false;
        if (fImporting || fReindex)
            return true;
        if (m_chain.Tip() == nullptr)
            return true;
        if (m_chain.Tip()->nChainWork < nMinimumChainWork)
            return true;
        if (m_chain.Tip()->GetBlockTime() < (GetTime() - nMaxTipAge))
            return true;
        LogPrintf("Leaving InitialBlockDownload (latching to false)\n");
        m_cached_finished_ibd.store(true, std::memory_order_relaxed);
        return false;
        */
    }
}


pub struct ChainStateInner {

    /*
       | Every received block is assigned a unique
       | and increasing identifier, so we know
       | which one to give priority in case of
       | a fork.
       |
       | Blocks loaded from disk are assigned
       | id 0, so start the counter at 1. 
       |
       */
    pub n_block_sequence_id:         i32, // default = 1
}

#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn is_current_for_fee_estimation(active_chainstate: &mut ChainState) -> bool {
    
    todo!();
        /*
            AssertLockHeld(cs_main);
        if (active_chainstate.IsInitialBlockDownload())
            return false;
        if (active_chainstate.m_chain.Tip()->GetBlockTime() < count_seconds(GetTime<seconds>() - MAX_FEE_ESTIMATION_TIP_AGE))
            return false;
        if (active_chainstate.m_chain.Height() < pindexBestHeader->nHeight - 1)
            return false;
        return true;
        */
}

impl ActivateBestChain for ChainState {

    /**
      | Find the best known block, and make it
      | the tip of the block chain. The result
      | is either failure or an activated best
      | chain. pblock is either nullptr or a
      | pointer to a block that is already loaded
      | (to avoid loading it again from disk).
      | 
      | ActivateBestChain is split into steps
      | (see ActivateBestChainStep) so that
      | we avoid holding cs_main for an extended
      | period of time; the length of this call
      | may be quite long during reindexing
      | or a substantial reorg.
      | 
      | May not be called with cs_main held.
      | May not be called in a validationinterface
      | callback.
      | 
      | 
      | -----------
      | @return
      | 
      | true unless a system error occurred
      |
      */
    #[LOCKS_EXCLUDED(cs_main)]
    fn activate_best_chain(&mut self, 
        state:  &mut BlockValidationState,
        pblock: Amo<Block>) -> bool {
        
        todo!();
        /*
            // Note that while we're often called here from ProcessNewBlock, this is
        // far from a guarantee. Things in the P2P/RPC will often end up calling
        // us in the middle of ProcessNewBlock - do not assume pblock is set
        // sanely for performance or correctness!
        AssertLockNotHeld(cs_main);

        // ABC maintains a fair degree of expensive-to-calculate internal state
        // because this function periodically releases cs_main so that it does not lock up other threads for too long
        // during large connects - and to allow for e.g. the callback queue to drain
        // we use m_cs_chainstate to enforce mutual exclusion so that only one caller may execute this function at a time
        LOCK(m_cs_chainstate);

        CBlockIndex *pindexMostWork = nullptr;
        CBlockIndex *pindexNewTip = nullptr;
        int nStopAtHeight = gArgs.GetIntArg("-stopatheight", DEFAULT_STOPATHEIGHT);
        do {
            // Block until the validation queue drains. This should largely
            // never happen in normal operation, however may happen during
            // reindex, causing memory blowup if we run too far ahead.
            // Note that if a validationinterface callback ends up calling
            // ActivateBestChain this may lead to a deadlock! We should
            // probably have a DEBUG_LOCKORDER test for this in the future.
            LimitValidationInterfaceQueue();

            {
                LOCK(cs_main);
                // Lock transaction pool for at least as long as it takes for connectTrace to be consumed
                LOCK(MempoolMutex());
                CBlockIndex* starting_tip = m_chain.Tip();
                bool blocks_connected = false;
                do {
                    // We absolutely may not unlock cs_main until we've made forward progress
                    // (with the exception of shutdown due to hardware issues, low disk space, etc).
                    ConnectTrace connectTrace; // Destructed before cs_main is unlocked

                    if (pindexMostWork == nullptr) {
                        pindexMostWork = FindMostWorkChain();
                    }

                    // Whether we have anything to do at all.
                    if (pindexMostWork == nullptr || pindexMostWork == m_chain.Tip()) {
                        break;
                    }

                    bool fInvalidFound = false;
                    std::shared_ptr<const CBlock> nullBlockPtr;
                    if (!ActivateBestChainStep(state, pindexMostWork, pblock && pblock->GetHash() == pindexMostWork->GetBlockHash() ? pblock : nullBlockPtr, fInvalidFound, connectTrace)) {
                        // A system error occurred
                        return false;
                    }
                    blocks_connected = true;

                    if (fInvalidFound) {
                        // Wipe cache, we may need another branch now.
                        pindexMostWork = nullptr;
                    }
                    pindexNewTip = m_chain.Tip();

                    for (const PerBlockConnectTrace& trace : connectTrace.GetBlocksConnected()) {
                        assert(trace.pblock && trace.pindex);
                        GetMainSignals().BlockConnected(trace.pblock, trace.pindex);
                    }
                } while (!m_chain.Tip() || (starting_tip && CBlockIndexWorkComparator()(m_chain.Tip(), starting_tip)));
                if (!blocks_connected) return true;

                const CBlockIndex* pindexFork = m_chain.FindFork(starting_tip);
                bool fInitialDownload = IsInitialBlockDownload();

                // Notify external listeners about the new tip.
                // Enqueue while holding cs_main to ensure that UpdatedBlockTip is called in the order in which blocks are connected
                if (pindexFork != pindexNewTip) {
                    // Notify ValidationInterface subscribers
                    GetMainSignals().UpdatedBlockTip(pindexNewTip, pindexFork, fInitialDownload);

                    // Always notify the UI if a new block tip was connected
                    uiInterface.NotifyBlockTip(GetSynchronizationState(fInitialDownload), pindexNewTip);
                }
            }
            // When we reach this point, we switched to a new tip (stored in pindexNewTip).

            if (nStopAtHeight && pindexNewTip && pindexNewTip->nHeight >= nStopAtHeight) StartShutdown();

            // We check shutdown only after giving ActivateBestChainStep a chance to run once so that we
            // never shutdown before connecting the genesis block during LoadChainTip(). Previously this
            // caused an assert() failure during shutdown in such cases as the UTXO DB flushing checks
            // that the best block hash is non-null.
            if (ShutdownRequested()) break;
        } while (pindexNewTip != pindexMostWork);
        CheckBlockIndex();

        // Write changes periodically to disk, after relay.
        if (!FlushStateToDisk(state, FlushStateMode::PERIODIC)) {
            return false;
        }

        return true;
        */
    }
}

impl ChainState {
    
    /**
      | Make mempool consistent after a reorg,
      | by re-adding or recursively erasing
      | disconnected block transactions from
      | the mempool, and also removing any other
      | transactions from the mempool that
      | are no longer valid given the new tip/height.
      | 
      | -----------
      | @note
      | 
      | we assume that disconnectpool only
      | contains transactions that are NOT
      | confirmed in the current chain nor already
      | in the mempool (otherwise, in-mempool
      | descendants of such transactions would
      | be removed).
      | 
      | Passing fAddToMempool=false will
      | skip trying to add the transactions
      | back, and instead just erase from the
      | mempool as needed.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main, m_mempool->cs)]
    pub fn maybe_update_mempool_for_reorg(&mut self, 
        disconnectpool: &mut DisconnectedBlockTransactions,
        add_to_mempool: bool)  {
        
        todo!();
        /*
            if (!m_mempool) return;

        AssertLockHeld(cs_main);
        AssertLockHeld(m_mempool->cs);
        std::vector<uint256> vHashUpdate;
        // disconnectpool's insertion_order index sorts the entries from
        // oldest to newest, but the oldest entry will be the last tx from the
        // latest mined block that was disconnected.
        // Iterate disconnectpool in reverse, so that we add transactions
        // back to the mempool starting with the earliest transaction that had
        // been previously seen in a block.
        auto it = disconnectpool.queuedTx.get<insertion_order>().rbegin();
        while (it != disconnectpool.queuedTx.get<insertion_order>().rend()) {
            // ignore validation errors in resurrected transactions
            if (!fAddToMempool || (*it)->IsCoinBase() ||
                AcceptToMemoryPool(
                    *this, *m_mempool, *it, true /* bypass_limits */).m_result_type !=
                        MempoolAcceptResult::ResultType::VALID) {
                // If the transaction doesn't make it in to the mempool, remove any
                // transactions that depend on it (which would now be orphans).
                m_mempool->removeRecursive(**it, MemPoolRemovalReason::REORG);
            } else if (m_mempool->exists(GenTxId::Txid((*it)->GetHash()))) {
                vHashUpdate.push_back((*it)->GetHash());
            }
            ++it;
        }
        disconnectpool.queuedTx.clear();
        // AcceptToMemoryPool/addUnchecked all assume that new mempool entries have
        // no in-mempool children, which is generally not true when adding
        // previously-confirmed transactions back to the mempool.
        // UpdateTransactionsFromBlock finds descendants of any transactions in
        // the disconnectpool that were added back and cleans up the mempool state.
        m_mempool->UpdateTransactionsFromBlock(vHashUpdate);

        // We also need to remove any now-immature transactions
        m_mempool->removeForReorg(*this, STANDARD_LOCKTIME_VERIFY_FLAGS);
        // Re-limit mempool size, in case we added any transactions
        LimitMempoolSize(
            *m_mempool,
            this->CoinsTip(),
            gArgs.GetIntArg("-maxmempool", DEFAULT_MAX_MEMPOOL_SIZE) * 1000000,
            hours{gArgs.GetIntArg("-mempoolexpiry", DEFAULT_MEMPOOL_EXPIRY)});
        */
    }
    
    /**
      | @returns whether or not the CoinsViews
      |          object has been fully initialized
      |          and we can safely flush this
      |          object to disk.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn can_flush_to_disk(&self) -> bool {
        
        todo!();
        /*
            return m_coins_views && m_coins_views->m_cacheview;
        */
    }

    /**
      | @return
      | 
      | A reference to the on-disk UTXO set database.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn coinsdb(&mut self) -> &mut CoinsViewDB {
        
        todo!();
        /*
            return m_coins_views->m_dbview;
        */
    }

    /**
      | @return
      | 
      | A reference to a wrapped view of the in-memory
      | UTXO set that handles disk read errors
      | gracefully.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn coins_error_catcher(&mut self) -> &mut CoinsViewErrorCatcher {
        
        todo!();
        /*
            return m_coins_views->m_catcherview;
        */
    }

    /**
      | Destructs all objects related to accessing
      | the UTXO set.
      |
      */
    pub fn reset_coins_views(&mut self)  {
        
        todo!();
        /*
            m_coins_views.reset();
        */
    }

    /* ------ Manual block validity manipulation:  ------ */

    /**
      | Indirection necessary to make lock
      | annotations work with an optional mempool.
      |
      */
    #[LOCK_RETURNED(m_mempool->cs)]
    pub fn mempool_mutex<T>(&self) -> *mut ReentrantMutex<T> {
        
        todo!();
        /*
            return m_mempool ? &m_mempool->cs : nullptr;
        */
    }

    pub fn new(
        mempool:                 Arc<Mutex<dyn ITxMemPool>>,
        blockman:                &mut BlockManager,
        chainman:                &mut ChainstateManager,
        from_snapshot_blockhash: Option<u256>) -> Self {
    
        todo!();
        /*


            : m_mempool(mempool),
          m_params(::Params()),
          m_blockman(blockman),
          m_chainman(chainman),
          m_from_snapshot_blockhash(from_snapshot_blockhash)
        */
    }
    
    /**
      | Initialize the CoinsViews UTXO set
      | database management data structures.
      | The in-memory cache is initialized
      | separately.
      | 
      | All parameters forwarded to CoinsViews.
      |
      */
    pub fn init_coinsdb(&mut self, 
        cache_size_bytes: usize,
        in_memory:        bool,
        should_wipe:      bool,
        leveldb_name:     Option<&str>)  {

        let leveldb_name: &str = leveldb_name.unwrap_or("chainstate");
        
        todo!();
        /*
            if (m_from_snapshot_blockhash) {
            leveldb_name += "_" + m_from_snapshot_blockhash->ToString();
        }

        m_coins_views = std::make_unique<CoinsViews>(
            leveldb_name, cache_size_bytes, in_memory, should_wipe);
        */
    }
    
    /**
      | Initialize the in-memory coins cache
      | (to be done after the health of the on-disk
      | database is verified).
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn init_coins_cache(&mut self, cache_size_bytes: usize)  {
        
        todo!();
        /*
            assert(m_coins_views != nullptr);
        m_coinstip_cache_size_bytes = cache_size_bytes;
        m_coins_views->InitCache();
        */
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn check_fork_warning_conditions(&mut self)  {
        
        todo!();
        /*
            AssertLockHeld(cs_main);

        // Before we get past initial download, we cannot reliably alert about forks
        // (we assume we don't get stuck on a fork before finishing our initial sync)
        if (IsInitialBlockDownload()) {
            return;
        }

        if (pindexBestInvalid && pindexBestInvalid->nChainWork > m_chain.Tip()->nChainWork + (GetBlockProof(*m_chain.Tip()) * 6)) {
            LogPrintf("%s: Warning: Found invalid chain at least ~6 blocks longer than our best chain.\nChain state database corruption likely.\n", __func__);
            SetfLargeWorkInvalidChainFound(true);
        } else {
            SetfLargeWorkInvalidChainFound(false);
        }
        */
    }

    /**
      | Called both upon regular invalid block
      | discovery *and* InvalidateBlock
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn invalid_chain_found(&mut self, pindex_new: *mut BlockIndex)  {
        
        todo!();
        /*
            if (!pindexBestInvalid || pindexNew->nChainWork > pindexBestInvalid->nChainWork)
            pindexBestInvalid = pindexNew;
        if (pindexBestHeader != nullptr && pindexBestHeader->GetAncestor(pindexNew->nHeight) == pindexNew) {
            pindexBestHeader = m_chain.Tip();
        }

        LogPrintf("%s: invalid block=%s  height=%d  log2_work=%f  date=%s\n", __func__,
          pindexNew->GetBlockHash().ToString(), pindexNew->nHeight,
          log(pindexNew->nChainWork.getdouble())/log(2.0), FormatISO8601DateTime(pindexNew->GetBlockTime()));
        CBlockIndex *tip = m_chain.Tip();
        assert (tip);
        LogPrintf("%s:  current best=%s  height=%d  log2_work=%f  date=%s\n", __func__,
          tip->GetBlockHash().ToString(), m_chain.Height(), log(tip->nChainWork.getdouble())/log(2.0),
          FormatISO8601DateTime(tip->GetBlockTime()));
        CheckForkWarningConditions();
        */
    }

    /**
      | Same as InvalidChainFound, above, except
      | not called directly from InvalidateBlock,
      | which does its own setBlockIndexCandidates
      | management.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn invalid_block_found(&mut self, 
        pindex: *mut BlockIndex,
        state:  &BlockValidationState)  {
        
        todo!();
        /*
            if (state.GetResult() != BlockValidationResult::BLOCK_MUTATED) {
            pindex->nStatus |= BLOCK_FAILED_VALID;
            m_blockman.m_failed_blocks.insert(pindex);
            setDirtyBlockIndex.insert(pindex);
            setBlockIndexCandidates.erase(pindex);
            InvalidChainFound(pindex);
        }
        */
    }

    /**
      | Block (dis)connection on a given view:
      |
      | Undo the effects of this block (with
      | given index) on the UTXO set represented
      | by coins.
      | 
      | When FAILED is returned, view is left
      | in an indeterminate state.
      |
      */
    pub fn disconnect_block(&mut self, 
        block:  &Block,
        pindex: *const BlockIndex,
        view:   &mut CoinsViewCache) -> DisconnectResult {
        
        todo!();
        /*
            bool fClean = true;

        CBlockUndo blockUndo;
        if (!UndoReadFromDisk(blockUndo, pindex)) {
            error("DisconnectBlock(): failure reading undo data");
            return DISCONNECT_FAILED;
        }

        if (blockUndo.vtxundo.size() + 1 != block.vtx.size()) {
            error("DisconnectBlock(): block and undo data inconsistent");
            return DISCONNECT_FAILED;
        }

        // undo transactions in reverse order
        for (int i = block.vtx.size() - 1; i >= 0; i--) {
            const CTransaction &tx = *(block.vtx[i]);
            uint256 hash = tx.GetHash();
            bool is_coinbase = tx.IsCoinBase();

            // Check that all outputs are available and match the outputs in the block itself
            // exactly.
            for (size_t o = 0; o < tx.vout.size(); o++) {
                if (!tx.vout[o].scriptPubKey.IsUnspendable()) {
                    OutPoint out(hash, o);
                    Coin coin;
                    bool is_spent = view.SpendCoin(out, &coin);
                    if (!is_spent || tx.vout[o] != coin.out || pindex->nHeight != coin.nHeight || is_coinbase != coin.fCoinBase) {
                        fClean = false; // transaction output mismatch
                    }
                }
            }

            // restore inputs
            if (i > 0) { // not coinbases
                CTxUndo &txundo = blockUndo.vtxundo[i-1];
                if (txundo.vprevout.size() != tx.vin.size()) {
                    error("DisconnectBlock(): transaction and undo data inconsistent");
                    return DISCONNECT_FAILED;
                }
                for (unsigned int j = tx.vin.size(); j-- > 0;) {
                    const OutPoint &out = tx.vin[j].prevout;
                    int res = ApplyTxInUndo(std::move(txundo.vprevout[j]), view, out);
                    if (res == DISCONNECT_FAILED) return DISCONNECT_FAILED;
                    fClean = fClean && res != DISCONNECT_UNCLEAN;
                }
                // At this point, all of txundo.vprevout should have been moved out.
            }
        }

        // move best block pointer to prevout block
        view.SetBestBlock(pindex->pprev->GetBlockHash());

        return fClean ? DISCONNECT_OK : DISCONNECT_UNCLEAN;
        */
    }
    
    /**
      | Apply the effects of this block (with
      | given index) on the UTXO set represented
      | by coins.
      | 
      | Validity checks that depend on the UTXO
      | set are also done; ConnectBlock() can
      | fail if those validity checks fail (among
      | other reasons).
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn connect_block(&mut self, 
        block:      &Block,
        state:      &mut BlockValidationState,
        pindex:     *mut BlockIndex,
        view:       &mut CoinsViewCache,
        just_check: Option<bool>) -> bool {

        let just_check: bool = just_check.unwrap_or(false);
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        assert(pindex);
        assert(*pindex->phashBlock == block.GetHash());
        int64_t nTimeStart = GetTimeMicros();

        // Check it again in case a previous version let a bad block in
        // NOTE: We don't currently (re-)invoke ContextualCheckBlock() or
        // ContextualCheckBlockHeader() here. This means that if we add a new
        // consensus rule that is enforced in one of those two functions, then we
        // may have let in a block that violates the rule prior to updating the
        // software, and we would NOT be enforcing the rule here. Fully solving
        // upgrade from one software version to the next after a consensus rule
        // change is potentially tricky and issue-specific (see NeedsRedownload()
        // for one approach that was used for BIP 141 deployment).
        // Also, currently the rule against blocks more than 2 hours in the future
        // is enforced in ContextualCheckBlockHeader(); we wouldn't want to
        // re-enforce that rule here (at least until we make it impossible for
        // GetAdjustedTime() to go backward).
        if (!CheckBlock(block, state, m_params.GetConsensus(), !fJustCheck, !fJustCheck)) {
            if (state.GetResult() == BlockValidationResult::BLOCK_MUTATED) {
                // We don't write down blocks to disk if they may have been
                // corrupted, so this should be impossible unless we're having hardware
                // problems.
                return AbortNode(state, "Corrupt block found indicating potential hardware failure; shutting down");
            }
            return error("%s: consensus::CheckBlock: %s", __func__, state.ToString());
        }

        // verify that the view's current state corresponds to the previous block
        uint256 hashPrevBlock = pindex->pprev == nullptr ? uint256() : pindex->pprev->GetBlockHash();
        assert(hashPrevBlock == view.GetBestBlock());

        nBlocksTotal++;

        // Special case for the genesis block, skipping connection of its transactions
        // (its coinbase is unspendable)
        if (block.GetHash() == m_params.GetConsensus().hashGenesisBlock) {
            if (!fJustCheck)
                view.SetBestBlock(pindex->GetBlockHash());
            return true;
        }

        bool fScriptChecks = true;
        if (!hashAssumeValid.IsNull()) {
            // We've been configured with the hash of a block which has been externally verified to have a valid history.
            // A suitable default value is included with the software and updated from time to time.  Because validity
            //  relative to a piece of software is an objective fact these defaults can be easily reviewed.
            // This setting doesn't force the selection of any particular chain but makes validating some faster by
            //  effectively caching the result of part of the verification.
            BlockMap::const_iterator  it = m_blockman.m_block_index.find(hashAssumeValid);
            if (it != m_blockman.m_block_index.end()) {
                if (it->second->GetAncestor(pindex->nHeight) == pindex &&
                    pindexBestHeader->GetAncestor(pindex->nHeight) == pindex &&
                    pindexBestHeader->nChainWork >= nMinimumChainWork) {
                    // This block is a member of the assumed verified chain and an ancestor of the best header.
                    // Script verification is skipped when connecting blocks under the
                    // assumevalid block. Assuming the assumevalid block is valid this
                    // is safe because block merkle hashes are still computed and checked,
                    // Of course, if an assumed valid block is invalid due to false scriptSigs
                    // this optimization would allow an invalid chain to be accepted.
                    // The equivalent time check discourages hash power from extorting the network via DOS attack
                    //  into accepting an invalid block through telling users they must manually set assumevalid.
                    //  Requiring a software change or burying the invalid block, regardless of the setting, makes
                    //  it hard to hide the implication of the demand.  This also avoids having release candidates
                    //  that are hardly doing any signature verification at all in testing without having to
                    //  artificially set the default assumed verified block further back.
                    // The test against nMinimumChainWork prevents the skipping when denied access to any chain at
                    //  least as good as the expected chain.
                    fScriptChecks = (GetBlockProofEquivalentTime(*pindexBestHeader, *pindex, *pindexBestHeader, m_params.GetConsensus()) <= 60 * 60 * 24 * 7 * 2);
                }
            }
        }

        int64_t nTime1 = GetTimeMicros(); nTimeCheck += nTime1 - nTimeStart;
        LogPrint(BCLog::BENCH, "    - Sanity checks: %.2fms [%.2fs (%.2fms/blk)]\n", MILLI * (nTime1 - nTimeStart), nTimeCheck * MICRO, nTimeCheck * MILLI / nBlocksTotal);

        // Do not allow blocks that contain transactions which 'overwrite' older transactions,
        // unless those are already completely spent.
        // If such overwrites are allowed, coinbases and transactions depending upon those
        // can be duplicated to remove the ability to spend the first instance -- even after
        // being sent to another address.
        // See BIP30, CVE-2012-1909, and http://r6.ca/blog/20120206T005236Z.html for more information.
        // This logic is not necessary for memory pool transactions, as AcceptToMemoryPool
        // already refuses previously-known transaction ids entirely.
        // This rule was originally applied to all blocks with a timestamp after March 15, 2012, 0:00 UTC.
        // Now that the whole chain is irreversibly beyond that time it is applied to all blocks except the
        // two in the chain that violate it. This prevents exploiting the issue against nodes during their
        // initial block download.
        bool fEnforceBIP30 = !((pindex->nHeight==91842 && pindex->GetBlockHash() == uint256S("0x00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec")) ||
                               (pindex->nHeight==91880 && pindex->GetBlockHash() == uint256S("0x00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721")));

        // Once BIP34 activated it was not possible to create new duplicate coinbases and thus other than starting
        // with the 2 existing duplicate coinbase pairs, not possible to create overwriting txs.  But by the
        // time BIP34 activated, in each of the existing pairs the duplicate coinbase had overwritten the first
        // before the first had been spent.  Since those coinbases are sufficiently buried it's no longer possible to create further
        // duplicate transactions descending from the known pairs either.
        // If we're on the known chain at height greater than where BIP34 activated, we can save the db accesses needed for the BIP30 check.

        // BIP34 requires that a block at height X (block X) has its coinbase
        // scriptSig start with a CScriptNum of X (indicated height X).  The above
        // logic of no longer requiring BIP30 once BIP34 activates is flawed in the
        // case that there is a block X before the BIP34 height of 227,931 which has
        // an indicated height Y where Y is greater than X.  The coinbase for block
        // X would also be a valid coinbase for block Y, which could be a BIP30
        // violation.  An exhaustive search of all mainnet coinbases before the
        // BIP34 height which have an indicated height greater than the block height
        // reveals many occurrences. The 3 lowest indicated heights found are
        // 209,921, 490,897, and 1,983,702 and thus coinbases for blocks at these 3
        // heights would be the first opportunity for BIP30 to be violated.

        // The search reveals a great many blocks which have an indicated height
        // greater than 1,983,702, so we simply remove the optimization to skip
        // BIP30 checking for blocks at height 1,983,702 or higher.  Before we reach
        // that block in another 25 years or so, we should take advantage of a
        // future consensus change to do a new and improved version of BIP34 that
        // will actually prevent ever creating any duplicate coinbases in the
        // future.
        static constexpr int BIP34_IMPLIES_BIP30_LIMIT = 1983702;

        // There is no potential to create a duplicate coinbase at block 209,921
        // because this is still before the BIP34 height and so explicit BIP30
        // checking is still active.

        // The final case is block 176,684 which has an indicated height of
        // 490,897. Unfortunately, this issue was not discovered until about 2 weeks
        // before block 490,897 so there was not much opportunity to address this
        // case other than to carefully analyze it and determine it would not be a
        // problem. Block 490,897 was, in fact, mined with a different coinbase than
        // block 176,684, but it is important to note that even if it hadn't been or
        // is remined on an alternate fork with a duplicate coinbase, we would still
        // not run into a BIP30 violation.  This is because the coinbase for 176,684
        // is spent in block 185,956 in transaction
        // d4f7fbbf92f4a3014a230b2dc70b8058d02eb36ac06b4a0736d9d60eaa9e8781.  This
        // spending transaction can't be duplicated because it also spends coinbase
        // 0328dd85c331237f18e781d692c92de57649529bd5edf1d01036daea32ffde29.  This
        // coinbase has an indicated height of over 4.2 billion, and wouldn't be
        // duplicatable until that height, and it's currently impossible to create a
        // chain that long. Nevertheless we may wish to consider a future soft fork
        // which retroactively prevents block 490,897 from creating a duplicate
        // coinbase. The two historical BIP30 violations often provide a confusing
        // edge case when manipulating the UTXO and it would be simpler not to have
        // another edge case to deal with.

        // testnet3 has no blocks before the BIP34 height with indicated heights
        // post BIP34 before approximately height 486,000,000 and presumably will
        // be reset before it reaches block 1,983,702 and starts doing unnecessary
        // BIP30 checking again.
        assert(pindex->pprev);
        CBlockIndex* pindexBIP34height = pindex->pprev->GetAncestor(m_params.GetConsensus().BIP34Height);
        //Only continue to enforce if we're below BIP34 activation height or the block hash at that height doesn't correspond.
        fEnforceBIP30 = fEnforceBIP30 && (!pindexBIP34height || !(pindexBIP34height->GetBlockHash() == m_params.GetConsensus().BIP34Hash));

        // TODO: Remove BIP30 checking from block height 1,983,702 on, once we have a
        // consensus change that ensures coinbases at those heights can not
        // duplicate earlier coinbases.
        if (fEnforceBIP30 || pindex->nHeight >= BIP34_IMPLIES_BIP30_LIMIT) {
            for (const auto& tx : block.vtx) {
                for (size_t o = 0; o < tx->vout.size(); o++) {
                    if (view.HaveCoin(OutPoint(tx->GetHash(), o))) {
                        LogPrintf("ERROR: ConnectBlock(): tried to overwrite transaction\n");
                        return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-txns-BIP30");
                    }
                }
            }
        }

        // Enforce BIP68 (sequence locks)
        int nLockTimeFlags = 0;
        if (DeploymentActiveAt(*pindex, m_params.GetConsensus(), consensus::DEPLOYMENT_CSV)) {
            nLockTimeFlags |= LOCKTIME_VERIFY_SEQUENCE;
        }

        // Get the script flags for this block
        unsigned int flags = GetBlockScriptFlags(pindex, m_params.GetConsensus());

        int64_t nTime2 = GetTimeMicros(); nTimeForks += nTime2 - nTime1;
        LogPrint(BCLog::BENCH, "    - Fork checks: %.2fms [%.2fs (%.2fms/blk)]\n", MILLI * (nTime2 - nTime1), nTimeForks * MICRO, nTimeForks * MILLI / nBlocksTotal);

        CBlockUndo blockundo;

        // Precomputed transaction data pointers must not be invalidated
        // until after `control` has run the script checks (potentially
        // in multiple threads). Preallocate the vector size so a new allocation
        // doesn't invalidate pointers into the vector, and keep txsdata in scope
        // for as long as `control`.
        CCheckQueueControl<CScriptCheck> control(fScriptChecks && g_parallel_script_checks ? &scriptcheckqueue : nullptr);
        std::vector<PrecomputedTransactionData> txsdata(block.vtx.size());

        std::vector<int> prevheights;
        CAmount nFees = 0;
        int nInputs = 0;
        int64_t nSigOpsCost = 0;
        blockundo.vtxundo.reserve(block.vtx.size() - 1);
        for (unsigned int i = 0; i < block.vtx.size(); i++)
        {
            const CTransaction &tx = *(block.vtx[i]);

            nInputs += tx.vin.size();

            if (!tx.IsCoinBase())
            {
                CAmount txfee = 0;
                TxValidationState tx_state;
                if (!consensus::CheckTxInputs(tx, tx_state, view, pindex->nHeight, txfee)) {
                    // Any transaction validation failure in ConnectBlock is a block consensus failure
                    state.Invalid(BlockValidationResult::BLOCK_CONSENSUS,
                                tx_state.GetRejectReason(), tx_state.GetDebugMessage());
                    return error("%s: consensus::CheckTxInputs: %s, %s", __func__, tx.GetHash().ToString(), state.ToString());
                }
                nFees += txfee;
                if (!MoneyRange(nFees)) {
                    LogPrintf("ERROR: %s: accumulated fee in the block out of range.\n", __func__);
                    return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-txns-accumulated-fee-outofrange");
                }

                // Check that transaction is BIP68 final
                // BIP68 lock checks (as opposed to nLockTime checks) must
                // be in ConnectBlock because they require the UTXO set
                prevheights.resize(tx.vin.size());
                for (size_t j = 0; j < tx.vin.size(); j++) {
                    prevheights[j] = view.AccessCoin(tx.vin[j].prevout).nHeight;
                }

                if (!SequenceLocks(tx, nLockTimeFlags, prevheights, *pindex)) {
                    LogPrintf("ERROR: %s: contains a non-BIP68-final transaction\n", __func__);
                    return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-txns-nonfinal");
                }
            }

            // GetTransactionSigOpCost counts 3 types of sigops:
            // * legacy (always)
            // * p2sh (when P2SH enabled in flags and excludes coinbase)
            // * witness (when witness enabled in flags and excludes coinbase)
            nSigOpsCost += GetTransactionSigOpCost(tx, view, flags);
            if (nSigOpsCost > MAX_BLOCK_SIGOPS_COST) {
                LogPrintf("ERROR: ConnectBlock(): too many sigops\n");
                return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-blk-sigops");
            }

            if (!tx.IsCoinBase())
            {
                std::vector<CScriptCheck> vChecks;
                bool fCacheResults = fJustCheck; /* Don't cache results if we're actually connecting blocks (still consult the cache, though) */
                TxValidationState tx_state;
                if (fScriptChecks && !CheckInputScripts(tx, tx_state, view, flags, fCacheResults, fCacheResults, txsdata[i], g_parallel_script_checks ? &vChecks : nullptr)) {
                    // Any transaction validation failure in ConnectBlock is a block consensus failure
                    state.Invalid(BlockValidationResult::BLOCK_CONSENSUS,
                                  tx_state.GetRejectReason(), tx_state.GetDebugMessage());
                    return error("ConnectBlock(): CheckInputScripts on %s failed with %s",
                        tx.GetHash().ToString(), state.ToString());
                }
                control.Add(vChecks);
            }

            CTxUndo undoDummy;
            if (i > 0) {
                blockundo.vtxundo.push_back(CTxUndo());
            }
            UpdateCoins(tx, view, i == 0 ? undoDummy : blockundo.vtxundo.back(), pindex->nHeight);
        }
        int64_t nTime3 = GetTimeMicros(); nTimeConnect += nTime3 - nTime2;
        LogPrint(BCLog::BENCH, "      - Connect %u transactions: %.2fms (%.3fms/tx, %.3fms/txin) [%.2fs (%.2fms/blk)]\n", (unsigned)block.vtx.size(), MILLI * (nTime3 - nTime2), MILLI * (nTime3 - nTime2) / block.vtx.size(), nInputs <= 1 ? 0 : MILLI * (nTime3 - nTime2) / (nInputs-1), nTimeConnect * MICRO, nTimeConnect * MILLI / nBlocksTotal);

        CAmount blockReward = nFees + GetBlockSubsidy(pindex->nHeight, m_params.GetConsensus());
        if (block.vtx[0]->GetValueOut() > blockReward) {
            LogPrintf("ERROR: ConnectBlock(): coinbase pays too much (actual=%d vs limit=%d)\n", block.vtx[0]->GetValueOut(), blockReward);
            return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-cb-amount");
        }

        if (!control.Wait()) {
            LogPrintf("ERROR: %s: CheckQueue failed\n", __func__);
            return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "block-validation-failed");
        }
        int64_t nTime4 = GetTimeMicros(); nTimeVerify += nTime4 - nTime2;
        LogPrint(BCLog::BENCH, "    - Verify %u txins: %.2fms (%.3fms/txin) [%.2fs (%.2fms/blk)]\n", nInputs - 1, MILLI * (nTime4 - nTime2), nInputs <= 1 ? 0 : MILLI * (nTime4 - nTime2) / (nInputs-1), nTimeVerify * MICRO, nTimeVerify * MILLI / nBlocksTotal);

        if (fJustCheck)
            return true;

        if (!WriteUndoDataForBlock(blockundo, state, pindex, m_params)) {
            return false;
        }

        if (!pindex->IsValid(BLOCK_VALID_SCRIPTS)) {
            pindex->RaiseValidity(BLOCK_VALID_SCRIPTS);
            setDirtyBlockIndex.insert(pindex);
        }

        assert(pindex->phashBlock);
        // add this block to the view's block chain
        view.SetBestBlock(pindex->GetBlockHash());

        int64_t nTime5 = GetTimeMicros(); nTimeIndex += nTime5 - nTime4;
        LogPrint(BCLog::BENCH, "    - Index writing: %.2fms [%.2fs (%.2fms/blk)]\n", MILLI * (nTime5 - nTime4), nTimeIndex * MICRO, nTimeIndex * MILLI / nBlocksTotal);

        TRACE6(validation, block_connected,
            block.GetHash().data(),
            pindex->nHeight,
            block.vtx.size(),
            nInputs,
            nSigOpsCost,
            GetTimeMicros() - nTimeStart // in microseconds (µs)
        );

        return true;
        */
    }
    
    /**
      | Dictates whether we need to flush the
      | cache to disk or not.
      |
      | @return the state of the size of the coins
      | cache.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn get_coins_cache_size_state(&mut self) -> CoinsCacheSizeState {
        
        todo!();
        /*
            return this->GetCoinsCacheSizeState(
            m_coinstip_cache_size_bytes,
            gArgs.GetIntArg("-maxmempool", DEFAULT_MAX_MEMPOOL_SIZE) * 1000000);
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    fn get_coins_cache_size_state_impl(&mut self, 
        max_coins_cache_size_bytes: usize,
        max_mempool_size_bytes:     usize) -> CoinsCacheSizeState {
        
        todo!();
        /*
            const int64_t nMempoolUsage = m_mempool ? m_mempool->DynamicMemoryUsage() : 0;
        int64_t cacheSize = CoinsTip().DynamicMemoryUsage();
        int64_t nTotalSpace =
            max_coins_cache_size_bytes + std::max<int64_t>(max_mempool_size_bytes - nMempoolUsage, 0);

        /// No need to periodic flush if at least this much space still available.
        static constexpr int64_t MAX_BLOCK_COINSDB_USAGE_BYTES = 10 * 1024 * 1024;  // 10MB
        int64_t large_threshold =
            std::max((9 * nTotalSpace) / 10, nTotalSpace - MAX_BLOCK_COINSDB_USAGE_BYTES);

        if (cacheSize > nTotalSpace) {
            LogPrintf("Cache size (%s) exceeds total space (%s)\n", cacheSize, nTotalSpace);
            return CoinsCacheSizeState::CRITICAL;
        } else if (cacheSize > large_threshold) {
            return CoinsCacheSizeState::LARGE;
        }
        return CoinsCacheSizeState::OK;
        */
    }
    
    /**
      | Update the on-disk chain state.
      | 
      | The caches and indexes are flushed depending
      | on the mode we're called with if they're
      | too large, if it's been a while since
      | the last write, or always and in all cases
      | if we're in prune mode and are deleting
      | files.
      | 
      | If FlushStateMode::NONE is used, then
      | FlushStateToDisk(...) won't do anything
      | besides checking if we need to prune.
      | 
      | 
      | -----------
      | @return
      | 
      | true unless a system error occurred
      |
      */
    pub fn flush_state_to_disk(&mut self, 
        state:                 &mut BlockValidationState,
        mode:                  FlushStateMode,
        n_manual_prune_height: Option<i32>) -> bool {

        let n_manual_prune_height: i32 = n_manual_prune_height.unwrap_or(0);
        
        todo!();
        /*
            LOCK(cs_main);
        assert(this->CanFlushToDisk());
        static microseconds nLastWrite{0};
        static microseconds nLastFlush{0};
        std::set<int> setFilesToPrune;
        bool full_flush_completed = false;

        const size_t coins_count = CoinsTip().GetCacheSize();
        const size_t coins_mem_usage = CoinsTip().DynamicMemoryUsage();

        try {
        {
            bool fFlushForPrune = false;
            bool fDoFullFlush = false;

            CoinsCacheSizeState cache_state = GetCoinsCacheSizeState();
            LOCK(cs_LastBlockFile);
            if (fPruneMode && (fCheckForPruning || nManualPruneHeight > 0) && !fReindex) {
                // make sure we don't prune above the blockfilterindexes bestblocks
                // pruning is height-based
                int last_prune = m_chain.Height(); // last height we can prune
                ForEachBlockFilterIndex([&](BlockFilterIndex& index) {
                   last_prune = std::max(1, std::min(last_prune, index.GetSummary().best_block_height));
                });

                if (nManualPruneHeight > 0) {
                    LOG_TIME_MILLIS_WITH_CATEGORY("find files to prune (manual)", BCLog::BENCH);

                    m_blockman.FindFilesToPruneManual(setFilesToPrune, std::min(last_prune, nManualPruneHeight), m_chain.Height());
                } else {
                    LOG_TIME_MILLIS_WITH_CATEGORY("find files to prune", BCLog::BENCH);

                    m_blockman.FindFilesToPrune(setFilesToPrune, m_params.PruneAfterHeight(), m_chain.Height(), last_prune, IsInitialBlockDownload());
                    fCheckForPruning = false;
                }
                if (!setFilesToPrune.empty()) {
                    fFlushForPrune = true;
                    if (!fHavePruned) {
                        m_blockman.m_block_tree_db->WriteFlag("prunedblockfiles", true);
                        fHavePruned = true;
                    }
                }
            }
            const auto nNow = GetTime<microseconds>();
            // Avoid writing/flushing immediately after startup.
            if (nLastWrite.count() == 0) {
                nLastWrite = nNow;
            }
            if (nLastFlush.count() == 0) {
                nLastFlush = nNow;
            }
            // The cache is large and we're within 10% and 10 MiB of the limit, but we have time now (not in the middle of a block processing).
            bool fCacheLarge = mode == FlushStateMode::PERIODIC && cache_state >= CoinsCacheSizeState::LARGE;
            // The cache is over the limit, we have to write now.
            bool fCacheCritical = mode == FlushStateMode::IF_NEEDED && cache_state >= CoinsCacheSizeState::CRITICAL;
            // It's been a while since we wrote the block index to disk. Do this frequently, so we don't need to redownload after a crash.
            bool fPeriodicWrite = mode == FlushStateMode::PERIODIC && nNow > nLastWrite + DATABASE_WRITE_INTERVAL;
            // It's been very long since we flushed the cache. Do this infrequently, to optimize cache usage.
            bool fPeriodicFlush = mode == FlushStateMode::PERIODIC && nNow > nLastFlush + DATABASE_FLUSH_INTERVAL;
            // Combine all conditions that result in a full cache flush.
            fDoFullFlush = (mode == FlushStateMode::ALWAYS) || fCacheLarge || fCacheCritical || fPeriodicFlush || fFlushForPrune;
            // Write blocks and block index to disk.
            if (fDoFullFlush || fPeriodicWrite) {
                // Depend on nMinDiskSpace to ensure we can write block index
                if (!CheckDiskSpace(gArgs.GetBlocksDirPath())) {
                    return AbortNode(state, "Disk space is too low!", _("Disk space is too low!"));
                }
                {
                    LOG_TIME_MILLIS_WITH_CATEGORY("write block and undo data to disk", BCLog::BENCH);

                    // First make sure all block and undo data is flushed to disk.
                    FlushBlockFile();
                }

                // Then update all block file information (which may refer to block and undo files).
                {
                    LOG_TIME_MILLIS_WITH_CATEGORY("write block index to disk", BCLog::BENCH);

                    std::vector<std::pair<int, const CBlockFileInfo*> > vFiles;
                    vFiles.reserve(setDirtyFileInfo.size());
                    for (std::set<int>::iterator it = setDirtyFileInfo.begin(); it != setDirtyFileInfo.end(); ) {
                        vFiles.push_back(std::make_pair(*it, &vinfoBlockFile[*it]));
                        setDirtyFileInfo.erase(it++);
                    }
                    std::vector<const CBlockIndex*> vBlocks;
                    vBlocks.reserve(setDirtyBlockIndex.size());
                    for (std::set<CBlockIndex*>::iterator it = setDirtyBlockIndex.begin(); it != setDirtyBlockIndex.end(); ) {
                        vBlocks.push_back(*it);
                        setDirtyBlockIndex.erase(it++);
                    }
                    if (!m_blockman.m_block_tree_db->WriteBatchSync(vFiles, nLastBlockFile, vBlocks)) {
                        return AbortNode(state, "Failed to write to block index database");
                    }
                }
                // Finally remove any pruned files
                if (fFlushForPrune) {
                    LOG_TIME_MILLIS_WITH_CATEGORY("unlink pruned files", BCLog::BENCH);

                    UnlinkPrunedFiles(setFilesToPrune);
                }
                nLastWrite = nNow;
            }
            // Flush best chain related state. This can only be done if the blocks / block index write was also done.
            if (fDoFullFlush && !CoinsTip().GetBestBlock().IsNull()) {
                LOG_TIME_MILLIS_WITH_CATEGORY(strprintf("write coins cache to disk (%d coins, %.2fkB)",
                    coins_count, coins_mem_usage / 1000), BCLog::BENCH);

                // Typical Coin structures on disk are around 48 bytes in size.
                // Pushing a new one to the database can cause it to be written
                // twice (once in the log, and once in the tables). This is already
                // an overestimation, as most will delete an existing entry or
                // overwrite one. Still, use a conservative safety factor of 2.
                if (!CheckDiskSpace(gArgs.GetDataDirNet(), 48 * 2 * 2 * CoinsTip().GetCacheSize())) {
                    return AbortNode(state, "Disk space is too low!", _("Disk space is too low!"));
                }
                // Flush the chainstate (which may refer to block index entries).
                if (!CoinsTip().Flush())
                    return AbortNode(state, "Failed to write to coin database");
                nLastFlush = nNow;
                full_flush_completed = true;
            }
        }
        if (full_flush_completed) {
            // Update best block in wallet (so we can detect restored wallets).
            GetMainSignals().ChainStateFlushed(m_chain.GetLocator());
        }
        } catch (const std::runtime_error& e) {
            return AbortNode(state, std::string("System error while flushing: ") + e.what());
        }
        return true;
        */
    }
    
    /**
      | Unconditionally flush all changes
      | to disk.
      |
      */
    pub fn force_flush_state_to_disk(&mut self)  {
        
        todo!();
        /*
            BlockValidationState state;
        if (!this->FlushStateToDisk(state, FlushStateMode::ALWAYS)) {
            LogPrintf("%s: failed to flush state (%s)\n", __func__, state.ToString());
        }
        */
    }
    
    /**
      | Prune blockfiles from the disk if necessary
      | and then flush chainstate changes if
      | we pruned.
      |
      */
    pub fn prune_and_flush(&mut self)  {
        
        todo!();
        /*
            BlockValidationState state;
        fCheckForPruning = true;
        if (!this->FlushStateToDisk(state, FlushStateMode::NONE)) {
            LogPrintf("%s: failed to flush state (%s)\n", __func__, state.ToString());
        }
        */
    }

    /**
      | Check warning conditions and do some
      | notifications on new chain tip set.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn update_tip(&mut self, pindex_new: *const BlockIndex)  {
        
        todo!();
        /*
            const auto& coins_tip = this->CoinsTip();

        // The remainder of the function isn't relevant if we are not acting on
        // the active chainstate, so return if need be.
        if (this != &m_chainman.ActiveChainstate()) {
            // Only log every so often so that we don't bury log messages at the tip.
            constexpr int BACKGROUND_LOG_INTERVAL = 2000;
            if (pindexNew->nHeight % BACKGROUND_LOG_INTERVAL == 0) {
                UpdateTipLog(coins_tip, pindexNew, m_params, __func__, "[background validation] ", "");
            }
            return;
        }

        // New best block
        if (m_mempool) {
            m_mempool->AddTransactionsUpdated(1);
        }

        {
            LOCK(g_best_block_mutex);
            g_best_block = pindexNew->GetBlockHash();
            g_best_block_cv.notify_all();
        }

        bilingual_str warning_messages;
        if (!this->IsInitialBlockDownload()) {
            const CBlockIndex* pindex = pindexNew;
            for (int bit = 0; bit < VERSIONBITS_NUM_BITS; bit++) {
                WarningBitsConditionChecker checker(bit);
                ThresholdState state = checker.GetStateFor(pindex, m_params.GetConsensus(), warningcache[bit]);
                if (state == ThresholdState::ACTIVE || state == ThresholdState::LOCKED_IN) {
                    const bilingual_str warning = strprintf(_("Unknown new rules activated (versionbit %i)"), bit);
                    if (state == ThresholdState::ACTIVE) {
                        DoWarning(warning);
                    } else {
                        AppendWarning(warning_messages, warning);
                    }
                }
            }
        }
        UpdateTipLog(coins_tip, pindexNew, m_params, __func__, "", warning_messages.original);
        */
    }

    /**
      | Apply the effects of a block disconnection
      | on the UTXO set.
      |
      | Disconnect m_chain's tip.
      | 
      | After calling, the mempool will be in
      | an inconsistent state, with transactions
      | from disconnected blocks being added
      | to disconnectpool. You should make
      | the mempool consistent again by calling
      | MaybeUpdateMempoolForReorg. with
      | cs_main held.
      | 
      | If disconnectpool is nullptr, then
      | no disconnected transactions are added
      | to disconnectpool (note that the caller
      | is responsible for mempool consistency
      | in any case).
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main, m_mempool->cs)]
    pub fn disconnect_tip(&mut self, 
        state:          &mut BlockValidationState,
        disconnectpool: *mut DisconnectedBlockTransactions) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        if (m_mempool) AssertLockHeld(m_mempool->cs);

        CBlockIndex *pindexDelete = m_chain.Tip();
        assert(pindexDelete);
        // Read block from disk.
        std::shared_ptr<CBlock> pblock = std::make_shared<CBlock>();
        CBlock& block = *pblock;
        if (!ReadBlockFromDisk(block, pindexDelete, m_params.GetConsensus())) {
            return error("DisconnectTip(): Failed to read block");
        }
        // Apply the block atomically to the chain state.
        int64_t nStart = GetTimeMicros();
        {
            CCoinsViewCache view(&CoinsTip());
            assert(view.GetBestBlock() == pindexDelete->GetBlockHash());
            if (DisconnectBlock(block, pindexDelete, view) != DISCONNECT_OK)
                return error("DisconnectTip(): DisconnectBlock %s failed", pindexDelete->GetBlockHash().ToString());
            bool flushed = view.Flush();
            assert(flushed);
        }
        LogPrint(BCLog::BENCH, "- Disconnect block: %.2fms\n", (GetTimeMicros() - nStart) * MILLI);
        // Write the chain state to disk, if necessary.
        if (!FlushStateToDisk(state, FlushStateMode::IF_NEEDED)) {
            return false;
        }

        if (disconnectpool && m_mempool) {
            // Save transactions to re-add to mempool at end of reorg
            for (auto it = block.vtx.rbegin(); it != block.vtx.rend(); ++it) {
                disconnectpool->addTransaction(*it);
            }
            while (disconnectpool->DynamicMemoryUsage() > MAX_DISCONNECTED_TX_POOL_SIZE * 1000) {
                // Drop the earliest entry, and remove its children from the mempool.
                auto it = disconnectpool->queuedTx.get<insertion_order>().begin();
                m_mempool->removeRecursive(**it, MemPoolRemovalReason::REORG);
                disconnectpool->removeEntry(it);
            }
        }

        m_chain.SetTip(pindexDelete->pprev);

        UpdateTip(pindexDelete->pprev);
        // Let wallets know transactions went from 1-confirmed to
        // 0-confirmed or conflicted:
        GetMainSignals().BlockDisconnected(pblock, pindexDelete);
        return true;
        */
    }

    /**
      | Connect a new block to m_chain. pblock
      | is either nullptr or a pointer to a CBlock
      | corresponding to pindexNew, to bypass
      | loading it again from disk.
      | 
      | The block is added to connectTrace if
      | connection succeeds.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main, m_mempool->cs)]
    pub fn connect_tip(&mut self, 
        state:          &mut BlockValidationState,
        pindex_new:     *mut BlockIndex,
        pblock:         &Arc<Block>,
        connect_trace:  &mut ConnectTrace,
        disconnectpool: &mut DisconnectedBlockTransactions) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        if (m_mempool) AssertLockHeld(m_mempool->cs);

        assert(pindexNew->pprev == m_chain.Tip());
        // Read block from disk.
        int64_t nTime1 = GetTimeMicros();
        std::shared_ptr<const CBlock> pthisBlock;
        if (!pblock) {
            std::shared_ptr<CBlock> pblockNew = std::make_shared<CBlock>();
            if (!ReadBlockFromDisk(*pblockNew, pindexNew, m_params.GetConsensus())) {
                return AbortNode(state, "Failed to read block");
            }
            pthisBlock = pblockNew;
        } else {
            pthisBlock = pblock;
        }
        const CBlock& blockConnecting = *pthisBlock;
        // Apply the block atomically to the chain state.
        int64_t nTime2 = GetTimeMicros(); nTimeReadFromDisk += nTime2 - nTime1;
        int64_t nTime3;
        LogPrint(BCLog::BENCH, "  - Load block from disk: %.2fms [%.2fs]\n", (nTime2 - nTime1) * MILLI, nTimeReadFromDisk * MICRO);
        {
            CCoinsViewCache view(&CoinsTip());
            bool rv = ConnectBlock(blockConnecting, state, pindexNew, view);
            GetMainSignals().BlockChecked(blockConnecting, state);
            if (!rv) {
                if (state.IsInvalid())
                    InvalidBlockFound(pindexNew, state);
                return error("%s: ConnectBlock %s failed, %s", __func__, pindexNew->GetBlockHash().ToString(), state.ToString());
            }
            nTime3 = GetTimeMicros(); nTimeConnectTotal += nTime3 - nTime2;
            assert(nBlocksTotal > 0);
            LogPrint(BCLog::BENCH, "  - Connect total: %.2fms [%.2fs (%.2fms/blk)]\n", (nTime3 - nTime2) * MILLI, nTimeConnectTotal * MICRO, nTimeConnectTotal * MILLI / nBlocksTotal);
            bool flushed = view.Flush();
            assert(flushed);
        }
        int64_t nTime4 = GetTimeMicros(); nTimeFlush += nTime4 - nTime3;
        LogPrint(BCLog::BENCH, "  - Flush: %.2fms [%.2fs (%.2fms/blk)]\n", (nTime4 - nTime3) * MILLI, nTimeFlush * MICRO, nTimeFlush * MILLI / nBlocksTotal);
        // Write the chain state to disk, if necessary.
        if (!FlushStateToDisk(state, FlushStateMode::IF_NEEDED)) {
            return false;
        }
        int64_t nTime5 = GetTimeMicros(); nTimeChainState += nTime5 - nTime4;
        LogPrint(BCLog::BENCH, "  - Writing chainstate: %.2fms [%.2fs (%.2fms/blk)]\n", (nTime5 - nTime4) * MILLI, nTimeChainState * MICRO, nTimeChainState * MILLI / nBlocksTotal);
        // Remove conflicting transactions from the mempool.;
        if (m_mempool) {
            m_mempool->removeForBlock(blockConnecting.vtx, pindexNew->nHeight);
            disconnectpool.removeForBlock(blockConnecting.vtx);
        }
        // Update m_chain & related variables.
        m_chain.SetTip(pindexNew);
        UpdateTip(pindexNew);

        int64_t nTime6 = GetTimeMicros(); nTimePostConnect += nTime6 - nTime5; nTimeTotal += nTime6 - nTime1;
        LogPrint(BCLog::BENCH, "  - Connect postprocess: %.2fms [%.2fs (%.2fms/blk)]\n", (nTime6 - nTime5) * MILLI, nTimePostConnect * MICRO, nTimePostConnect * MILLI / nBlocksTotal);
        LogPrint(BCLog::BENCH, "- Connect block: %.2fms [%.2fs (%.2fms/blk)]\n", (nTime6 - nTime1) * MILLI, nTimeTotal * MICRO, nTimeTotal * MILLI / nBlocksTotal);

        connectTrace.BlockConnected(pindexNew, std::move(pthisBlock));
        return true;
        */
    }

    /**
      | Return the tip of the chain with the most
      | work in it, that isn't known to be invalid
      | (it's however far from certain to be
      | valid).
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn find_most_work_chain(&mut self) -> *mut BlockIndex {
        
        todo!();
        /*
            do {
            CBlockIndex *pindexNew = nullptr;

            // Find the best candidate header.
            {
                std::set<CBlockIndex*, CBlockIndexWorkComparator>::reverse_iterator it = setBlockIndexCandidates.rbegin();
                if (it == setBlockIndexCandidates.rend())
                    return nullptr;
                pindexNew = *it;
            }

            // Check whether all blocks on the path between the currently active chain and the candidate are valid.
            // Just going until the active chain is an optimization, as we know all blocks in it are valid already.
            CBlockIndex *pindexTest = pindexNew;
            bool fInvalidAncestor = false;
            while (pindexTest && !m_chain.Contains(pindexTest)) {
                assert(pindexTest->HaveTxsDownloaded() || pindexTest->nHeight == 0);

                // Pruned nodes may have entries in setBlockIndexCandidates for
                // which block files have been deleted.  Remove those as candidates
                // for the most work chain if we come across them; we can't switch
                // to a chain unless we have all the non-active-chain parent blocks.
                bool fFailedChain = pindexTest->nStatus & BLOCK_FAILED_MASK;
                bool fMissingData = !(pindexTest->nStatus & BLOCK_HAVE_DATA);
                if (fFailedChain || fMissingData) {
                    // Candidate chain is not usable (either invalid or missing data)
                    if (fFailedChain && (pindexBestInvalid == nullptr || pindexNew->nChainWork > pindexBestInvalid->nChainWork))
                        pindexBestInvalid = pindexNew;
                    CBlockIndex *pindexFailed = pindexNew;
                    // Remove the entire chain from the set.
                    while (pindexTest != pindexFailed) {
                        if (fFailedChain) {
                            pindexFailed->nStatus |= BLOCK_FAILED_CHILD;
                        } else if (fMissingData) {
                            // If we're missing data, then add back to m_blocks_unlinked,
                            // so that if the block arrives in the future we can try adding
                            // to setBlockIndexCandidates again.
                            m_blockman.m_blocks_unlinked.insert(
                                std::make_pair(pindexFailed->pprev, pindexFailed));
                        }
                        setBlockIndexCandidates.erase(pindexFailed);
                        pindexFailed = pindexFailed->pprev;
                    }
                    setBlockIndexCandidates.erase(pindexTest);
                    fInvalidAncestor = true;
                    break;
                }
                pindexTest = pindexTest->pprev;
            }
            if (!fInvalidAncestor)
                return pindexNew;
        } while(true);
        */
    }

    /**
      | Delete all entries in setBlockIndexCandidates
      | that are worse than the current tip.
      |
      */
    pub fn prune_block_index_candidates(&mut self)  {
        
        todo!();
        /*
            // Note that we can't delete the current block itself, as we may need to return to it later in case a
        // reorganization to a better block fails.
        std::set<CBlockIndex*, CBlockIndexWorkComparator>::iterator it = setBlockIndexCandidates.begin();
        while (it != setBlockIndexCandidates.end() && setBlockIndexCandidates.value_comp()(*it, m_chain.Tip())) {
            setBlockIndexCandidates.erase(it++);
        }
        // Either the current tip or a successor of it we're working towards is left in setBlockIndexCandidates.
        assert(!setBlockIndexCandidates.empty());
        */
    }

    /**
      | Try to make some progress towards making
      | pindexMostWork the active block. pblock
      | is either nullptr or a pointer to a CBlock
      | corresponding to pindexMostWork.
      | 
      | 
      | -----------
      | @return
      | 
      | true unless a system error occurred
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main, m_mempool->cs)]
    pub fn activate_best_chain_step(&mut self, 
        state:            &mut BlockValidationState,
        pindex_most_work: *mut BlockIndex,
        pblock:           &Arc<Block>,
        invalid_found:    &mut bool,
        connect_trace:    &mut ConnectTrace) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        if (m_mempool) AssertLockHeld(m_mempool->cs);

        const CBlockIndex* pindexOldTip = m_chain.Tip();
        const CBlockIndex* pindexFork = m_chain.FindFork(pindexMostWork);

        // Disconnect active blocks which are no longer in the best chain.
        bool fBlocksDisconnected = false;
        DisconnectedBlockTransactions disconnectpool;
        while (m_chain.Tip() && m_chain.Tip() != pindexFork) {
            if (!DisconnectTip(state, &disconnectpool)) {
                // This is likely a fatal error, but keep the mempool consistent,
                // just in case. Only remove from the mempool in this case.
                MaybeUpdateMempoolForReorg(disconnectpool, false);

                // If we're unable to disconnect a block during normal operation,
                // then that is a failure of our local system -- we should abort
                // rather than stay on a less work chain.
                AbortNode(state, "Failed to disconnect block; see debug.log for details");
                return false;
            }
            fBlocksDisconnected = true;
        }

        // Build list of new blocks to connect (in descending height order).
        std::vector<CBlockIndex*> vpindexToConnect;
        bool fContinue = true;
        int nHeight = pindexFork ? pindexFork->nHeight : -1;
        while (fContinue && nHeight != pindexMostWork->nHeight) {
            // Don't iterate the entire list of potential improvements toward the best tip, as we likely only need
            // a few blocks along the way.
            int nTargetHeight = std::min(nHeight + 32, pindexMostWork->nHeight);
            vpindexToConnect.clear();
            vpindexToConnect.reserve(nTargetHeight - nHeight);
            CBlockIndex* pindexIter = pindexMostWork->GetAncestor(nTargetHeight);
            while (pindexIter && pindexIter->nHeight != nHeight) {
                vpindexToConnect.push_back(pindexIter);
                pindexIter = pindexIter->pprev;
            }
            nHeight = nTargetHeight;

            // Connect new blocks.
            for (CBlockIndex* pindexConnect : reverse_iterate(vpindexToConnect)) {
                if (!ConnectTip(state, pindexConnect, pindexConnect == pindexMostWork ? pblock : std::shared_ptr<const CBlock>(), connectTrace, disconnectpool)) {
                    if (state.IsInvalid()) {
                        // The block violates a consensus rule.
                        if (state.GetResult() != BlockValidationResult::BLOCK_MUTATED) {
                            InvalidChainFound(vpindexToConnect.front());
                        }
                        state = BlockValidationState();
                        fInvalidFound = true;
                        fContinue = false;
                        break;
                    } else {
                        // A system error occurred (disk space, database error, ...).
                        // Make the mempool consistent with the current tip, just in case
                        // any observers try to use it before shutdown.
                        MaybeUpdateMempoolForReorg(disconnectpool, false);
                        return false;
                    }
                } else {
                    PruneBlockIndexCandidates();
                    if (!pindexOldTip || m_chain.Tip()->nChainWork > pindexOldTip->nChainWork) {
                        // We're in a better position than we were. Return temporarily to release the lock.
                        fContinue = false;
                        break;
                    }
                }
            }
        }

        if (fBlocksDisconnected) {
            // If any blocks were disconnected, disconnectpool may be non empty.  Add
            // any disconnected transactions back to the mempool.
            MaybeUpdateMempoolForReorg(disconnectpool, true);
        }
        if (m_mempool) m_mempool->check(this->CoinsTip(), this->m_chain.Height() + 1);

        CheckForkWarningConditions();

        return true;
        */
    }
    
    
    /**
      | Mark a block as precious and reorganize.
      | 
      | May not be called in a validationinterface
      | callback.
      |
      */
    #[LOCKS_EXCLUDED(cs_main)]
    pub fn precious_block(&mut self, 
        state:  &mut BlockValidationState,
        pindex: *mut BlockIndex) -> bool {
        
        todo!();
        /*
            {
            LOCK(cs_main);
            if (pindex->nChainWork < m_chain.Tip()->nChainWork) {
                // Nothing to do, this block is not at the tip.
                return true;
            }
            if (m_chain.Tip()->nChainWork > nLastPreciousChainwork) {
                // The chain has been extended since the last call, reset the counter.
                nBlockReverseSequenceId = -1;
            }
            nLastPreciousChainwork = m_chain.Tip()->nChainWork;
            setBlockIndexCandidates.erase(pindex);
            pindex->nSequenceId = nBlockReverseSequenceId;
            if (nBlockReverseSequenceId > std::numeric_limits<int32_t>::min()) {
                // We can't keep reducing the counter if somebody really wants to
                // call preciousblock 2**31-1 times on the same set of tips...
                nBlockReverseSequenceId--;
            }
            if (pindex->IsValid(BLOCK_VALID_TRANSACTIONS) && pindex->HaveTxsDownloaded()) {
                setBlockIndexCandidates.insert(pindex);
                PruneBlockIndexCandidates();
            }
        }

        return ActivateBestChain(state, std::shared_ptr<const CBlock>());
        */
    }
    
    /**
      | Mark a block as invalid.
      |
      */
    #[LOCKS_EXCLUDED(cs_main)]
    pub fn invalidate_block(&mut self, 
        state:  &mut BlockValidationState,
        pindex: *mut BlockIndex) -> bool {
        
        todo!();
        /*
            // Genesis block can't be invalidated
        assert(pindex);
        if (pindex->nHeight == 0) return false;

        CBlockIndex* to_mark_failed = pindex;
        bool pindex_was_in_chain = false;
        int disconnected = 0;

        // We do not allow ActivateBestChain() to run while InvalidateBlock() is
        // running, as that could cause the tip to change while we disconnect
        // blocks.
        LOCK(m_cs_chainstate);

        // We'll be acquiring and releasing cs_main below, to allow the validation
        // callbacks to run. However, we should keep the block index in a
        // consistent state as we disconnect blocks -- in particular we need to
        // add equal-work blocks to setBlockIndexCandidates as we disconnect.
        // To avoid walking the block index repeatedly in search of candidates,
        // build a map once so that we can look up candidate blocks by chain
        // work as we go.
        std::multimap<const ArithU256, CBlockIndex *> candidate_blocks_by_work;

        {
            LOCK(cs_main);
            for (const auto& entry : m_blockman.m_block_index) {
                CBlockIndex *candidate = entry.second;
                // We don't need to put anything in our active chain into the
                // multimap, because those candidates will be found and considered
                // as we disconnect.
                // Instead, consider only non-active-chain blocks that have at
                // least as much work as where we expect the new tip to end up.
                if (!m_chain.Contains(candidate) &&
                        !CBlockIndexWorkComparator()(candidate, pindex->pprev) &&
                        candidate->IsValid(BLOCK_VALID_TRANSACTIONS) &&
                        candidate->HaveTxsDownloaded()) {
                    candidate_blocks_by_work.insert(std::make_pair(candidate->nChainWork, candidate));
                }
            }
        }

        // Disconnect (descendants of) pindex, and mark them invalid.
        while (true) {
            if (ShutdownRequested()) break;

            // Make sure the queue of validation callbacks doesn't grow unboundedly.
            LimitValidationInterfaceQueue();

            LOCK(cs_main);
            // Lock for as long as disconnectpool is in scope to make sure MaybeUpdateMempoolForReorg is
            // called after DisconnectTip without unlocking in between
            LOCK(MempoolMutex());
            if (!m_chain.Contains(pindex)) break;
            pindex_was_in_chain = true;
            CBlockIndex *invalid_walk_tip = m_chain.Tip();

            // ActivateBestChain considers blocks already in m_chain
            // unconditionally valid already, so force disconnect away from it.
            DisconnectedBlockTransactions disconnectpool;
            bool ret = DisconnectTip(state, &disconnectpool);
            // DisconnectTip will add transactions to disconnectpool.
            // Adjust the mempool to be consistent with the new tip, adding
            // transactions back to the mempool if disconnecting was successful,
            // and we're not doing a very deep invalidation (in which case
            // keeping the mempool up to date is probably futile anyway).
            MaybeUpdateMempoolForReorg(disconnectpool, /* fAddToMempool = */ (++disconnected <= 10) && ret);
            if (!ret) return false;
            assert(invalid_walk_tip->pprev == m_chain.Tip());

            // We immediately mark the disconnected blocks as invalid.
            // This prevents a case where pruned nodes may fail to invalidateblock
            // and be left unable to start as they have no tip candidates (as there
            // are no blocks that meet the "have data and are not invalid per
            // nStatus" criteria for inclusion in setBlockIndexCandidates).
            invalid_walk_tip->nStatus |= BLOCK_FAILED_VALID;
            setDirtyBlockIndex.insert(invalid_walk_tip);
            setBlockIndexCandidates.erase(invalid_walk_tip);
            setBlockIndexCandidates.insert(invalid_walk_tip->pprev);
            if (invalid_walk_tip->pprev == to_mark_failed && (to_mark_failed->nStatus & BLOCK_FAILED_VALID)) {
                // We only want to mark the last disconnected block as BLOCK_FAILED_VALID; its children
                // need to be BLOCK_FAILED_CHILD instead.
                to_mark_failed->nStatus = (to_mark_failed->nStatus ^ BLOCK_FAILED_VALID) | BLOCK_FAILED_CHILD;
                setDirtyBlockIndex.insert(to_mark_failed);
            }

            // Add any equal or more work headers to setBlockIndexCandidates
            auto candidate_it = candidate_blocks_by_work.lower_bound(invalid_walk_tip->pprev->nChainWork);
            while (candidate_it != candidate_blocks_by_work.end()) {
                if (!CBlockIndexWorkComparator()(candidate_it->second, invalid_walk_tip->pprev)) {
                    setBlockIndexCandidates.insert(candidate_it->second);
                    candidate_it = candidate_blocks_by_work.erase(candidate_it);
                } else {
                    ++candidate_it;
                }
            }

            // Track the last disconnected block, so we can correct its BLOCK_FAILED_CHILD status in future
            // iterations, or, if it's the last one, call InvalidChainFound on it.
            to_mark_failed = invalid_walk_tip;
        }

        CheckBlockIndex();

        {
            LOCK(cs_main);
            if (m_chain.Contains(to_mark_failed)) {
                // If the to-be-marked invalid block is in the active chain, something is interfering and we can't proceed.
                return false;
            }

            // Mark pindex (or the last disconnected block) as invalid, even when it never was in the main chain
            to_mark_failed->nStatus |= BLOCK_FAILED_VALID;
            setDirtyBlockIndex.insert(to_mark_failed);
            setBlockIndexCandidates.erase(to_mark_failed);
            m_blockman.m_failed_blocks.insert(to_mark_failed);

            // If any new blocks somehow arrived while we were disconnecting
            // (above), then the pre-calculation of what should go into
            // setBlockIndexCandidates may have missed entries. This would
            // technically be an inconsistency in the block index, but if we clean
            // it up here, this should be an essentially unobservable error.
            // Loop back over all block index entries and add any missing entries
            // to setBlockIndexCandidates.
            BlockMap::iterator it = m_blockman.m_block_index.begin();
            while (it != m_blockman.m_block_index.end()) {
                if (it->second->IsValid(BLOCK_VALID_TRANSACTIONS) && it->second->HaveTxsDownloaded() && !setBlockIndexCandidates.value_comp()(it->second, m_chain.Tip())) {
                    setBlockIndexCandidates.insert(it->second);
                }
                it++;
            }

            InvalidChainFound(to_mark_failed);
        }

        // Only notify about a new block tip if the active chain was modified.
        if (pindex_was_in_chain) {
            uiInterface.NotifyBlockTip(GetSynchronizationState(IsInitialBlockDownload()), to_mark_failed->pprev);
        }
        return true;
        */
    }
    
    /**
      | Remove invalidity status from a block
      | and its descendants.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn reset_block_failure_flags(&mut self, pindex: *mut BlockIndex)  {
        
        todo!();
        /*
            AssertLockHeld(cs_main);

        int nHeight = pindex->nHeight;

        // Remove the invalidity flag from this block and all its descendants.
        BlockMap::iterator it = m_blockman.m_block_index.begin();
        while (it != m_blockman.m_block_index.end()) {
            if (!it->second->IsValid() && it->second->GetAncestor(nHeight) == pindex) {
                it->second->nStatus &= ~BLOCK_FAILED_MASK;
                setDirtyBlockIndex.insert(it->second);
                if (it->second->IsValid(BLOCK_VALID_TRANSACTIONS) && it->second->HaveTxsDownloaded() && setBlockIndexCandidates.value_comp()(m_chain.Tip(), it->second)) {
                    setBlockIndexCandidates.insert(it->second);
                }
                if (it->second == pindexBestInvalid) {
                    // Reset invalid block marker if it was pointing to one of those.
                    pindexBestInvalid = nullptr;
                }
                m_blockman.m_failed_blocks.erase(it->second);
            }
            it++;
        }

        // Remove the invalidity flag from all ancestors too.
        while (pindex != nullptr) {
            if (pindex->nStatus & BLOCK_FAILED_MASK) {
                pindex->nStatus &= ~BLOCK_FAILED_MASK;
                setDirtyBlockIndex.insert(pindex);
                m_blockman.m_failed_blocks.erase(pindex);
            }
            pindex = pindex->pprev;
        }
        */
    }

    /**
      | Mark a block as having its data received
      | and checked (up to BLOCK_VALID_TRANSACTIONS).
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn received_block_transactions(&mut self, 
        block:      &Block,
        pindex_new: *mut BlockIndex,
        pos:        &FlatFilePos)  {
        
        todo!();
        /*
            pindexNew->nTx = block.vtx.size();
        pindexNew->nChainTx = 0;
        pindexNew->nFile = pos.nFile;
        pindexNew->nDataPos = pos.nPos;
        pindexNew->nUndoPos = 0;
        pindexNew->nStatus |= BLOCK_HAVE_DATA;
        if (DeploymentActiveAt(*pindexNew, m_params.GetConsensus(), consensus::DEPLOYMENT_SEGWIT)) {
            pindexNew->nStatus |= BLOCK_OPT_WITNESS;
        }
        pindexNew->RaiseValidity(BLOCK_VALID_TRANSACTIONS);
        setDirtyBlockIndex.insert(pindexNew);

        if (pindexNew->pprev == nullptr || pindexNew->pprev->HaveTxsDownloaded()) {
            // If pindexNew is the genesis block or all parents are BLOCK_VALID_TRANSACTIONS.
            std::deque<CBlockIndex*> queue;
            queue.push_back(pindexNew);

            // Recursively process any descendant blocks that now may be eligible to be connected.
            while (!queue.empty()) {
                CBlockIndex *pindex = queue.front();
                queue.pop_front();
                pindex->nChainTx = (pindex->pprev ? pindex->pprev->nChainTx : 0) + pindex->nTx;
                pindex->nSequenceId = nBlockSequenceId++;
                if (m_chain.Tip() == nullptr || !setBlockIndexCandidates.value_comp()(pindex, m_chain.Tip())) {
                    setBlockIndexCandidates.insert(pindex);
                }
                std::pair<std::multimap<CBlockIndex*, CBlockIndex*>::iterator, std::multimap<CBlockIndex*, CBlockIndex*>::iterator> range = m_blockman.m_blocks_unlinked.equal_range(pindex);
                while (range.first != range.second) {
                    std::multimap<CBlockIndex*, CBlockIndex*>::iterator it = range.first;
                    queue.push_back(it->second);
                    range.first++;
                    m_blockman.m_blocks_unlinked.erase(it);
                }
            }
        } else {
            if (pindexNew->pprev && pindexNew->pprev->IsValid(BLOCK_VALID_TREE)) {
                m_blockman.m_blocks_unlinked.insert(std::make_pair(pindexNew->pprev, pindexNew));
            }
        }
        */
    }
    
    /**
      | Store block on disk. If dbp is non-nullptr,
      | the file is known to already reside on
      | disk
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn accept_block(&mut self, 
        pblock:    &Arc<Block>,
        state:     &mut BlockValidationState,
        ppindex:   *mut *mut BlockIndex,
        requested: bool,
        dbp:       *const FlatFilePos,
        new_block: *mut bool) -> bool {
        
        todo!();
        /*
            const CBlock& block = *pblock;

        if (fNewBlock) *fNewBlock = false;
        AssertLockHeld(cs_main);

        CBlockIndex *pindexDummy = nullptr;
        CBlockIndex *&pindex = ppindex ? *ppindex : pindexDummy;

        bool accepted_header = m_blockman.AcceptBlockHeader(block, state, m_params, &pindex);
        CheckBlockIndex();

        if (!accepted_header)
            return false;

        // Try to process all requested blocks that we don't have, but only
        // process an unrequested block if it's new and has enough work to
        // advance our tip, and isn't too many blocks ahead.
        bool fAlreadyHave = pindex->nStatus & BLOCK_HAVE_DATA;
        bool fHasMoreOrSameWork = (m_chain.Tip() ? pindex->nChainWork >= m_chain.Tip()->nChainWork : true);
        // Blocks that are too out-of-order needlessly limit the effectiveness of
        // pruning, because pruning will not delete block files that contain any
        // blocks which are too close in height to the tip.  Apply this test
        // regardless of whether pruning is enabled; it should generally be safe to
        // not process unrequested blocks.
        bool fTooFarAhead = (pindex->nHeight > int(m_chain.Height() + MIN_BLOCKS_TO_KEEP));

        // TODO: Decouple this function from the block download logic by removing fRequested
        // This requires some new chain data structure to efficiently look up if a
        // block is in a chain leading to a candidate for best tip, despite not
        // being such a candidate itself.

        // TODO: deal better with return value and error conditions for duplicate
        // and unrequested blocks.
        if (fAlreadyHave) return true;
        if (!fRequested) {  // If we didn't ask for it:
            if (pindex->nTx != 0) return true;    // This is a previously-processed block that was pruned
            if (!fHasMoreOrSameWork) return true; // Don't process less-work chains
            if (fTooFarAhead) return true;        // Block height is too high

            // Protect against DoS attacks from low-work chains.
            // If our tip is behind, a peer could try to send us
            // low-work blocks on a fake chain that we would never
            // request; don't process these.
            if (pindex->nChainWork < nMinimumChainWork) return true;
        }

        if (!CheckBlock(block, state, m_params.GetConsensus()) ||
            !ContextualCheckBlock(block, state, m_params.GetConsensus(), pindex->pprev)) {
            if (state.IsInvalid() && state.GetResult() != BlockValidationResult::BLOCK_MUTATED) {
                pindex->nStatus |= BLOCK_FAILED_VALID;
                setDirtyBlockIndex.insert(pindex);
            }
            return error("%s: %s", __func__, state.ToString());
        }

        // Header is valid/has work, merkle tree and segwit merkle tree are good...RELAY NOW
        // (but if it does not build on our best tip, let the SendMessages loop relay it)
        if (!IsInitialBlockDownload() && m_chain.Tip() == pindex->pprev)
            GetMainSignals().NewPoWValidBlock(pindex, pblock);

        // Write block to history file
        if (fNewBlock) *fNewBlock = true;
        try {
            FlatFilePos blockPos = SaveBlockToDisk(block, pindex->nHeight, m_chain, m_params, dbp);
            if (blockPos.IsNull()) {
                state.Error(strprintf("%s: Failed to find position to write new block to disk", __func__));
                return false;
            }
            ReceivedBlockTransactions(block, pindex, blockPos);
        } catch (const std::runtime_error& e) {
            return AbortNode(state, std::string("System error: ") + e.what());
        }

        FlushStateToDisk(state, FlushStateMode::NONE);

        CheckBlockIndex();

        return true;
        */
    }

    /**
      | Load the persisted mempool from disk
      |
      */
    pub fn load_mempool(&mut self, args: &ArgsManager)  {
        
        todo!();
        /*
            if (!m_mempool) return;
        if (args.GetBoolArg("-persistmempool", DEFAULT_PERSIST_MEMPOOL)) {
            ::LoadMempool(*m_mempool, *this);
        }
        m_mempool->SetIsLoaded(!ShutdownRequested());
        */
    }
    
    /**
      | Update the chain tip based on database
      | information, i.e. CoinsTip()'s best
      | block.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn load_chain_tip(&mut self) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        const CCoinsViewCache& coins_cache = CoinsTip();
        assert(!coins_cache.GetBestBlock().IsNull()); // Never called when the coins view is empty
        const CBlockIndex* tip = m_chain.Tip();

        if (tip && tip->GetBlockHash() == coins_cache.GetBestBlock()) {
            return true;
        }

        // Load pointer to end of best chain
        CBlockIndex* pindex = m_blockman.LookupBlockIndex(coins_cache.GetBestBlock());
        if (!pindex) {
            return false;
        }
        m_chain.SetTip(pindex);
        PruneBlockIndexCandidates();

        tip = m_chain.Tip();
        LogPrintf("Loaded best chain: hashBestChain=%s height=%d date=%s progress=%f\n",
                  tip->GetBlockHash().ToString(),
                  m_chain.Height(),
                  FormatISO8601DateTime(tip->GetBlockTime()),
                  GuessVerificationProgress(m_params.TxData(), tip));
        return true;
        */
    }

    /**
      | Apply the effects of a block on the utxo
      | cache, ignoring that it may already
      | have been applied.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn rollforward_block(&mut self, 
        pindex: *const BlockIndex,
        inputs: &mut CoinsViewCache) -> bool {
        
        todo!();
        /*
            // TODO: merge with ConnectBlock
        CBlock block;
        if (!ReadBlockFromDisk(block, pindex, m_params.GetConsensus())) {
            return error("ReplayBlock(): ReadBlockFromDisk failed at %d, hash=%s", pindex->nHeight, pindex->GetBlockHash().ToString());
        }

        for (const CTransactionRef& tx : block.vtx) {
            if (!tx->IsCoinBase()) {
                for (const CTxIn &txin : tx->vin) {
                    inputs.SpendCoin(txin.prevout);
                }
            }
            // Pass check = true as every addition may be an overwrite.
            AddCoins(inputs, *tx, pindex->nHeight, true);
        }
        return true;
        */
    }
    
    /**
      | Replay blocks that aren't fully applied
      | to the database.
      |
      */
    pub fn replay_blocks(&mut self) -> bool {
        
        todo!();
        /*
            LOCK(cs_main);

        CCoinsView& db = this->CoinsDB();
        CCoinsViewCache cache(&db);

        std::vector<uint256> hashHeads = db.GetHeadBlocks();
        if (hashHeads.empty()) return true; // We're already in a consistent state.
        if (hashHeads.size() != 2) return error("ReplayBlocks(): unknown inconsistent state");

        uiInterface.ShowProgress(_("Replaying blocks…").translated, 0, false);
        LogPrintf("Replaying blocks\n");

        const CBlockIndex* pindexOld = nullptr;  // Old tip during the interrupted flush.
        const CBlockIndex* pindexNew;            // New tip during the interrupted flush.
        const CBlockIndex* pindexFork = nullptr; // Latest block common to both the old and the new tip.

        if (m_blockman.m_block_index.count(hashHeads[0]) == 0) {
            return error("ReplayBlocks(): reorganization to unknown block requested");
        }
        pindexNew = m_blockman.m_block_index[hashHeads[0]];

        if (!hashHeads[1].IsNull()) { // The old tip is allowed to be 0, indicating it's the first flush.
            if (m_blockman.m_block_index.count(hashHeads[1]) == 0) {
                return error("ReplayBlocks(): reorganization from unknown block requested");
            }
            pindexOld = m_blockman.m_block_index[hashHeads[1]];
            pindexFork = LastCommonAncestor(pindexOld, pindexNew);
            assert(pindexFork != nullptr);
        }

        // Rollback along the old branch.
        while (pindexOld != pindexFork) {
            if (pindexOld->nHeight > 0) { // Never disconnect the genesis block.
                CBlock block;
                if (!ReadBlockFromDisk(block, pindexOld, m_params.GetConsensus())) {
                    return error("RollbackBlock(): ReadBlockFromDisk() failed at %d, hash=%s", pindexOld->nHeight, pindexOld->GetBlockHash().ToString());
                }
                LogPrintf("Rolling back %s (%i)\n", pindexOld->GetBlockHash().ToString(), pindexOld->nHeight);
                DisconnectResult res = DisconnectBlock(block, pindexOld, cache);
                if (res == DISCONNECT_FAILED) {
                    return error("RollbackBlock(): DisconnectBlock failed at %d, hash=%s", pindexOld->nHeight, pindexOld->GetBlockHash().ToString());
                }
                // If DISCONNECT_UNCLEAN is returned, it means a non-existing UTXO was deleted, or an existing UTXO was
                // overwritten. It corresponds to cases where the block-to-be-disconnect never had all its operations
                // applied to the UTXO set. However, as both writing a UTXO and deleting a UTXO are idempotent operations,
                // the result is still a version of the UTXO set with the effects of that block undone.
            }
            pindexOld = pindexOld->pprev;
        }

        // Roll forward from the forking point to the new tip.
        int nForkHeight = pindexFork ? pindexFork->nHeight : 0;
        for (int nHeight = nForkHeight + 1; nHeight <= pindexNew->nHeight; ++nHeight) {
            const CBlockIndex* pindex = pindexNew->GetAncestor(nHeight);
            LogPrintf("Rolling forward %s (%i)\n", pindex->GetBlockHash().ToString(), nHeight);
            uiInterface.ShowProgress(_("Replaying blocks…").translated, (int) ((nHeight - nForkHeight) * 100.0 / (pindexNew->nHeight - nForkHeight)) , false);
            if (!RollforwardBlock(pindex, cache)) return false;
        }

        cache.SetBestBlock(pindexNew->GetBlockHash());
        cache.Flush();
        uiInterface.ShowProgress("", 100, false);
        return true;
        */
    }
    
    /**
      | Whether the chain state needs to be redownloaded
      | due to lack of witness data
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn needs_redownload(&self) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_main);

        // At and above m_params.SegwitHeight, segwit consensus rules must be validated
        CBlockIndex* block{m_chain.Tip()};

        while (block != nullptr && DeploymentActiveAt(*block, m_params.GetConsensus(), consensus::DEPLOYMENT_SEGWIT)) {
            if (!(block->nStatus & BLOCK_OPT_WITNESS)) {
                // block is insufficiently validated for a segwit client
                return true;
            }
            block = block->pprev;
        }

        return false;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn unload_block_index(&mut self)  {
        
        todo!();
        /*
            nBlockSequenceId = 1;
        setBlockIndexCandidates.clear();
        */
    }
    
    /**
      | Ensures we have a genesis block in the
      | block tree, possibly writing one to
      | disk.
      |
      */
    pub fn load_genesis_block(&mut self) -> bool {
        
        todo!();
        /*
            LOCK(cs_main);

        // Check whether we're already initialized by checking for genesis in
        // m_blockman.m_block_index. Note that we can't use m_chain here, since it is
        // set based on the coins db, not the block index db, which is the only
        // thing loaded at this point.
        if (m_blockman.m_block_index.count(m_params.GenesisBlock().GetHash()))
            return true;

        try {
            const CBlock& block = m_params.GenesisBlock();
            FlatFilePos blockPos = SaveBlockToDisk(block, 0, m_chain, m_params, nullptr);
            if (blockPos.IsNull())
                return error("%s: writing genesis block to disk failed", __func__);
            CBlockIndex *pindex = m_blockman.AddToBlockIndex(block);
            ReceivedBlockTransactions(block, pindex, blockPos);
        } catch (const std::runtime_error& e) {
            return error("%s: failed to write genesis block: %s", __func__, e.what());
        }

        return true;
        */
    }
    
    /**
      | Import blocks from an external file
      |
      */
    pub fn load_external_block_file(&mut self, 
        file_in: *mut libc::FILE,
        dbp:     Option<*mut FlatFilePos>)  {
        
        todo!();
        /*
            // Map of disk positions for blocks with unknown parent (only used for reindex)
        static std::multimap<uint256, FlatFilePos> mapBlocksUnknownParent;
        int64_t nStart = GetTimeMillis();

        int nLoaded = 0;
        try {
            // This takes over fileIn and calls fclose() on it in the BufferedFile destructor
            BufferedFile blkdat(fileIn, 2*MAX_BLOCK_SERIALIZED_SIZE, MAX_BLOCK_SERIALIZED_SIZE+8, SER_DISK, CLIENT_VERSION);
            uint64_t nRewind = blkdat.GetPos();
            while (!blkdat.eof()) {
                if (ShutdownRequested()) return;

                blkdat.SetPos(nRewind);
                nRewind++; // start one byte further next time, in case of failure
                blkdat.SetLimit(); // remove former limit
                unsigned int nSize = 0;
                try {
                    // locate a header
                    unsigned char buf[CMessageHeader::MESSAGE_START_SIZE];
                    blkdat.FindByte(m_params.MessageStart()[0]);
                    nRewind = blkdat.GetPos()+1;
                    blkdat >> buf;
                    if (memcmp(buf, m_params.MessageStart(), CMessageHeader::MESSAGE_START_SIZE)) {
                        continue;
                    }
                    // read size
                    blkdat >> nSize;
                    if (nSize < 80 || nSize > MAX_BLOCK_SERIALIZED_SIZE)
                        continue;
                } catch (const std::exception&) {
                    // no valid block header found; don't complain
                    break;
                }
                try {
                    // read block
                    uint64_t nBlockPos = blkdat.GetPos();
                    if (dbp)
                        dbp->nPos = nBlockPos;
                    blkdat.SetLimit(nBlockPos + nSize);
                    std::shared_ptr<CBlock> pblock = std::make_shared<CBlock>();
                    CBlock& block = *pblock;
                    blkdat >> block;
                    nRewind = blkdat.GetPos();

                    uint256 hash = block.GetHash();
                    {
                        LOCK(cs_main);
                        // detect out of order blocks, and store them for later
                        if (hash != m_params.GetConsensus().hashGenesisBlock && !m_blockman.LookupBlockIndex(block.hashPrevBlock)) {
                            LogPrint(BCLog::REINDEX, "%s: Out of order block %s, parent %s not known\n", __func__, hash.ToString(),
                                    block.hashPrevBlock.ToString());
                            if (dbp)
                                mapBlocksUnknownParent.insert(std::make_pair(block.hashPrevBlock, *dbp));
                            continue;
                        }

                        // process in case the block isn't known yet
                        CBlockIndex* pindex = m_blockman.LookupBlockIndex(hash);
                        if (!pindex || (pindex->nStatus & BLOCK_HAVE_DATA) == 0) {
                          BlockValidationState state;
                          if (AcceptBlock(pblock, state, nullptr, true, dbp, nullptr)) {
                              nLoaded++;
                          }
                          if (state.IsError()) {
                              break;
                          }
                        } else if (hash != m_params.GetConsensus().hashGenesisBlock && pindex->nHeight % 1000 == 0) {
                            LogPrint(BCLog::REINDEX, "Block Import: already had block %s at height %d\n", hash.ToString(), pindex->nHeight);
                        }
                    }

                    // Activate the genesis block so normal node progress can continue
                    if (hash == m_params.GetConsensus().hashGenesisBlock) {
                        BlockValidationState state;
                        if (!ActivateBestChain(state, nullptr)) {
                            break;
                        }
                    }

                    NotifyHeaderTip(*this);

                    // Recursively process earlier encountered successors of this block
                    std::deque<uint256> queue;
                    queue.push_back(hash);
                    while (!queue.empty()) {
                        uint256 head = queue.front();
                        queue.pop_front();
                        std::pair<std::multimap<uint256, FlatFilePos>::iterator, std::multimap<uint256, FlatFilePos>::iterator> range = mapBlocksUnknownParent.equal_range(head);
                        while (range.first != range.second) {
                            std::multimap<uint256, FlatFilePos>::iterator it = range.first;
                            std::shared_ptr<CBlock> pblockrecursive = std::make_shared<CBlock>();
                            if (ReadBlockFromDisk(*pblockrecursive, it->second, m_params.GetConsensus())) {
                                LogPrint(BCLog::REINDEX, "%s: Processing out of order child %s of %s\n", __func__, pblockrecursive->GetHash().ToString(),
                                        head.ToString());
                                LOCK(cs_main);
                                BlockValidationState dummy;
                                if (AcceptBlock(pblockrecursive, dummy, nullptr, true, &it->second, nullptr)) {
                                    nLoaded++;
                                    queue.push_back(pblockrecursive->GetHash());
                                }
                            }
                            range.first++;
                            mapBlocksUnknownParent.erase(it);
                            NotifyHeaderTip(*this);
                        }
                    }
                } catch (const std::exception& e) {
                    LogPrintf("%s: Deserialize or I/O error - %s\n", __func__, e.what());
                }
            }
        } catch (const std::runtime_error& e) {
            AbortNode(std::string("System error: ") + e.what());
        }
        LogPrintf("Loaded %i blocks from external file in %dms\n", nLoaded, GetTimeMillis() - nStart);
        */
    }
    
    /**
      | Make various assertions about the state
      | of the block index.
      | 
      | By default this only executes fully
      | when using the Regtest chain; see: fCheckBlockIndex.
      |
      */
    pub fn check_block_index(&mut self)  {
        
        todo!();
        /*
            if (!fCheckBlockIndex) {
            return;
        }

        LOCK(cs_main);

        // During a reindex, we read the genesis block and call CheckBlockIndex before ActivateBestChain,
        // so we have the genesis block in m_blockman.m_block_index but no active chain. (A few of the
        // tests when iterating the block tree require that m_chain has been initialized.)
        if (m_chain.Height() < 0) {
            assert(m_blockman.m_block_index.size() <= 1);
            return;
        }

        // Build forward-pointing map of the entire block tree.
        std::multimap<CBlockIndex*,CBlockIndex*> forward;
        for (const std::pair<const uint256, CBlockIndex*>& entry : m_blockman.m_block_index) {
            forward.insert(std::make_pair(entry.second->pprev, entry.second));
        }

        assert(forward.size() == m_blockman.m_block_index.size());

        std::pair<std::multimap<CBlockIndex*,CBlockIndex*>::iterator,std::multimap<CBlockIndex*,CBlockIndex*>::iterator> rangeGenesis = forward.equal_range(nullptr);
        CBlockIndex *pindex = rangeGenesis.first->second;
        rangeGenesis.first++;
        assert(rangeGenesis.first == rangeGenesis.second); // There is only one index entry with parent nullptr.

        // Iterate over the entire block tree, using depth-first search.
        // Along the way, remember whether there are blocks on the path from genesis
        // block being explored which are the first to have certain properties.
        size_t nNodes = 0;
        int nHeight = 0;
        CBlockIndex* pindexFirstInvalid = nullptr; // Oldest ancestor of pindex which is invalid.
        CBlockIndex* pindexFirstMissing = nullptr; // Oldest ancestor of pindex which does not have BLOCK_HAVE_DATA.
        CBlockIndex* pindexFirstNeverProcessed = nullptr; // Oldest ancestor of pindex for which nTx == 0.
        CBlockIndex* pindexFirstNotTreeValid = nullptr; // Oldest ancestor of pindex which does not have BLOCK_VALID_TREE (regardless of being valid or not).
        CBlockIndex* pindexFirstNotTransactionsValid = nullptr; // Oldest ancestor of pindex which does not have BLOCK_VALID_TRANSACTIONS (regardless of being valid or not).
        CBlockIndex* pindexFirstNotChainValid = nullptr; // Oldest ancestor of pindex which does not have BLOCK_VALID_CHAIN (regardless of being valid or not).
        CBlockIndex* pindexFirstNotScriptsValid = nullptr; // Oldest ancestor of pindex which does not have BLOCK_VALID_SCRIPTS (regardless of being valid or not).
        while (pindex != nullptr) {
            nNodes++;
            if (pindexFirstInvalid == nullptr && pindex->nStatus & BLOCK_FAILED_VALID) pindexFirstInvalid = pindex;
            // Assumed-valid index entries will not have data since we haven't downloaded the
            // full block yet.
            if (pindexFirstMissing == nullptr && !(pindex->nStatus & BLOCK_HAVE_DATA) && !pindex->IsAssumedValid()) {
                pindexFirstMissing = pindex;
            }
            if (pindexFirstNeverProcessed == nullptr && pindex->nTx == 0) pindexFirstNeverProcessed = pindex;
            if (pindex->pprev != nullptr && pindexFirstNotTreeValid == nullptr && (pindex->nStatus & BLOCK_VALID_MASK) < BLOCK_VALID_TREE) pindexFirstNotTreeValid = pindex;

            if (pindex->pprev != nullptr && !pindex->IsAssumedValid()) {
                // Skip validity flag checks for BLOCK_ASSUMED_VALID index entries, since these
                // *_VALID_MASK flags will not be present for index entries we are temporarily assuming
                // valid.
                if (pindexFirstNotTransactionsValid == nullptr &&
                        (pindex->nStatus & BLOCK_VALID_MASK) < BLOCK_VALID_TRANSACTIONS) {
                    pindexFirstNotTransactionsValid = pindex;
                }

                if (pindexFirstNotChainValid == nullptr &&
                        (pindex->nStatus & BLOCK_VALID_MASK) < BLOCK_VALID_CHAIN) {
                    pindexFirstNotChainValid = pindex;
                }

                if (pindexFirstNotScriptsValid == nullptr &&
                        (pindex->nStatus & BLOCK_VALID_MASK) < BLOCK_VALID_SCRIPTS) {
                    pindexFirstNotScriptsValid = pindex;
                }
            }

            // Begin: actual consistency checks.
            if (pindex->pprev == nullptr) {
                // Genesis block checks.
                assert(pindex->GetBlockHash() == m_params.GetConsensus().hashGenesisBlock); // Genesis block's hash must match.
                assert(pindex == m_chain.Genesis()); // The current active chain's genesis block must be this block.
            }
            if (!pindex->HaveTxsDownloaded()) assert(pindex->nSequenceId <= 0); // nSequenceId can't be set positive for blocks that aren't linked (negative is used for preciousblock)
            // VALID_TRANSACTIONS is equivalent to nTx > 0 for all nodes (whether or not pruning has occurred).
            // HAVE_DATA is only equivalent to nTx > 0 (or VALID_TRANSACTIONS) if no pruning has occurred.
            // Unless these indexes are assumed valid and pending block download on a
            // background chainstate.
            if (!fHavePruned && !pindex->IsAssumedValid()) {
                // If we've never pruned, then HAVE_DATA should be equivalent to nTx > 0
                assert(!(pindex->nStatus & BLOCK_HAVE_DATA) == (pindex->nTx == 0));
                assert(pindexFirstMissing == pindexFirstNeverProcessed);
            } else {
                // If we have pruned, then we can only say that HAVE_DATA implies nTx > 0
                if (pindex->nStatus & BLOCK_HAVE_DATA) assert(pindex->nTx > 0);
            }
            if (pindex->nStatus & BLOCK_HAVE_UNDO) assert(pindex->nStatus & BLOCK_HAVE_DATA);
            if (pindex->IsAssumedValid()) {
                // Assumed-valid blocks should have some nTx value.
                assert(pindex->nTx > 0);
                // Assumed-valid blocks should connect to the main chain.
                assert((pindex->nStatus & BLOCK_VALID_MASK) >= BLOCK_VALID_TREE);
            } else {
                // Otherwise there should only be an nTx value if we have
                // actually seen a block's transactions.
                assert(((pindex->nStatus & BLOCK_VALID_MASK) >= BLOCK_VALID_TRANSACTIONS) == (pindex->nTx > 0)); // This is pruning-independent.
            }
            // All parents having had data (at some point) is equivalent to all parents being VALID_TRANSACTIONS, which is equivalent to HaveTxsDownloaded().
            assert((pindexFirstNeverProcessed == nullptr) == pindex->HaveTxsDownloaded());
            assert((pindexFirstNotTransactionsValid == nullptr) == pindex->HaveTxsDownloaded());
            assert(pindex->nHeight == nHeight); // nHeight must be consistent.
            assert(pindex->pprev == nullptr || pindex->nChainWork >= pindex->pprev->nChainWork); // For every block except the genesis block, the chainwork must be larger than the parent's.
            assert(nHeight < 2 || (pindex->pskip && (pindex->pskip->nHeight < nHeight))); // The pskip pointer must point back for all but the first 2 blocks.
            assert(pindexFirstNotTreeValid == nullptr); // All m_blockman.m_block_index entries must at least be TREE valid
            if ((pindex->nStatus & BLOCK_VALID_MASK) >= BLOCK_VALID_TREE) assert(pindexFirstNotTreeValid == nullptr); // TREE valid implies all parents are TREE valid
            if ((pindex->nStatus & BLOCK_VALID_MASK) >= BLOCK_VALID_CHAIN) assert(pindexFirstNotChainValid == nullptr); // CHAIN valid implies all parents are CHAIN valid
            if ((pindex->nStatus & BLOCK_VALID_MASK) >= BLOCK_VALID_SCRIPTS) assert(pindexFirstNotScriptsValid == nullptr); // SCRIPTS valid implies all parents are SCRIPTS valid
            if (pindexFirstInvalid == nullptr) {
                // Checks for not-invalid blocks.
                assert((pindex->nStatus & BLOCK_FAILED_MASK) == 0); // The failed mask cannot be set for blocks without invalid parents.
            }
            if (!CBlockIndexWorkComparator()(pindex, m_chain.Tip()) && pindexFirstNeverProcessed == nullptr) {
                if (pindexFirstInvalid == nullptr) {
                    const bool is_active = this == &m_chainman.ActiveChainstate();

                    // If this block sorts at least as good as the current tip and
                    // is valid and we have all data for its parents, it must be in
                    // setBlockIndexCandidates.  m_chain.Tip() must also be there
                    // even if some data has been pruned.
                    //
                    // Don't perform this check for the background chainstate since
                    // its setBlockIndexCandidates shouldn't have some entries (i.e. those past the
                    // snapshot block) which do exist in the block index for the active chainstate.
                    if (is_active && (pindexFirstMissing == nullptr || pindex == m_chain.Tip())) {
                        assert(setBlockIndexCandidates.count(pindex));
                    }
                    // If some parent is missing, then it could be that this block was in
                    // setBlockIndexCandidates but had to be removed because of the missing data.
                    // In this case it must be in m_blocks_unlinked -- see test below.
                }
            } else { // If this block sorts worse than the current tip or some ancestor's block has never been seen, it cannot be in setBlockIndexCandidates.
                assert(setBlockIndexCandidates.count(pindex) == 0);
            }
            // Check whether this block is in m_blocks_unlinked.
            std::pair<std::multimap<CBlockIndex*,CBlockIndex*>::iterator,std::multimap<CBlockIndex*,CBlockIndex*>::iterator> rangeUnlinked = m_blockman.m_blocks_unlinked.equal_range(pindex->pprev);
            bool foundInUnlinked = false;
            while (rangeUnlinked.first != rangeUnlinked.second) {
                assert(rangeUnlinked.first->first == pindex->pprev);
                if (rangeUnlinked.first->second == pindex) {
                    foundInUnlinked = true;
                    break;
                }
                rangeUnlinked.first++;
            }
            if (pindex->pprev && (pindex->nStatus & BLOCK_HAVE_DATA) && pindexFirstNeverProcessed != nullptr && pindexFirstInvalid == nullptr) {
                // If this block has block data available, some parent was never received, and has no invalid parents, it must be in m_blocks_unlinked.
                assert(foundInUnlinked);
            }
            if (!(pindex->nStatus & BLOCK_HAVE_DATA)) assert(!foundInUnlinked); // Can't be in m_blocks_unlinked if we don't HAVE_DATA
            if (pindexFirstMissing == nullptr) assert(!foundInUnlinked); // We aren't missing data for any parent -- cannot be in m_blocks_unlinked.
            if (pindex->pprev && (pindex->nStatus & BLOCK_HAVE_DATA) && pindexFirstNeverProcessed == nullptr && pindexFirstMissing != nullptr) {
                // We HAVE_DATA for this block, have received data for all parents at some point, but we're currently missing data for some parent.
                assert(fHavePruned); // We must have pruned.
                // This block may have entered m_blocks_unlinked if:
                //  - it has a descendant that at some point had more work than the
                //    tip, and
                //  - we tried switching to that descendant but were missing
                //    data for some intermediate block between m_chain and the
                //    tip.
                // So if this block is itself better than m_chain.Tip() and it wasn't in
                // setBlockIndexCandidates, then it must be in m_blocks_unlinked.
                if (!CBlockIndexWorkComparator()(pindex, m_chain.Tip()) && setBlockIndexCandidates.count(pindex) == 0) {
                    if (pindexFirstInvalid == nullptr) {
                        assert(foundInUnlinked);
                    }
                }
            }
            // assert(pindex->GetBlockHash() == pindex->GetBlockHeader().GetHash()); // Perhaps too slow
            // End: actual consistency checks.

            // Try descending into the first subnode.
            std::pair<std::multimap<CBlockIndex*,CBlockIndex*>::iterator,std::multimap<CBlockIndex*,CBlockIndex*>::iterator> range = forward.equal_range(pindex);
            if (range.first != range.second) {
                // A subnode was found.
                pindex = range.first->second;
                nHeight++;
                continue;
            }
            // This is a leaf node.
            // Move upwards until we reach a node of which we have not yet visited the last child.
            while (pindex) {
                // We are going to either move to a parent or a sibling of pindex.
                // If pindex was the first with a certain property, unset the corresponding variable.
                if (pindex == pindexFirstInvalid) pindexFirstInvalid = nullptr;
                if (pindex == pindexFirstMissing) pindexFirstMissing = nullptr;
                if (pindex == pindexFirstNeverProcessed) pindexFirstNeverProcessed = nullptr;
                if (pindex == pindexFirstNotTreeValid) pindexFirstNotTreeValid = nullptr;
                if (pindex == pindexFirstNotTransactionsValid) pindexFirstNotTransactionsValid = nullptr;
                if (pindex == pindexFirstNotChainValid) pindexFirstNotChainValid = nullptr;
                if (pindex == pindexFirstNotScriptsValid) pindexFirstNotScriptsValid = nullptr;
                // Find our parent.
                CBlockIndex* pindexPar = pindex->pprev;
                // Find which child we just visited.
                std::pair<std::multimap<CBlockIndex*,CBlockIndex*>::iterator,std::multimap<CBlockIndex*,CBlockIndex*>::iterator> rangePar = forward.equal_range(pindexPar);
                while (rangePar.first->second != pindex) {
                    assert(rangePar.first != rangePar.second); // Our parent must have at least the node we're coming from as child.
                    rangePar.first++;
                }
                // Proceed to the next one.
                rangePar.first++;
                if (rangePar.first != rangePar.second) {
                    // Move to the sibling.
                    pindex = rangePar.first->second;
                    break;
                } else {
                    // Move up further.
                    pindex = pindexPar;
                    nHeight--;
                    continue;
                }
            }
        }

        // Check that we actually traversed the entire map.
        assert(nNodes == forward.size());
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn to_string(&mut self) -> String {
        
        todo!();
        /*
            CBlockIndex* tip = m_chain.Tip();
        return strprintf("Chainstate [%s] @ height %d (%s)",
                         m_from_snapshot_blockhash ? "snapshot" : "ibd",
                         tip ? tip->nHeight : -1, tip ? tip->GetBlockHash().ToString() : "null");
        */
    }
    
    /**
      | Resize the CoinsViews caches dynamically
      | and flush state to disk.
      |
      | @returns true unless an error occurred
      | during the flush.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn resize_coins_caches(&mut self, 
        coinstip_size: usize,
        coinsdb_size:  usize) -> bool {
        
        todo!();
        /*
            if (coinstip_size == m_coinstip_cache_size_bytes &&
                coinsdb_size == m_coinsdb_cache_size_bytes) {
            // Cache sizes are unchanged, no need to continue.
            return true;
        }
        size_t old_coinstip_size = m_coinstip_cache_size_bytes;
        m_coinstip_cache_size_bytes = coinstip_size;
        m_coinsdb_cache_size_bytes = coinsdb_size;
        CoinsDB().ResizeCache(coinsdb_size);

        LogPrintf("[%s] resized coinsdb cache to %.1f MiB\n",
            this->ToString(), coinsdb_size * (1.0 / 1024 / 1024));
        LogPrintf("[%s] resized coinstip cache to %.1f MiB\n",
            this->ToString(), coinstip_size * (1.0 / 1024 / 1024));

        BlockValidationState state;
        bool ret;

        if (coinstip_size > old_coinstip_size) {
            // Likely no need to flush if cache sizes have grown.
            ret = FlushStateToDisk(state, FlushStateMode::IF_NEEDED);
        } else {
            // Otherwise, flush state to disk and deallocate the in-memory coins map.
            ret = FlushStateToDisk(state, FlushStateMode::ALWAYS);
            CoinsTip().ReallocateCache();
        }
        return ret;
        */
    }
}

