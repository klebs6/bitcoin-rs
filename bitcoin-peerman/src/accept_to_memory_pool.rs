// ---------------- [ File: bitcoin-peerman/src/accept_to_memory_pool.rs ]
crate::ix!();

/**
  | (Try to) add a transaction to the memory
  | pool.
  | 
  | -----------
  | @param[in] bypass_limits
  | 
  | When true, don't enforce mempool fee
  | limits.
  | ----------
  | @param[in] test_accept
  | 
  | When true, run validation checks but
  | don't submit to mempool.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn accept_to_memory_pool(
    active_chainstate: &mut dyn ChainStateInterface,
    pool:              Amo<TxMemPool>,
    tx:                TransactionRef,
    bypass_limits:     bool,
    test_accept:       Option<bool>) -> MempoolAcceptResult 
{
    let test_accept: bool = test_accept.unwrap_or(false);
    
    todo!();
        /*
            return AcceptToMemoryPoolWithTime(Params(), pool, active_chainstate, tx, GetTime(), bypass_limits, test_accept);
        */
}

/**
  | (try to) add transaction to memory pool
  | with a specified acceptance time *
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn accept_to_memory_pool_with_time(
    chainparams:       &ChainParams,
    pool:              Amo<TxMemPool>,
    active_chainstate: &mut dyn ChainStateInterface,
    tx:                &TransactionRef,
    n_accept_time:     i64,
    bypass_limits:     bool,
    test_accept:       bool) -> MempoolAcceptResult 
{
    todo!();
        /*
            std::vector<OutPoint> coins_to_uncache;
        MemPoolAccept::ATMPArgs args { chainparams, nAcceptTime, bypass_limits, coins_to_uncache,
                                       test_accept, /* m_allow_bip125_replacement */ true };

        const MempoolAcceptResult result = MemPoolAccept(pool, active_chainstate).AcceptSingleTransaction(tx, args);
        if (result.m_result_type != MempoolAcceptResult::ResultType::VALID) {
            // Remove coins that were not present in the coins cache before calling
            // AcceptSingleTransaction(); this is to prevent memory DoS in case we receive a large
            // number of invalid transactions that attempt to overrun the in-memory coins cache
            // (`CCoinsViewCache::cacheCoins`).

            for (const OutPoint& hashTx : coins_to_uncache)
                active_chainstate.CoinsTip().Uncache(hashTx);
        }
        // After we've (potentially) uncached entries, ensure our coins cache is still within its size limits
        BlockValidationState state_dummy;
        active_chainstate.FlushStateToDisk(state_dummy, FlushStateMode::PERIODIC);
        return result;
        */
}
