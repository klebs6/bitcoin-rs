crate::ix!();

#[EXCLUSIVE_LOCKS_REQUIRED(cs, cs_main)]
pub fn remove_for_reorg(
    txmempool:         &mut TxMemPool,
    active_chainstate: &mut ChainState,
    flags:             i32)  {
    
    todo!();
    /*
        // Remove transactions spending a coinbase which are now immature and no-longer-final transactions
    AssertLockHeld(cs);
    setEntries txToRemove;
    for (IndexedTransactionSetConstIterator it = mapTx.begin(); it != mapTx.end(); it++) {
        const CTransaction& tx = it->GetTx();
        LockPoints lp = it->GetLockPoints();
        bool validLP =  TestLockPointValidity(active_chainstate.m_chain, &lp);
        CCoinsViewMemPool view_mempool(&active_chainstate.CoinsTip(), *this);
        if (!CheckFinalTx(active_chainstate.m_chain.Tip(), tx, flags)
            || !CheckSequenceLocks(active_chainstate.m_chain.Tip(), view_mempool, tx, flags, &lp, validLP)) {
            // Note if CheckSequenceLocks fails the LockPoints may still be invalid
            // So it's critical that we remove the tx and not depend on the LockPoints.
            txToRemove.insert(it);
        } else if (it->GetSpendsCoinbase()) {
            for (const CTxIn& txin : tx.vin) {
                IndexedTransactionSetConstIterator it2 = mapTx.find(txin.prevout.hash);
                if (it2 != mapTx.end())
                    continue;
                const Coin &coin = active_chainstate.CoinsTip().AccessCoin(txin.prevout);
                if (m_check_ratio != 0) assert(!coin.IsSpent());
                unsigned int nMemPoolHeight = active_chainstate.m_chain.Tip()->nHeight + 1;
                if (coin.IsSpent() || (coin.IsCoinBase() && ((signed long)nMemPoolHeight) - coin.nHeight < COINBASE_MATURITY)) {
                    txToRemove.insert(it);
                    break;
                }
            }
        }
        if (!validLP) {
            mapTx.modify(it, update_lock_points(lp));
        }
    }
    setEntries setAllRemoves;
    for (txiter it : txToRemove) {
        CalculateDescendants(it, setAllRemoves);
    }
    RemoveStaged(setAllRemoves, false, MemPoolRemovalReason::REORG);
    */
}

