crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/miner.h]

pub const DEFAULT_PRINTPRIORITY: bool = false;

pub struct BlockTemplate
{
    block:                   Block,
    tx_fees:                 Vec<Amount>,
    tx_sig_ops_cost:         Vec<i64>,
    vch_coinbase_commitment: Vec<u8>,
}

/**
  | Container for tracking updates to ancestor
  | feerate as we include (parent) transactions
  | in a block
  |
  */
pub struct TxMemPoolModifiedEntry {
    iter:                         TxMemPoolTxIter,
    n_size_with_ancestors:        u64,
    n_mod_fees_with_ancestors:    Amount,
    n_sig_op_cost_with_ancestors: i64,
}

impl TxMemPoolModifiedEntry {

    pub fn new(entry: TxMemPoolTxIter) -> Self {
    
        todo!();

        /*
            iter = entry;
            nSizeWithAncestors = entry->GetSizeWithAncestors();
            nModFeesWithAncestors = entry->GetModFeesWithAncestors();
            nSigOpCostWithAncestors = entry->GetSigOpCostWithAncestors();
        */
    }
    
    pub fn get_modified_fee(&self) -> i64 {

        todo!();

        /*
        self.iter.get_modified_fee()
        */
    }
    
    pub fn get_size_with_ancestors(&self) -> u64 {
        
        todo!();

        /*
        self.n_size_with_ancestors
        */
    }
    
    pub fn get_mod_fees_with_ancestors(&self) -> Amount {
        
        todo!();

        /*
        self.n_mod_fees_with_ancestors
        */
    }
    
    pub fn get_tx_size(&self) -> usize {

        todo!();

        /*
        self.iter.get_tx_size()
        */
    }
    
    pub fn get_tx(&self) -> &Transaction {
        
        todo!();

        /*
        self.iter.get_tx()
        */
    }
}

/**
  | Comparator for TxMemPoolTxIter
  | objects.
  | 
  | It simply compares the internal memory
  | address of the CTxMemPoolEntry object
  | pointed to. This means it has no meaning,
  | and is only useful for using them as key
  | in other indexes.
  |
  */
pub struct CompareTxMemPoolIter {

}

impl CompareTxMemPoolIter {
    
    pub fn invoke(&self, 
        a: &TxMemPoolTxIter,
        b: &TxMemPoolTxIter) -> bool {
        
        todo!();
        /*
            return &(*a) < &(*b);
        */
    }
}

pub mod modifiedentry_iter {
    use super::*;

    pub type ResultType = TxMemPoolTxIter;

    pub fn invoke(entry: &TxMemPoolModifiedEntry) -> ResultType {
        
        todo!();
        /*
            return entry.iter;
        */
    }
}

/**
  | A comparator that sorts transactions based on
  | number of ancestors.
  |
  | This is sufficient to sort an ancestor package
  | in an order that is valid to appear in a block.
  */
pub struct CompareTxIterByAncestorCount {

}

impl CompareTxIterByAncestorCount {
    
    pub fn invoke(&self, 
        a: &TxMemPoolTxIter,
        b: &TxMemPoolTxIter) -> bool {
        
        todo!();
        /*
            if (a->GetCountWithAncestors() != b->GetCountWithAncestors())
                return a->GetCountWithAncestors() < b->GetCountWithAncestors();
            return CompareIteratorByHash()(a, b);
        */
    }
}

lazy_static!{
    /*
    typedef boost::multi_index_container<
        CTxMemPoolModifiedEntry,
        boost::multi_index::indexed_by<
            boost::multi_index::ordered_unique<
                modifiedentry_iter,
                CompareCTxMemPoolIter
            >,
            // sorted by modified ancestor fee rate
            boost::multi_index::ordered_non_unique<
                // Reuse same tag from CTxMemPool's similar index
                boost::multi_index::tag<ancestor_score>,
                boost::multi_index::identity<CTxMemPoolModifiedEntry>,
                CompareTxMemPoolEntryByAncestorFee
            >
        >
    > indexed_modified_transaction_set;

    typedef indexed_modified_transaction_set::nth_index<0>::type::iterator modtxiter;
    typedef indexed_modified_transaction_set::index<ancestor_score>::type::iterator modtxscoreiter;
    */
}

pub type IndexedModifiedTransactionSet = Broken;

///------------------------
pub struct UpdateForParentInclusion {
    iter: TxMemPoolTxIter,
}

impl UpdateForParentInclusion {

    pub fn new(it: TxMemPoolTxIter) -> Self {
    
        todo!();
        /*
        : iter(it),

        
        */
    }
    
    pub fn invoke(&mut self, e: &mut TxMemPoolModifiedEntry)  {
        
        todo!();
        /*
            e.nModFeesWithAncestors -= iter->GetFee();
            e.nSizeWithAncestors -= iter->GetTxSize();
            e.nSigOpCostWithAncestors -= iter->GetSigOpCost();
        */
    }
}

/**
  | Generate a new block, without valid
  | proof-of-work
  |
  */
pub struct BlockAssembler {

    /**
      | The constructed block template
      |
      */
    pblocktemplate:       Box<BlockTemplate>,

    /**
      | Configuration parameters for the block
      | size
      |
      */
    include_witness:      bool,

    n_block_max_weight:   u32,
    block_min_fee_rate:   FeeRate,

    /**
      | Information on the current status of
      | the block
      |
      */
    n_block_weight:       u64,

    n_block_tx:           u64,
    n_block_sig_ops_cost: u64,
    n_fees:               Amount,
    in_block:             TxMemPoolSetEntries,

    /**
      | Chain context for the block
      |
      */
    n_height:             i32,

    n_lock_time_cutoff:   i64,
    chainparams:          Arc<ChainParams>,
    mempool:              Arc<TxMemPool>,
    chainstate:           Arc<Mutex<ChainState>>,
}

pub mod block_assembler {

    use super::*;

    pub struct Options {
        n_block_max_weight: usize,
        block_min_fee_rate: FeeRate,
    }

    impl Default for Options {
        
        fn default() -> Self {
        
            todo!();
            /*


                blockMinFeeRate = CFeeRate(DEFAULT_BLOCK_MIN_TX_FEE);
            nBlockMaxWeight = DEFAULT_BLOCK_MAX_WEIGHT;
            */
        }
    }

    lazy_static!{
        /*
        static std::optional<int64_t> m_last_block_num_txs{};
            static std::optional<int64_t> m_last_block_weight{};
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/miner.cpp]

pub fn update_time(
        pblock:           *mut BlockHeader,
        consensus_params: &ChainConsensusParams,
        pindex_prev:      *const BlockIndex) -> i64 {
    
    todo!();
        /*
            int64_t nOldTime = pblock->nTime;
        int64_t nNewTime = std::max(pindexPrev->GetMedianTimePast()+1, GetAdjustedTime());

        if (nOldTime < nNewTime)
            pblock->nTime = nNewTime;

        // Updating time can change work required on testnet:
        if (consensusParams.fPowAllowMinDifficultyBlocks)
            pblock->nBits = GetNextWorkRequired(pindexPrev, pblock, consensusParams);

        return nNewTime - nOldTime;
        */
}

/**
  | Update an old GenerateCoinbaseCommitment
  | from CreateNewBlock after the block
  | txs have changed
  |
  */
pub fn regenerate_commitments(
    block:    &mut Block,
    chainman: &mut ChainstateManager)  {
    
    todo!();
        /*
            CMutableTransaction tx{*block.vtx.at(0)};
        tx.vout.erase(tx.vout.begin() + GetWitnessCommitmentIndex(block));
        block.vtx.at(0) = MakeTransactionRef(tx);

        CBlockIndex* prev_block = 
    [&]() { LOCK(::cs_main);  return chainman.m_blockman.LookupBlockIndex(block.hashPrevBlock) }()
    ;
        GenerateCoinbaseCommitment(block, prev_block, Params().GetConsensus());

        block.hashMerkleRoot = BlockMerkleRoot(block);
        */
}

pub fn default_options() -> block_assembler::Options {
    
    todo!();
        /*
            // Block resource limits
        // If -blockmaxweight is not given, limit to DEFAULT_BLOCK_MAX_WEIGHT
        BlockAssembler::Options options;
        options.nBlockMaxWeight = gArgs.GetIntArg("-blockmaxweight", DEFAULT_BLOCK_MAX_WEIGHT);
        if (gArgs.IsArgSet("-blockmintxfee")) {
            std::optional<CAmount> parsed = ParseMoney(gArgs.GetArg("-blockmintxfee", ""));
            options.blockMinFeeRate = CFeeRate{parsed.value_or(DEFAULT_BLOCK_MIN_TX_FEE)};
        } else {
            options.blockMinFeeRate = CFeeRate{DEFAULT_BLOCK_MIN_TX_FEE};
        }
        return options;
        */
}

impl BlockAssembler {

    pub fn new_with_options(
        chainstate: &mut ChainState,
        mempool:    &TxMemPool,
        params:     &ChainParams,
        options:    &block_assembler::Options) -> Self {
    
        todo!();
        /*
        : chainparams(params),
        : mempool(mempool),
        : chainstate(chainstate),

            blockMinFeeRate = options.blockMinFeeRate;
        // Limit weight to between 4K and MAX_BLOCK_WEIGHT-4K for sanity:
        nBlockMaxWeight = std::max<size_t>(4000, std::min<size_t>(MAX_BLOCK_WEIGHT - 4000, options.nBlockMaxWeight));
        */
    }
    
    pub fn new(
        chainstate: &mut ChainState,
        mempool:    &TxMemPool,
        params:     &ChainParams) -> Self {
    
        todo!();
        /*
        : block_assembler(chainstate, mempool, params, DefaultOptions()),

        
        */
    }
    
    /**
      | Clear the block's state and prepare
      | for assembling a new block
      |
      */
    pub fn reset_block(&mut self)  {
        
        todo!();
        /*
            inBlock.clear();

        // Reserve space for coinbase tx
        nBlockWeight = 4000;
        nBlockSigOpsCost = 400;
        fIncludeWitness = false;

        // These counters do not include coinbase tx
        nBlockTx = 0;
        nFees = 0;
        */
    }
    
    /**
      | Construct a new block template with
      | coinbase to scriptPubKeyIn
      |
      */
    pub fn create_new_block(&mut self, script_pub_key_in: &Script) -> Box<BlockTemplate> {
        
        todo!();
        /*
            int64_t nTimeStart = GetTimeMicros();

        resetBlock();

        pblocktemplate.reset(new CBlockTemplate());

        if(!pblocktemplate.get())
            return nullptr;
        CBlock* const pblock = &pblocktemplate->block; // pointer for convenience

        // Add dummy coinbase tx as first transaction
        pblock->vtx.emplace_back();
        pblocktemplate->vTxFees.push_back(-1); // updated at end
        pblocktemplate->vTxSigOpsCost.push_back(-1); // updated at end

        LOCK2(cs_main, m_mempool.cs);
        CBlockIndex* pindexPrev = m_chainstate.m_chain.Tip();
        assert(pindexPrev != nullptr);
        nHeight = pindexPrev->nHeight + 1;

        pblock->nVersion = g_versionbitscache.ComputeBlockVersion(pindexPrev, chainparams.GetConsensus());
        // -regtest only: allow overriding block.nVersion with
        // -blockversion=N to test forking scenarios
        if (chainparams.MineBlocksOnDemand())
            pblock->nVersion = gArgs.GetIntArg("-blockversion", pblock->nVersion);

        pblock->nTime = GetAdjustedTime();
        const int64_t nMedianTimePast = pindexPrev->GetMedianTimePast();

        nLockTimeCutoff = (STANDARD_LOCKTIME_VERIFY_FLAGS & LOCKTIME_MEDIAN_TIME_PAST)
                           ? nMedianTimePast
                           : pblock->GetBlockTime();

        // Decide whether to include witness transactions
        // This is only needed in case the witness softfork activation is reverted
        // (which would require a very deep reorganization).
        // Note that the mempool would accept transactions with witness data before
        // the deployment is active, but we would only ever mine blocks after activation
        // unless there is a massive block reorganization with the witness softfork
        // not activated.
        // TODO: replace this with a call to main to assess validity of a mempool
        // transaction (which in most cases can be a no-op).
        fIncludeWitness = DeploymentActiveAfter(pindexPrev, chainparams.GetConsensus(), consensus::DEPLOYMENT_SEGWIT);

        int nPackagesSelected = 0;
        int nDescendantsUpdated = 0;
        addPackageTxs(nPackagesSelected, nDescendantsUpdated);

        int64_t nTime1 = GetTimeMicros();

        m_last_block_num_txs = nBlockTx;
        m_last_block_weight = nBlockWeight;

        // Create coinbase transaction.
        CMutableTransaction coinbaseTx;
        coinbaseTx.vin.resize(1);
        coinbaseTx.vin[0].prevout.SetNull();
        coinbaseTx.vout.resize(1);
        coinbaseTx.vout[0].scriptPubKey = scriptPubKeyIn;
        coinbaseTx.vout[0].nValue = nFees + GetBlockSubsidy(nHeight, chainparams.GetConsensus());
        coinbaseTx.vin[0].scriptSig = CScript() << nHeight << OP_0;
        pblock->vtx[0] = MakeTransactionRef(std::move(coinbaseTx));
        pblocktemplate->vchCoinbaseCommitment = GenerateCoinbaseCommitment(*pblock, pindexPrev, chainparams.GetConsensus());
        pblocktemplate->vTxFees[0] = -nFees;

        LogPrintf("CreateNewBlock(): block weight: %u txs: %u fees: %ld sigops %d\n", GetBlockWeight(*pblock), nBlockTx, nFees, nBlockSigOpsCost);

        // Fill in header
        pblock->hashPrevBlock  = pindexPrev->GetBlockHash();
        UpdateTime(pblock, chainparams.GetConsensus(), pindexPrev);
        pblock->nBits          = GetNextWorkRequired(pindexPrev, pblock, chainparams.GetConsensus());
        pblock->nNonce         = 0;
        pblocktemplate->vTxSigOpsCost[0] = WITNESS_SCALE_FACTOR * GetLegacySigOpCount(*pblock->vtx[0]);

        BlockValidationState state;
        if (!TestBlockValidity(state, chainparams, m_chainstate, *pblock, pindexPrev, false, false)) {
            throw std::runtime_error(strprintf("%s: TestBlockValidity failed: %s", __func__, state.ToString()));
        }
        int64_t nTime2 = GetTimeMicros();

        LogPrint(BCLog::BENCH, "CreateNewBlock() packages: %.2fms (%d packages, %d updated descendants), validity: %.2fms (total %.2fms)\n", 0.001 * (nTime1 - nTimeStart), nPackagesSelected, nDescendantsUpdated, 0.001 * (nTime2 - nTime1), 0.001 * (nTime2 - nTimeStart));

        return std::move(pblocktemplate);
        */
    }
    
    /**
      | Remove confirmed (inBlock) entries
      | from given set
      |
      */
    pub fn only_unconfirmed(&mut self, test_set: &mut TxMemPoolSetEntries)  {
        
        todo!();
        /*
            for (CTxMemPool::setEntries::iterator iit = testSet.begin(); iit != testSet.end(); ) {
            // Only test txs not already in the block
            if (inBlock.count(*iit)) {
                testSet.erase(iit++);
            }
            else {
                iit++;
            }
        }
        */
    }
    
    /**
      | Test if a new package would "fit" in the
      | block
      |
      */
    pub fn test_package(&self, 
        package_size:         u64,
        package_sig_ops_cost: i64) -> bool {
        
        todo!();
        /*
            // TODO: switch to weight-based accounting for packages instead of vsize-based accounting.
        if (nBlockWeight + WITNESS_SCALE_FACTOR * packageSize >= nBlockMaxWeight)
            return false;
        if (nBlockSigOpsCost + packageSigOpsCost >= MAX_BLOCK_SIGOPS_COST)
            return false;
        return true;
        */
    }

    /**
      | Perform checks on each transaction
      | in a package: locktime, premature-witness,
      | serialized size (if necessary)
      | 
      | These checks should always succeed,
      | and they're here only as an extra check
      | in case of suboptimal node configuration
      |
      -----------------------------
      | Perform transaction-level checks before adding
      | to block:
      |
      | - transaction finality (locktime)
      |
      | - premature witness (in case segwit
      |   transactions are added to mempool before
      |   segwit activation)
      */
    pub fn test_package_transactions(&self, package: &TxMemPoolSetEntries) -> bool {
        
        todo!();
        /*
            for (TxMemPoolTxIter it : package) {
            if (!IsFinalTx(it->GetTx(), nHeight, nLockTimeCutoff))
                return false;
            if (!fIncludeWitness && it->GetTx().HasWitness())
                return false;
        }
        return true;
        */
    }
    
    /**
      | Add a tx to the block
      |
      */
    pub fn add_to_block(&mut self, iter: TxMemPoolTxIter)  {
        
        todo!();
        /*
            pblocktemplate->block.vtx.emplace_back(iter->GetSharedTx());
        pblocktemplate->vTxFees.push_back(iter->GetFee());
        pblocktemplate->vTxSigOpsCost.push_back(iter->GetSigOpCost());
        nBlockWeight += iter->GetTxWeight();
        ++nBlockTx;
        nBlockSigOpsCost += iter->GetSigOpCost();
        nFees += iter->GetFee();
        inBlock.insert(iter);

        bool fPrintPriority = gArgs.GetBoolArg("-printpriority", DEFAULT_PRINTPRIORITY);
        if (fPrintPriority) {
            LogPrintf("fee rate %s txid %s\n",
                      CFeeRate(iter->GetModifiedFee(), iter->GetTxSize()).ToString(),
                      iter->GetTx().GetHash().ToString());
        }
        */
    }
    
    /**
      | Add descendants of given transactions
      | to mapModifiedTx with ancestor state
      | updated assuming given transactions
      | are inBlock. Returns number of updated
      | descendants.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mempool.cs)]
    pub fn update_packages_for_added(&mut self, 
        already_added:   &TxMemPoolSetEntries,
        map_modified_tx: &mut IndexedModifiedTransactionSet) -> i32 {
        
        todo!();
        /*
            int nDescendantsUpdated = 0;
        for (TxMemPoolTxIter it : alreadyAdded) {
            CTxMemPool::setEntries descendants;
            m_mempool.CalculateDescendants(it, descendants);
            // Insert all descendants (not yet in block) into the modified set
            for (TxMemPoolTxIter desc : descendants) {
                if (alreadyAdded.count(desc))
                    continue;
                ++nDescendantsUpdated;
                modtxiter mit = mapModifiedTx.find(desc);
                if (mit == mapModifiedTx.end()) {
                    CTxMemPoolModifiedEntry modEntry(desc);
                    modEntry.nSizeWithAncestors -= it->GetTxSize();
                    modEntry.nModFeesWithAncestors -= it->GetModifiedFee();
                    modEntry.nSigOpCostWithAncestors -= it->GetSigOpCost();
                    mapModifiedTx.insert(modEntry);
                } else {
                    mapModifiedTx.modify(mit, update_for_parent_inclusion(it));
                }
            }
        }
        return nDescendantsUpdated;
        */
    }

    /**
      | Return true if given transaction from
      | mapTx has already been evaluated, or
      | if the transaction's cached data in
      | mapTx is incorrect.
      |
      | Skip entries in mapTx that are already in
      | a block or are present in mapModifiedTx (which
      | implies that the mapTx ancestor state is stale
      | due to ancestor inclusion in the block)
      |
      | Also skip transactions that we've already
      | failed to add. This can happen if we consider
      | a transaction in mapModifiedTx and it fails: we
      | can then potentially consider it again while
      | walking mapTx.  It's currently guaranteed to
      | fail again, but as a belt-and-suspenders check
      | we put it in failedTx and avoid re-evaluation,
      | since the re-evaluation would be using cached
      | size/sigops/fee values that are not actually
      | correct.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mempool.cs)]
    pub fn skip_map_tx_entry(&mut self, 
        it:              TxMemPoolTxIter,
        map_modified_tx: &mut IndexedModifiedTransactionSet,
        failed_tx:       &mut TxMemPoolSetEntries) -> bool {
        
        todo!();
        /*
            assert(it != m_mempool.mapTx.end());
        return mapModifiedTx.count(it) || inBlock.count(it) || failedTx.count(it);
        */
    }
    
    /**
      | Sort the package in an order that is valid
      | to appear in a block
      |
      */
    pub fn sort_for_block(&mut self, 
        package:        &TxMemPoolSetEntries,
        sorted_entries: &mut Vec<TxMemPoolTxIter>)  {
        
        todo!();
        /*
            // Sort package by ancestor count
        // If a transaction A depends on transaction B, then A's ancestor count
        // must be greater than B's.  So this is sufficient to validly order the
        // transactions for block inclusion.
        sortedEntries.clear();
        sortedEntries.insert(sortedEntries.begin(), package.begin(), package.end());
        std::sort(sortedEntries.begin(), sortedEntries.end(), CompareTxIterByAncestorCount());
        */
    }

    /**
      | Add transactions based on feerate including
      | unconfirmed ancestors
      | 
      | Increments nPackagesSelected / nDescendantsUpdated
      | with corresponding statistics from
      | the package selection (for logging
      | statistics).
      |
      ------------------------------
      | This transaction selection algorithm orders the
      | mempool based on feerate of a transaction
      | including all unconfirmed ancestors.
      |
      | Since we don't remove transactions from the
      | mempool as we select them for block inclusion,
      | we need an alternate method of updating the
      | feerate of a transaction with its
      | not-yet-selected ancestors as we go.
      |
      | This is accomplished by walking the in-mempool
      | descendants of selected transactions and
      | storing a temporary modified state in
      | mapModifiedTxs.
      |
      | Each time through the loop, we compare the best
      | transaction in mapModifiedTxs with the next
      | transaction in the mempool to decide what
      | transaction package to work on next.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mempool.cs)]
    pub fn add_package_txs(&mut self, 
        n_packages_selected:   &mut i32,
        n_descendants_updated: &mut i32)  {
        
        todo!();
        /*
            // mapModifiedTx will store sorted packages after they are modified
        // because some of their txs are already in the block
        indexed_modified_transaction_set mapModifiedTx;
        // Keep track of entries that failed inclusion, to avoid duplicate work
        CTxMemPool::setEntries failedTx;

        // Start by adding all descendants of previously added txs to mapModifiedTx
        // and modifying them for their already included ancestors
        UpdatePackagesForAdded(inBlock, mapModifiedTx);

        CTxMemPool::indexed_transaction_set::index<ancestor_score>::type::iterator mi = m_mempool.mapTx.get<ancestor_score>().begin();
        TxMemPoolTxIter iter;

        // Limit the number of attempts to add transactions to the block when it is
        // close to full; this is just a simple heuristic to finish quickly if the
        // mempool has a lot of entries.
        const int64_t MAX_CONSECUTIVE_FAILURES = 1000;
        int64_t nConsecutiveFailed = 0;

        while (mi != m_mempool.mapTx.get<ancestor_score>().end() || !mapModifiedTx.empty()) {
            // First try to find a new transaction in mapTx to evaluate.
            if (mi != m_mempool.mapTx.get<ancestor_score>().end() &&
                SkipMapTxEntry(m_mempool.mapTx.project<0>(mi), mapModifiedTx, failedTx)) {
                ++mi;
                continue;
            }

            // Now that mi is not stale, determine which transaction to evaluate:
            // the next entry from mapTx, or the best from mapModifiedTx?
            bool fUsingModified = false;

            modtxscoreiter modit = mapModifiedTx.get<ancestor_score>().begin();
            if (mi == m_mempool.mapTx.get<ancestor_score>().end()) {
                // We're out of entries in mapTx; use the entry from mapModifiedTx
                iter = modit->iter;
                fUsingModified = true;
            } else {
                // Try to compare the mapTx entry to the mapModifiedTx entry
                iter = m_mempool.mapTx.project<0>(mi);
                if (modit != mapModifiedTx.get<ancestor_score>().end() &&
                        CompareTxMemPoolEntryByAncestorFee()(*modit, CTxMemPoolModifiedEntry(iter))) {
                    // The best entry in mapModifiedTx has higher score
                    // than the one from mapTx.
                    // Switch which transaction (package) to consider
                    iter = modit->iter;
                    fUsingModified = true;
                } else {
                    // Either no entry in mapModifiedTx, or it's worse than mapTx.
                    // Increment mi for the next loop iteration.
                    ++mi;
                }
            }

            // We skip mapTx entries that are inBlock, and mapModifiedTx shouldn't
            // contain anything that is inBlock.
            assert(!inBlock.count(iter));

            uint64_t packageSize = iter->GetSizeWithAncestors();
            CAmount packageFees = iter->GetModFeesWithAncestors();
            int64_t packageSigOpsCost = iter->GetSigOpCostWithAncestors();
            if (fUsingModified) {
                packageSize = modit->nSizeWithAncestors;
                packageFees = modit->nModFeesWithAncestors;
                packageSigOpsCost = modit->nSigOpCostWithAncestors;
            }

            if (packageFees < blockMinFeeRate.GetFee(packageSize)) {
                // Everything else we might consider has a lower fee rate
                return;
            }

            if (!TestPackage(packageSize, packageSigOpsCost)) {
                if (fUsingModified) {
                    // Since we always look at the best entry in mapModifiedTx,
                    // we must erase failed entries so that we can consider the
                    // next best entry on the next loop iteration
                    mapModifiedTx.get<ancestor_score>().erase(modit);
                    failedTx.insert(iter);
                }

                ++nConsecutiveFailed;

                if (nConsecutiveFailed > MAX_CONSECUTIVE_FAILURES && nBlockWeight >
                        nBlockMaxWeight - 4000) {
                    // Give up if we're close to full and haven't succeeded in a while
                    break;
                }
                continue;
            }

            CTxMemPool::setEntries ancestors;
            uint64_t nNoLimit = std::numeric_limits<uint64_t>::max();
            std::string dummy;
            m_mempool.CalculateMemPoolAncestors(*iter, ancestors, nNoLimit, nNoLimit, nNoLimit, nNoLimit, dummy, false);

            onlyUnconfirmed(ancestors);
            ancestors.insert(iter);

            // Test if all tx's are Final
            if (!TestPackageTransactions(ancestors)) {
                if (fUsingModified) {
                    mapModifiedTx.get<ancestor_score>().erase(modit);
                    failedTx.insert(iter);
                }
                continue;
            }

            // This transaction will make it in; reset the failed counter.
            nConsecutiveFailed = 0;

            // Package can be added. Sort the entries in a valid order.
            std::vector<TxMemPoolTxIter> sortedEntries;
            SortForBlock(ancestors, sortedEntries);

            for (size_t i=0; i<sortedEntries.size(); ++i) {
                AddToBlock(sortedEntries[i]);
                // Erase from the modified set, if present
                mapModifiedTx.erase(sortedEntries[i]);
            }

            ++nPackagesSelected;

            // Update transactions that depend on each of these
            nDescendantsUpdated += UpdatePackagesForAdded(ancestors, mapModifiedTx);
        }
        */
    }
}

/**
  | Modify the extranonce in a block
  |
  */
pub fn increment_extra_nonce(
        pblock:        *mut Block,
        pindex_prev:   *const BlockIndex,
        n_extra_nonce: &mut u32)  {
    
    todo!();
        /*
        // Update nExtraNonce
        static uint256 hashPrevBlock;
        if (hashPrevBlock != pblock->hashPrevBlock)
        {
            nExtraNonce = 0;
            hashPrevBlock = pblock->hashPrevBlock;
        }
        ++nExtraNonce;
        unsigned int nHeight = pindexPrev->nHeight+1; // Height first in coinbase required for block.version=2
        CMutableTransaction txCoinbase(*pblock->vtx[0]);
        txCoinbase.vin[0].scriptSig = (CScript() << nHeight << CScriptNum(nExtraNonce));
        assert(txCoinbase.vin[0].scriptSig.size() <= 100);

        pblock->vtx[0] = MakeTransactionRef(std::move(txCoinbase));
        pblock->hashMerkleRoot = BlockMerkleRoot(*pblock);
        */
}
