crate::ix!();

/**
  | Construct wallet tx status struct.
  |
  */
pub fn make_wallet_tx_status(
        wallet: &Wallet,
        wtx:    &WalletTx) -> WalletTxStatus {
    
    todo!();
        /*
            WalletTxStatus result;
        result.block_height = wtx.m_confirm.block_height > 0 ? wtx.m_confirm.block_height : std::numeric_limits<int>::max();
        result.blocks_to_maturity = wallet.GetTxBlocksToMaturity(wtx);
        result.depth_in_main_chain = wallet.GetTxDepthInMainChain(wtx);
        result.time_received = wtx.nTimeReceived;
        result.lock_time = wtx.tx->nLockTime;
        result.is_final = wallet.chain().checkFinalTx(*wtx.tx);
        result.is_trusted = CachedTxIsTrusted(wallet, wtx);
        result.is_abandoned = wtx.isAbandoned();
        result.is_coinbase = wtx.IsCoinBase();
        result.is_in_main_chain = wallet.IsTxInMainChain(wtx);
        return result;
        */
}
