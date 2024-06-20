crate::ix!();

/**
  | Use a macro instead of a function for
  | conditional logging to prevent evaluating
  | arguments when logging is not enabled.
  |
  | NOTE: The lambda captures all local variables
  | by value.
  */
macro_rules! enqueue_and_log_event {
    ($event:ident, 
     $fmt:ident, 
     $name:ident, 
     $($arg:ident),*) => {
        /*
        
            do {                                                       
                auto local_name = (name);                              
                LOG_EVENT("Enqueuing " fmt, local_name, __VA_ARGS__);  
                m_internals->m_schedulerClient.AddToProcessQueue([=] { 
                    LOG_EVENT(fmt, local_name, __VA_ARGS__);           
                    event();                                           
                });                                                    
            } while (0)
        */
    }
}

macro_rules! log_event {
    ($fmt:ident, $($arg:ident),*) => {
        /*
        
            LogPrint(BCLog::VALIDATION, fmt "\n", __VA_ARGS__)
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/validation.h]

/**
  | Default for -minrelaytxfee, minimum
  | relay fee for transactions
  |
  */
pub const DEFAULT_MIN_RELAY_TX_FEE: Amount = 1000;

/**
  | Default for -limitancestorcount,
  | max number of in-mempool ancestors
  |
  */
pub const DEFAULT_ANCESTOR_LIMIT: usize = 25;

/**
  | Default for -limitancestorsize, maximum
  | kilobytes of tx + all in-mempool ancestors
  |
  */
pub const DEFAULT_ANCESTOR_SIZE_LIMIT: usize = 101;

/**
  | Default for -limitdescendantcount,
  | max number of in-mempool descendants
  |
  */
pub const DEFAULT_DESCENDANT_LIMIT: usize = 25;

/**
  | Default for -limitdescendantsize,
  | maximum kilobytes of in-mempool descendants
  |
  */
pub const DEFAULT_DESCENDANT_SIZE_LIMIT: usize = 101;

/**
  | Default for -mempoolexpiry, expiration
  | time for mempool transactions in hours
  |
  */
pub const DEFAULT_MEMPOOL_EXPIRY: usize = 336;

/**
  | Maximum number of dedicated script-checking
  | threads allowed
  |
  */
pub const MAX_SCRIPTCHECK_THREADS: usize = 15;

/**
  | -par default (number of script-checking
  | threads, 0 = auto)
  |
  */
pub const DEFAULT_SCRIPTCHECK_THREADS: usize = 0;

pub const DEFAULT_MAX_TIP_AGE:         i64 = 24 * 60 * 60;

pub const DEFAULT_CHECKPOINTS_ENABLED: bool = true;
pub const DEFAULT_TXINDEX:             bool = false;
pub const DEFAULT_COINSTATSINDEX:      bool = false;

pub const DEFAULT_BLOCKFILTERINDEX: &'static str = "0";

/**
  | Default for -persistmempool
  |
  */
pub const DEFAULT_PERSIST_MEMPOOL: bool = true;

/**
  | Default for -stopatheight
  |
  */
pub const DEFAULT_STOPATHEIGHT: usize = 0;

/**
  | Block files containing a block-height
  | within MIN_BLOCKS_TO_KEEP of ActiveChain().Tip()
  | will not be pruned.
  |
  */
pub const MIN_BLOCKS_TO_KEEP: usize = 288;

pub const DEFAULT_CHECKBLOCKS: usize = 6;
pub const DEFAULT_CHECKLEVEL:  usize = 3;

/**
  | Require that user allocate at least 550 MiB for
  | block & undo files (blk???.dat and rev???.dat)
  |
  | At 1MB per block, 288 blocks = 288MB.
  |
  | Add 15% for Undo data = 331MB
  |
  | Add 20% for Orphan block rate = 397MB
  |
  | We want the low water mark after pruning to be
  | at least 397 MB and since we prune in full
  | block file chunks, we need the high water mark
  | which triggers the prune to be one 128MB block
  | file + added 15% undo data = 147MB greater for
  | a total of 545MB
  |
  | Setting the target to >= 550 MiB will make it
  | likely we can respect the target.
  */
pub const MIN_DISK_SPACE_FOR_BLOCK_FILES: u64 = 550 * 1024 * 1024;


lazy_static!{
    /*
    extern RecursiveMutex cs_main;
    extern Mutex g_best_block_mutex;
    extern std::condition_variable g_best_block_cv;
    /** Used to notify getblocktemplate RPC of new tips. */
    extern uint256 g_best_block;
    /** Whether there are dedicated script-checking threads running.
     * False indicates all script checking is done on the main threadMessageHandler thread.
     */
    extern bool g_parallel_script_checks;
    extern bool fRequireStandard;
    extern bool fCheckBlockIndex;
    extern bool fCheckpointsEnabled;
    /** A fee rate smaller than this is considered zero fee (for relaying, mining and transaction creation) */
    extern CFeeRate minRelayTxFee;
    /** If the tip is older than this (in seconds), the node is considered to be in initial block download. */
    extern int64_t nMaxTipAge;

    /** Block hash whose ancestors we will assume to have valid scripts without checking them. */
    extern uint256 hashAssumeValid;

    /** Minimum work we will assume exists on some valid chain. */
    extern arith_uint256 nMinimumChainWork;

    /** Best header we've seen so far (used for getheaders queries' starting points). */
    extern CBlockIndex *pindexBestHeader;

    /** Documentation for argument 'checklevel'. */
    extern const std::vector<std::string> CHECKLEVEL_DOC;
    */
}

/* ------- Transaction validation functions  ------- */

/**
  | Closure representing one script verification
  | 
  | -----------
  | @note
  | 
  | this stores references to the spending
  | transaction
  |
  */
pub struct ScriptCheck {
    tx_out:      TxOut,
    ptx_to:      Arc<Transaction>,
    n_in:        u32,
    n_flags:     u32,
    cache_store: bool,
    error:       ScriptError,
    txdata:      Arc<Mutex<PrecomputedTransactionData>>,
}

impl Default for ScriptCheck {
    
    fn default() -> Self {
        todo!();
        /*
        : ptx_to(nullptr),
        : n_in(0),
        : n_flags(0),
        : cache_store(false),
        : error(SCRIPT_ERR_UNKNOWN_ERROR),
        */
    }
}

impl ScriptCheck {

    pub fn new(
        out_in:     &TxOut,
        tx_to_in:   &Transaction,
        n_in_in:    u32,
        n_flags_in: u32,
        cache_in:   bool,
        txdata_in:  Arc<Mutex<PrecomputedTransactionData>>) -> Self {
    
        todo!();
        /*
            : m_tx_out(outIn), ptxTo(&txToIn), nIn(nInIn), nFlags(nFlagsIn), cacheStore(cacheIn), error(SCRIPT_ERR_UNKNOWN_ERROR), txdata(txdataIn)
        */
    }
    
    pub fn swap(&mut self, check: &mut ScriptCheck)  {
        
        todo!();
        /*
            std::swap(ptxTo, check.ptxTo);
            std::swap(m_tx_out, check.m_tx_out);
            std::swap(nIn, check.nIn);
            std::swap(nFlags, check.nFlags);
            std::swap(cacheStore, check.cacheStore);
            std::swap(error, check.error);
            std::swap(txdata, check.txdata);
        */
    }
    
    pub fn get_script_error(&self) -> ScriptError {
        
        todo!();
        /*
            return error;
        */
    }
}

/*
  | Functions for validating blocks and
  | updating the block tree
  |
  */



pub type FopenFn = unsafe extern "C" fn(*const i8, *const i8) -> *mut libc::FILE ;
//fn(_0: &Path, _1: *const u8) -> *mut libc::FILE;


//-------------------------------------------[.cpp/bitcoin/src/validation.cpp]

pub const MICRO: f32 = 0.000001;
pub const MILLI: f32 = 0.001;

/**
  | An extra transaction can be added to
  | a package, as long as it only has one ancestor
  | and is no larger than this.
  | 
  | Not really any reason to make this configurable
  | as it doesn't materially change DoS
  | parameters.
  |
  */
pub const EXTRA_DESCENDANT_TX_SIZE_LIMIT: usize = 10000;

/**
  | Maximum kilobytes for transactions
  | to store for processing during reorg
  |
  */
pub const MAX_DISCONNECTED_TX_POOL_SIZE: usize = 20000;

/**
  | Time to wait_mut between writing blocks/block
  | index to disk.
  |
  */
pub const DATABASE_WRITE_INTERVAL: Duration = Duration::hours(1);

/**
  | Time to wait between flushing chainstate
  | to disk.
  |
  */
pub const DATABASE_FLUSH_INTERVAL: Duration = Duration::hours(24);

/**
  | Maximum age of our tip for us to be considered
  | current for fee estimation
  |
  */
pub const MAX_FEE_ESTIMATION_TIP_AGE: Duration = Duration::hours(3);

lazy_static!{
    static ref CHECKLEVEL_DOC: Vec<&'static str> = vec!{
        "level 0 reads the blocks from disk",
        "level 1 verifies block validity",
        "level 2 verifies undo data",
        "level 3 checks disconnection of tip blocks",
        "level 4 tries to reconnect the blocks",
        "each level includes the checks of the previous levels",
    };
}

/**
  | Mutex to guard access to validation
  | specific variables, such as reading
  | or changing the chainstate.
  | 
  | This may also need to be locked when updating
  | the transaction pool, e.g. on
  | 
  | AcceptToMemoryPool. See CTxMemPool::cs
  | comment for details.
  | 
  | The transaction pool has a separate
  | lock to allow reading from it and the
  | chainstate at the same time.
  |
  */
lazy_static!{

    pub static ref CS_MAIN:              Arc<Mutex<()>>  = Default::default();
    pub static ref G_BEST_BLOCK_MUTEX:   Arc<Mutex<()>>  = Default::default();
    pub static ref G_BEST_BLOCK_CV:      Condvar         = Condvar::default();
    pub static ref G_BEST_BLOCK:         u256            = u256::default();
    pub static ref HASH_ASSUME_VALID:    u256            = u256::default();
    pub static ref N_MINIMUM_CHAIN_WORK: ArithU256       = ArithU256::default();

    pub static ref PINDEX_BEST_HEADER:       Arc<Mutex<Option<Arc<BlockIndex>>>> = Arc::new(Mutex::new(None));
    pub static ref G_PARALLEL_SCRIPT_CHECKS: AtomicBool = AtomicBool::new(false);
    pub static ref REQUIRE_STANDARD:         AtomicBool = AtomicBool::new(true);
    pub static ref CHECK_BLOCK_INDEX:        AtomicBool = AtomicBool::new(false);
    pub static ref CHECKPOINTS_ENABLED:      AtomicBool = AtomicBool::new(DEFAULT_CHECKPOINTS_ENABLED);
    pub static ref N_MAX_TIP_AGE:            AtomicI64  = AtomicI64::new(DEFAULT_MAX_TIP_AGE);
    pub static ref MIN_RELAY_TX_FEE:         FeeRate    = FeeRate::new(DEFAULT_MIN_RELAY_TX_FEE);
    pub static ref PINDEX_BEST_INVALID:      Arc<Mutex<BlockIndex>> = Default::default();
}

/**
  | Internal stuff from blockstorage ...
  |
  */
lazy_static!{
    /*
    extern RecursiveMutex cs_LastBlockFile;
    extern std::vector<CBlockFileInfo> vinfoBlockFile;
    extern int nLastBlockFile;
    extern bool fCheckForPruning;
    extern std::set<CBlockIndex*> setDirtyBlockIndex;
    extern std::set<int> setDirtyFileInfo;
    */
}

/**
  | ... TODO move fully to blockstorage
  |
  */
pub fn flush_block_file(
    finalize:      Option<bool>,
    finalize_undo: Option<bool>) {

    let finalize:      bool = finalize.unwrap_or(false);
    let finalize_undo: bool = finalize_undo.unwrap_or(false);

    todo!();
        /*
        
        */
}

/**
  | Check if transaction will be final in
  | the next block to be created.
  | 
  | Calls IsFinalTx() with current block
  | height and appropriate block time.
  | 
  | See consensus/consensus.h for flag
  | definitions.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn check_final_tx(
        active_chain_tip: Arc<BlockIndex>,
        tx:               &Transaction,
        flags:            Option<i32>) -> bool {

    let flags: i32 = flags.unwrap_or(-1);
    
    todo!();
        /*
        AssertLockHeld(cs_main);
        assert(active_chain_tip); // TODO: Make active_chain_tip a reference

        // By convention a negative value for flags indicates that the
        // current network-enforced consensus rules should be used. In
        // a future soft-fork scenario that would mean checking which
        // rules would be enforced for the next block and setting the
        // appropriate flags. At the present time no soft-forks are
        // scheduled, so no flags are set.
        flags = std::max(flags, 0);

        // CheckFinalTx() uses active_chain_tip.Height()+1 to evaluate
        // nLockTime because when IsFinalTx() is called within
        // AcceptBlock(), the height of the block *being*
        // evaluated is what is used. Thus if we want to know if a
        // transaction can be part of the *next* block, we need to call
        // IsFinalTx() with one more than active_chain_tip.Height().
        const int nBlockHeight = active_chain_tip->nHeight + 1;

        // BIP113 requires that time-locked transactions have nLockTime set to
        // less than the median time of the previous block they're contained in.
        // When the next block is created its previous block will be the current
        // chain tip, so we use that to calculate the median time passed to
        // IsFinalTx() if LOCKTIME_MEDIAN_TIME_PAST is set.
        const int64_t nBlockTime = (flags & LOCKTIME_MEDIAN_TIME_PAST)
                                 ? active_chain_tip->GetMedianTimePast()
                                 : GetAdjustedTime();

        return IsFinalTx(tx, nBlockHeight, nBlockTime);
        */
}

/**
  | Check if transaction will be BIP68 final
  | in the next block to be created on top
  | of tip.
  | 
  | -----------
  | @param[in] tip
  | 
  | Chain tip to check tx sequence locks
  | against. For example, the tip of the
  | current active chain.
  | ----------
  | @param[in] coins_view
  | 
  | Any CCoinsView that provides access
  | to the relevant coins for checking sequence
  | locks. For example, it can be a CCoinsViewCache
  | that isn't connected to anything but
  | contains all the relevant coins, or
  | a CCoinsViewMemPool that is connected
  | to the mempool and chainstate UTXO set.
  | In the latter case, the caller is responsible
  | for holding the appropriate locks to
  | ensure that calls to GetCoin() return
  | correct coins.
  | 
  | Simulates calling SequenceLocks()
  | with data from the tip passed in.
  | 
  | Optionally stores in LockPoints the
  | resulting height and time calculated
  | and the hash of the block needed for calculation
  | or skips the calculation and uses the
  | LockPoints passed in for evaluation.
  | 
  | The LockPoints should not be considered
  | valid if CheckSequenceLocks returns
  | false.
  | 
  | See consensus/consensus.h for flag
  | definitions.
  |
  */
pub fn check_sequence_locks(
        tip:                      Arc<Mutex<BlockIndex>>,
        coins_view:               &dyn CoinsView,
        tx:                       &Transaction,
        flags:                    i32,
        lp:                       Amo<LockPoints>,
        use_existing_lock_points: Option<bool>) -> bool {
    
    let use_existing_lock_points: bool = use_existing_lock_points.unwrap_or(false);

    todo!();
        /*
            assert(tip != nullptr);

        CBlockIndex index;
        index.pprev = tip;
        // CheckSequenceLocks() uses active_chainstate.m_chain.Height()+1 to evaluate
        // height based locks because when SequenceLocks() is called within
        // ConnectBlock(), the height of the block *being*
        // evaluated is what is used.
        // Thus if we want to know if a transaction can be part of the
        // *next* block, we need to use one more than active_chainstate.m_chain.Height()
        index.nHeight = tip->nHeight + 1;

        std::pair<int, int64_t> lockPair;
        if (useExistingLockPoints) {
            assert(lp);
            lockPair.first = lp->height;
            lockPair.second = lp->time;
        }
        else {
            std::vector<int> prevheights;
            prevheights.resize(tx.vin.size());
            for (size_t txinIndex = 0; txinIndex < tx.vin.size(); txinIndex++) {
                const CTxIn& txin = tx.vin[txinIndex];
                Coin coin;
                if (!coins_view.GetCoin(txin.prevout, coin)) {
                    return error("%s: Missing input", __func__);
                }
                if (coin.nHeight == MEMPOOL_HEIGHT) {
                    // Assume all mempool transaction confirm in the next block
                    prevheights[txinIndex] = tip->nHeight + 1;
                } else {
                    prevheights[txinIndex] = coin.nHeight;
                }
            }
            lockPair = CalculateSequenceLocks(tx, flags, prevheights, index);
            if (lp) {
                lp->height = lockPair.first;
                lp->time = lockPair.second;
                // Also store the hash of the block with the highest height of
                // all the blocks which have sequence locked prevouts.
                // This hash needs to still be on the chain
                // for these LockPoint calculations to be valid
                // Note: It is impossible to correctly calculate a maxInputBlock
                // if any of the sequence locked inputs depend on unconfirmed txs,
                // except in the special case where the relative lock time/height
                // is 0, which is equivalent to no sequence lock. Since we assume
                // input height of tip+1 for mempool txs and test the resulting
                // lockPair from CalculateSequenceLocks against tip+1.  We know
                // EvaluateSequenceLocks will fail if there was a non-zero sequence
                // lock on a mempool input, so we can use the return value of
                // CheckSequenceLocks to indicate the LockPoints validity
                int maxInputHeight = 0;
                for (const int height : prevheights) {
                    // Can ignore mempool inputs since we'll fail if they had non-zero locks
                    if (height != tip->nHeight+1) {
                        maxInputHeight = std::max(maxInputHeight, height);
                    }
                }
                lp->maxInputBlock = tip->GetAncestor(maxInputHeight);
            }
        }
        return EvaluateSequenceLocks(index, lockPair);
        */
}

#[EXCLUSIVE_LOCKS_REQUIRED(pool.cs, ::cs_main)]
pub fn limit_mempool_size(
        pool:        &mut TxMemPool,
        coins_cache: &mut CoinsViewCache,
        limit:       usize,
        age:         Duration /* seconds */)  {
    
    todo!();
        /*
            int expired = pool.Expire(GetTime<seconds>() - age);
        if (expired != 0) {
            LogPrint(BCLog::MEMPOOL, "Expired %i transactions from the memory pool\n", expired);
        }

        std::vector<OutPoint> vNoSpendsRemaining;
        pool.TrimToSize(limit, &vNoSpendsRemaining);
        for (const OutPoint& removed : vNoSpendsRemaining)
            coins_cache.Uncache(removed);
        */
}

/**
  | Checks to avoid mempool polluting consensus
  | critical paths since cached signature
  | and script validity results will be
  | reused if we validate this transaction
  | again during block validation.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main, pool.cs)]
pub fn check_inputs_from_mempool_and_cache(
        tx:        &Transaction,
        state:     &mut TxValidationState,
        view:      &CoinsViewCache,
        pool:      &TxMemPool,
        flags:     u32,
        txdata:    &mut PrecomputedTransactionData,
        coins_tip: &mut CoinsViewCache) -> bool {
    
    todo!();
        /*
            AssertLockHeld(cs_main);
        AssertLockHeld(pool.cs);

        assert(!tx.IsCoinBase());
        for (const CTxIn& txin : tx.vin) {
            const Coin& coin = view.AccessCoin(txin.prevout);

            // This coin was checked in PreChecks and MemPoolAccept
            // has been holding cs_main since then.
            Assume(!coin.IsSpent());
            if (coin.IsSpent()) return false;

            // If the Coin is available, there are 2 possibilities:
            // it is available in our current ChainstateActive UTXO set,
            // or it's a UTXO provided by a transaction in our mempool.
            // Ensure the scriptPubKeys in Coins from CoinsView are correct.
            const CTransactionRef& txFrom = pool.get(txin.prevout.hash);
            if (txFrom) {
                assert(txFrom->GetHash() == txin.prevout.hash);
                assert(txFrom->vout.size() > txin.prevout.n);
                assert(txFrom->vout[txin.prevout.n] == coin.out);
            } else {
                const Coin& coinFromUTXOSet = coins_tip.AccessCoin(txin.prevout);
                assert(!coinFromUTXOSet.IsSpent());
                assert(coinFromUTXOSet.out == coin.out);
            }
        }

        // Call CheckInputScripts() to cache signature and script validity against current tip consensus rules.
        return CheckInputScripts(tx, state, view, flags, /* cacheSigStore= */ true, /* cacheFullScriptStore= */ true, txdata);
        */
}

pub fn get_block_subsidy(
        n_height:         i32,
        consensus_params: &ChainConsensusParams) -> Amount {
    
    todo!();
        /*
            int halvings = nHeight / consensusParams.nSubsidyHalvingInterval;
        // Force block reward to zero when right shift is undefined.
        if (halvings >= 64)
            return 0;

        CAmount nSubsidy = 50 * COIN;
        // Subsidy is cut in half every 210,000 blocks which will occur approximately every 4 years.
        nSubsidy >>= halvings;
        return nSubsidy;
        */
}

impl ScriptCheck {
    
    pub fn invoke(&mut self) -> bool {
        
        todo!();
        /*
            const CScript &scriptSig = ptxTo->vin[nIn].scriptSig;
        const CScriptWitness *witness = &ptxTo->vin[nIn].scriptWitness;
        return VerifyScript(scriptSig, m_tx_out.scriptPubKey, witness, nFlags, CachingTransactionSignatureChecker(ptxTo, nIn, m_tx_out.nValue, cacheStore, *txdata), &error);
        */
    }
}


lazy_static!{
    /*
    static CuckooCache::cache<uint256, SignatureCacheHasher> g_scriptExecutionCache;
    static CSHA256 g_scriptExecutionCacheHasher;
    */
}

/**
  | Initializes the script-execution
  | cache
  |
  */
pub fn init_script_execution_cache()  {
    
    todo!();
        /*
            // Setup the salted hasher
        uint256 nonce = GetRandHash();
        // We want the nonce to be 64 bytes long to force the hasher to process
        // this chunk, which makes later hash computations more efficient. We
        // just write our 32-byte entropy twice to fill the 64 bytes.
        g_scriptExecutionCacheHasher.Write(nonce.begin(), 32);
        g_scriptExecutionCacheHasher.Write(nonce.begin(), 32);
        // nMaxCacheSize is unsigned. If -maxsigcachesize is set to zero,
        // setup_bytes creates the minimum possible cache (2 elements).
        size_t nMaxCacheSize = std::min(std::max((int64_t)0, gArgs.GetIntArg("-maxsigcachesize", DEFAULT_MAX_SIG_CACHE_SIZE) / 2), MAX_MAX_SIG_CACHE_SIZE) * ((size_t) 1 << 20);
        size_t nElems = g_scriptExecutionCache.setup_bytes(nMaxCacheSize);
        LogPrintf("Using %zu MiB out of %zu/2 requested for script execution cache, able to store %zu elements\n",
                (nElems*sizeof(uint256)) >>20, (nMaxCacheSize*2)>>20, nElems);
        */
}

/**
  | Check whether all of this transaction's
  | input scripts succeed.
  | 
  | This involves ECDSA signature checks
  | so can be computationally intensive.
  | This function should only be called
  | after the cheap sanity checks in
  | 
  | CheckTxInputs passed.
  | 
  | If pvChecks is not nullptr, script checks
  | are pushed onto it instead of being performed
  | inline. Any script checks which are
  | not necessary (eg due to script execution
  | cache hits) are, obviously, not pushed
  | onto pvChecks/run.
  | 
  | Setting cacheSigStore/cacheFullScriptStore
  | to false will remove elements from the
  | corresponding cache which are matched.
  | This is useful for checking blocks where
  | we will likely never need the cache entry
  | again.
  | 
  | -----------
  | @note
  | 
  | we may set state.reason to
  | 
  | NOT_STANDARD for extra soft-fork flags
  | in flags, block-checking callers should
  | probably reset it to CONSENSUS in such
  | cases.
  | 
  | Non-static (and re-declared) in src/test/txvalidationcache_tests.cpp
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn check_input_scripts(
        tx:                      &Transaction,
        state:                   &mut TxValidationState,
        inputs:                  &CoinsViewCache,
        flags:                   u32,
        cache_sig_store:         bool,
        cache_full_script_store: bool,
        txdata:                  &mut PrecomputedTransactionData,
        pv_checks:               Amo<Vec<ScriptCheck>>) -> bool {
    
    todo!();
        /*
            if (tx.IsCoinBase()) return true;

        if (pvChecks) {
            pvChecks->reserve(tx.vin.size());
        }

        // First check if script executions have been cached with the same
        // flags. Note that this assumes that the inputs provided are
        // correct (ie that the transaction hash which is in tx's prevouts
        // properly commits to the scriptPubKey in the inputs view of that
        // transaction).
        uint256 hashCacheEntry;
        CSHA256 hasher = g_scriptExecutionCacheHasher;
        hasher.Write(tx.GetWitnessHash().begin(), 32).Write((unsigned char*)&flags, sizeof(flags)).Finalize(hashCacheEntry.begin());
        AssertLockHeld(cs_main); //TODO: Remove this requirement by making CuckooCache not require external locks
        if (g_scriptExecutionCache.contains(hashCacheEntry, !cacheFullScriptStore)) {
            return true;
        }

        if (!txdata.m_spent_outputs_ready) {
            std::vector<CTxOut> spent_outputs;
            spent_outputs.reserve(tx.vin.size());

            for (const auto& txin : tx.vin) {
                const OutPoint& prevout = txin.prevout;
                const Coin& coin = inputs.AccessCoin(prevout);
                assert(!coin.IsSpent());
                spent_outputs.emplace_back(coin.out);
            }
            txdata.Init(tx, std::move(spent_outputs));
        }
        assert(txdata.m_spent_outputs.size() == tx.vin.size());

        for (unsigned int i = 0; i < tx.vin.size(); i++) {

            // We very carefully only pass in things to CScriptCheck which
            // are clearly committed to by tx' witness hash. This provides
            // a sanity check that our caching is not introducing consensus
            // failures through additional data in, eg, the coins being
            // spent being checked as a part of CScriptCheck.

            // Verify signature
            CScriptCheck check(txdata.m_spent_outputs[i], tx, i, flags, cacheSigStore, &txdata);
            if (pvChecks) {
                pvChecks->push_back(CScriptCheck());
                check.swap(pvChecks->back());
            } else if (!check()) {
                if (flags & STANDARD_NOT_MANDATORY_VERIFY_FLAGS) {
                    // Check whether the failure was caused by a
                    // non-mandatory script verification check, such as
                    // non-standard DER encodings or non-null dummy
                    // arguments; if so, ensure we return NOT_STANDARD
                    // instead of CONSENSUS to avoid downstream users
                    // splitting the network between upgraded and
                    // non-upgraded nodes by banning CONSENSUS-failing
                    // data providers.
                    CScriptCheck check2(txdata.m_spent_outputs[i], tx, i,
                            flags & ~STANDARD_NOT_MANDATORY_VERIFY_FLAGS, cacheSigStore, &txdata);
                    if (check2())
                        return state.Invalid(TxValidationResult::TX_NOT_STANDARD, strprintf("non-mandatory-script-verify-flag (%s)", ScriptErrorString(check.GetScriptError())));
                }
                // MANDATORY flag failures correspond to
                // TxValidationResult::TX_CONSENSUS. Because CONSENSUS
                // failures are the most serious case of validation
                // failures, we may need to consider using
                // RECENT_CONSENSUS_CHANGE for any script failure that
                // could be due to non-upgraded nodes which we may want to
                // support, to avoid splitting the network (but this
                // depends on the details of how net_processing handles
                // such errors).
                return state.Invalid(TxValidationResult::TX_CONSENSUS, strprintf("mandatory-script-verify-flag-failed (%s)", ScriptErrorString(check.GetScriptError())));
            }
        }

        if (cacheFullScriptStore && !pvChecks) {
            // We executed all of the provided scripts, and were told to
            // cache the result. Do so now.
            g_scriptExecutionCache.insert(hashCacheEntry);
        }

        return true;
        */
}

pub fn abort_node(
        state:        &mut BlockValidationState,
        str_message:  &String,
        user_message: Option<&BilingualStr>) -> bool {

    let df = BilingualStr::default();
    let user_message: &BilingualStr = user_message.unwrap_or(&df);
    
    todo!();
        /*
            AbortNode(strMessage, userMessage);
        return state.Error(strMessage);
        */
}

/**
  | Restore the UTXO in a Coin at a given OutPoint
  | 
  | -----------
  | @param undo
  | 
  | The Coin to be restored.
  | ----------
  | @param view
  | 
  | The coins view to which to apply the changes.
  | ----------
  | @param out
  | 
  | The out point that corresponds to the
  | tx input.
  | 
  | -----------
  | @return
  | 
  | A DisconnectResult as an int
  |
  */
pub fn apply_tx_in_undo(
        undo: Coin,
        view: &mut CoinsViewCache,
        out:  &OutPoint) -> i32 {
    
    todo!();
        /*
            bool fClean = true;

        if (view.HaveCoin(out)) fClean = false; // overwriting transaction output

        if (undo.nHeight == 0) {
            // Missing undo metadata (height and coinbase). Older versions included this
            // information only in undo records for the last spend of a transactions'
            // outputs. This implies that it must be present for some other output of the same tx.
            const Coin& alternate = AccessByTxid(view, out.hash);
            if (!alternate.IsSpent()) {
                undo.nHeight = alternate.nHeight;
                undo.fCoinBase = alternate.fCoinBase;
            } else {
                return DISCONNECT_FAILED; // adding output for transaction without known metadata
            }
        }
        // If the coin already exists as an unspent coin in the cache, then the
        // possible_overwrite parameter to AddCoin must be set to true. We have
        // already checked whether an unspent coin exists above using HaveCoin, so
        // we don't need to guess. When fClean is false, an unspent coin already
        // existed and it is an overwrite.
        view.AddCoin(out, std::move(undo), !fClean);

        return fClean ? DISCONNECT_OK : DISCONNECT_UNCLEAN;
        */
}

lazy_static!{
    /*
    static CCheckQueue<CScriptCheck> scriptcheckqueue(128);
    */
}

/**
  | Run instances of script checking worker
  | threads
  |
  */
pub fn start_script_check_worker_threads(threads_num: i32)  {
    
    todo!();
        /*
            scriptcheckqueue.StartWorkerThreads(threads_num);
        */
}

/**
  | Stop all of the script checking worker
  | threads
  |
  */
pub fn stop_script_check_worker_threads()  {
    
    todo!();
        /*
            scriptcheckqueue.StopWorkerThreads();
        */
}

/**
  | Threshold condition checker that triggers
  | when unknown versionbits are seen on
  | the network.
  |
  */
pub struct WarningBitsConditionChecker {
    bit:  i32,
}

impl AbstractThresholdConditionChecker for WarningBitsConditionChecker {

}

impl abstract_threshold_condition_checker::Interface for WarningBitsConditionChecker { }

impl abstract_threshold_condition_checker::MinActivationHeight for WarningBitsConditionChecker { 

}

impl abstract_threshold_condition_checker::BeginTime for WarningBitsConditionChecker { 

    fn begin_time(&self, params: &ChainConsensusParams) -> i64 {
        
        todo!();
        /*
            return 0;
        */
    }
}

impl abstract_threshold_condition_checker::EndTime for WarningBitsConditionChecker { 

    fn end_time(&self, params: &ChainConsensusParams) -> i64 {
        
        todo!();
        /*
            return std::numeric_limits<int64_t>::max();
        */
    }
}

impl abstract_threshold_condition_checker::Condition for WarningBitsConditionChecker { 

    fn condition(&self, 
        pindex: *const BlockIndex,
        params: &ChainConsensusParams) -> bool {
        
        todo!();
        /*
            return pindex->nHeight >= params.MinBIP9WarningHeight &&
                   ((pindex->nVersion & VERSIONBITS_TOP_MASK) == VERSIONBITS_TOP_BITS) &&
                   ((pindex->nVersion >> bit) & 1) != 0 &&
                   ((g_versionbitscache.ComputeBlockVersion(pindex->pprev, params) >> bit) & 1) == 0;
        */
    }
}

impl abstract_threshold_condition_checker::Threshold for WarningBitsConditionChecker { 

    fn threshold(&self, params: &ChainConsensusParams) -> i32 {
        
        todo!();
        /*
            return params.nRuleChangeActivationThreshold;
        */
    }
}

impl abstract_threshold_condition_checker::Period for WarningBitsConditionChecker { 

    fn period(&self, params: &ChainConsensusParams) -> i32 {
        
        todo!();
        /*
            return params.nMinerConfirmationWindow;
        */
    }
}

impl WarningBitsConditionChecker {

    pub fn new(bit_in: i32) -> Self {
    
        todo!();
        /*
        : bit(bitIn),
        */
    }
}

lazy_static!{
    /*
    static ThresholdConditionCache warningcache[VERSIONBITS_NUM_BITS] GUARDED_BY(cs_main);
    */
}

/**
  | Returns the script flags which should
  | be checked for a given block
  |
  */
pub fn get_block_script_flags(
        pindex:          Arc<BlockIndex>,
        consensusparams: &ChainConsensusParams) -> u32 {
    
    todo!();
        /*
            unsigned int flags = SCRIPT_VERIFY_NONE;

        // BIP16 didn't become active until Apr 1 2012 (on mainnet, and
        // retroactively applied to testnet)
        // However, only one historical block violated the P2SH rules (on both
        // mainnet and testnet), so for simplicity, always leave P2SH
        // on except for the one violating block.
        if (consensusparams.BIP16Exception.IsNull() || // no bip16 exception on this chain
            pindex->phashBlock == nullptr || // this is a new candidate block, eg from TestBlockValidity()
            *pindex->phashBlock != consensusparams.BIP16Exception) // this block isn't the historical exception
        {
            // Enforce WITNESS rules whenever P2SH is in effect
            flags |= SCRIPT_VERIFY_P2SH | SCRIPT_VERIFY_WITNESS;
        }

        // Enforce the DERSIG (BIP66) rule
        if (DeploymentActiveAt(*pindex, consensusparams, consensus::DEPLOYMENT_DERSIG)) {
            flags |= SCRIPT_VERIFY_DERSIG;
        }

        // Enforce CHECKLOCKTIMEVERIFY (BIP65)
        if (DeploymentActiveAt(*pindex, consensusparams, consensus::DEPLOYMENT_CLTV)) {
            flags |= SCRIPT_VERIFY_CHECKLOCKTIMEVERIFY;
        }

        // Enforce CHECKSEQUENCEVERIFY (BIP112)
        if (DeploymentActiveAt(*pindex, consensusparams, consensus::DEPLOYMENT_CSV)) {
            flags |= SCRIPT_VERIFY_CHECKSEQUENCEVERIFY;
        }

        // Enforce Taproot (BIP340-BIP342)
        if (DeploymentActiveAt(*pindex, consensusparams, consensus::DEPLOYMENT_TAPROOT)) {
            flags |= SCRIPT_VERIFY_TAPROOT;
        }

        // Enforce BIP147 NULLDUMMY (activated simultaneously with segwit)
        if (DeploymentActiveAt(*pindex, consensusparams, consensus::DEPLOYMENT_SEGWIT)) {
            flags |= SCRIPT_VERIFY_NULLDUMMY;
        }

        return flags;
        */
}

lazy_static!{
    /*
    static int64_t nTimeCheck = 0;
    static int64_t nTimeForks = 0;
    static int64_t nTimeVerify = 0;
    static int64_t nTimeConnect = 0;
    static int64_t nTimeIndex = 0;
    static int64_t nTimeTotal = 0;
    static int64_t nBlocksTotal = 0;
    */
}

pub fn do_warning(warning: &BilingualStr)  {
    
    todo!();
        /*
            static bool fWarned = false;
        SetMiscWarning(warning);
        if (!fWarned) {
            AlertNotify(warning.original);
            fWarned = true;
        }
        */
}

/**
  | Private helper function that concatenates
  | warning messages.
  |
  */
pub fn append_warning(
        res:  &mut BilingualStr,
        warn: &BilingualStr)  {
    
    todo!();
        /*
            if (!res.empty()) res += Untranslated(", ");
        res += warn;
        */
}

lazy_static!{
    /*
    static int64_t nTimeReadFromDisk = 0;
    static int64_t nTimeConnectTotal = 0;
    static int64_t nTimeFlush = 0;
    static int64_t nTimeChainState = 0;
    static int64_t nTimePostConnect = 0;
    */
}

pub fn check_block_header(
        block:            &BlockHeader,
        state:            &mut BlockValidationState,
        consensus_params: &ChainConsensusParams,
        checkpow:         Option<bool>) -> bool {
    let checkpow: bool = checkpow.unwrap_or(true);

    todo!();
        /*
            // Check proof of work matches claimed amount
        if (fCheckPOW && !CheckProofOfWork(block.GetHash(), block.nBits, consensusParams))
            return state.Invalid(BlockValidationResult::BLOCK_INVALID_HEADER, "high-hash", "proof of work failed");

        return true;
        */
}

/**
  | Context-independent validity checks
  |
  */
pub fn check_block(
        block:             &Block,
        state:             &mut BlockValidationState,
        consensus_params:  &ChainConsensusParams,
        checkpow:          Option<bool>,
        check_merkle_root: Option<bool>) -> bool {
    
    let checkpow:          bool = checkpow.unwrap_or(true);
    let check_merkle_root: bool = check_merkle_root.unwrap_or(true);

    todo!();
        /*
            // These are checks that are independent of context.

        if (block.fChecked)
            return true;

        // Check that the header is valid (particularly PoW).  This is mostly
        // redundant with the call in AcceptBlockHeader.
        if (!CheckBlockHeader(block, state, consensusParams, fCheckPOW))
            return false;

        // Signet only: check block solution
        if (consensusParams.signet_blocks && fCheckPOW && !CheckSignetBlockSolution(block, consensusParams)) {
            return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-signet-blksig", "signet block signature validation failure");
        }

        // Check the merkle root.
        if (fCheckMerkleRoot) {
            bool mutated;
            uint256 hashMerkleRoot2 = BlockMerkleRoot(block, &mutated);
            if (block.hashMerkleRoot != hashMerkleRoot2)
                return state.Invalid(BlockValidationResult::BLOCK_MUTATED, "bad-txnmrklroot", "hashMerkleRoot mismatch");

            // Check for merkle tree malleability (CVE-2012-2459): repeating sequences
            // of transactions in a block without affecting the merkle root of a block,
            // while still invalidating it.
            if (mutated)
                return state.Invalid(BlockValidationResult::BLOCK_MUTATED, "bad-txns-duplicate", "duplicate transaction");
        }

        // All potential-corruption validation must be done before we do any
        // transaction validation, as otherwise we may mark the header as invalid
        // because we receive the wrong transactions for it.
        // Note that witness malleability is checked in ContextualCheckBlock, so no
        // checks that use witness data may be performed here.

        // Size limits
        if (block.vtx.empty() || block.vtx.size() * WITNESS_SCALE_FACTOR > MAX_BLOCK_WEIGHT || ::GetSerializeSize(block, PROTOCOL_VERSION | SERIALIZE_TRANSACTION_NO_WITNESS) * WITNESS_SCALE_FACTOR > MAX_BLOCK_WEIGHT)
            return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-blk-length", "size limits failed");

        // First transaction must be coinbase, the rest must not be
        if (block.vtx.empty() || !block.vtx[0]->IsCoinBase())
            return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-cb-missing", "first tx is not coinbase");
        for (unsigned int i = 1; i < block.vtx.size(); i++)
            if (block.vtx[i]->IsCoinBase())
                return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-cb-multiple", "more than one coinbase");

        // Check transactions
        // Must check for duplicate inputs (see CVE-2018-17144)
        for (const auto& tx : block.vtx) {
            TxValidationState tx_state;
            if (!CheckTransaction(*tx, tx_state)) {
                // CheckBlock() does context-free validation checks. The only
                // possible failures are consensus failures.
                assert(tx_state.GetResult() == TxValidationResult::TX_CONSENSUS);
                return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, tx_state.GetRejectReason(),
                                     strprintf("Transaction check failed (tx hash %s) %s", tx->GetHash().ToString(), tx_state.GetDebugMessage()));
            }
        }
        unsigned int nSigOps = 0;
        for (const auto& tx : block.vtx)
        {
            nSigOps += GetLegacySigOpCount(*tx);
        }
        if (nSigOps * WITNESS_SCALE_FACTOR > MAX_BLOCK_SIGOPS_COST)
            return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-blk-sigops", "out-of-bounds SigOpCount");

        if (fCheckPOW && fCheckMerkleRoot)
            block.fChecked = true;

        return true;
        */
}

/**
  | Update uncommitted block structures
  | (currently: only the witness reserved
  | value). This is safe for submitted blocks.
  |
  */
pub fn update_uncommitted_block_structures(
        block:            &mut Block,
        pindex_prev:      Arc<BlockIndex>,
        consensus_params: &ChainConsensusParams)  {
    
    todo!();
        /*
            int commitpos = GetWitnessCommitmentIndex(block);
        static const std::vector<unsigned char> nonce(32, 0x00);
        if (commitpos != NO_WITNESS_COMMITMENT && DeploymentActiveAfter(pindexPrev, consensusParams, consensus::DEPLOYMENT_SEGWIT) && !block.vtx[0]->HasWitness()) {
            CMutableTransaction tx(*block.vtx[0]);
            tx.vin[0].scriptWitness.stack.resize(1);
            tx.vin[0].scriptWitness.stack[0] = nonce;
            block.vtx[0] = MakeTransactionRef(std::move(tx));
        }
        */
}

/**
  | Produce the necessary coinbase commitment
  | for a block (modifies the hash, don't
  | call for mined blocks).
  |
  */
pub fn generate_coinbase_commitment(
        block:            &mut Block,
        pindex_prev:      Arc<BlockIndex>,
        consensus_params: &ChainConsensusParams) -> Vec<u8> {
    
    todo!();
        /*
            std::vector<unsigned char> commitment;
        int commitpos = GetWitnessCommitmentIndex(block);
        std::vector<unsigned char> ret(32, 0x00);
        if (commitpos == NO_WITNESS_COMMITMENT) {
            uint256 witnessroot = BlockWitnessMerkleRoot(block, nullptr);
            CHash256().Write(witnessroot).Write(ret).Finalize(witnessroot);
            CTxOut out;
            out.nValue = 0;
            out.scriptPubKey.resize(MINIMUM_WITNESS_COMMITMENT);
            out.scriptPubKey[0] = OP_RETURN;
            out.scriptPubKey[1] = 0x24;
            out.scriptPubKey[2] = 0xaa;
            out.scriptPubKey[3] = 0x21;
            out.scriptPubKey[4] = 0xa9;
            out.scriptPubKey[5] = 0xed;
            memcpy(&out.scriptPubKey[6], witnessroot.begin(), 32);
            commitment = std::vector<unsigned char>(out.scriptPubKey.begin(), out.scriptPubKey.end());
            CMutableTransaction tx(*block.vtx[0]);
            tx.vout.push_back(out);
            block.vtx[0] = MakeTransactionRef(std::move(tx));
        }
        UpdateUncommittedBlockStructures(block, pindexPrev, consensusParams);
        return commitment;
        */
}


/**
  | @note
  | 
  | This function is not currently invoked
  | by ConnectBlock(), so we should consider
  | upgrade issues if we change which consensus
  | rules are enforced in this function
  | (eg by adding a new consensus rule).
  | See comment in ConnectBlock().
  | ----------
  | @note
  | 
  | -reindex-chainstate skips the validation
  | that happens here!
  |
  */
pub fn contextual_check_block(
        block:            &Block,
        state:            &mut BlockValidationState,
        consensus_params: &ChainConsensusParams,
        pindex_prev:      Arc<BlockIndex>) -> bool {
    
    todo!();
        /*
            const int nHeight = pindexPrev == nullptr ? 0 : pindexPrev->nHeight + 1;

        // Enforce BIP113 (Median Time Past).
        int nLockTimeFlags = 0;
        if (DeploymentActiveAfter(pindexPrev, consensusParams, consensus::DEPLOYMENT_CSV)) {
            assert(pindexPrev != nullptr);
            nLockTimeFlags |= LOCKTIME_MEDIAN_TIME_PAST;
        }

        int64_t nLockTimeCutoff = (nLockTimeFlags & LOCKTIME_MEDIAN_TIME_PAST)
                                  ? pindexPrev->GetMedianTimePast()
                                  : block.GetBlockTime();

        // Check that all transactions are finalized
        for (const auto& tx : block.vtx) {
            if (!IsFinalTx(*tx, nHeight, nLockTimeCutoff)) {
                return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-txns-nonfinal", "non-final transaction");
            }
        }

        // Enforce rule that the coinbase starts with serialized block height
        if (DeploymentActiveAfter(pindexPrev, consensusParams, consensus::DEPLOYMENT_HEIGHTINCB))
        {
            CScript expect = CScript() << nHeight;
            if (block.vtx[0]->vin[0].scriptSig.size() < expect.size() ||
                !std::equal(expect.begin(), expect.end(), block.vtx[0]->vin[0].scriptSig.begin())) {
                return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-cb-height", "block height mismatch in coinbase");
            }
        }

        // Validation for witness commitments.
        // * We compute the witness hash (which is the hash including witnesses) of all the block's transactions, except the
        //   coinbase (where 0x0000....0000 is used instead).
        // * The coinbase scriptWitness is a stack of a single 32-byte vector, containing a witness reserved value (unconstrained).
        // * We build a merkle tree with all those witness hashes as leaves (similar to the hashMerkleRoot in the block header).
        // * There must be at least one output whose scriptPubKey is a single 36-byte push, the first 4 bytes of which are
        //   {0xaa, 0x21, 0xa9, 0xed}, and the following 32 bytes are SHA256^2(witness root, witness reserved value). In case there are
        //   multiple, the last one is used.
        bool fHaveWitness = false;
        if (DeploymentActiveAfter(pindexPrev, consensusParams, consensus::DEPLOYMENT_SEGWIT)) {
            int commitpos = GetWitnessCommitmentIndex(block);
            if (commitpos != NO_WITNESS_COMMITMENT) {
                bool malleated = false;
                uint256 hashWitness = BlockWitnessMerkleRoot(block, &malleated);
                // The malleation check is ignored; as the transaction tree itself
                // already does not permit it, it is impossible to trigger in the
                // witness tree.
                if (block.vtx[0]->vin[0].scriptWitness.stack.size() != 1 || block.vtx[0]->vin[0].scriptWitness.stack[0].size() != 32) {
                    return state.Invalid(BlockValidationResult::BLOCK_MUTATED, "bad-witness-nonce-size", strprintf("%s : invalid witness reserved value size", __func__));
                }
                CHash256().Write(hashWitness).Write(block.vtx[0]->vin[0].scriptWitness.stack[0]).Finalize(hashWitness);
                if (memcmp(hashWitness.begin(), &block.vtx[0]->vout[commitpos].scriptPubKey[6], 32)) {
                    return state.Invalid(BlockValidationResult::BLOCK_MUTATED, "bad-witness-merkle-match", strprintf("%s : witness merkle commitment mismatch", __func__));
                }
                fHaveWitness = true;
            }
        }

        // No witness data is allowed in blocks that don't commit to witness data, as this would otherwise leave room for spam
        if (!fHaveWitness) {
          for (const auto& tx : block.vtx) {
                if (tx->HasWitness()) {
                    return state.Invalid(BlockValidationResult::BLOCK_MUTATED, "unexpected-witness", strprintf("%s : unexpected witness data found", __func__));
                }
            }
        }

        // After the coinbase witness reserved value and commitment are verified,
        // we can check if the block weight passes (before we've checked the
        // coinbase witness, it would be possible for the weight to be too
        // large by filling up the coinbase witness, which doesn't change
        // the block hash, so we couldn't mark the block as permanently
        // failed).
        if (GetBlockWeight(block) > MAX_BLOCK_WEIGHT) {
            return state.Invalid(BlockValidationResult::BLOCK_CONSENSUS, "bad-blk-weight", strprintf("%s : weight limit failed", __func__));
        }

        return true;
        */
}

/* -------------- BLOCK PRUNING CODE  -------------- */

pub const MEMPOOL_DUMP_VERSION: u64 = 1;


/**
  | Dump the mempool to disk.
  |
  */
pub fn dump_mempool(
        pool:                    &TxMemPool,
        mockable_fopen_function: Option<FopenFn>,
        skip_file_commit:        Option<bool>) -> bool {

    let mockable_fopen_function: FopenFn = mockable_fopen_function.unwrap_or(libc::fopen);
    let skip_file_commit:           bool = skip_file_commit.unwrap_or(false);
    
    todo!();
        /*
            int64_t start = GetTimeMicros();

        std::map<uint256, CAmount> mapDeltas;
        std::vector<TxMemPoolInfo> vinfo;
        std::set<uint256> unbroadcast_txids;

        static Mutex dump_mutex;
        LOCK(dump_mutex);

        {
            LOCK(pool.cs);
            for (const auto &i : pool.mapDeltas) {
                mapDeltas[i.first] = i.second;
            }
            vinfo = pool.infoAll();
            unbroadcast_txids = pool.GetUnbroadcastTxs();
        }

        int64_t mid = GetTimeMicros();

        try {
            FILE* filestr{mockable_fopen_function(gArgs.GetDataDirNet() / "mempool.dat.new", "wb")};
            if (!filestr) {
                return false;
            }

            CAutoFile file(filestr, SER_DISK, CLIENT_VERSION);

            uint64_t version = MEMPOOL_DUMP_VERSION;
            file << version;

            file << (uint64_t)vinfo.size();
            for (const auto& i : vinfo) {
                file << *(i.tx);
                file << int64_t{count_seconds(i.m_time)};
                file << int64_t{i.nFeeDelta};
                mapDeltas.erase(i.tx->GetHash());
            }

            file << mapDeltas;

            LogPrintf("Writing %d unbroadcast transactions to disk.\n", unbroadcast_txids.size());
            file << unbroadcast_txids;

            if (!skip_file_commit && !FileCommit(file.Get()))
                throw std::runtime_error("FileCommit failed");
            file.fclose();
            if (!RenameOver(gArgs.GetDataDirNet() / "mempool.dat.new", gArgs.GetDataDirNet() / "mempool.dat")) {
                throw std::runtime_error("Rename failed");
            }
            int64_t last = GetTimeMicros();
            LogPrintf("Dumped mempool: %gs to copy, %gs to dump\n", (mid-start)*MICRO, (last-mid)*MICRO);
        } catch (const std::exception& e) {
            LogPrintf("Failed to dump mempool: %s. Continuing anyway.\n", e.what());
            return false;
        }
        return true;
        */
}

pub fn alert_notify(str_message: &String)  {
    
    todo!();
        /*
            uiInterface.NotifyAlertChanged();
    #if HAVE_SYSTEM
        std::string strCmd = gArgs.GetArg("-alertnotify", "");
        if (strCmd.empty()) return;

        // Alert text should be plain ascii coming from a trusted source, but to
        // be safe we first strip anything not in safeChars, then add single quotes around
        // the whole string before passing it to the shell:
        std::string singleQuote("'");
        std::string safeStatus = SanitizeString(strMessage);
        safeStatus = singleQuote+safeStatus+singleQuote;
        boost::replace_all(strCmd, "%s", safeStatus);

        std::thread t(runCommand, strCmd);
        t.detach(); // thread runs free
    #endif
        */
}

pub fn update_coins(
        tx:       &Transaction,
        inputs:   &mut CoinsViewCache,
        txundo:   &mut TxUndo,
        n_height: i32)  {
    
    todo!();
        /*
            // mark inputs spent
        if (!tx.IsCoinBase()) {
            txundo.vprevout.reserve(tx.vin.size());
            for (const CTxIn &txin : tx.vin) {
                txundo.vprevout.emplace_back();
                bool is_spent = inputs.SpendCoin(txin.prevout, &txundo.vprevout.back());
                assert(is_spent);
            }
        }
        // add outputs
        AddCoins(inputs, tx, nHeight);
        */
}

#[LOCKS_EXCLUDED(cs_main)]
pub fn limit_validation_interface_queue()  {
    
    todo!();
        /*
            AssertLockNotHeld(cs_main);

        if (GetMainSignals().CallbacksPending() > 10) {
            SyncWithValidationInterfaceQueue();
        }
        */
}

//-------------------------------------------[.cpp/bitcoin/src/consensus/validation.h]

/**
  | Index marker for when no witness commitment
  | is present in a coinbase transaction.
  |
  */
pub const NO_WITNESS_COMMITMENT: i32 = -1;

/**
  | Minimum size of a witness commitment
  | structure. Defined in BIP 141. *
  |
  */
pub const MINIMUM_WITNESS_COMMITMENT: usize = 38;

/**
  | A "reason" why a transaction was invalid,
  | suitable for determining whether the
  | provider of the transaction should
  | be banned/ignored/disconnected/etc.
  |
  */
#[derive(PartialEq,Eq,Clone,Debug)]
pub enum TxValidationResult {

    /**
      | initial value. Tx has not yet been rejected
      |
      */
    TX_RESULT_UNSET = 0,     

    /**
      | invalid by consensus rules
      |
      */
    TX_CONSENSUS,            

    /**
      | Invalid by a change to consensus rules
      | more recent than SegWit.
      | 
      | Currently unused as there are no such
      | consensus rule changes, and any download
      | sources realistically need to support
      | SegWit in order to provide useful data,
      | so differentiating between always-invalid
      | and invalid-by-pre-SegWit-soft-fork
      | is uninteresting.
      |
      */
    TX_RECENT_CONSENSUS_CHANGE,

    /**
      | inputs (covered by txid) failed policy
      | rules
      |
      */
    TX_INPUTS_NOT_STANDARD,   

    /**
      | otherwise didn't meet our local policy
      | rules
      |
      */
    TX_NOT_STANDARD,          

    /**
      | transaction was missing some of its
      | inputs
      |
      */
    TX_MISSING_INPUTS,        

    /**
      | transaction spends a coinbase too early,
      | or violates locktime/sequence locks
      |
      */
    TX_PREMATURE_SPEND,       

    /**
      | Transaction might have a witness prior
      | to SegWit activation, or witness may
      | have been malleated (which includes
      | non-standard witnesses).
      |
      */
    TX_WITNESS_MUTATED,

    /**
      | Transaction is missing a witness.
      |
      */
    TX_WITNESS_STRIPPED,

    /**
      | Tx already in mempool or conflicts with
      | a tx in the chain (if it conflicts with
      | another tx in mempool, we use MEMPOOL_POLICY
      | as it failed to reach the RBF threshold)
      | 
      | Currently this is only used if the transaction
      | already exists in the mempool or on chain.
      |
      */
    TX_CONFLICT,

    /**
      | violated mempool's fee/size/descendant/RBF/etc
      | limits
      |
      */
    TX_MEMPOOL_POLICY,        
}

pub struct TxValidationState {
    base: ValidationState<TxValidationResult>,
}

impl TxValidationState {

    delegate! {
        to self.base {

            pub fn invalid(&mut self, 
                result:        TxValidationResult,
                reject_reason: Option<&str>,
                debug_message: Option<&str>) -> bool;
            
            pub fn error(&mut self, reject_reason: &String) -> bool;
            
            pub fn is_valid(&self) -> bool;
            
            pub fn is_invalid(&self) -> bool;
            
            pub fn is_error(&self) -> bool;
            
            pub fn get_result(&self) -> TxValidationResult;
            
            pub fn get_reject_reason(&self) -> String;
            
            pub fn get_debug_message(&self) -> String;
            
            pub fn to_string(&self) -> String;
        }
    }
}

/**
  | These implement the weight = (stripped_size
  | * 4) + witness_size formula, using only
  | serialization with and without witness data. As
  | witness_size is equal to total_size
  | - stripped_size, this formula is identical to:
  | weight = (stripped_size * 3) + total_size.
  */
#[inline] pub fn get_transaction_weight(tx: &Transaction) -> i64 {
    
    todo!();
        /*
            return ::GetSerializeSize(tx, PROTOCOL_VERSION | SERIALIZE_TRANSACTION_NO_WITNESS) * (WITNESS_SCALE_FACTOR - 1) + ::GetSerializeSize(tx, PROTOCOL_VERSION);
        */
}

#[inline] pub fn get_block_weight(block: &Block) -> i64 {
    
    todo!();
        /*
            return ::GetSerializeSize(block, PROTOCOL_VERSION | SERIALIZE_TRANSACTION_NO_WITNESS) * (WITNESS_SCALE_FACTOR - 1) + ::GetSerializeSize(block, PROTOCOL_VERSION);
        */
}

#[inline] pub fn get_transaction_input_weight(txin: &TxIn) -> i64 {
    
    todo!();
        /*
            // scriptWitness size is added here because witnesses and txins are split up in segwit serialization.
        return ::GetSerializeSize(txin, PROTOCOL_VERSION | SERIALIZE_TRANSACTION_NO_WITNESS) * (WITNESS_SCALE_FACTOR - 1) + ::GetSerializeSize(txin, PROTOCOL_VERSION) + ::GetSerializeSize(txin.scriptWitness.stack, PROTOCOL_VERSION);
        */
}

/**
  | Compute at which vout of the block's
  | coinbase transaction the witness commitment
  | occurs, or -1 if not found
  |
  */
#[inline] pub fn get_witness_commitment_index(block: &Block) -> i32 {
    
    todo!();
        /*
            int commitpos = NO_WITNESS_COMMITMENT;
        if (!block.vtx.empty()) {
            for (size_t o = 0; o < block.vtx[0]->vout.size(); o++) {
                const CTxOut& vout = block.vtx[0]->vout[o];
                if (vout.scriptPubKey.size() >= MINIMUM_WITNESS_COMMITMENT &&
                    vout.scriptPubKey[0] == OP_RETURN &&
                    vout.scriptPubKey[1] == 0x24 &&
                    vout.scriptPubKey[2] == 0xaa &&
                    vout.scriptPubKey[3] == 0x21 &&
                    vout.scriptPubKey[4] == 0xa9 &&
                    vout.scriptPubKey[5] == 0xed) {
                    commitpos = o;
                }
            }
        }
        return commitpos;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/consensus/tx_verify.h]

/* ------- Transaction validation functions  ------- */

/*
  | Auxiliary functions for transaction
  | validation (ideally should not be exposed)
  |
  */

//-------------------------------------------[.cpp/bitcoin/src/consensus/tx_verify.cpp]

/**
  | Check if transaction is final and can
  | be included in a block with the specified
  | height and time. Consensus critical.
  |
  */
pub fn is_final_tx(
        tx:             &Transaction,
        n_block_height: i32,
        n_block_time:   i64) -> bool {
    
    todo!();
        /*
            if (tx.nLockTime == 0)
            return true;
        if ((int64_t)tx.nLockTime < ((int64_t)tx.nLockTime < LOCKTIME_THRESHOLD ? (int64_t)nBlockHeight : nBlockTime))
            return true;

        // Even if tx.nLockTime isn't satisfied by nBlockHeight/nBlockTime, a
        // transaction is still considered final if all inputs' nSequence ==
        // SEQUENCE_FINAL (0xffffffff), in which case nLockTime is ignored.
        //
        // Because of this behavior OP_CHECKLOCKTIMEVERIFY/CheckLockTime() will
        // also check that the spending input's nSequence != SEQUENCE_FINAL,
        // ensuring that an unsatisfied nLockTime value will actually cause
        // IsFinalTx() to return false here:
        for (const auto& txin : tx.vin) {
            if (!(txin.nSequence == CTxIn::SEQUENCE_FINAL))
                return false;
        }
        return true;
        */
}

/**
  | Calculates the block height and previous
  | block's median time past at which the
  | transaction will be considered final
  | in the context of BIP 68.
  | 
  | Also removes from the vector of input
  | heights any entries which did not correspond
  | to sequence locked inputs as they do
  | not affect the calculation.
  |
  */
pub fn calculate_sequence_locks(
        tx:           &Transaction,
        flags:        i32,
        prev_heights: &mut Vec<i32>,
        block:        &BlockIndex) -> (i32,i64) {
    
    todo!();
        /*
            assert(prevHeights.size() == tx.vin.size());

        // Will be set to the equivalent height- and time-based nLockTime
        // values that would be necessary to satisfy all relative lock-
        // time constraints given our view of block chain history.
        // The semantics of nLockTime are the last invalid height/time, so
        // use -1 to have the effect of any height or time being valid.
        int nMinHeight = -1;
        int64_t nMinTime = -1;

        // tx.nVersion is signed integer so requires cast to unsigned otherwise
        // we would be doing a signed comparison and half the range of nVersion
        // wouldn't support BIP 68.
        bool fEnforceBIP68 = static_cast<uint32_t>(tx.nVersion) >= 2
                          && flags & LOCKTIME_VERIFY_SEQUENCE;

        // Do not enforce sequence numbers as a relative lock time
        // unless we have been instructed to
        if (!fEnforceBIP68) {
            return std::make_pair(nMinHeight, nMinTime);
        }

        for (size_t txinIndex = 0; txinIndex < tx.vin.size(); txinIndex++) {
            const CTxIn& txin = tx.vin[txinIndex];

            // Sequence numbers with the most significant bit set are not
            // treated as relative lock-times, nor are they given any
            // consensus-enforced meaning at this point.
            if (txin.nSequence & CTxIn::SEQUENCE_LOCKTIME_DISABLE_FLAG) {
                // The height of this input is not relevant for sequence locks
                prevHeights[txinIndex] = 0;
                continue;
            }

            int nCoinHeight = prevHeights[txinIndex];

            if (txin.nSequence & CTxIn::SEQUENCE_LOCKTIME_TYPE_FLAG) {
                int64_t nCoinTime = block.GetAncestor(std::max(nCoinHeight-1, 0))->GetMedianTimePast();
                // NOTE: Subtract 1 to maintain nLockTime semantics
                // BIP 68 relative lock times have the semantics of calculating
                // the first block or time at which the transaction would be
                // valid. When calculating the effective block time or height
                // for the entire transaction, we switch to using the
                // semantics of nLockTime which is the last invalid block
                // time or height.  Thus we subtract 1 from the calculated
                // time or height.

                // Time-based relative lock-times are measured from the
                // smallest allowed timestamp of the block containing the
                // txout being spent, which is the median time past of the
                // block prior.
                nMinTime = std::max(nMinTime, nCoinTime + (int64_t)((txin.nSequence & CTxIn::SEQUENCE_LOCKTIME_MASK) << CTxIn::SEQUENCE_LOCKTIME_GRANULARITY) - 1);
            } else {
                nMinHeight = std::max(nMinHeight, nCoinHeight + (int)(txin.nSequence & CTxIn::SEQUENCE_LOCKTIME_MASK) - 1);
            }
        }

        return std::make_pair(nMinHeight, nMinTime);
        */
}

pub fn evaluate_sequence_locks(
        block:     &BlockIndex,
        lock_pair: (i32,i64)) -> bool {
    
    todo!();
        /*
            assert(block.pprev);
        int64_t nBlockTime = block.pprev->GetMedianTimePast();
        if (lockPair.first >= block.nHeight || lockPair.second >= nBlockTime)
            return false;

        return true;
        */
}

/**
  | Check if transaction is final per BIP
  | 68 sequence numbers and can be included
  | in a block.
  | 
  | Consensus critical. Takes as input
  | a list of heights at which tx's inputs
  | (in order) confirmed.
  |
  */
pub fn sequence_locks(
        tx:           &Transaction,
        flags:        i32,
        prev_heights: &mut Vec<i32>,
        block:        &BlockIndex) -> bool {
    
    todo!();
        /*
            return EvaluateSequenceLocks(block, CalculateSequenceLocks(tx, flags, prevHeights, block));
        */
}

/**
  | Count ECDSA signature operations the
  | old-fashioned (pre-0.6) way
  | 
  | 
  | -----------
  | @return
  | 
  | number of sigops this transaction's
  | outputs will produce when spent @see
  | CTransaction::FetchInputs
  |
  */
pub fn get_legacy_sig_op_count(tx: &Transaction) -> u32 {
    
    todo!();
        /*
            unsigned int nSigOps = 0;
        for (const auto& txin : tx.vin)
        {
            nSigOps += txin.scriptSig.GetSigOpCount(false);
        }
        for (const auto& txout : tx.vout)
        {
            nSigOps += txout.scriptPubKey.GetSigOpCount(false);
        }
        return nSigOps;
        */
}

/**
  | Count ECDSA signature operations in
  | pay-to-script-hash inputs.
  | 
  | -----------
  | @param[in] mapInputs
  | 
  | Map of previous transactions that have
  | outputs we're spending
  | 
  | -----------
  | @return
  | 
  | maximum number of sigops required to
  | validate this transaction's inputs
  | @see CTransaction::FetchInputs
  |
  */
pub fn get_p2sh_sig_op_count(
        tx:     &Transaction,
        inputs: &CoinsViewCache) -> u32 {
    
    todo!();
        /*
            if (tx.IsCoinBase())
            return 0;

        unsigned int nSigOps = 0;
        for (unsigned int i = 0; i < tx.vin.size(); i++)
        {
            const Coin& coin = inputs.AccessCoin(tx.vin[i].prevout);
            assert(!coin.IsSpent());
            const CTxOut &prevout = coin.out;
            if (prevout.scriptPubKey.IsPayToScriptHash())
                nSigOps += prevout.scriptPubKey.GetSigOpCount(tx.vin[i].scriptSig);
        }
        return nSigOps;
        */
}

/**
  | Compute total signature operation
  | cost of a transaction.
  | 
  | -----------
  | @param[in] tx
  | 
  | Transaction for which we are computing
  | the cost
  | ----------
  | @param[in] inputs
  | 
  | Map of previous transactions that have
  | outputs we're spending
  | ----------
  | @param[in] flags
  | 
  | Script verification flags
  | 
  | -----------
  | @return
  | 
  | Total signature operation cost of tx
  |
  */
pub fn get_transaction_sig_op_cost(
        tx:     &Transaction,
        inputs: &CoinsViewCache,
        flags:  u32) -> i64 {
    
    todo!();
        /*
            int64_t nSigOps = GetLegacySigOpCount(tx) * WITNESS_SCALE_FACTOR;

        if (tx.IsCoinBase())
            return nSigOps;

        if (flags & SCRIPT_VERIFY_P2SH) {
            nSigOps += GetP2SHSigOpCount(tx, inputs) * WITNESS_SCALE_FACTOR;
        }

        for (unsigned int i = 0; i < tx.vin.size(); i++)
        {
            const Coin& coin = inputs.AccessCoin(tx.vin[i].prevout);
            assert(!coin.IsSpent());
            const CTxOut &prevout = coin.out;
            nSigOps += CountWitnessSigOps(tx.vin[i].scriptSig, prevout.scriptPubKey, &tx.vin[i].scriptWitness, flags);
        }
        return nSigOps;
        */
}

/**
  | Check whether all inputs of this transaction
  | are valid (no double spends and amounts)
  | 
  | This does not modify the UTXO set. This
  | does not check scripts and sigs.
  | 
  | -----------
  | @param[out] txfee
  | 
  | Set to the transaction fee if successful.
  | 
  | Preconditions: tx.IsCoinBase() is
  | false.
  |
  */
pub fn check_tx_inputs(
    tx:             &Transaction,
    state:          &mut TxValidationState,
    inputs:         &CoinsViewCache,
    n_spend_height: i32,
    txfee:          &mut Amount) -> bool {
    
    todo!();
    /*
        // are the actual inputs available?
    if (!inputs.HaveInputs(tx)) {
        return state.Invalid(TxValidationResult::TX_MISSING_INPUTS, "bad-txns-inputs-missingorspent",
                         strprintf("%s: inputs missing/spent", __func__));
    }

    CAmount nValueIn = 0;
    for (unsigned int i = 0; i < tx.vin.size(); ++i) {
        const OutPoint &prevout = tx.vin[i].prevout;
        const Coin& coin = inputs.AccessCoin(prevout);
        assert(!coin.IsSpent());

        // If prev is coinbase, check that it's matured
        if (coin.IsCoinBase() && nSpendHeight - coin.nHeight < COINBASE_MATURITY) {
            return state.Invalid(TxValidationResult::TX_PREMATURE_SPEND, "bad-txns-premature-spend-of-coinbase",
                strprintf("tried to spend coinbase at depth %d", nSpendHeight - coin.nHeight));
        }

        // Check for negative or overflow input values
        nValueIn += coin.out.nValue;
        if (!MoneyRange(coin.out.nValue) || !MoneyRange(nValueIn)) {
            return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-inputvalues-outofrange");
        }
    }

    const CAmount value_out = tx.GetValueOut();
    if (nValueIn < value_out) {
        return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-in-belowout",
            strprintf("value in (%s) < value out (%s)", FormatMoney(nValueIn), FormatMoney(value_out)));
    }

    // Tally transaction fees
    const CAmount txfee_aux = nValueIn - value_out;
    if (!MoneyRange(txfee_aux)) {
        return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-fee-outofrange");
    }

    txfee = txfee_aux;
    return true;
    */
}

//-------------------------------------------[.cpp/bitcoin/src/consensus/tx_check.h]
//-------------------------------------------[.cpp/bitcoin/src/consensus/tx_check.cpp]

/**
  | Context-independent transaction
  | checking code that can be called outside
  | the bitcoin server and doesn't depend
  | on chain or mempool state. Transaction
  | verification code that does call server
  | functions or depend on server state
  | belongs in tx_verify.h/cpp instead.
  |
  */
pub fn check_transaction(
        tx:    &Transaction,
        state: &mut TxValidationState) -> bool {
    
    todo!();
        /*
            // Basic checks that don't depend on any context
        if (tx.vin.empty())
            return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-vin-empty");
        if (tx.vout.empty())
            return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-vout-empty");
        // Size limits (this doesn't take the witness into account, as that hasn't been checked for malleability)
        if (::GetSerializeSize(tx, PROTOCOL_VERSION | SERIALIZE_TRANSACTION_NO_WITNESS) * WITNESS_SCALE_FACTOR > MAX_BLOCK_WEIGHT)
            return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-oversize");

        // Check for negative or overflow output values (see CVE-2010-5139)
        CAmount nValueOut = 0;
        for (const auto& txout : tx.vout)
        {
            if (txout.nValue < 0)
                return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-vout-negative");
            if (txout.nValue > MAX_MONEY)
                return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-vout-toolarge");
            nValueOut += txout.nValue;
            if (!MoneyRange(nValueOut))
                return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-txouttotal-toolarge");
        }

        // Check for duplicate inputs (see CVE-2018-17144)
        // While consensus::CheckTxInputs does check if all inputs of a tx are available, and UpdateCoins marks all inputs
        // of a tx as spent, it does not check if the tx has duplicate inputs.
        // Failure to run this check will result in either a crash or an inflation bug, depending on the implementation of
        // the underlying coins database.
        std::set<OutPoint> vInOutPoints;
        for (const auto& txin : tx.vin) {
            if (!vInOutPoints.insert(txin.prevout).second)
                return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-inputs-duplicate");
        }

        if (tx.IsCoinBase())
        {
            if (tx.vin[0].scriptSig.size() < 2 || tx.vin[0].scriptSig.size() > 100)
                return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-cb-length");
        }
        else
        {
            for (const auto& txin : tx.vin)
                if (txin.prevout.IsNull())
                    return state.Invalid(TxValidationResult::TX_CONSENSUS, "bad-txns-prevout-null");
        }

        return true;
        */
}
