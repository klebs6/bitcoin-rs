crate::ix!();

/**
  | RAII wrapper for VerifyDB: Verify consistency
  | of the block and coin databases
  |
  */
pub struct VerifyDB {

}

///---------------------
impl Drop for VerifyDB {
    fn drop(&mut self) {
        todo!();
        /*
            uiInterface.ShowProgress("", 100, false);
        */
    }
}

impl Default for VerifyDB {

    fn default() -> Self {
    
        todo!();
        /*
            uiInterface.ShowProgress(_("Verifying blocks…").translated, 0, false);
        */
    }
}

impl VerifyDB {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn verifydb(&mut self, 
        chainstate:    &mut ChainState,
        chainparams:   &ChainParams,
        coinsview:     &mut dyn CoinsView,
        n_check_level: i32,
        n_check_depth: i32) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_main);

        if (chainstate.m_chain.Tip() == nullptr || chainstate.m_chain.Tip()->pprev == nullptr)
            return true;

        // Verify blocks in the best chain
        if (nCheckDepth <= 0 || nCheckDepth > chainstate.m_chain.Height())
            nCheckDepth = chainstate.m_chain.Height();
        nCheckLevel = std::max(0, std::min(4, nCheckLevel));
        LogPrintf("Verifying last %i blocks at level %i\n", nCheckDepth, nCheckLevel);
        CCoinsViewCache coins(&coinsview);
        CBlockIndex* pindex;
        CBlockIndex* pindexFailure = nullptr;
        int nGoodTransactions = 0;
        BlockValidationState state;
        int reportDone = 0;
        LogPrintf("[0%%]..."); /* Continued */

        const bool is_snapshot_cs{!chainstate.m_from_snapshot_blockhash};

        for (pindex = chainstate.m_chain.Tip(); pindex && pindex->pprev; pindex = pindex->pprev) {
            const int percentageDone = std::max(1, std::min(99, (int)(((double)(chainstate.m_chain.Height() - pindex->nHeight)) / (double)nCheckDepth * (nCheckLevel >= 4 ? 50 : 100))));
            if (reportDone < percentageDone/10) {
                // report every 10% step
                LogPrintf("[%d%%]...", percentageDone); /* Continued */
                reportDone = percentageDone/10;
            }
            uiInterface.ShowProgress(_("Verifying blocks…").translated, percentageDone, false);
            if (pindex->nHeight <= chainstate.m_chain.Height()-nCheckDepth)
                break;
            if ((fPruneMode || is_snapshot_cs) && !(pindex->nStatus & BLOCK_HAVE_DATA)) {
                // If pruning or running under an assumeutxo snapshot, only go
                // back as far as we have data.
                LogPrintf("VerifyDB(): block verification stopping at height %d (pruning, no data)\n", pindex->nHeight);
                break;
            }
            CBlock block;
            // check level 0: read from disk
            if (!ReadBlockFromDisk(block, pindex, chainparams.GetConsensus()))
                return error("VerifyDB(): *** ReadBlockFromDisk failed at %d, hash=%s", pindex->nHeight, pindex->GetBlockHash().ToString());
            // check level 1: verify block validity
            if (nCheckLevel >= 1 && !CheckBlock(block, state, chainparams.GetConsensus()))
                return error("%s: *** found bad block at %d, hash=%s (%s)\n", __func__,
                             pindex->nHeight, pindex->GetBlockHash().ToString(), state.ToString());
            // check level 2: verify undo validity
            if (nCheckLevel >= 2 && pindex) {
                CBlockUndo undo;
                if (!pindex->GetUndoPos().IsNull()) {
                    if (!UndoReadFromDisk(undo, pindex)) {
                        return error("VerifyDB(): *** found bad undo data at %d, hash=%s\n", pindex->nHeight, pindex->GetBlockHash().ToString());
                    }
                }
            }
            // check level 3: check for inconsistencies during memory-only disconnect of tip blocks
            size_t curr_coins_usage = coins.DynamicMemoryUsage() + chainstate.CoinsTip().DynamicMemoryUsage();

            if (nCheckLevel >= 3 && curr_coins_usage <= chainstate.m_coinstip_cache_size_bytes) {
                assert(coins.GetBestBlock() == pindex->GetBlockHash());
                DisconnectResult res = chainstate.DisconnectBlock(block, pindex, coins);
                if (res == DISCONNECT_FAILED) {
                    return error("VerifyDB(): *** irrecoverable inconsistency in block data at %d, hash=%s", pindex->nHeight, pindex->GetBlockHash().ToString());
                }
                if (res == DISCONNECT_UNCLEAN) {
                    nGoodTransactions = 0;
                    pindexFailure = pindex;
                } else {
                    nGoodTransactions += block.vtx.size();
                }
            }
            if (ShutdownRequested()) return true;
        }
        if (pindexFailure)
            return error("VerifyDB(): *** coin database inconsistencies found (last %i blocks, %i good transactions before that)\n", chainstate.m_chain.Height() - pindexFailure->nHeight + 1, nGoodTransactions);

        // store block count as we move pindex at check level >= 4
        int block_count = chainstate.m_chain.Height() - pindex->nHeight;

        // check level 4: try reconnecting blocks
        if (nCheckLevel >= 4) {
            while (pindex != chainstate.m_chain.Tip()) {
                const int percentageDone = std::max(1, std::min(99, 100 - (int)(((double)(chainstate.m_chain.Height() - pindex->nHeight)) / (double)nCheckDepth * 50)));
                if (reportDone < percentageDone/10) {
                    // report every 10% step
                    LogPrintf("[%d%%]...", percentageDone); /* Continued */
                    reportDone = percentageDone/10;
                }
                uiInterface.ShowProgress(_("Verifying blocks…").translated, percentageDone, false);
                pindex = chainstate.m_chain.Next(pindex);
                CBlock block;
                if (!ReadBlockFromDisk(block, pindex, chainparams.GetConsensus()))
                    return error("VerifyDB(): *** ReadBlockFromDisk failed at %d, hash=%s", pindex->nHeight, pindex->GetBlockHash().ToString());
                if (!chainstate.ConnectBlock(block, state, pindex, coins)) {
                    return error("VerifyDB(): *** found unconnectable block at %d, hash=%s (%s)", pindex->nHeight, pindex->GetBlockHash().ToString(), state.ToString());
                }
                if (ShutdownRequested()) return true;
            }
        }

        LogPrintf("[DONE].\n");
        LogPrintf("No coin database inconsistencies in last %i blocks (%i transactions)\n", block_count, nGoodTransactions);

        return true;
        */
    }
}
