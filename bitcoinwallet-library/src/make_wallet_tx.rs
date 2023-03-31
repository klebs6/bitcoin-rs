crate::ix!();

/**
  | Construct wallet tx struct.
  |
  */
pub fn make_wallet_tx(
        wallet: &mut Wallet,
        wtx:    &WalletTx) -> WalletTx {
    
    todo!();
        /*
            LOCK(wallet.cs_wallet);
        WalletTx result;
        result.tx = wtx.tx;
        result.txin_is_mine.reserve(wtx.tx->vin.size());
        for (const auto& txin : wtx.tx->vin) {
            result.txin_is_mine.emplace_back(InputIsMine(wallet, txin));
        }
        result.txout_is_mine.reserve(wtx.tx->vout.size());
        result.txout_address.reserve(wtx.tx->vout.size());
        result.txout_address_is_mine.reserve(wtx.tx->vout.size());
        for (const auto& txout : wtx.tx->vout) {
            result.txout_is_mine.emplace_back(wallet.IsMine(txout));
            result.txout_address.emplace_back();
            result.txout_address_is_mine.emplace_back(ExtractDestination(txout.scriptPubKey, result.txout_address.back()) ?
                                                          wallet.IsMine(result.txout_address.back()) :
                                                          ISMINE_NO);
        }
        result.credit = CachedTxGetCredit(wallet, wtx, ISMINE_ALL);
        result.debit = CachedTxGetDebit(wallet, wtx, ISMINE_ALL);
        result.change = CachedTxGetChange(wallet, wtx);
        result.time = wtx.GetTxTime();
        result.value_map = wtx.mapValue;
        result.is_coinbase = wtx.IsCoinBase();
        return result;
        */
}
