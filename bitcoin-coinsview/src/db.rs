// ---------------- [ File: bitcoin-coinsview/src/db.rs ]
crate::ix!();

pub const DB_COIN:         char = 'C';
pub const DB_COINS:        char = 'c';
pub const DB_BLOCK_FILES:  char = 'f';
pub const DB_BLOCK_INDEX:  char = 'b';
pub const DB_BEST_BLOCK:   char = 'B';
pub const DB_HEAD_BLOCKS:  char = 'H';
pub const DB_FLAG:         char = 'F';
pub const DB_REINDEX_FLAG: char = 'R';
pub const DB_LAST_BLOCK:   char = 'l';

/**
  | Keys used in previous version that might
  | still be found in the DB:
  |
  */
pub const DB_TXINDEX_BLOCK: char = 'T';
pub const DB_TXINDEX:       char = 't';

/**
  | CCoinsView backed by the coin database
  | (chainstate/)
  |
  */
pub struct CoinsViewDB {
    db:        Box<DBWrapper>,
    ldb_path:  Box<Path>,
    is_memory: bool,
}

impl CoinsView for CoinsViewDB {

}

impl GetCoin for CoinsViewDB {

    fn get_coin(&self, 
        outpoint: &OutPoint,
        coin:     &mut Coin) -> bool {
        
        todo!();
        /*
            return m_db->Read(CoinEntry(&outpoint), coin);
        */
    }
}

impl HaveCoin for CoinsViewDB {

    fn have_coin(&self, outpoint: &OutPoint) -> bool {
        
        todo!();
        /*
            return m_db->Exists(CoinEntry(&outpoint));
        */
    }
}

impl GetBestBlock for CoinsViewDB {

    fn get_best_block(&self) -> u256 {
        
        todo!();
        /*
            uint256 hashBestChain;
        if (!m_db->Read(DB_BEST_BLOCK, hashBestChain))
            return uint256();
        return hashBestChain;
        */
    }
}

impl GetHeadBlocks for CoinsViewDB {

    fn get_head_blocks(&self) -> Vec<u256> {
        
        todo!();
        /*
            std::vector<uint256> vhashHeadBlocks;
        if (!m_db->Read(DB_HEAD_BLOCKS, vhashHeadBlocks)) {
            return std::vector<uint256>();
        }
        return vhashHeadBlocks;
        */
    }
}

impl BatchWrite for CoinsViewDB {

    fn batch_write(&mut self, 
        map_coins:  &mut CoinsMap,
        hash_block: &u256) -> bool {
        
        todo!();
        /*
            CDBBatch batch(*m_db);
        size_t count = 0;
        size_t changed = 0;
        size_t batch_size = (size_t)gArgs.GetIntArg("-dbbatchsize", nDefaultDbBatchSize);
        int crash_simulate = gArgs.GetIntArg("-dbcrashratio", 0);
        assert(!hashBlock.IsNull());

        uint256 old_tip = GetBestBlock();
        if (old_tip.IsNull()) {
            // We may be in the middle of replaying.
            std::vector<uint256> old_heads = GetHeadBlocks();
            if (old_heads.size() == 2) {
                assert(old_heads[0] == hashBlock);
                old_tip = old_heads[1];
            }
        }

        // In the first batch, mark the database as being in the middle of a
        // transition from old_tip to hashBlock.
        // A vector is used for future extensibility, as we may want to support
        // interrupting after partial writes from multiple independent reorgs.
        batch.Erase(DB_BEST_BLOCK);
        batch.Write(DB_HEAD_BLOCKS, Vector(hashBlock, old_tip));

        for (coins_map::iterator it = mapCoins.begin(); it != mapCoins.end();) {
            if (it->second.flags & CCoinsCacheEntry::DIRTY) {
                CoinEntry entry(&it->first);
                if (it->second.coin.IsSpent())
                    batch.Erase(entry);
                else
                    batch.Write(entry, it->second.coin);
                changed++;
            }
            count++;
            coins_map::iterator itOld = it++;
            mapCoins.erase(itOld);
            if (batch.SizeEstimate() > batch_size) {
                LogPrint(LogFlags::COINDB, "Writing partial batch of %.2f MiB\n", batch.SizeEstimate() * (1.0 / 1048576.0));
                m_db->WriteBatch(batch);
                batch.Clear();
                if (crash_simulate) {
                    static FastRandomContext rng;
                    if (rng.randrange(crash_simulate) == 0) {
                        LogPrintf("Simulating a crash. Goodbye.\n");
                        _Exit(0);
                    }
                }
            }
        }

        // In the last batch, mark the database as consistent with hashBlock again.
        batch.Erase(DB_HEAD_BLOCKS);
        batch.Write(DB_BEST_BLOCK, hashBlock);

        LogPrint(LogFlags::COINDB, "Writing final batch of %.2f MiB\n", batch.SizeEstimate() * (1.0 / 1048576.0));
        bool ret = m_db->WriteBatch(batch);
        LogPrint(LogFlags::COINDB, "Committed %u changed transaction outputs (out of %u) to coin database...\n", (unsigned int)changed, (unsigned int)count);
        return ret;
        */
    }
}

impl EstimateSize for CoinsViewDB {

    fn estimate_size(&self) -> usize {
        
        todo!();
        /*
            return m_db->EstimateSize(DB_COIN, uint8_t(DB_COIN + 1));
        */
    }
}

impl Cursor for CoinsViewDB {

    fn cursor(&self) -> Option<Box<CoinsViewCursor>> {
        
        todo!();
        /*
            auto i = std::make_unique<CCoinsViewDBCursor>(
            const_cast<CDBWrapper&>(*m_db).NewIterator(), GetBestBlock());
        /* It seems that there are no "const iterators" for LevelDB.  Since we
           only need read operations on it, use a const-cast to get around
           that restriction.  */
        i->pcursor->Seek(DB_COIN);
        // Cache key of first record
        if (i->pcursor->Valid()) {
            CoinEntry entry(&i->keyTmp.second);
            i->pcursor->GetKey(entry);
            i->keyTmp.first = entry.key;
        } else {
            i->keyTmp.first = 0; // Make sure Valid() and GetKey() return false
        }
        return i;
        */
    }
}

impl CoinsViewDB {

    /**
      | Attempt to update from an older database
      | format. Returns whether an error occurred.
      |
      | Upgrade the database from older formats.
      | 
      | Currently implemented: from the per-tx
      | utxo model (0.8..0.14.x) to per-txout.
      |
      */
    pub fn upgrade(&mut self) -> bool {
        
        todo!();
        /*
            std::unique_ptr<CDBIterator> pcursor(m_db->NewIterator());
        pcursor->Seek(std::make_pair(DB_COINS, uint256()));
        if (!pcursor->Valid()) {
            return true;
        }

        int64_t count = 0;
        LogPrintf("Upgrading utxo-set database...\n");
        LogPrintf("[0%%]..."); /* Continued */
        uiInterface.ShowProgress(_("Upgrading UTXO database").translated, 0, true);
        size_t batch_size = 1 << 24;
        CDBBatch batch(*m_db);
        int reportDone = 0;
        std::pair<unsigned char, uint256> key;
        std::pair<unsigned char, uint256> prev_key = {DB_COINS, uint256()};
        while (pcursor->Valid()) {
            if (ShutdownRequested()) {
                break;
            }
            if (pcursor->GetKey(key) && key.first == DB_COINS) {
                if (count++ % 256 == 0) {
                    uint32_t high = 0x100 * *key.second.begin() + *(key.second.begin() + 1);
                    int percentageDone = (int)(high * 100.0 / 65536.0 + 0.5);
                    uiInterface.ShowProgress(_("Upgrading UTXO database").translated, percentageDone, true);
                    if (reportDone < percentageDone/10) {
                        // report max. every 10% step
                        LogPrintf("[%d%%]...", percentageDone); /* Continued */
                        reportDone = percentageDone/10;
                    }
                }
                CCoins old_coins;
                if (!pcursor->GetValue(old_coins)) {
                    return error("%s: cannot parse CCoins record", __func__);
                }
                OutPoint outpoint(key.second, 0);
                for (size_t i = 0; i < old_coins.vout.size(); ++i) {
                    if (!old_coins.vout[i].IsNull() && !old_coins.vout[i].scriptPubKey.IsUnspendable()) {
                        Coin newcoin(std::move(old_coins.vout[i]), old_coins.nHeight, old_coins.fCoinBase);
                        outpoint.n = i;
                        CoinEntry entry(&outpoint);
                        batch.Write(entry, newcoin);
                    }
                }
                batch.Erase(key);
                if (batch.SizeEstimate() > batch_size) {
                    m_db->WriteBatch(batch);
                    batch.Clear();
                    m_db->CompactRange(prev_key, key);
                    prev_key = key;
                }
                pcursor->Next();
            } else {
                break;
            }
        }
        m_db->WriteBatch(batch);
        m_db->CompactRange({DB_COINS, uint256()}, key);
        uiInterface.ShowProgress("", 100, false);
        LogPrintf("[%s].\n", ShutdownRequested() ? "CANCELLED" : "DONE");
        return !ShutdownRequested();
        */
    }

    /**
      | @param[in] ldb_path
      | 
      | Location in the filesystem where leveldb
      | data will be stored.
      |
      */
    pub fn new(
        ldb_path:     Box<Path>,
        n_cache_size: usize,
        memory:       bool,
        wipe:         bool) -> Self {
    
        todo!();
        /*
           :
           m_db(std::make_unique<CDBWrapper>(ldb_path, nCacheSize, fMemory, fWipe, true)),
           m_ldb_path(ldb_path),
           m_is_memory(fMemory)
           */
    }
    
    /**
      | Dynamically alter the underlying leveldb
      | cache size.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
    pub fn resize_cache(&mut self, new_cache_size: usize)  {
        
        todo!();
        /*
            // We can't do this operation with an in-memory DB since we'll lose all the coins upon
        // reset.
        if (!m_is_memory) {
            // Have to do a reset first to get the original `m_db` state to release its
            // filesystem lock.
            m_db.reset();
            m_db = std::make_unique<CDBWrapper>(
                m_ldb_path, new_cache_size, m_is_memory, /*fWipe*/ false, /*obfuscate*/ true);
        }
        */
    }
}
