crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/coins.h]

pub enum CoinsCacheSizeState
{
    /**
      | The coins cache is in immediate need
      | of a flush.
      |
      */
    CRITICAL = 2,

    /**
      | The cache is at >= 90% capacity.
      |
      */
    LARGE = 1,
    OK    = 0
}



pub type CoinsMap = HashMap<OutPoint,CoinsCacheEntry,SaltedOutpointHasher>;

/**
  | CoinsView that adds a memory cache for
  | transactions to another CoinsView
  |
  | By deleting the copy constructor, we
  | prevent accidentally using it when
  | one intends to create a cache on top of
  | a base cache.
  |
  */
pub struct CoinsViewCache {
    base: CoinsViewBacked,

    /**
      | Make mutable so that we can "fill the
      | cache" even from Get-methods declared
      | as "const".
      |
      */
    hash_block:         RefCell<u256>,
    cache_coins:        RefCell<CoinsMap>,

    /**
      | Cached dynamic memory usage for the
      | inner Coin objects.
      |
      */
    cached_coins_usage: RefCell<usize>,
}

impl From<*mut dyn CoinsView> for CoinsViewCache {

    fn from(base_in: *mut dyn CoinsView) -> Self {
    
        todo!();
        /*
        : coins_view_backed(baseIn),
        : cached_coins_usage(0),
        */
    }
}
    
impl CoinsViewCache {

    /* ---------- Standard CoinsView methods  ---------- */
    
    pub fn cursor(&self) -> Box<CoinsViewCursor> {
        
        todo!();
        /*
            throw std::logic_error("CoinsViewCache cursor iteration not supported.");
        */
    }

    /**
      | Calculate the size of the cache (in bytes)
      |
      */
    pub fn dynamic_memory_usage(&self) -> usize {
        
        todo!();
        /*
            return memusage::DynamicUsage(cacheCoins) + cachedCoinsUsage;
        */
    }
    
    /**
      | @note
      | 
      | this is marked const, but may actually
      | append to `cacheCoins`, increasing
      | memory usage.
      |
      */
    pub fn fetch_coin<'a, I>(&self, outpoint: &OutPoint) -> I where I: Iterator<Item = <CoinsMap as IntoIterator>::Item> {
        
        todo!();
        /*
            coins_map::iterator it = cacheCoins.find(outpoint);
        if (it != cacheCoins.end())
            return it;
        Coin tmp;
        if (!base->GetCoin(outpoint, tmp))
            return cacheCoins.end();
        coins_map::iterator ret = cacheCoins.emplace(std::piecewise_construct, std::forward_as_tuple(outpoint), std::forward_as_tuple(std::move(tmp))).first;
        if (ret->second.coin.IsSpent()) {
            // The parent only has an empty entry for this outpoint; we can consider our
            // version as fresh.
            ret->second.flags = CCoinsCacheEntry::FRESH;
        }
        cachedCoinsUsage += ret->second.coin.DynamicMemoryUsage();
        return ret;
        */
    }
    
    pub fn get_coin(&self, 
        outpoint: &OutPoint,
        coin:     &mut Coin) -> bool {
        
        todo!();
        /*
            coins_map::const_iterator it = FetchCoin(outpoint);
        if (it != cacheCoins.end()) {
            coin = it->second.coin;
            return !coin.IsSpent();
        }
        return false;
        */
    }
    
    /**
      | Add a coin. Set possible_overwrite
      | to true if an unspent version may already
      | exist in the cache.
      |
      */
    pub fn add_coin(&mut self, 
        outpoint:           &OutPoint,
        coin:               Coin,
        possible_overwrite: bool)  {
        
        todo!();
        /*
            assert(!coin.IsSpent());
        if (coin.out.scriptPubKey.IsUnspendable()) return;
        coins_map::iterator it;
        bool inserted;
        std::tie(it, inserted) = cacheCoins.emplace(std::piecewise_construct, std::forward_as_tuple(outpoint), std::tuple<>());
        bool fresh = false;
        if (!inserted) {
            cachedCoinsUsage -= it->second.coin.DynamicMemoryUsage();
        }
        if (!possible_overwrite) {
            if (!it->second.coin.IsSpent()) {
                throw std::logic_error("Attempted to overwrite an unspent coin (when possible_overwrite is false)");
            }
            // If the coin exists in this cache as a spent coin and is DIRTY, then
            // its spentness hasn't been flushed to the parent cache. We're
            // re-adding the coin to this cache now but we can't mark it as FRESH.
            // If we mark it FRESH and then spend it before the cache is flushed
            // we would remove it from this cache and would never flush spentness
            // to the parent cache.
            //
            // Re-adding a spent coin can happen in the case of a re-org (the coin
            // is 'spent' when the block adding it is disconnected and then
            // re-added when it is also added in a newly connected block).
            //
            // If the coin doesn't exist in the current cache, or is spent but not
            // DIRTY, then it can be marked FRESH.
            fresh = !(it->second.flags & CCoinsCacheEntry::DIRTY);
        }
        it->second.coin = std::move(coin);
        it->second.flags |= CCoinsCacheEntry::DIRTY | (fresh ? CCoinsCacheEntry::FRESH : 0);
        cachedCoinsUsage += it->second.coin.DynamicMemoryUsage();
        */
    }
    
    /**
      | Emplace a coin into cacheCoins without
      | performing any checks, marking the
      | emplaced coin as dirty.
      | 
      | NOT FOR GENERAL USE. Used only when loading
      | coins from a UTXO snapshot. @sa ChainstateManager::PopulateAndValidateSnapshot()
      |
      */
    pub fn emplace_coin_internaldanger(&mut self, 
        outpoint: OutPoint,
        coin:     Coin)  {

        *self.cached_coins_usage.borrow_mut() += coin.dynamic_memory_usage();

        self.cache_coins.borrow_mut().insert(
            outpoint,
            CoinsCacheEntry {
                coin: coin,
                flags: CoinsCacheEntryFlags::DIRTY,
            }
        );
    }
    
    /**
      | Spend a coin. Pass moveto in order to
      | get the deleted data.
      | 
      | If no unspent output exists for the passed
      | outpoint, this call has no effect.
      |
      */
    pub fn spend_coin(&mut self, 
        outpoint: &OutPoint,
        moveout:  *mut Coin) -> bool {
        
        todo!();
        /*
            coins_map::iterator it = FetchCoin(outpoint);
        if (it == cacheCoins.end()) return false;
        cachedCoinsUsage -= it->second.coin.DynamicMemoryUsage();
        if (moveout) {
            *moveout = std::move(it->second.coin);
        }
        if (it->second.flags & CCoinsCacheEntry::FRESH) {
            cacheCoins.erase(it);
        } else {
            it->second.flags |= CCoinsCacheEntry::DIRTY;
            it->second.coin.Clear();
        }
        return true;
        */
    }
    
    /**
      | Return a reference to Coin in the cache,
      | or coinEmpty if not found. This is more
      | efficient than GetCoin.
      | 
      | Generally, do not hold the reference
      | returned for more than a short scope.
      | 
      | While the current implementation allows
      | for modifications to the contents of
      | the cache while holding the reference,
      | this behavior should not be relied on!
      | To be safe, best to not hold the returned
      | reference through any other calls to
      | this cache.
      |
      */
    pub fn access_coin(&self, outpoint: &OutPoint) -> &Coin {
        
        todo!();
        /*
            coins_map::const_iterator it = FetchCoin(outpoint);
        if (it == cacheCoins.end()) {
            return coinEmpty;
        } else {
            return it->second.coin;
        }
        */
    }
    
    pub fn have_coin(&self, outpoint: &OutPoint) -> bool {
        
        todo!();
        /*
            coins_map::const_iterator it = FetchCoin(outpoint);
        return (it != cacheCoins.end() && !it->second.coin.IsSpent());
        */
    }
    
    /**
      | Check if we have the given utxo already
      | loaded in this cache.
      | 
      | The semantics are the same as HaveCoin(),
      | but no calls to the backing CoinsView
      | are made.
      |
      */
    pub fn have_coin_in_cache(&self, outpoint: &OutPoint) -> bool {
        
        todo!();
        /*
            coins_map::const_iterator it = cacheCoins.find(outpoint);
        return (it != cacheCoins.end() && !it->second.coin.IsSpent());
        */
    }
    
    pub fn get_best_block(&self) -> u256 {
        
        todo!();
        /*
            if (hashBlock.IsNull())
            hashBlock = base->GetBestBlock();
        return hashBlock;
        */
    }
    
    pub fn set_best_block(&mut self, hash_block_in: &u256)  {
        
        todo!();
        /*
            hashBlock = hashBlockIn;
        */
    }
    
    pub fn batch_write(&mut self, 
        map_coins:     &mut CoinsMap,
        hash_block_in: &u256) -> bool {
        
        todo!();
        /*
            for (coins_map::iterator it = mapCoins.begin(); it != mapCoins.end(); it = mapCoins.erase(it)) {
            // Ignore non-dirty entries (optimization).
            if (!(it->second.flags & CCoinsCacheEntry::DIRTY)) {
                continue;
            }
            coins_map::iterator itUs = cacheCoins.find(it->first);
            if (itUs == cacheCoins.end()) {
                // The parent cache does not have an entry, while the child cache does.
                // We can ignore it if it's both spent and FRESH in the child
                if (!(it->second.flags & CCoinsCacheEntry::FRESH && it->second.coin.IsSpent())) {
                    // Create the coin in the parent cache, move the data up
                    // and mark it as dirty.
                    CCoinsCacheEntry& entry = cacheCoins[it->first];
                    entry.coin = std::move(it->second.coin);
                    cachedCoinsUsage += entry.coin.DynamicMemoryUsage();
                    entry.flags = CCoinsCacheEntry::DIRTY;
                    // We can mark it FRESH in the parent if it was FRESH in the child
                    // Otherwise it might have just been flushed from the parent's cache
                    // and already exist in the grandparent
                    if (it->second.flags & CCoinsCacheEntry::FRESH) {
                        entry.flags |= CCoinsCacheEntry::FRESH;
                    }
                }
            } else {
                // Found the entry in the parent cache
                if ((it->second.flags & CCoinsCacheEntry::FRESH) && !itUs->second.coin.IsSpent()) {
                    // The coin was marked FRESH in the child cache, but the coin
                    // exists in the parent cache. If this ever happens, it means
                    // the FRESH flag was misapplied and there is a logic error in
                    // the calling code.
                    throw std::logic_error("FRESH flag misapplied to coin that exists in parent cache");
                }

                if ((itUs->second.flags & CCoinsCacheEntry::FRESH) && it->second.coin.IsSpent()) {
                    // The grandparent cache does not have an entry, and the coin
                    // has been spent. We can just delete it from the parent cache.
                    cachedCoinsUsage -= itUs->second.coin.DynamicMemoryUsage();
                    cacheCoins.erase(itUs);
                } else {
                    // A normal modification.
                    cachedCoinsUsage -= itUs->second.coin.DynamicMemoryUsage();
                    itUs->second.coin = std::move(it->second.coin);
                    cachedCoinsUsage += itUs->second.coin.DynamicMemoryUsage();
                    itUs->second.flags |= CCoinsCacheEntry::DIRTY;
                    // NOTE: It isn't safe to mark the coin as FRESH in the parent
                    // cache. If it already existed and was spent in the parent
                    // cache then marking it FRESH would prevent that spentness
                    // from being flushed to the grandparent.
                }
            }
        }
        hashBlock = hashBlockIn;
        return true;
        */
    }
    
    /**
      | Push the modifications applied to this
      | cache to its base.
      | 
      | Failure to call this method before destruction
      | will cause the changes to be forgotten.
      | 
      | If false is returned, the state of this
      | cache (and its backing view) will be
      | undefined.
      |
      */
    pub fn flush(&mut self) -> bool {
        
        todo!();
        /*
            bool fOk = base->BatchWrite(cacheCoins, hashBlock);
        cacheCoins.clear();
        cachedCoinsUsage = 0;
        return fOk;
        */
    }
    
    /**
      | Removes the UTXO with the given outpoint
      | from the cache, if it is not modified.
      |
      */
    pub fn uncache(&mut self, hash: &OutPoint)  {
        
        todo!();
        /*
            coins_map::iterator it = cacheCoins.find(hash);
        if (it != cacheCoins.end() && it->second.flags == 0) {
            cachedCoinsUsage -= it->second.coin.DynamicMemoryUsage();
            cacheCoins.erase(it);
        }
        */
    }

    /**
      | Calculate the size of the cache (in number
      | of transaction outputs)
      |
      */
    pub fn get_cache_size(&self) -> u32 {
        
        todo!();
        /*
            return cacheCoins.size();
        */
    }

    /**
      | Check whether all prevouts of the transaction
      | are present in the UTXO set represented
      | by this view
      |
      */
    pub fn have_inputs(&self, tx: &Transaction) -> bool {
        
        todo!();
        /*
        if (!tx.IsCoinBase()) {
            for (unsigned int i = 0; i < tx.vin.size(); i++) {
                if (!HaveCoin(tx.vin[i].prevout)) {
                    return false;
                }
            }
        }
        return true;
        */
    }

    /**
      | Force a reallocation of the cache map. This is
      | required when downsizing the cache because the
      | map's allocator may be hanging onto a lot of
      | memory despite having called .clear().
      |
      | See:
      | https://stackoverflow.com/questions/42114044/how-to-release-unordered-map-memory
      */
    pub fn reallocate_cache(&mut self)  {
        
        todo!();
        /*
            // Cache should be empty when we're calling this.
        assert(cacheCoins.size() == 0);
        cacheCoins.~CCoinsMap();
        ::new (&cacheCoins) CCoinsMap();
        */
    }
}
