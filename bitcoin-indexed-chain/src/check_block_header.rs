// ---------------- [ File: bitcoin-indexed-chain/src/check_block_header.rs ]
crate::ix!();

/**
  | Context-dependent validity checks.
  | 
  | By "context", we mean only the previous
  | block headers, but not the UTXO set;
  | UTXO-related validity checks are done
  | in ConnectBlock().
  | 
  | -----------
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
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn contextual_check_block_header(
    block:           &BlockHeader,
    state:           &mut BlockValidationState,
    blockman:        &mut BlockManager,
    params:          &ChainParams,
    pindex_prev:     Arc<BlockIndex>,
    n_adjusted_time: i64) -> bool 
{
    todo!();
        /*
            assert(pindexPrev != nullptr);
        const int nHeight = pindexPrev->nHeight + 1;

        // Check proof of work
        const ChainConsensusParams& consensusParams = params.GetConsensus();
        if (block.nBits != GetNextWorkRequired(pindexPrev, &block, consensusParams))
            return state.Invalid(BlockValidationResult::BLOCK_INVALID_HEADER, "bad-diffbits", "incorrect proof of work");

        // Check against checkpoints
        if (fCheckpointsEnabled) {
            // Don't accept any forks from the main chain prior to last checkpoint.
            // GetLastCheckpoint finds the last checkpoint in MapCheckpoints that's in our
            // BlockIndex().
            CBlockIndex* pcheckpoint = blockman.GetLastCheckpoint(params.Checkpoints());
            if (pcheckpoint && nHeight < pcheckpoint->nHeight) {
                LogPrintf("ERROR: %s: forked chain older than last checkpoint (height %d)\n", __func__, nHeight);
                return state.Invalid(BlockValidationResult::BLOCK_CHECKPOINT, "bad-fork-prior-to-checkpoint");
            }
        }

        // Check timestamp against prev
        if (block.GetBlockTime() <= pindexPrev->GetMedianTimePast())
            return state.Invalid(BlockValidationResult::BLOCK_INVALID_HEADER, "time-too-old", "block's timestamp is too early");

        // Check timestamp
        if (block.GetBlockTime() > nAdjustedTime + MAX_FUTURE_BLOCK_TIME)
            return state.Invalid(BlockValidationResult::BLOCK_TIME_FUTURE, "time-too-new", "block timestamp too far in the future");

        // Reject blocks with outdated version
        if ((block.nVersion < 2 && DeploymentActiveAfter(pindexPrev, consensusParams, consensus::DEPLOYMENT_HEIGHTINCB)) ||
            (block.nVersion < 3 && DeploymentActiveAfter(pindexPrev, consensusParams, consensus::DEPLOYMENT_DERSIG)) ||
            (block.nVersion < 4 && DeploymentActiveAfter(pindexPrev, consensusParams, consensus::DEPLOYMENT_CLTV))) {
                return state.Invalid(BlockValidationResult::BLOCK_INVALID_HEADER, strprintf("bad-version(0x%08x)", block.nVersion),
                                     strprintf("rejected nVersion=0x%08x block", block.nVersion));
        }

        return true;
        */
}
