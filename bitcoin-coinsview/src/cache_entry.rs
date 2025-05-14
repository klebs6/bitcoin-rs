// ---------------- [ File: bitcoin-coinsview/src/cache_entry.rs ]
crate::ix!();

/**
  | A Coin in one level of the coins database
  | caching hierarchy.
  | 
  | A coin can either be:
  | 
  | - unspent or spent (in which case the
  | Coin object will be nulled out - see Coin.Clear())
  | 
  | - DIRTY or not DIRTY
  | 
  | - FRESH or not FRESH
  | 
  | Out of these 2^3 = 8 states, only some
  | combinations are valid:
  | 
  | - unspent, FRESH, DIRTY (e.g. a new coin
  | created in the cache)
  | 
  | - unspent, not FRESH, DIRTY (e.g. a coin
  | changed in the cache during a reorg)
  | 
  | - unspent, not FRESH, not DIRTY (e.g.
  | an unspent coin fetched from the parent
  | cache)
  | 
  | - spent, FRESH, not DIRTY (e.g. a spent
  | coin fetched from the parent cache)
  | 
  | - spent, not FRESH, DIRTY (e.g. a coin
  | is spent and spentness needs to be flushed
  | to the parent)
  |
  */
pub struct CoinsCacheEntry {

    /**
      | The actual cached data.
      |
      */
    pub coin:  Coin,

    pub flags: CoinsCacheEntryFlags,
}

pub enum CoinsCacheEntryFlags {

    /**
      | DIRTY means the CCoinsCacheEntry is
      | potentially different from the version in
      | the parent cache. 
      |
      | Failure to mark a coin as DIRTY when it is
      | potentially different from the parent
      | cache will cause a consensus failure,
      | since the coin's state won't get written
      | to the parent when the cache is flushed.
      |
      */
    DIRTY = 1 << 0,

    /**
      | FRESH means the parent cache does not have
      | this coin or that it is a spent coin in
      | the parent cache. 
      |
      | If a FRESH coin in the cache is later
      | spent, it can be deleted entirely and
      | doesn't ever need to be flushed to the
      | parent. 
      |
      | This is a performance optimization. 
      |
      | Marking a coin as FRESH when it exists
      | unspent in the parent cache will cause
      | a consensus failure, since it might not be
      | deleted from the parent when this cache is
      | flushed.
      |
      */
    FRESH = 1 << 1,
}

impl Default for CoinsCacheEntry {
    
    fn default() -> Self {
        todo!();
        /*
        : flags(0),

        
        */
    }
}

impl From<Coin> for CoinsCacheEntry {
    fn from(coin: Coin) -> Self {
    
        todo!();
        /*
        : coin(std::move(coin_)),
        : flags(0),

        
        */
    }
}

impl CoinsCacheEntry {

    pub fn new(
        coin: Coin,
        flag: u8) -> Self {
    
        todo!();
        /*
        : coin(std::move(coin_)),
        : flags(flag),

        
        */
    }
}
