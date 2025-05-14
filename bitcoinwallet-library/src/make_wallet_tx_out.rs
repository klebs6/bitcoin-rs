// ---------------- [ File: bitcoinwallet-library/src/make_wallet_tx_out.rs ]
crate::ix!();

/**
  | Construct wallet TxOut struct.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn make_wallet_tx_out(
        wallet: &Wallet,
        wtx:    &WalletTx,
        n:      i32,
        depth:  i32) -> WalletTxOut {
    
    todo!();
        /*
            WalletTxOut result;
        result.txout = wtx.tx->vout[n];
        result.time = wtx.GetTxTime();
        result.depth_in_main_chain = depth;
        result.is_spent = wallet.IsSpent(wtx.GetHash(), n);
        return result;
        */
}
