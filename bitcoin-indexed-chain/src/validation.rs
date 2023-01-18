crate::ix!();

/**
  | Atomically test acceptance of a package.
  | If the package only contains one tx,
  | package rules still apply. Package
  | validation does not allow BIP125 replacements,
  | so the transaction(s) cannot spend
  | the same inputs as any transaction in
  | the mempool.
  | 
  | -----------
  | @param[in] txns
  | 
  | Group of transactions which may be independent
  | or contain parent-child dependencies.
  | The transactions must not conflict
  | with each other, i.e., must not spend
  | the same inputs. If any dependencies
  | exist, parents must appear anywhere
  | in the list before their children.
  | 
  | -----------
  | @return
  | 
  | a PackageMempoolAcceptResult which
  | includes a MempoolAcceptResult for
  | each transaction.
  | 
  | If a transaction fails, validation
  | will exit early and some results may
  | be missing.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn process_new_package(
        active_chainstate: &mut ChainState,
        pool:              &mut TxMemPool,
        package:           &Package,
        test_accept:       bool) -> PackageMempoolAcceptResult {
    
    todo!();
        /*
            AssertLockHeld(cs_main);
        assert(test_accept); // Only allow package accept dry-runs (testmempoolaccept RPC).
        assert(!package.empty());
        assert(std::all_of(package.cbegin(), package.cend(), [](const auto& tx){return tx != nullptr;}));

        std::vector<OutPoint> coins_to_uncache;
        const ChainParams& chainparams = Params();
        MemPoolAccept::ATMPArgs args { chainparams, GetTime(), /* bypass_limits */ false, coins_to_uncache,
                                       test_accept, /* m_allow_bip125_replacement */ false };
        const PackageMempoolAcceptResult result = MemPoolAccept(pool, active_chainstate).AcceptMultipleTransactions(package, args);

        // Uncache coins pertaining to transactions that were not submitted to the mempool.
        for (const OutPoint& hashTx : coins_to_uncache) {
            active_chainstate.CoinsTip().Uncache(hashTx);
        }
        return result;
        */
}

/**
  | Check a block is completely valid from
  | start to finish (only works on top of
  | our current best block)
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn test_block_validity(
        state:             &mut BlockValidationState,
        chainparams:       &ChainParams,
        chainstate:        &mut ChainState,
        block:             &Block,
        pindex_prev:       Arc<Mutex<BlockIndex>>,
        checkpow:          Option<bool>,
        check_merkle_root: Option<bool>) -> bool {

    let checkpow:          bool = checkpow.unwrap_or(true);
    let check_merkle_root: bool = check_merkle_root.unwrap_or(true);
    
    todo!();
        /*
            AssertLockHeld(cs_main);
        assert(pindexPrev && pindexPrev == chainstate.m_chain.Tip());
        CCoinsViewCache viewNew(&chainstate.CoinsTip());
        uint256 block_hash(block.GetHash());
        CBlockIndex indexDummy(block);
        indexDummy.pprev = pindexPrev;
        indexDummy.nHeight = pindexPrev->nHeight + 1;
        indexDummy.phashBlock = &block_hash;

        // NOTE: CheckBlockHeader is called by CheckBlock
        if (!ContextualCheckBlockHeader(block, state, chainstate.m_blockman, chainparams, pindexPrev, GetAdjustedTime()))
            return error("%s: consensus::ContextualCheckBlockHeader: %s", __func__, state.ToString());
        if (!CheckBlock(block, state, chainparams.GetConsensus(), fCheckPOW, fCheckMerkleRoot))
            return error("%s: consensus::CheckBlock: %s", __func__, state.ToString());
        if (!ContextualCheckBlock(block, state, chainparams.GetConsensus(), pindexPrev))
            return error("%s: consensus::ContextualCheckBlock: %s", __func__, state.ToString());
        if (!chainstate.ConnectBlock(block, state, &indexDummy, viewNew, true)) {
            return false;
        }
        assert(state.IsValid());

        return true;
        */
}

/**
  | Prune block files up to a given height
  |
  | This function is called from the RPC
  | code for pruneblockchain
  |
  */
pub fn prune_block_files_manual(
        active_chainstate:     &mut ChainState,
        n_manual_prune_height: i32)  {
    
    todo!();
        /*
            BlockValidationState state;
        if (!active_chainstate.FlushStateToDisk(
                state, FlushStateMode::NONE, nManualPruneHeight)) {
            LogPrintf("%s: failed to flush state (%s)\n", __func__, state.ToString());
        }
        */
}
