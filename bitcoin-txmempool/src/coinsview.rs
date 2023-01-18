crate::ix!();

/**
  | CoinsView that brings transactions
  | from a mempool into view.
  | 
  | It does not check for spendings by memory
  | pool transactions.
  | 
  | Instead, it provides access to all Coins
  | which are either unspent in the base
  | CCoinsView, are outputs from any mempool
  | transaction, or are tracked temporarily
  | to allow transaction dependencies
  | in package validation.
  | 
  | This allows transaction replacement
  | to work as expected, as you want to have
  | all inputs "available" to check signatures,
  | and any cycles in the dependency graph
  | are checked directly in AcceptToMemoryPool.
  | 
  | It also allows you to sign a double-spend
  | directly in signrawtransactionwithkey
  | and signrawtransactionwithwallet,
  | as long as the conflicting transaction
  | is not yet confirmed.
  |
  */
pub struct CoinsViewMemPool {

    base: CoinsViewBacked,

    /**
      | Coins made available by transactions
      | being validated. Tracking these allows
      | for package validation, since we can
      | access transaction outputs without
      | submitting them to mempool.
      |
      */
    temp_added: HashMap<OutPoint,Coin,SaltedOutpointHasher>,

    mempool:    Arc<TxMemPool>,

}

impl CoinsViewMemPool {
    
    pub fn new(
        base_in:    *mut Box<dyn CoinsView>,
        mempool_in: &TxMemPool) -> Self {
    
        todo!();
        /*
        : coins_view_backed(baseIn),
        : mempool(mempoolIn),

        
        */
    }
    
    pub fn get_coin(&self, 
        outpoint: &OutPoint,
        coin:     &mut Coin) -> bool {
        
        todo!();
        /*
            // Check to see if the inputs are made available by another tx in the package.
        // These Coins would not be available in the underlying CoinsView.
        if (auto it = m_temp_added.find(outpoint); it != m_temp_added.end()) {
            coin = it->second;
            return true;
        }

        // If an entry in the mempool exists, always return that one, as it's guaranteed to never
        // conflict with the underlying cache, and it cannot have pruned entries (as it contains full)
        // transactions. First checking the underlying cache risks returning a pruned entry instead.
        CTransactionRef ptx = mempool.get(outpoint.hash);
        if (ptx) {
            if (outpoint.n < ptx->vout.size()) {
                coin = Coin(ptx->vout[outpoint.n], MEMPOOL_HEIGHT, false);
                return true;
            } else {
                return false;
            }
        }
        return base->GetCoin(outpoint, coin);
        */
    }
    
    /**
      | Add the coins created by this transaction.
      | These coins are only temporarily stored
      | in m_temp_added and cannot be flushed
      | to the back end. Only used for package
      | validation.
      |
      */
    pub fn package_add_transaction(&mut self, tx: &TransactionRef)  {
        
        todo!();
        /*
            for (unsigned int n = 0; n < tx->vout.size(); ++n) {
            m_temp_added.emplace(OutPoint(tx->GetHash(), n), Coin(tx->vout[n], MEMPOOL_HEIGHT, false));
        }
        */
    }
}
