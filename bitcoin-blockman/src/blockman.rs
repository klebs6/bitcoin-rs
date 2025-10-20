// ---------------- [ File: bitcoin-blockman/src/blockman.rs ]
crate::ix!();

/**
  | Maintains a tree of blocks (stored in
  | `m_block_index`) which is consulted
  | to determine where the most-work tip
  | is.
  | 
  | This data is used mostly in `ChainState`
  | - information about, e.g., candidate
  | tips is not maintained here.
  |
  */
pub struct BlockManager {

    /**
      | In order to efficiently track invalidity
      | of headers, we keep the set of blocks
      | which we tried to connect and found to
      | be invalid here (ie which were set to
      | BLOCK_FAILED_VALID since the last
      | restart). We can then walk this set and
      | check if a new header is a descendant
      | of something in this set, preventing
      | us from having to walk m_block_index
      | when we try to connect a bad block and
      | fail.
      | 
      | While this is more complicated than
      | marking everything which descends
      | from an invalid block as invalid at the
      | time we discover it to be invalid, doing
      | so would require walking all of m_block_index
      | to find all descendants. Since this
      | case should be very rare, keeping track
      | of all BLOCK_FAILED_VALID blocks in a set
      | should be just fine and work just as well.
      | 
      | Because we already walk m_block_index
      | in height-order at startup, we go ahead
      | and mark descendants of invalid blocks
      | as FAILED_CHILD at that time, instead
      | of putting things in this set.
      |
      */
    failed_blocks:   HashSet<*mut BlockIndex>,

    /**
      | All pairs A->B, where A (or one of its
      | ancestors) misses transactions, but
      | B has transactions.
      | 
      | Pruned nodes may have entries where
      | B is missing data.
      |
      */
    blocks_unlinked: MultiMap<*mut BlockIndex,*mut BlockIndex>,

    //TODO: #[GUARDED_BY(cs_main)]
    inner: BlockManInner,
}

pub struct BlockManInner {
    block_index:     BlockMap,
    block_tree_db:   Box<BlockTreeDB>,
}

impl Drop for BlockManager {
    fn drop(&mut self) {
        todo!();
        /*
            Unload();
        */
    }
}

impl BlockManager {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn add_to_block_index(&mut self, block: &BlockHeader) -> *mut BlockIndex {
        
        todo!();
        /*
            AssertLockHeld(cs_main);

        // Check for duplicate
        uint256 hash = block.GetHash();
        BlockMap::iterator it = m_block_index.find(hash);
        if (it != m_block_index.end())
            return it->second;

        // Construct new block index object
        CBlockIndex* pindexNew = new CBlockIndex(block);
        // We assign the sequence id to blocks only when the full data is available,
        // to avoid miners withholding blocks but broadcasting headers, to get a
        // competitive advantage.
        pindexNew->nSequenceId = 0;
        BlockMap::iterator mi = m_block_index.insert(std::make_pair(hash, pindexNew)).first;
        pindexNew->phashBlock = &((*mi).first);
        BlockMap::iterator miPrev = m_block_index.find(block.hashPrevBlock);
        if (miPrev != m_block_index.end())
        {
            pindexNew->pprev = (*miPrev).second;
            pindexNew->nHeight = pindexNew->pprev->nHeight + 1;
            pindexNew->BuildSkip();
        }
        pindexNew->nTimeMax = (pindexNew->pprev ? std::max(pindexNew->pprev->nTimeMax, pindexNew->nTime) : pindexNew->nTime);
        pindexNew->nChainWork = (pindexNew->pprev ? pindexNew->pprev->nChainWork : 0) + GetBlockProof(*pindexNew);
        pindexNew->RaiseValidity(BLOCK_VALID_TREE);
        if (pindexBestHeader == nullptr || pindexBestHeader->nChainWork < pindexNew->nChainWork)
            pindexBestHeader = pindexNew;

        setDirtyBlockIndex.insert(pindexNew);

        return pindexNew;
        */
    }
    
    /**
      | Returns last CBlockIndex* that is a
      | checkpoint
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn get_last_checkpoint(&mut self, data: &CheckpointData) -> *mut BlockIndex {
        
        todo!();
        /*
            const MapCheckpoints& checkpoints = data.mapCheckpoints;

        for (const MapCheckpoints::value_type& i : reverse_iterate(checkpoints))
        {
            const uint256& hash = i.second;
            CBlockIndex* pindex = LookupBlockIndex(hash);
            if (pindex) {
                return pindex;
            }
        }
        return nullptr;
        */
    }
    
    /**
      | If a block header hasn't already been
      | seen, call CheckBlockHeader on it,
      | ensure that it doesn't descend from
      | an invalid block, and then add it to m_block_index.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn accept_block_header(&mut self, 
        block:       &BlockHeader,
        state:       &mut BlockValidationState,
        chainparams: &ChainParams,
        ppindex:     *mut *mut BlockIndex) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        // Check for duplicate
        uint256 hash = block.GetHash();
        BlockMap::iterator miSelf = m_block_index.find(hash);
        if (hash != chainparams.GetConsensus().hashGenesisBlock) {
            if (miSelf != m_block_index.end()) {
                // Block header is already known.
                CBlockIndex* pindex = miSelf->second;
                if (ppindex)
                    *ppindex = pindex;
                if (pindex->nStatus & BLOCK_FAILED_MASK) {
                    LogPrint(LogFlags::VALIDATION, "%s: block %s is marked invalid\n", __func__, hash.ToString());
                    return state.Invalid(BlockValidationResult::BLOCK_CACHED_INVALID, "duplicate");
                }
                return true;
            }

            if (!CheckBlockHeader(block, state, chainparams.GetConsensus())) {
                LogPrint(LogFlags::VALIDATION, "%s: consensus::CheckBlockHeader: %s, %s\n", __func__, hash.ToString(), state.ToString());
                return false;
            }

            // Get prev block index
            CBlockIndex* pindexPrev = nullptr;
            BlockMap::iterator mi = m_block_index.find(block.hashPrevBlock);
            if (mi == m_block_index.end()) {
                LogPrint(LogFlags::VALIDATION, "%s: %s prev block not found\n", __func__, hash.ToString());
                return state.Invalid(BlockValidationResult::BLOCK_MISSING_PREV, "prev-blk-not-found");
            }
            pindexPrev = (*mi).second;
            if (pindexPrev->nStatus & BLOCK_FAILED_MASK) {
                LogPrint(LogFlags::VALIDATION, "%s: %s prev block invalid\n", __func__, hash.ToString());
                return state.Invalid(BlockValidationResult::BLOCK_INVALID_PREV, "bad-prevblk");
            }
            if (!ContextualCheckBlockHeader(block, state, *this, chainparams, pindexPrev, GetAdjustedTime())) {
                LogPrint(LogFlags::VALIDATION, "%s: consensus::ContextualCheckBlockHeader: %s, %s\n", __func__, hash.ToString(), state.ToString());
                return false;
            }

            /* Determine if this block descends from any block which has been found
             * invalid (m_failed_blocks), then mark pindexPrev and any blocks between
             * them as failed. For example:
             *
             *                D3
             *              /
             *      B2 - C2
             *    /         \
             *  A             D2 - E2 - F2
             *    \
             *      B1 - C1 - D1 - E1
             *
             * In the case that we attempted to reorg from E1 to F2, only to find
             * C2 to be invalid, we would mark D2, E2, and F2 as BLOCK_FAILED_CHILD
             * but NOT D3 (it was not in any of our candidate sets at the time).
             *
             * In any case D3 will also be marked as BLOCK_FAILED_CHILD at restart
             * in LoadBlockIndex.
             */
            if (!pindexPrev->IsValid(BLOCK_VALID_SCRIPTS)) {
                // The above does not mean "invalid": it checks if the previous block
                // hasn't been validated up to BLOCK_VALID_SCRIPTS. This is a performance
                // optimization, in the common case of adding a new block to the tip,
                // we don't need to iterate over the failed blocks list.
                for (const CBlockIndex* failedit : m_failed_blocks) {
                    if (pindexPrev->GetAncestor(failedit->nHeight) == failedit) {
                        assert(failedit->nStatus & BLOCK_FAILED_VALID);
                        CBlockIndex* invalid_walk = pindexPrev;
                        while (invalid_walk != failedit) {
                            invalid_walk->nStatus |= BLOCK_FAILED_CHILD;
                            setDirtyBlockIndex.insert(invalid_walk);
                            invalid_walk = invalid_walk->pprev;
                        }
                        LogPrint(LogFlags::VALIDATION, "%s: %s prev block invalid\n", __func__, hash.ToString());
                        return state.Invalid(BlockValidationResult::BLOCK_INVALID_PREV, "bad-prevblk");
                    }
                }
            }
        }
        CBlockIndex* pindex = AddToBlockIndex(block);

        if (ppindex)
            *ppindex = pindex;

        return true;
        */
    }

    /**
      | Prune block and undo files (blk???.dat
      | and undo???.dat) so that the disk space
      | used is less than a user-defined target.
      | 
      | The user sets the target (in MB) on the
      | command line or in config file. This
      | will be run on startup and whenever new
      | space is allocated in a block or undo
      | file, staying below the target. Changing
      | back to unpruned requires a reindex
      | (which in this case means the blockchain
      | must be re-downloaded.)
      | 
      | Pruning functions are called from FlushStateToDisk
      | when the global fCheckForPruning flag
      | has been set.
      | 
      | Block and undo files are deleted in lock-step
      | (when blk00003.dat is deleted, so is
      | rev00003.dat.)
      | 
      | Pruning cannot take place until the
      | longest chain is at least a certain length
      | (100000 on mainnet, 1000 on testnet,
      | 1000 on regtest).
      | 
      | Pruning will never delete a block within
      | a defined distance (currently 288)
      | from the active chain's tip.
      | 
      | The block index is updated by unsetting
      | HAVE_DATA and HAVE_UNDO for any blocks
      | that were stored in the deleted files.
      | 
      | A db flag records the fact that at least
      | some block files have been pruned.
      | 
      | -----------
      | @param[out] setFilesToPrune
      | 
      | The set of file indices that can be unlinked
      | will be returned
      |
      */
    pub fn find_files_to_prune(&mut self, 
        set_files_to_prune:   &mut HashSet<i32>,
        n_prune_after_height: u64,
        chain_tip_height:     i32,
        prune_height:         i32,
        is_ibd:               bool)  {

        todo!();

        /*

        lock2!(cs_main, cs_last_block_file);

        if chain_tip_height < 0 || n_prune_target == 0 {
            return;
        }

        if chain_tip_height as u64 <= n_prune_after_height {
            return;
        }
        
        unsigned int nLastBlockWeCanPrune = std::min(prune_height, chain_tip_height - static_cast<int>(MIN_BLOCKS_TO_KEEP));
        uint64_t nCurrentUsage = CalculateCurrentUsage();
        // We don't check to prune until after we've allocated new space for files
        // So we should leave a buffer under our target to account for another allocation
        // before the next pruning.
        uint64_t nBuffer = BLOCKFILE_CHUNK_SIZE + UNDOFILE_CHUNK_SIZE;
        uint64_t nBytesToPrune;
        int count = 0;

        if (nCurrentUsage + nBuffer >= nPruneTarget) {
            // On a prune event, the chainstate DB is flushed.
            // To avoid excessive prune events negating the benefit of high dbcache
            // values, we should not prune too rapidly.
            // So when pruning in IBD, increase the buffer a bit to avoid a re-prune too soon.
            if (is_ibd) {
                // Since this is only relevant during IBD, we use a fixed 10%
                nBuffer += nPruneTarget / 10;
            }

            for (int fileNumber = 0; fileNumber < nLastBlockFile; fileNumber++) {
                nBytesToPrune = vinfoBlockFile[fileNumber].nSize + vinfoBlockFile[fileNumber].nUndoSize;

                if (vinfoBlockFile[fileNumber].nSize == 0) {
                    continue;
                }

                if (nCurrentUsage + nBuffer < nPruneTarget) { // are we below our target?
                    break;
                }

                // don't prune files that could have a block within MIN_BLOCKS_TO_KEEP of the main chain's tip but keep scanning
                if (vinfoBlockFile[fileNumber].nHeightLast > nLastBlockWeCanPrune) {
                    continue;
                }

                PruneOneBlockFile(fileNumber);
                // Queue up the files for removal
                setFilesToPrune.insert(fileNumber);
                nCurrentUsage -= nBytesToPrune;
                count++;
            }
        }

        LogPrint(LogFlags::PRUNE, "Prune: target=%dMiB actual=%dMiB diff=%dMiB max_prune_height=%d removed %d blk/rev pairs\n",
               nPruneTarget/1024/1024, nCurrentUsage/1024/1024,
               ((int64_t)nPruneTarget - (int64_t)nCurrentUsage)/1024/1024,
               nLastBlockWeCanPrune, count);
        */
    }
    
    /**
      | Create a new block index entry for a given
      | block hash
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn insert_block_index(&mut self, hash: &u256) -> *mut BlockIndex {
        
        todo!();
        /*
            AssertLockHeld(cs_main);

        if (hash.IsNull())
            return nullptr;

        // Return existing
        BlockMap::iterator mi = m_block_index.find(hash);
        if (mi != m_block_index.end())
            return (*mi).second;

        // Create new
        CBlockIndex* pindexNew = new CBlockIndex();
        mi = m_block_index.insert(std::make_pair(hash, pindexNew)).first;
        pindexNew->phashBlock = &((*mi).first);

        return pindexNew;
        */
    }
    
    /**
      | Load the blocktree off disk and into
      | memory. Populate certain metadata
      | per index entry (nStatus, nChainWork,
      | nTimeMax, etc.) as well as peripheral
      | collections like setDirtyBlockIndex.
      | 
      | -----------
      | @param[out] block_index_candidates
      | 
      | Fill this set with any valid blocks for
      | which we've downloaded all transactions.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn load_block_index(&mut self, 
        consensus_params:       &ChainConsensusParams,
        block_index_candidates: &mut HashSet<*mut BlockIndex,BlockIndexWorkComparator>) -> bool {
        
        todo!();
        /*
            if (!m_block_tree_db->LoadBlockIndexGuts(consensus_params, [this](const uint256& hash) EXCLUSIVE_LOCKS_REQUIRED(cs_main) { return this->InsertBlockIndex(hash); })) {
            return false;
        }

        // Calculate nChainWork
        std::vector<std::pair<int, CBlockIndex*> > vSortedByHeight;
        vSortedByHeight.reserve(m_block_index.size());
        for (const std::pair<const uint256, CBlockIndex*>& item : m_block_index)
        {
            CBlockIndex* pindex = item.second;
            vSortedByHeight.push_back(std::make_pair(pindex->nHeight, pindex));
        }
        sort(vSortedByHeight.begin(), vSortedByHeight.end());
        for (const std::pair<int, CBlockIndex*>& item : vSortedByHeight)
        {
            if (ShutdownRequested()) return false;
            CBlockIndex* pindex = item.second;
            pindex->nChainWork = (pindex->pprev ? pindex->pprev->nChainWork : 0) + GetBlockProof(*pindex);
            pindex->nTimeMax = (pindex->pprev ? std::max(pindex->pprev->nTimeMax, pindex->nTime) : pindex->nTime);
            // We can link the chain of blocks for which we've received transactions at some point.
            // Pruned nodes may have deleted the block.
            if (pindex->nTx > 0) {
                if (pindex->pprev) {
                    if (pindex->pprev->HaveTxsDownloaded()) {
                        pindex->nChainTx = pindex->pprev->nChainTx + pindex->nTx;
                    } else {
                        pindex->nChainTx = 0;
                        m_blocks_unlinked.insert(std::make_pair(pindex->pprev, pindex));
                    }
                } else {
                    pindex->nChainTx = pindex->nTx;
                }
            }
            if (!(pindex->nStatus & BLOCK_FAILED_MASK) && pindex->pprev && (pindex->pprev->nStatus & BLOCK_FAILED_MASK)) {
                pindex->nStatus |= BLOCK_FAILED_CHILD;
                setDirtyBlockIndex.insert(pindex);
            }
            if (pindex->IsAssumedValid() ||
                    (pindex->IsValid(BLOCK_VALID_TRANSACTIONS) &&
                     (pindex->HaveTxsDownloaded() || pindex->pprev == nullptr))) {
                block_index_candidates.insert(pindex);
            }
            if (pindex->nStatus & BLOCK_FAILED_MASK && (!pindexBestInvalid || pindex->nChainWork > pindexBestInvalid->nChainWork))
                pindexBestInvalid = pindex;
            if (pindex->pprev)
                pindex->BuildSkip();
            if (pindex->IsValid(BLOCK_VALID_TREE) && (pindexBestHeader == nullptr || CBlockIndexWorkComparator()(pindexBestHeader, pindex)))
                pindexBestHeader = pindex;
        }

        return true;
        */
    }
    
    /**
      | Clear all data members.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn unload(&mut self)  {
        
        self.failed_blocks.clear();
        self.blocks_unlinked.clear();

        todo!();
        /*

        for (const BlockMap::value_type& entry : m_block_index) {
            delete entry.second;
        }

        m_block_index.clear();
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn load_block_indexdb(&mut self, set_block_index_candidates: &mut HashSet<*mut BlockIndex,BlockIndexWorkComparator>) -> bool {
        
        todo!();
        /*
            if (!LoadBlockIndex(
                ::Params().GetConsensus(),
                setBlockIndexCandidates)) {
            return false;
        }

        // Load block file info
        m_block_tree_db->ReadLastBlockFile(nLastBlockFile);
        vinfoBlockFile.resize(nLastBlockFile + 1);
        LogPrintf("%s: last block file = %i\n", __func__, nLastBlockFile);
        for (int nFile = 0; nFile <= nLastBlockFile; nFile++) {
            m_block_tree_db->ReadBlockFileInfo(nFile, vinfoBlockFile[nFile]);
        }
        LogPrintf("%s: last block file info: %s\n", __func__, vinfoBlockFile[nLastBlockFile].ToString());
        for (int nFile = nLastBlockFile + 1; true; nFile++) {
            CBlockFileInfo info;
            if (m_block_tree_db->ReadBlockFileInfo(nFile, info)) {
                vinfoBlockFile.push_back(info);
            } else {
                break;
            }
        }

        // Check presence of blk files
        LogPrintf("Checking all blk files are present...\n");
        std::set<int> setBlkDataFiles;
        for (const std::pair<const uint256, CBlockIndex*>& item : m_block_index) {
            CBlockIndex* pindex = item.second;
            if (pindex->nStatus & BLOCK_HAVE_DATA) {
                setBlkDataFiles.insert(pindex->nFile);
            }
        }
        for (std::set<int>::iterator it = setBlkDataFiles.begin(); it != setBlkDataFiles.end(); it++)
        {
            FlatFilePos pos(*it, 0);
            if (CAutoFile(OpenBlockFile(pos, true), SER_DISK, CLIENT_VERSION).IsNull()) {
                return false;
            }
        }

        // Check whether we have ever pruned block & undo files
        m_block_tree_db->ReadFlag("prunedblockfiles", fHavePruned);
        if (fHavePruned)
            LogPrintf("LoadBlockIndexDB(): Block files have previously been pruned\n");

        // Check whether we need to continue reindexing
        bool fReindexing = false;
        m_block_tree_db->ReadReindexing(fReindexing);
        if(fReindexing) fReindex = true;

        return true;
        */
    }
    
    /**
      | Mark one block file as pruned (modify
      | associated database entries)
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn prune_one_block_file(&mut self, file_number: i32)  {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        LOCK(cs_LastBlockFile);

        for (const auto& entry : m_block_index) {
            CBlockIndex* pindex = entry.second;
            if (pindex->nFile == fileNumber) {
                pindex->nStatus &= ~BLOCK_HAVE_DATA;
                pindex->nStatus &= ~BLOCK_HAVE_UNDO;
                pindex->nFile = 0;
                pindex->nDataPos = 0;
                pindex->nUndoPos = 0;
                setDirtyBlockIndex.insert(pindex);

                // Prune from m_blocks_unlinked -- any block we prune would have
                // to be downloaded again in order to consider its chain, at which
                // point it would be considered as a candidate for
                // m_blocks_unlinked or setBlockIndexCandidates.
                auto range = m_blocks_unlinked.equal_range(pindex->pprev);
                while (range.first != range.second) {
                    std::multimap<CBlockIndex *, CBlockIndex *>::iterator _it = range.first;
                    range.first++;
                    if (_it->second == pindex) {
                        m_blocks_unlinked.erase(_it);
                    }
                }
            }
        }


        vinfo_block_file[file_number].set_null();
        set_dirty_file_info.insert(file_number);

        */
    }
    
    /**
      | Calculate the block/rev files to delete
      | based on height specified by user with
      | RPC command pruneblockchain
      |
      */
    pub fn find_files_to_prune_manual(&mut self, 
        set_files_to_prune:    &mut HashSet<i32>,
        n_manual_prune_height: i32,
        chain_tip_height:      i32)  {
        
        todo!();
        /*
            assert(fPruneMode && nManualPruneHeight > 0);

        LOCK2(cs_main, cs_LastBlockFile);
        if (chain_tip_height < 0) {
            return;
        }

        // last block to prune is the lesser of (user-specified height, MIN_BLOCKS_TO_KEEP from the tip)
        unsigned int nLastBlockWeCanPrune = std::min((unsigned)nManualPruneHeight, chain_tip_height - MIN_BLOCKS_TO_KEEP);
        int count = 0;
        for (int fileNumber = 0; fileNumber < nLastBlockFile; fileNumber++) {
            if (vinfoBlockFile[fileNumber].nSize == 0 || vinfoBlockFile[fileNumber].nHeightLast > nLastBlockWeCanPrune) {
                continue;
            }
            PruneOneBlockFile(fileNumber);
            setFilesToPrune.insert(fileNumber);
            count++;
        }
        LogPrintf("Prune (Manual): prune_height=%d removed %d blk/rev pairs\n", nLastBlockWeCanPrune, count);
        */
    }

    /**
      | Return the spend height, which is one
      | more than the inputs.GetBestBlock().
      | 
      | While checking, GetBestBlock() refers
      | to the parent block. (protected by cs_main)
      | 
      | This is also true for mempool checks.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn get_spend_height(&mut self, inputs: &CoinsViewCache) -> i32 {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        CBlockIndex* pindexPrev = LookupBlockIndex(inputs.GetBestBlock());
        return pindexPrev->nHeight + 1;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn lookup_block_index(&self, hash: &u256) -> Option<Arc<BlockIndex>> {
        
        todo!();
        /*
            AssertLockHeld(cs_main);
        BlockMap::const_iterator it = m_block_index.find(hash);
        return it == m_block_index.end() ? nullptr : it->second;
        */
    }
}
