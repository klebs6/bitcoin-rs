// ---------------- [ File: bitcoin-chainman/src/chainman.rs ]
crate::ix!();

/**
  | Provides an interface for creating
  | and interacting with one or two chainstates:
  | an IBD chainstate generated by downloading
  | blocks, and an optional snapshot chainstate
  | loaded from a UTXO snapshot. Managed
  | chainstates can be maintained at different
  | heights simultaneously.
  | 
  | This class provides abstractions that
  | allow the retrieval of the current most-work
  | chainstate ("Active") as well as chainstates
  | which may be in background use to validate
  | UTXO snapshots.
  | 
  | Definitions:
  | 
  | -IBD chainstate*: a chainstate whose
  | current state has been "fully" validated
  | by the initial block download process.
  | 
  | -Snapshot chainstate*: a chainstate
  | populated by loading in an assumeutxo
  | UTXO snapshot.
  | 
  | -Active chainstate*: the chainstate
  | containing the current most-work chain.
  | Consulted by most parts of the system
  | (net_processing, wallet) as a reflection
  | of the current chain and UTXO set.
  | 
  | This may either be an IBD chainstate
  | or a snapshot chainstate.
  | 
  | -Background IBD chainstate*: an IBD
  | chainstate for which the
  | 
  | IBD process is happening in the background
  | while use of the active (snapshot) chainstate
  | allows the rest of the system to function.
  |
  */
pub struct ChainstateManager {

    /**
      | If true, the assumed-valid chainstate
      | has been fully validated by the background
      | validation chainstate.
      |
      */
    pub snapshot_validated:   bool, // default = { false }

    pub load_block:           Thread,

    /**
      | The total number of bytes available
      | for us to use across all in-memory coins
      | caches. This will be split somehow across
      | chainstates.
      |
      */
    pub total_coinstip_cache: i64, // default = { 0 }

    /**
      | The total number of bytes available
      | for us to use across all leveldb coins
      | databases. This will be split somehow
      | across chainstates.
      |
      */
    pub total_coinsdb_cache:  i64, // default = { 0 }

    //TODO: #[GUARDED_BY(cs_main)]
    pub inner: ChainstateManagerInner,
}

pub struct ChainstateManagerInner {

    /**
      | The chainstate used under normal operation
      | (i.e. "regular" IBD) or, if a snapshot is
      | in use, for background validation.
      |
      | Its contents (including on-disk data) will
      | be deleted *upon shutdown* after
      | background validation of the snapshot has
      | completed. We do not free the chainstate
      | contents immediately after it finishes
      | validation to cautiously avoid a case
      | where some other part of the system is
      | still using this pointer
      | (e.g. net_processing).
      |
      | Once this pointer is set to
      | a corresponding chainstate, it will not be
      | reset until init.cpp:Shutdown().
      |
      | This is especially important when, e.g.,
      | calling ActivateBestChain() on all
      | chainstates because we are not able to
      | hold ::cs_main going into that call.
      */
    pub ibd_chainstate: Box<dyn ChainStateInterface>,

    /**
      | A chainstate initialized on the basis of
      | a UTXO snapshot. If this is non-null, it
      | is always our active chainstate.
      |
      | Once this pointer is set to
      | a corresponding chainstate, it will not be
      | reset until init.cpp:Shutdown().
      |
      | This is especially important when, e.g.,
      | calling ActivateBestChain() on all
      | chainstates because we are not able to
      | hold ::cs_main going into that call.
      */
    pub snapshot_chainstate: Box<dyn ChainStateInterface>,

    /**
      | Points to either the ibd or snapshot
      | chainstate; indicates our most-work chain.
      |
      | Once this pointer is set to
      | a corresponding chainstate, it will not be
      | reset until init.cpp:Shutdown().
      |
      | This is especially important when, e.g.,
      | calling ActivateBestChain() on all
      | chainstates because we are not able to
      | hold ::cs_main going into that call.
      */
    pub active_chainstate: *mut dyn ChainStateInterface, // default = { nullptr }

    /**
      | A single BlockManager instance is shared
      | across each constructed chainstate to
      | avoid duplicating block metadata.
      */
    pub blockman:             BlockManager,
}

impl Drop for ChainstateManager {

    fn drop(&mut self) {
        todo!();
        /*
            LOCK(::cs_main);
            UnloadBlockIndex(/* mempool */ nullptr, *this);
            Reset();
        */
    }
}

impl ChainstateManager {

    pub fn active_chain(&self) -> &mut dyn ChainInterface {
        
        todo!();
        /*
            return ActiveChainstate().m_chain;
        */
    }
    
    pub fn active_height(&self) -> i32 {
        
        todo!();
        /*
            return ActiveChain().Height();
        */
    }
    
    pub fn active_tip(&self) -> *mut BlockIndex {
        
        todo!();
        /*
            return ActiveChain().Tip();
        */
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn block_index(&mut self) -> &mut BlockMap {
        
        todo!();
        /*
            return m_blockman.m_block_index;
        */
    }

    /**
      | Is there a snapshot in use and has it been
      | fully validated?
      |
      */
    pub fn is_snapshot_validated(&self) -> bool {
        
        todo!();
        /*
            return m_snapshot_validated;
        */
    }

    /**
      | Exposed wrapper for AcceptBlockHeader
      |
      | Process incoming block headers.
      | 
      | May not be called in a validationinterface
      | callback.
      | 
      | -----------
      | @param[in] block
      | 
      | The block headers themselves
      | ----------
      | @param[out] state
      | 
      | This may be set to an Error state if any
      | error occurred processing them
      | ----------
      | @param[in] chainparams
      | 
      | The params for the chain we want to connect
      | to
      | ----------
      | @param[out] ppindex
      | 
      | If set, the pointer will be set to point
      | to the last new block index object for
      | the given headers
      |
      */
    #[LOCKS_EXCLUDED(cs_main)]
    pub fn process_new_block_headers(&mut self, 
        headers:     &Vec<BlockHeader>,
        state:       &mut BlockValidationState,
        chainparams: &ChainParams,
        ppindex:     Option<Arc<BlockIndex>>) -> bool {
        
        todo!();
        /*
            AssertLockNotHeld(cs_main);
        {
            LOCK(cs_main);
            for (const CBlockHeader& header : headers) {
                CBlockIndex *pindex = nullptr; // Use a temp pindex instead of ppindex to avoid a const_cast
                bool accepted = m_blockman.AcceptBlockHeader(
                    header, state, chainparams, &pindex);
                ActiveChainstate().CheckBlockIndex();

                if (!accepted) {
                    return false;
                }
                if (ppindex) {
                    *ppindex = pindex;
                }
            }
        }
        if (NotifyHeaderTip(ActiveChainstate())) {
            if (ActiveChainstate().IsInitialBlockDownload() && ppindex && *ppindex) {
                LogPrintf("Synchronizing blockheaders, height: %d (~%.2f%%)\n", (*ppindex)->nHeight, 100.0/((*ppindex)->nHeight+(GetAdjustedTime() - (*ppindex)->GetBlockTime()) / Params().GetConsensus().nPowTargetSpacing) * (*ppindex)->nHeight);
            }
        }
        return true;
        */
    }
    
    /**
      | Process an incoming block. This only
      | returns after the best known valid block
      | is made active. Note that it does not,
      | however, guarantee that the specific
      | block passed to it has been checked for
      | validity!
      | 
      | If you want to *possibly* get feedback
      | on whether block is valid, you must install
      | a CValidationInterface (see validationinterface.h)
      | - this will have its BlockChecked method
      | called whenever *any* block completes
      | validation.
      | 
      | -----------
      | @note
      | 
      | we guarantee that either the proof-of-work
      | is valid on block, or (and possibly also)
      | BlockChecked will have been called.
      | 
      | May not be called in a validationinterface
      | callback.
      | 
      | -----------
      | @param[in] block
      | 
      | The block we want to process.
      | ----------
      | @param[in] force_processing
      | 
      | Process this block even if unrequested;
      | used for non-network block sources.
      | ----------
      | @param[out] new_block
      | 
      | A boolean which is set to indicate if
      | the block was first received via this
      | call
      | 
      | -----------
      | @return
      | 
      | If the block was processed, independently
      | of block validity
      |
      */
    #[LOCKS_EXCLUDED(cs_main)]
    pub fn process_new_block(&mut self, 
        chainparams:      &ChainParams,
        block:            Amo<Block>,
        force_processing: bool,
        new_block:        *mut bool) -> bool {
        
        todo!();
        /*
            AssertLockNotHeld(cs_main);

        {
            CBlockIndex *pindex = nullptr;
            if (new_block) *new_block = false;
            BlockValidationState state;

            // CheckBlock() does not support multi-threaded block validation because CBlock::fChecked can cause data race.
            // Therefore, the following critical section must include the CheckBlock() call as well.
            LOCK(cs_main);

            // Skipping AcceptBlock() for CheckBlock() failures means that we will never mark a block as invalid if
            // CheckBlock() fails.  This is protective against consensus failure if there are any unknown forms of block
            // malleability that cause CheckBlock() to fail; see e.g. CVE-2012-2459 and
            // https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2019-February/016697.html.  Because CheckBlock() is
            // not very expensive, the anti-DoS benefits of caching failure (of a definitely-invalid block) are not substantial.
            bool ret = CheckBlock(*block, state, chainparams.GetConsensus());
            if (ret) {
                // Store to disk
                ret = ActiveChainstate().AcceptBlock(block, state, &pindex, force_processing, nullptr, new_block);
            }
            if (!ret) {
                GetMainSignals().BlockChecked(*block, state);
                return error("%s: AcceptBlock FAILED (%s)", __func__, state.ToString());
            }
        }

        NotifyHeaderTip(ActiveChainstate());

        BlockValidationState state; // Only used to report errors, not invalidity - ignore it
        if (!ActiveChainstate().ActivateBestChain(state, block)) {
            return error("%s: ActivateBestChain failed (%s)", __func__, state.ToString());
        }

        return true;
        */
    }
    
    /**
      | Load the block tree and coins database
      | from disk, initializing state if we're
      | running with -reindex
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn load_block_index(&mut self) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        // Load block index from databases
        bool needs_init = fReindex;
        if (!fReindex) {
            bool ret = m_blockman.LoadBlockIndexDB(ActiveChainstate().setBlockIndexCandidates);
            if (!ret) return false;
            needs_init = m_blockman.m_block_index.empty();
        }

        if (needs_init) {
            // Everything here is for *new* reindex/DBs. Thus, though
            // LoadBlockIndexDB may have set fReindex if we shut down
            // mid-reindex previously, we don't check fReindex and
            // instead only check it prior to LoadBlockIndexDB to set
            // needs_init.

            LogPrintf("Initializing databases...\n");
        }
        return true;
        */
    }

    pub fn snapshot_blockhash(&self) -> Option<u256> {
        
        todo!();
        /*
            LOCK(::cs_main);
        if (m_active_chainstate && m_active_chainstate->m_from_snapshot_blockhash) {
            // If a snapshot chainstate exists, it will always be our active.
            return m_active_chainstate->m_from_snapshot_blockhash;
        }
        return std::nullopt;
        */
    }
    
    /**
      | Get all chainstates currently being
      | used.
      |
      */
    pub fn get_all(&mut self) -> Vec<*mut dyn ChainStateInterface> {
        
        todo!();
        /*
            LOCK(::cs_main);
        std::vector<ChainState*> out;

        if (!IsSnapshotValidated() && m_ibd_chainstate) {
            out.push_back(m_ibd_chainstate.get());
        }

        if (m_snapshot_chainstate) {
            out.push_back(m_snapshot_chainstate.get());
        }

        return out;
        */
    }
    
    /**
      | Instantiate a new chainstate and assign
      | it based upon whether it is from a snapshot.
      | 
      | -----------
      | @param[in] mempool
      | 
      | The mempool to pass to the chainstate
      | constructor
      | ----------
      | @param[in] snapshot_blockhash
      | 
      | If given, signify that this chainstate
      | is based on a snapshot.
      |
      */
    #[LIFETIMEBOUND] 
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn initialize_chainstate(&mut self, 
        mempool:            Arc<Mutex<dyn ITxMemPool>>,
        snapshot_blockhash: &Option<u256>) -> &mut dyn ChainStateInterface {

        todo!();
        /*
            bool is_snapshot = snapshot_blockhash.has_value();
        std::unique_ptr<ChainState>& to_modify =
            is_snapshot ? m_snapshot_chainstate : m_ibd_chainstate;

        if (to_modify) {
            throw std::logic_error("should not be overwriting a chainstate");
        }
        to_modify.reset(new ChainState(mempool, m_blockman, *this, snapshot_blockhash));

        // Snapshot chainstates and initial IBD chaintates always become active.
        if (is_snapshot || (!is_snapshot && !m_active_chainstate)) {
            LogPrintf("Switching active chainstate to %s\n", to_modify->ToString());
            m_active_chainstate = to_modify.get();
        } else {
            throw std::logic_error("unexpected chainstate activation");
        }

        return *to_modify;
        */
    }
    
    /**
      | Construct and activate a Chainstate on the
      | basis of UTXO snapshot data.
      |
      | Steps:
      |
      | - Initialize an unused ChainState.
      |
      | - Load its `CoinsViews` contents from
      | `coins_file`.
      |
      | - Verify that the hash of the resulting
      |   coinsdb matches the expected hash per
      |   assumeutxo chain parameters.
      |
      | - Wait for our headers chain to include
      | the base block of the snapshot.
      |
      | - "Fast forward" the tip of the new
      |   chainstate to the base of the snapshot,
      |   faking nTx* block index data along the
      |   way.
      |
      | - Move the new chainstate to
      |   `m_snapshot_chainstate` and make it our
      |   ChainstateActive().
      */
    pub fn activate_snapshot(&mut self, 
        coins_file: &mut AutoFile,
        metadata:   &SnapshotMetadata,
        in_memory:  bool) -> bool {
        
        todo!();
        /*
            uint256 base_blockhash = metadata.m_base_blockhash;

        if (this->SnapshotBlockhash()) {
            LogPrintf("[snapshot] can't activate a snapshot-based chainstate more than once\n");
            return false;
        }

        int64_t current_coinsdb_cache_size{0};
        int64_t current_coinstip_cache_size{0};

        // Cache percentages to allocate to each chainstate.
        //
        // These particular percentages don't matter so much since they will only be
        // relevant during snapshot activation; caches are rebalanced at the conclusion of
        // this function. We want to give (essentially) all available cache capacity to the
        // snapshot to aid the bulk load later in this function.
        static constexpr double IBD_CACHE_PERC = 0.01;
        static constexpr double SNAPSHOT_CACHE_PERC = 0.99;

        {
            LOCK(::cs_main);
            // Resize the coins caches to ensure we're not exceeding memory limits.
            //
            // Allocate the majority of the cache to the incoming snapshot chainstate, since
            // (optimistically) getting to its tip will be the top priority. We'll need to call
            // `MaybeRebalanceCaches()` once we're done with this function to ensure
            // the right allocation (including the possibility that no snapshot was activated
            // and that we should restore the active chainstate caches to their original size).
            //
            current_coinsdb_cache_size = this->ActiveChainstate().m_coinsdb_cache_size_bytes;
            current_coinstip_cache_size = this->ActiveChainstate().m_coinstip_cache_size_bytes;

            // Temporarily resize the active coins cache to make room for the newly-created
            // snapshot chain.
            this->ActiveChainstate().ResizeCoinsCaches(
                static_cast<size_t>(current_coinstip_cache_size * IBD_CACHE_PERC),
                static_cast<size_t>(current_coinsdb_cache_size * IBD_CACHE_PERC));
        }

        auto snapshot_chainstate = 
        [&]() { LOCK(::cs_main);  return std::make_unique<ChainState>( /* mempool */ nullptr, m_blockman, *this, base_blockhash) }()
        ;

        {
            LOCK(::cs_main);
            snapshot_chainstate->InitCoinsDB(
                static_cast<size_t>(current_coinsdb_cache_size * SNAPSHOT_CACHE_PERC),
                in_memory, false, "chainstate");
            snapshot_chainstate->InitCoinsCache(
                static_cast<size_t>(current_coinstip_cache_size * SNAPSHOT_CACHE_PERC));
        }

        const bool snapshot_ok = this->PopulateAndValidateSnapshot(
            *snapshot_chainstate, coins_file, metadata);

        if (!snapshot_ok) {
            
        [&]() { LOCK(::cs_main);  this->MaybeRebalanceCaches() }()
        ;
            return false;
        }

        {
            LOCK(::cs_main);
            assert(!m_snapshot_chainstate);
            m_snapshot_chainstate.swap(snapshot_chainstate);
            const bool chaintip_loaded = m_snapshot_chainstate->LoadChainTip();
            assert(chaintip_loaded);

            m_active_chainstate = m_snapshot_chainstate.get();

            LogPrintf("[snapshot] successfully activated snapshot %s\n", base_blockhash.ToString());
            LogPrintf("[snapshot] (%.2f MB)\n",
                m_snapshot_chainstate->CoinsTip().DynamicMemoryUsage() / (1000 * 1000));

            this->MaybeRebalanceCaches();
        }
        return true;
        */
    }
    
    /**
      | Internal helper for ActivateSnapshot().
      |
      */
    pub fn populate_and_validate_snapshot(&mut self, 
        snapshot_chainstate: &mut dyn ChainStateInterface,
        coins_file:          &mut AutoFile,
        metadata:            &SnapshotMetadata) -> bool {
        
        todo!();
        /*
            // It's okay to release cs_main before we're done using `coins_cache` because we know
        // that nothing else will be referencing the newly created snapshot_chainstate yet.
        CCoinsViewCache& coins_cache = *
        [&]() { LOCK(::cs_main);  return &snapshot_chainstate.CoinsTip() }()
        ;

        uint256 base_blockhash = metadata.m_base_blockhash;

        CBlockIndex* snapshot_start_block = 
        [&]() { LOCK(::cs_main);  return m_blockman.LookupBlockIndex(base_blockhash) }()
        ;

        if (!snapshot_start_block) {
            // Needed for GetUTXOStats and ExpectedAssumeutxo to determine the height and to avoid a crash when base_blockhash.IsNull()
            LogPrintf("[snapshot] Did not find snapshot start blockheader %s\n",
                      base_blockhash.ToString());
            return false;
        }

        int base_height = snapshot_start_block->nHeight;
        auto maybe_au_data = ExpectedAssumeutxo(base_height, ::Params());

        if (!maybe_au_data) {
            LogPrintf("[snapshot] assumeutxo height in snapshot metadata not recognized " /* Continued */
                      "(%d) - refusing to load snapshot\n", base_height);
            return false;
        }

        const AssumeutxoData& au_data = *maybe_au_data;

        OutPoint outpoint;
        Coin coin;
        const uint64_t coins_count = metadata.m_coins_count;
        uint64_t coins_left = metadata.m_coins_count;

        LogPrintf("[snapshot] loading coins from snapshot %s\n", base_blockhash.ToString());
        int64_t flush_now{0};
        int64_t coins_processed{0};

        while (coins_left > 0) {
            try {
                coins_file >> outpoint;
                coins_file >> coin;
            } catch (const std::ios_base::failure&) {
                LogPrintf("[snapshot] bad snapshot format or truncated snapshot after deserializing %d coins\n",
                          coins_count - coins_left);
                return false;
            }
            if (coin.nHeight > base_height ||
                outpoint.n >= std::numeric_limits<decltype(outpoint.n)>::max() // Avoid integer wrap-around in coinstats.cpp:ApplyHash
            ) {
                LogPrintf("[snapshot] bad snapshot data after deserializing %d coins\n",
                          coins_count - coins_left);
                return false;
            }

            coins_cache.EmplaceCoinInternalDANGER(std::move(outpoint), std::move(coin));

            --coins_left;
            ++coins_processed;

            if (coins_processed % 1000000 == 0) {
                LogPrintf("[snapshot] %d coins loaded (%.2f%%, %.2f MB)\n",
                    coins_processed,
                    static_cast<float>(coins_processed) * 100 / static_cast<float>(coins_count),
                    coins_cache.DynamicMemoryUsage() / (1000 * 1000));
            }

            // Batch write and flush (if we need to) every so often.
            //
            // If our average Coin size is roughly 41 bytes, checking every 120,000 coins
            // means <5MB of memory imprecision.
            if (coins_processed % 120000 == 0) {
                if (ShutdownRequested()) {
                    return false;
                }

                const auto snapshot_cache_state = 
        [&]() { LOCK(::cs_main);  return snapshot_chainstate.GetCoinsCacheSizeState() }()
        ;

                if (snapshot_cache_state >=
                        CoinsCacheSizeState::CRITICAL) {
                    LogPrintf("[snapshot] flushing coins cache (%.2f MB)... ", /* Continued */
                        coins_cache.DynamicMemoryUsage() / (1000 * 1000));
                    flush_now = GetTimeMillis();

                    // This is a hack - we don't know what the actual best block is, but that
                    // doesn't matter for the purposes of flushing the cache here. We'll set this
                    // to its correct value (`base_blockhash`) below after the coins are loaded.
                    coins_cache.SetBestBlock(GetRandHash());

                    coins_cache.Flush();
                    LogPrintf("done (%.2fms)\n", GetTimeMillis() - flush_now);
                }
            }
        }

        // Important that we set this. This and the coins_cache accesses above are
        // sort of a layer violation, but either we reach into the innards of
        // CCoinsViewCache here or we have to invert some of the ChainState to
        // embed them in a snapshot-activation-specific CCoinsViewCache bulk load
        // method.
        coins_cache.SetBestBlock(base_blockhash);

        bool out_of_coins{false};
        try {
            coins_file >> outpoint;
        } catch (const std::ios_base::failure&) {
            // We expect an exception since we should be out of coins.
            out_of_coins = true;
        }
        if (!out_of_coins) {
            LogPrintf("[snapshot] bad snapshot - coins left over after deserializing %d coins\n",
                coins_count);
            return false;
        }

        LogPrintf("[snapshot] loaded %d (%.2f MB) coins from snapshot %s\n",
            coins_count,
            coins_cache.DynamicMemoryUsage() / (1000 * 1000),
            base_blockhash.ToString());

        LogPrintf("[snapshot] flushing snapshot chainstate to disk\n");
        // No need to acquire cs_main since this chainstate isn't being used yet.
        coins_cache.Flush(); // TODO: if #17487 is merged, add erase=false here for better performance.

        assert(coins_cache.GetBestBlock() == base_blockhash);

        CCoinsStats stats{CoinStatsHashType::HASH_SERIALIZED};
        auto breakpoint_fnc = [] { /* TODO insert breakpoint here? */ };

        // As above, okay to immediately release cs_main here since no other context knows
        // about the snapshot_chainstate.
        CCoinsViewDB* snapshot_coinsdb = 
        [&]() { LOCK(::cs_main);  return &snapshot_chainstate.CoinsDB() }()
        ;

        if (!GetUTXOStats(snapshot_coinsdb, 
        [&]() { LOCK(::cs_main);  return std::ref(m_blockman) }()
        , stats, breakpoint_fnc)) {
            LogPrintf("[snapshot] failed to generate coins stats\n");
            return false;
        }

        // Assert that the deserialized chainstate contents match the expected assumeutxo value.
        if (AssumeutxoHash{stats.hashSerialized} != au_data.hash_serialized) {
            LogPrintf("[snapshot] bad snapshot content hash: expected %s, got %s\n",
                au_data.hash_serialized.ToString(), stats.hashSerialized.ToString());
            return false;
        }

        snapshot_chainstate.m_chain.SetTip(snapshot_start_block);

        // The remainder of this function requires modifying data protected by cs_main.
        LOCK(::cs_main);

        // Fake various pieces of CBlockIndex state:
        CBlockIndex* index = nullptr;
        for (int i = 0; i <= snapshot_chainstate.m_chain.Height(); ++i) {
            index = snapshot_chainstate.m_chain[i];

            // Fake nTx so that LoadBlockIndex() loads assumed-valid CBlockIndex
            // entries (among other things)
            if (!index->nTx) {
                index->nTx = 1;
            }
            // Fake nChainTx so that GuessVerificationProgress reports accurately
            index->nChainTx = index->pprev ? index->pprev->nChainTx + index->nTx : 1;

            // Mark unvalidated block index entries beneath the snapshot base block as assumed-valid.
            if (!index->IsValid(BLOCK_VALID_SCRIPTS)) {
                // This flag will be removed once the block is fully validated by a
                // background chainstate.
                index->nStatus |= BLOCK_ASSUMED_VALID;
            }

            // Fake BLOCK_OPT_WITNESS so that ChainState::NeedsRedownload()
            // won't ask to rewind the entire assumed-valid chain on startup.
            if (index->pprev && DeploymentActiveAt(*index, ::Params().GetConsensus(), consensus::DEPLOYMENT_SEGWIT)) {
                index->nStatus |= BLOCK_OPT_WITNESS;
            }

            setDirtyBlockIndex.insert(index);
            // Changes to the block index will be flushed to disk after this call
            // returns in `ActivateSnapshot()`, when `MaybeRebalanceCaches()` is
            // called, since we've added a snapshot chainstate and therefore will
            // have to downsize the IBD chainstate, which will result in a call to
            // `FlushStateToDisk(ALWAYS)`.
        }

        assert(index);
        index->nChainTx = au_data.nChainTx;
        snapshot_chainstate.setBlockIndexCandidates.insert(snapshot_start_block);

        LogPrintf("[snapshot] validated snapshot (%.2f MB)\n",
            coins_cache.DynamicMemoryUsage() / (1000 * 1000));
        return true;
        */
    }
    
    /**
      | The most-work chain.
      |
      */
    pub fn active_chainstate(&self) -> &mut dyn ChainStateInterface {
        
        todo!();
        /*
            LOCK(::cs_main);
        assert(m_active_chainstate);
        return *m_active_chainstate;
        */
    }
    
    /**
      | @return
      | 
      | true if a snapshot-based chainstate
      | is in use. Also implies that a background
      | validation chainstate is also in use.
      |
      */
    pub fn is_snapshot_active(&self) -> bool {
        
        todo!();
        /*
            LOCK(::cs_main);
        return m_snapshot_chainstate && m_active_chainstate == m_snapshot_chainstate.get();
        */
    }
    
    /**
      | Unload block index and chain data before
      | shutdown.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn unload(&mut self)  {
        
        todo!();
        /*
            for (ChainState* chainstate : this->GetAll()) {
            chainstate->m_chain.SetTip(nullptr);
            chainstate->UnloadBlockIndex();
        }

        m_blockman.Unload();
        */
    }
    
    /**
      | Clear (deconstruct) chainstate data.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            LOCK(::cs_main);
        m_ibd_chainstate.reset();
        m_snapshot_chainstate.reset();
        m_active_chainstate = nullptr;
        m_snapshot_validated = false;
        */
    }
    
    /**
      | Check to see if caches are out of balance
      | and if so, call ResizeCoinsCaches()
      | as needed.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn maybe_rebalance_caches(&mut self)  {
        
        todo!();
        /*
            if (m_ibd_chainstate && !m_snapshot_chainstate) {
            LogPrintf("[snapshot] allocating all cache to the IBD chainstate\n");
            // Allocate everything to the IBD chainstate.
            m_ibd_chainstate->ResizeCoinsCaches(m_total_coinstip_cache, m_total_coinsdb_cache);
        }
        else if (m_snapshot_chainstate && !m_ibd_chainstate) {
            LogPrintf("[snapshot] allocating all cache to the snapshot chainstate\n");
            // Allocate everything to the snapshot chainstate.
            m_snapshot_chainstate->ResizeCoinsCaches(m_total_coinstip_cache, m_total_coinsdb_cache);
        }
        else if (m_ibd_chainstate && m_snapshot_chainstate) {
            // If both chainstates exist, determine who needs more cache based on IBD status.
            //
            // Note: shrink caches first so that we don't inadvertently overwhelm available memory.
            if (m_snapshot_chainstate->IsInitialBlockDownload()) {
                m_ibd_chainstate->ResizeCoinsCaches(
                    m_total_coinstip_cache * 0.05, m_total_coinsdb_cache * 0.05);
                m_snapshot_chainstate->ResizeCoinsCaches(
                    m_total_coinstip_cache * 0.95, m_total_coinsdb_cache * 0.95);
            } else {
                m_snapshot_chainstate->ResizeCoinsCaches(
                    m_total_coinstip_cache * 0.05, m_total_coinsdb_cache * 0.05);
                m_ibd_chainstate->ResizeCoinsCaches(
                    m_total_coinstip_cache * 0.95, m_total_coinsdb_cache * 0.95);
            }
        }
        */
    }
}
