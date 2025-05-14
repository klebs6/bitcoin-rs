// ---------------- [ File: bitcoin-indexed-chain/src/update_tip_log.rs ]
crate::ix!();

#[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
pub fn update_tip_log(
    coins_tip:        &CoinsViewCache,
    tip:              Arc<BlockIndex>,
    params:           &ChainParams,
    func_name:        &String,
    prefix:           &String,
    warning_messages: &String)  {

    todo!();
        /*
            AssertLockHeld(::cs_main);
        LogPrintf("%s%s: new best=%s height=%d version=0x%08x log2_work=%f tx=%lu date='%s' progress=%f cache=%.1fMiB(%utxo)%s\n",
            prefix, func_name,
            tip->GetBlockHash().ToString(), tip->nHeight, tip->nVersion,
            log(tip->nChainWork.getdouble()) / log(2.0), (unsigned long)tip->nChainTx,
            FormatISO8601DateTime(tip->GetBlockTime()),
            GuessVerificationProgress(params.TxData(), tip),
            coins_tip.DynamicMemoryUsage() * (1.0 / (1 << 20)),
            coins_tip.GetCacheSize(),
            !warning_messages.empty() ? strprintf(" warning='%s'", warning_messages) : "");
        */
}

