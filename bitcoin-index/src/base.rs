crate::ix!();

/**
  | Base class for indices of blockchain
  | data. This implements
  | 
  | CValidationInterface and ensures
  | blocks are indexed sequentially according
  | to their position in the active chain.
  |
  */
pub struct BaseIndex {

    /**
      | Whether the index is in sync with the main
      | chain. The flag is flipped from false to
      | true once, after which point this starts
      | processing ValidationInterface notifications
      | to stay in sync.
      */
    synced:           AtomicBool, // default = { false }

    /**
      | The last block in the chain that the index
      | is in sync with.
      */
    best_block_index: Atomic<*mut BlockIndex>, // default = { nullptr }

    thread_sync:      Thread,
    interrupt:        ThreadInterrupt,
    chainstate:       *mut dyn ChainStateInterface, // default = { nullptr }
}

impl ValidationInterface for BaseIndex { 

}

impl UpdatedBlockTip               for BaseIndex { }
impl TransactionAddedToMempool     for BaseIndex { }
impl TransactionRemovedFromMempool for BaseIndex { }
impl BlockConnected                for BaseIndex { }
impl BlockDisconnected             for BaseIndex { }
impl ChainStateFlushed             for BaseIndex { }
impl BlockChecked                  for BaseIndex { }
impl NewPoWValidBlock              for BaseIndex { }

pub trait BaseIndexInterface: 

    /*
       | Get the name of the index for display
       | in logs.
       |
       */
    GetName

    + Init
    + WriteBlock
    + CommitInternal
    + Rewind
    + GetDB {}

//-------------------------------------------[.cpp/bitcoin/src/index/base.cpp]

pub const DB_BEST_BLOCK:              char = 'B';
pub const SYNC_LOG_INTERVAL:           i64 = 30; // seconds
pub const SYNC_LOCATOR_WRITE_INTERVAL: i64 = 30; // seconds

pub fn fatal_error<Args>(
        fmt:  *const u8,
        args: &Args)  {

    todo!();
        /*
            std::string strMessage = tfm::format(fmt, args...);
        SetMiscWarning(Untranslated(strMessage));
        LogPrintf("*** %s\n", strMessage);
        AbortError("A fatal internal error occurred, see debug.log for details");
        StartShutdown();
        */
}

impl Drop for BaseIndex {

    /**
      | Destructor interrupts sync thread
      | if running and blocks until it exits.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            Interrupt();
        Stop();
        */
    }
}

impl BaseIndex {

    pub fn current_index(&mut self) -> *const BlockIndex {
        
        todo!();
        /*
            return m_best_block_index.load(); }{
        */
    }
    
    pub fn init(&mut self) -> bool {
        
        todo!();
        /*
            CBlockLocator locator;
        if (!GetDB().ReadBestBlock(locator)) {
            locator.SetNull();
        }

        LOCK(cs_main);
        CChain& active_chain = m_chainstate->m_chain;
        if (locator.IsNull()) {
            m_best_block_index = nullptr;
        } else {
            m_best_block_index = m_chainstate->m_blockman.FindForkInGlobalIndex(active_chain, locator);
        }
        m_synced = m_best_block_index.load() == active_chain.Tip();
        if (!m_synced) {
            bool prune_violation = false;
            if (!m_best_block_index) {
                // index is not built yet
                // make sure we have all block data back to the genesis
                const CBlockIndex* block = active_chain.Tip();
                while (block->pprev && (block->pprev->nStatus & BLOCK_HAVE_DATA)) {
                    block = block->pprev;
                }
                prune_violation = block != active_chain.Genesis();
            }
            // in case the index has a best block set and is not fully synced
            // check if we have the required blocks to continue building the index
            else {
                const CBlockIndex* block_to_test = m_best_block_index.load();
                if (!active_chain.Contains(block_to_test)) {
                    // if the bestblock is not part of the mainchain, find the fork
                    // and make sure we have all data down to the fork
                    block_to_test = active_chain.FindFork(block_to_test);
                }
                const CBlockIndex* block = active_chain.Tip();
                prune_violation = true;
                // check backwards from the tip if we have all block data until we reach the indexes bestblock
                while (block_to_test && block->pprev && (block->pprev->nStatus & BLOCK_HAVE_DATA)) {
                    if (block_to_test == block) {
                        prune_violation = false;
                        break;
                    }
                    block = block->pprev;
                }
            }
            if (prune_violation) {
                return InitError(strprintf(Untranslated("%s best block of the index goes beyond pruned data. Please disable the index or reindex (which will download the whole blockchain again)"), GetName()));
            }
        }
        return true;
        */
    }
    
    /**
      | Sync the index with the block index
      | starting from the current best block.
      |
      | Intended to be run in its own thread,
      | m_thread_sync, and can be interrupted with
      | m_interrupt. Once the index gets in sync,
      | the m_synced flag is set and the
      | BlockConnected ValidationInterface
      | callback takes over and the sync thread
      | exits.
      */
    pub fn thread_sync(&mut self)  {
        
        todo!();
        /*
            SetSyscallSandboxPolicy(SyscallSandboxPolicy::TX_INDEX);
        const CBlockIndex* pindex = m_best_block_index.load();
        if (!m_synced) {
            auto& consensus_params = Params().GetConsensus();

            int64_t last_log_time = 0;
            int64_t last_locator_write_time = 0;
            while (true) {
                if (m_interrupt) {
                    m_best_block_index = pindex;
                    // No need to handle errors in Commit. If it fails, the error will be already be
                    // logged. The best way to recover is to continue, as index cannot be corrupted by
                    // a missed commit to disk for an advanced index state.
                    Commit();
                    return;
                }

                {
                    LOCK(cs_main);
                    const CBlockIndex* pindex_next = NextSyncBlock(pindex, m_chainstate->m_chain);
                    if (!pindex_next) {
                        m_best_block_index = pindex;
                        m_synced = true;
                        // No need to handle errors in Commit. See rationale above.
                        Commit();
                        break;
                    }
                    if (pindex_next->pprev != pindex && !Rewind(pindex, pindex_next->pprev)) {
                        FatalError("%s: Failed to rewind index %s to a previous chain tip",
                                   __func__, GetName());
                        return;
                    }
                    pindex = pindex_next;
                }

                int64_t current_time = GetTime();
                if (last_log_time + SYNC_LOG_INTERVAL < current_time) {
                    LogPrintf("Syncing %s with block chain from height %d\n",
                              GetName(), pindex->nHeight);
                    last_log_time = current_time;
                }

                if (last_locator_write_time + SYNC_LOCATOR_WRITE_INTERVAL < current_time) {
                    m_best_block_index = pindex;
                    last_locator_write_time = current_time;
                    // No need to handle errors in Commit. See rationale above.
                    Commit();
                }

                CBlock block;
                if (!ReadBlockFromDisk(block, pindex, consensus_params)) {
                    FatalError("%s: Failed to read block %s from disk",
                               __func__, pindex->GetBlockHash().ToString());
                    return;
                }
                if (!WriteBlock(block, pindex)) {
                    FatalError("%s: Failed to write block %s to index database",
                               __func__, pindex->GetBlockHash().ToString());
                    return;
                }
            }
        }

        if (pindex) {
            LogPrintf("%s is enabled at height %d\n", GetName(), pindex->nHeight);
        } else {
            LogPrintf("%s is enabled\n", GetName());
        }
        */
    }
    
    /**
      | Write the current index state (eg. chain
      | block locator and subclass-specific items)
      | to disk.
      |
      | Recommendations for error handling:
      |
      | If called on a successor of the previous
      | committed best block in the index, the
      | index can continue processing without risk
      | of corruption, though the index state will
      | need to catch up from further behind on
      | reboot. If the new state is not
      | a successor of the previous state (due to
      | a chain reorganization), the index must
      | halt until Commit succeeds or else it
      | could end up getting corrupted.
      */
    pub fn commit(&mut self) -> bool {
        
        todo!();
        /*
            CDBBatch batch(GetDB());
        if (!CommitInternal(batch) || !GetDB().WriteBatch(batch)) {
            return error("%s: Failed to commit latest %s state", __func__, GetName());
        }
        return true;
        */
    }
    
    pub fn commit_internal(&mut self, batch: &mut DBBatch) -> bool {
        
        todo!();
        /*
            LOCK(cs_main);
        GetDB().WriteBestBlock(batch, m_chainstate->m_chain.GetLocator(m_best_block_index));
        return true;
        */
    }
    
    pub fn rewind(&mut self, 
        current_tip: *const BlockIndex,
        new_tip:     *const BlockIndex) -> bool {
        
        todo!();
        /*
            assert(current_tip == m_best_block_index);
        assert(current_tip->GetAncestor(new_tip->nHeight) == new_tip);

        // In the case of a reorg, ensure persisted block locator is not stale.
        // Pruning has a minimum of 288 blocks-to-keep and getting the index
        // out of sync may be possible but a users fault.
        // In case we reorg beyond the pruned depth, ReadBlockFromDisk would
        // throw and lead to a graceful shutdown
        m_best_block_index = new_tip;
        if (!Commit()) {
            // If commit fails, revert the best block index to avoid corruption.
            m_best_block_index = current_tip;
            return false;
        }

        return true;
        */
    }
    
    pub fn block_connected(&mut self, 
        block:  &Arc<Block>,
        pindex: *const BlockIndex)  {
        
        todo!();
        /*
            if (!m_synced) {
            return;
        }

        const CBlockIndex* best_block_index = m_best_block_index.load();
        if (!best_block_index) {
            if (pindex->nHeight != 0) {
                FatalError("%s: First block connected is not the genesis block (height=%d)",
                           __func__, pindex->nHeight);
                return;
            }
        } else {
            // Ensure block connects to an ancestor of the current best block. This should be the case
            // most of the time, but may not be immediately after the sync thread catches up and sets
            // m_synced. Consider the case where there is a reorg and the blocks on the stale branch are
            // in the ValidationInterface queue backlog even after the sync thread has caught up to the
            // new chain tip. In this unlikely event, log a warning and let the queue clear.
            if (best_block_index->GetAncestor(pindex->nHeight - 1) != pindex->pprev) {
                LogPrintf("%s: WARNING: Block %s does not connect to an ancestor of " /* Continued */
                          "known best chain (tip=%s); not updating index\n",
                          __func__, pindex->GetBlockHash().ToString(),
                          best_block_index->GetBlockHash().ToString());
                return;
            }
            if (best_block_index != pindex->pprev && !Rewind(best_block_index, pindex->pprev)) {
                FatalError("%s: Failed to rewind index %s to a previous chain tip",
                           __func__, GetName());
                return;
            }
        }

        if (WriteBlock(*block, pindex)) {
            m_best_block_index = pindex;
        } else {
            FatalError("%s: Failed to write block %s to index",
                       __func__, pindex->GetBlockHash().ToString());
            return;
        }
        */
    }
    
    pub fn chain_state_flushed(&mut self, locator: &BlockLocator)  {
        
        todo!();
        /*
            if (!m_synced) {
            return;
        }

        const uint256& locator_tip_hash = locator.vHave.front();
        const CBlockIndex* locator_tip_index;
        {
            LOCK(cs_main);
            locator_tip_index = m_chainstate->m_blockman.LookupBlockIndex(locator_tip_hash);
        }

        if (!locator_tip_index) {
            FatalError("%s: First block (hash=%s) in locator was not found",
                       __func__, locator_tip_hash.ToString());
            return;
        }

        // This checks that ChainStateFlushed callbacks are received after BlockConnected. The check may fail
        // immediately after the sync thread catches up and sets m_synced. Consider the case where
        // there is a reorg and the blocks on the stale branch are in the ValidationInterface queue
        // backlog even after the sync thread has caught up to the new chain tip. In this unlikely
        // event, log a warning and let the queue clear.
        const CBlockIndex* best_block_index = m_best_block_index.load();
        if (best_block_index->GetAncestor(locator_tip_index->nHeight) != locator_tip_index) {
            LogPrintf("%s: WARNING: Locator contains block (hash=%s) not on known best " /* Continued */
                      "chain (tip=%s); not writing index locator\n",
                      __func__, locator_tip_hash.ToString(),
                      best_block_index->GetBlockHash().ToString());
            return;
        }

        // No need to handle errors in Commit. If it fails, the error will be already be logged. The
        // best way to recover is to continue, as index cannot be corrupted by a missed commit to disk
        // for an advanced index state.
        Commit();
        */
    }
    
    /**
      | Blocks the current thread until the index
      | is caught up to the current state of the
      | block chain. This only blocks if the index
      | has gotten in sync once and only needs to
      | process blocks in the ValidationInterface
      | queue. If the index is catching up from
      | far behind, this method does not block and
      | immediately returns false.
      */
    #[LOCKS_EXCLUDED(::cs_main)]
    pub fn block_until_synced_to_current_chain(&self) -> bool {
        
        todo!();

        /*
        AssertLockNotHeld(cs_main);

        if !synced {
            return false;
        }

        {
            //  Skip the queue-draining stuff if we know we're caught up with
            //  m_chain.Tip().
            LOCK(cs_main);

            let chain_tip: *mut BlockIndex = (*chainstate).chain.tip();

            let best_block_index: *mut BlockIndex = best_block_index.load();

            if (*best_block_index).get_ancestor((*chain_tip).n_height) == chain_tip {
                return true;
            }
        }

        log_printf("%s: %s is catching up on block notifications\n", func, get_name());

        sync_with_validation_interface_queue();

        true
        */
    }
    
    pub fn interrupt(&mut self)  {
        
        todo!();
        /*
            m_interrupt();
        */
    }
    
    /**
      | Start initializes the sync state and
      | registers the instance as
      | a ValidationInterface so that it stays in
      | sync with blockchain updates.
      */
    pub fn start(&mut self, active_chainstate: &mut dyn ChainStateInterface) -> bool {
        
        todo!();
        /*
            m_chainstate = &active_chainstate;
        // Need to register this ValidationInterface before running Init(), so that
        // callbacks are not missed if Init sets m_synced to true.
        RegisterValidationInterface(this);
        if (!Init()) {
            return false;
        }

        m_thread_sync = std::thread(&util::TraceThread, GetName(), [this] { ThreadSync(); });
        return true;
        */
    }
    
    /**
      | Stops the instance from staying in sync
      | with blockchain updates.
      |
      */
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            UnregisterValidationInterface(this);

        if (m_thread_sync.joinable()) {
            m_thread_sync.join();
        }
        */
    }
    
    /**
      | Get a summary of the index and its state.
      |
      */
    pub fn get_summary(&self) -> IndexSummary {
        
        todo!();

        /*
        let summary: IndexSummary = IndexSummary::new();

        summary.name = get_name();

        summary.synced = synced;

        summary.best_block_height = match best_block_index {
            true   => (*best_block_index.load()).n_height,
            false  => 0
        };

        summary
        */
    }
}

