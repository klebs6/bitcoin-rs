// ---------------- [ File: bitcoin-coinselect/src/coin.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/node/coin.h]
//-------------------------------------------[.cpp/bitcoin/src/node/coin.cpp]

/**
  | Look up unspent output information.
  | Returns coins in the mempool and in the
  | current chain UTXO set. Iterates through
  | all the keys in the map and populates
  | the values.
  | 
  | -----------
  | @param[in] node
  | 
  | The node context to use for lookup
  | ----------
  | @param[in,out] coins
  | 
  | map to fill
  |
  */
pub fn find_coins(
        node:  &NodeContext,
        coins: &mut HashMap<OutPoint,Coin>)  {
    
    todo!();
        /*
            assert(node.mempool);
        assert(node.chainman);
        LOCK2(cs_main, node.mempool->cs);
        CCoinsViewCache& chain_view = node.chainman->ActiveChainstate().CoinsTip();
        CCoinsViewMemPool mempool_view(&chain_view, *node.mempool);
        for (auto& coin : coins) {
            if (!mempool_view.GetCoin(coin.first, coin.second)) {
                // Either the coin is not in the CCoinsViewCache or is spent. Clear it.
                coin.second.Clear();
            }
        }
        */
}
