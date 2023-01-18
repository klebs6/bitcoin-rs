crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/index/base.h]
//-------------------------------------------[.cpp/bitcoin/src/index/blockfilterindex.h]
//-------------------------------------------[.cpp/bitcoin/src/index/blockfilterindex.cpp]

/**
  | The index database stores three items
  | for each block: the disk location of
  | the encoded filter, its dSHA256 hash,
  | and the header. Those belonging to blocks
  | on the active chain are indexed by height,
  | and those belonging to blocks that have
  | been reorganized out of the active chain
  | are indexed by block hash. This ensures
  | that filter data for any block that becomes
  | part of the active chain can always be
  | retrieved, alleviating timing concerns.
  | 
  | The filters themselves are stored in
  | flat files and referenced by the LevelDB
  | entries. This minimizes the amount
  | of data written to LevelDB and keeps
  | the database values constant size.
  | The disk location of the next block filter
  | to be written (represented as a FlatFilePos)
  | is stored under the DB_FILTER_POS key.
  | 
  | Keys for the height index have the type
  | [DB_BLOCK_HEIGHT, uint32 (BE)]. The
  | height is represented as big-endian
  | so that sequential reads of filters
  | by height are fast.
  | 
  | Keys for the hash index have the type
  | [DB_BLOCK_HASH, uint256].
  |
  */
pub const DB_BLOCK_HASH:   char = 's';
pub const DB_BLOCK_HEIGHT: char = 't';
pub const DB_FILTER_POS:   char = 'P';

pub const MAX_FLTR_FILE_SIZE: u32 = 0x1000000; // 16 MiB

/**
  | The pre-allocation chunk size for fltr?????.dat
  | files
  |
  */
pub const FLTR_FILE_CHUNK_SIZE: u32 = 0x100000; // 1 MiB

/**
  | Maximum size of the cfheaders cache
  | 
  | We have a limit to prevent a bug in filling
  | this cache potentially turning into
  | an OOM. At 2000 entries, this cache is
  | big enough for a 2,000,000 length block
  | chain, which we should be enough until
  | ~2047.
  |
  */
pub const CF_HEADERS_CACHE_MAX_SZ: usize = 2000;

pub struct BlockFilterIndexDBVal {
    hash:   u256,
    header: u256,
    pos:    FlatFilePos,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(BlockFilterIndexDBVal, obj) {
        READWRITE(obj.hash, obj.header, obj.pos); 
    }
    */
}

///----------------
pub struct BlockFilterIndexDBHeightKey {
    height: i32,
}

impl BlockFilterIndexDBHeightKey {

    pub fn new(height_in: i32) -> Self {
    
        todo!();
        /*
        : height(height_in),
        */
    }
    
    pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            ser_writedata8(s, DB_BLOCK_HEIGHT);
            ser_writedata32be(s, height);
        */
    }
    
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            const uint8_t prefix{ser_readdata8(s)};
            if (prefix != DB_BLOCK_HEIGHT) {
                throw std::ios_base::failure("Invalid format for block filter index DB height key");
            }
            height = ser_readdata32be(s);
        */
    }
}

///----------------
pub struct BlockFilterIndexDBHashKey {
    hash: u256,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(BlockFilterIndexDBHashKey, obj) {
            uint8_t prefix{DB_BLOCK_HASH};
            READWRITE(prefix);
            if (prefix != DB_BLOCK_HASH) {
                throw std::ios_base::failure("Invalid format for block filter index DB hash key");
            }

            READWRITE(obj.hash);
        }
    */
}

impl BlockFilterIndexDBHashKey {

    pub fn new(hash_in: &u256) -> Self {
    
        todo!();
        /*
        : hash(hash_in),
        */
    }
}

lazy_static!{
    /*
    static std::map<BlockFilterType, BlockFilterIndex> g_filter_indexes;
    */
}

pub fn lookup_one(
        db:          &DBWrapper,
        block_index: *const BlockIndex,
        result:      &mut BlockFilterIndexDBVal) -> bool {
    
    todo!();
        /*
            // First check if the result is stored under the height index and the value there matches the
        // block hash. This should be the case if the block is on the active chain.
        std::pair<uint256, DBVal> read_out;
        if (!db.Read(BlockFilterIndexDBHeightKey(block_index->nHeight), read_out)) {
            return false;
        }
        if (read_out.first == block_index->GetBlockHash()) {
            result = std::move(read_out.second);
            return true;
        }

        // If value at the height index corresponds to an different block, the result will be stored in
        // the hash index.
        return db.Read(BlockFilterIndexDBHashKey(block_index->GetBlockHash()), result);
        */
}

pub fn lookup_range(
        db:           &mut DBWrapper,
        index_name:   &String,
        start_height: i32,
        stop_index:   *const BlockIndex,
        results:      &mut Vec<BlockFilterIndexDBVal>) -> bool {
    
    todo!();
        /*
            if (start_height < 0) {
            return error("%s: start height (%d) is negative", __func__, start_height);
        }
        if (start_height > stop_index->nHeight) {
            return error("%s: start height (%d) is greater than stop height (%d)",
                         __func__, start_height, stop_index->nHeight);
        }

        size_t results_size = static_cast<size_t>(stop_index->nHeight - start_height + 1);
        std::vector<std::pair<uint256, DBVal>> values(results_size);

        BlockFilterIndexDBHeightKey key(start_height);
        std::unique_ptr<CDBIterator> db_it(db.NewIterator());
        db_it->Seek(BlockFilterIndexDBHeightKey(start_height));
        for (int height = start_height; height <= stop_index->nHeight; ++height) {
            if (!db_it->Valid() || !db_it->GetKey(key) || key.height != height) {
                return false;
            }

            size_t i = static_cast<size_t>(height - start_height);
            if (!db_it->GetValue(values[i])) {
                return error("%s: unable to read value in %s at key (%c, %d)",
                             __func__, index_name, DB_BLOCK_HEIGHT, height);
            }

            db_it->Next();
        }

        results.resize(results_size);

        // Iterate backwards through block indexes collecting results in order to access the block hash
        // of each entry in case we need to look it up in the hash index.
        for (const CBlockIndex* block_index = stop_index;
             block_index && block_index->nHeight >= start_height;
             block_index = block_index->pprev) {
            uint256 block_hash = block_index->GetBlockHash();

            size_t i = static_cast<size_t>(block_index->nHeight - start_height);
            if (block_hash == values[i].first) {
                results[i] = std::move(values[i].second);
                continue;
            }

            if (!db.Read(BlockFilterIndexDBHashKey(block_hash), results[i])) {
                return error("%s: unable to read value in %s at key (%c, %s)",
                             __func__, index_name, DB_BLOCK_HASH, block_hash.ToString());
            }
        }

        return true;
        */
}

pub fn block_filter_index_copy_height_index_to_hash_index(
        db_it:        &mut DBIterator,
        batch:        &mut DBBatch,
        index_name:   &String,
        start_height: i32,
        stop_height:  i32) -> bool {
    
    todo!();
        /*
            BlockFilterIndexDBHeightKey key(start_height);
        db_it.Seek(key);

        for (int height = start_height; height <= stop_height; ++height) {
            if (!db_it.GetKey(key) || key.height != height) {
                return error("%s: unexpected key in %s: expected (%c, %d)",
                             __func__, index_name, DB_BLOCK_HEIGHT, height);
            }

            std::pair<uint256, DBVal> value;
            if (!db_it.GetValue(value)) {
                return error("%s: unable to read value in %s at key (%c, %d)",
                             __func__, index_name, DB_BLOCK_HEIGHT, height);
            }

            batch.Write(BlockFilterIndexDBHashKey(value.first), std::move(value.second));

            db_it.Next();
        }
        return true;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/index/coinstatsindex.h]

/**
  | CoinStatsIndex maintains statistics
  | on the UTXO set.
  |
  */
pub struct CoinStatsIndex {
    base:                                 BaseIndex,
    name:                                 String,
    db:                                   Box<BaseIndexDB>,
    muhash:                               MuHash3072,
    transaction_output_count:             u64, // default = { 0 }
    bogo_size:                            u64, // default = { 0 }
    total_amount:                         Amount, // default = { 0 }
    total_subsidy:                        Amount, // default = { 0 }
    total_unspendable_amount:             Amount, // default = { 0 }
    total_prevout_spent_amount:           Amount, // default = { 0 }
    total_new_outputs_ex_coinbase_amount: Amount, // default = { 0 }
    total_coinbase_amount:                Amount, // default = { 0 }
    total_unspendables_genesis_block:     Amount, // default = { 0 }
    total_unspendables_bip30:             Amount, // default = { 0 }
    total_unspendables_scripts:           Amount, // default = { 0 }
    total_unspendables_unclaimed_rewards: Amount, // default = { 0 }
}

impl CoinStatsIndex {
    
    pub fn getdb(&self) -> &mut BaseIndexDB {
        
        todo!();
        /*
            return *m_db;
        */
    }
    
    pub fn get_name(&self) -> *const u8 {
        
        todo!();
        /*
            return "coinstatsindex";
        */
    }

    /**
      | Constructs the index, which becomes
      | available to be queried.
      |
      */
    pub fn new(
        n_cache_size: usize,
        memory:       Option<bool>,
        wipe:         Option<bool>) -> Self {
    
        let memory: bool = memory.unwrap_or(false);
        let wipe:   bool = wipe.unwrap_or(false);

        todo!();
        /*


            fs::path path{gArgs.GetDataDirNet() / "indexes" / "coinstats"};
        fs::create_directories(path);

        m_db = std::make_unique<CoinStatsIndex::DB>(path / "db", n_cache_size, f_memory, f_wipe);
        */
    }
    
    pub fn write_block(&mut self, 
        block:  &Block,
        pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            CBlockUndo block_undo;
        const CAmount block_subsidy{GetBlockSubsidy(pindex->nHeight, Params().GetConsensus())};
        m_total_subsidy += block_subsidy;

        // Ignore genesis block
        if (pindex->nHeight > 0) {
            if (!UndoReadFromDisk(block_undo, pindex)) {
                return false;
            }

            std::pair<uint256, DBVal> read_out;
            if (!m_db->Read(CoinStatsIndexDBHeightKey(pindex->nHeight - 1), read_out)) {
                return false;
            }

            uint256 expected_block_hash{pindex->pprev->GetBlockHash()};
            if (read_out.first != expected_block_hash) {
                LogPrintf("WARNING: previous block header belongs to unexpected block %s; expected %s\n",
                          read_out.first.ToString(), expected_block_hash.ToString());

                if (!m_db->Read(CoinStatsIndexDBHashKey(expected_block_hash), read_out)) {
                    return error("%s: previous block header not found; expected %s",
                                 __func__, expected_block_hash.ToString());
                }
            }

            // TODO: Deduplicate BIP30 related code
            bool is_bip30_block{(pindex->nHeight == 91722 && pindex->GetBlockHash() == uint256S("0x00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e")) ||
                                (pindex->nHeight == 91812 && pindex->GetBlockHash() == uint256S("0x00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f"))};

            // Add the new utxos created from the block
            for (size_t i = 0; i < block.vtx.size(); ++i) {
                const auto& tx{block.vtx.at(i)};

                // Skip duplicate txid coinbase transactions (BIP30).
                if (is_bip30_block && tx->IsCoinBase()) {
                    m_total_unspendable_amount += block_subsidy;
                    m_total_unspendables_bip30 += block_subsidy;
                    continue;
                }

                for (uint32_t j = 0; j < tx->vout.size(); ++j) {
                    const CTxOut& out{tx->vout[j]};
                    Coin coin{out, pindex->nHeight, tx->IsCoinBase()};
                    OutPoint outpoint{tx->GetHash(), j};

                    // Skip unspendable coins
                    if (coin.out.scriptPubKey.IsUnspendable()) {
                        m_total_unspendable_amount += coin.out.nValue;
                        m_total_unspendables_scripts += coin.out.nValue;
                        continue;
                    }

                    m_muhash.Insert(MakeUCharSpan(TxOutSer(outpoint, coin)));

                    if (tx->IsCoinBase()) {
                        m_total_coinbase_amount += coin.out.nValue;
                    } else {
                        m_total_new_outputs_ex_coinbase_amount += coin.out.nValue;
                    }

                    ++m_transaction_output_count;
                    m_total_amount += coin.out.nValue;
                    m_bogo_size += GetBogoSize(coin.out.scriptPubKey);
                }

                // The coinbase tx has no undo data since no former output is spent
                if (!tx->IsCoinBase()) {
                    const auto& tx_undo{block_undo.vtxundo.at(i - 1)};

                    for (size_t j = 0; j < tx_undo.vprevout.size(); ++j) {
                        Coin coin{tx_undo.vprevout[j]};
                        OutPoint outpoint{tx->vin[j].prevout.hash, tx->vin[j].prevout.n};

                        m_muhash.Remove(MakeUCharSpan(TxOutSer(outpoint, coin)));

                        m_total_prevout_spent_amount += coin.out.nValue;

                        --m_transaction_output_count;
                        m_total_amount -= coin.out.nValue;
                        m_bogo_size -= GetBogoSize(coin.out.scriptPubKey);
                    }
                }
            }
        } else {
            // genesis block
            m_total_unspendable_amount += block_subsidy;
            m_total_unspendables_genesis_block += block_subsidy;
        }

        // If spent prevouts + block subsidy are still a higher amount than
        // new outputs + coinbase + current unspendable amount this means
        // the miner did not claim the full block reward. Unclaimed block
        // rewards are also unspendable.
        const CAmount unclaimed_rewards{(m_total_prevout_spent_amount + m_total_subsidy) - (m_total_new_outputs_ex_coinbase_amount + m_total_coinbase_amount + m_total_unspendable_amount)};
        m_total_unspendable_amount += unclaimed_rewards;
        m_total_unspendables_unclaimed_rewards += unclaimed_rewards;

        std::pair<uint256, DBVal> value;
        value.first = pindex->GetBlockHash();
        value.second.transaction_output_count = m_transaction_output_count;
        value.second.bogo_size = m_bogo_size;
        value.second.total_amount = m_total_amount;
        value.second.total_subsidy = m_total_subsidy;
        value.second.total_unspendable_amount = m_total_unspendable_amount;
        value.second.total_prevout_spent_amount = m_total_prevout_spent_amount;
        value.second.total_new_outputs_ex_coinbase_amount = m_total_new_outputs_ex_coinbase_amount;
        value.second.total_coinbase_amount = m_total_coinbase_amount;
        value.second.total_unspendables_genesis_block = m_total_unspendables_genesis_block;
        value.second.total_unspendables_bip30 = m_total_unspendables_bip30;
        value.second.total_unspendables_scripts = m_total_unspendables_scripts;
        value.second.total_unspendables_unclaimed_rewards = m_total_unspendables_unclaimed_rewards;

        uint256 out;
        m_muhash.Finalize(out);
        value.second.muhash = out;

        CDBBatch batch(*m_db);
        batch.Write(CoinStatsIndexDBHeightKey(pindex->nHeight), value);
        batch.Write(DB_MUHASH, m_muhash);
        return m_db->WriteBatch(batch);
        */
    }
    
    pub fn rewind(&mut self, 
        current_tip: *const BlockIndex,
        new_tip:     *const BlockIndex) -> bool {
        
        todo!();
        /*
            assert(current_tip->GetAncestor(new_tip->nHeight) == new_tip);

        CDBBatch batch(*m_db);
        std::unique_ptr<CDBIterator> db_it(m_db->NewIterator());

        // During a reorg, we need to copy all hash digests for blocks that are
        // getting disconnected from the height index to the hash index so we can
        // still find them when the height index entries are overwritten.
        if (!CopyHeightIndexToHashIndex(*db_it, batch, m_name, new_tip->nHeight, current_tip->nHeight)) {
            return false;
        }

        if (!m_db->WriteBatch(batch)) return false;

        {
            LOCK(cs_main);
            CBlockIndex* iter_tip{m_chainstate->m_blockman.LookupBlockIndex(current_tip->GetBlockHash())};
            const auto& consensus_params{Params().GetConsensus()};

            do {
                CBlock block;

                if (!ReadBlockFromDisk(block, iter_tip, consensus_params)) {
                    return error("%s: Failed to read block %s from disk",
                                 __func__, iter_tip->GetBlockHash().ToString());
                }

                ReverseBlock(block, iter_tip);

                iter_tip = iter_tip->GetAncestor(iter_tip->nHeight - 1);
            } while (new_tip != iter_tip);
        }

        return BaseIndex::Rewind(current_tip, new_tip);
        */
    }
    
    /**
      | Look up stats for a specific block using
      | CBlockIndex
      |
      */
    pub fn look_up_stats(&self, 
        block_index: *const BlockIndex,
        coins_stats: &mut CoinsStats) -> bool {
        
        todo!();
        /*
            DBVal entry;
        if (!LookUpOne(*m_db, block_index, entry)) {
            return false;
        }

        coins_stats.hashSerialized = entry.muhash;
        coins_stats.nTransactionOutputs = entry.transaction_output_count;
        coins_stats.nBogoSize = entry.bogo_size;
        coins_stats.nTotalAmount = entry.total_amount;
        coins_stats.total_subsidy = entry.total_subsidy;
        coins_stats.total_unspendable_amount = entry.total_unspendable_amount;
        coins_stats.total_prevout_spent_amount = entry.total_prevout_spent_amount;
        coins_stats.total_new_outputs_ex_coinbase_amount = entry.total_new_outputs_ex_coinbase_amount;
        coins_stats.total_coinbase_amount = entry.total_coinbase_amount;
        coins_stats.total_unspendables_genesis_block = entry.total_unspendables_genesis_block;
        coins_stats.total_unspendables_bip30 = entry.total_unspendables_bip30;
        coins_stats.total_unspendables_scripts = entry.total_unspendables_scripts;
        coins_stats.total_unspendables_unclaimed_rewards = entry.total_unspendables_unclaimed_rewards;

        return true;
        */
    }
    
    pub fn init(&mut self) -> bool {
        
        todo!();
        /*
            if (!m_db->Read(DB_MUHASH, m_muhash)) {
            // Check that the cause of the read failure is that the key does not
            // exist. Any other errors indicate database corruption or a disk
            // failure, and starting the index would cause further corruption.
            if (m_db->Exists(DB_MUHASH)) {
                return error("%s: Cannot read current %s state; index may be corrupted",
                             __func__, GetName());
            }
        }

        if (!BaseIndex::Init()) return false;

        const CBlockIndex* pindex{CurrentIndex()};

        if (pindex) {
            DBVal entry;
            if (!LookUpOne(*m_db, pindex, entry)) {
                return false;
            }

            m_transaction_output_count = entry.transaction_output_count;
            m_bogo_size = entry.bogo_size;
            m_total_amount = entry.total_amount;
            m_total_subsidy = entry.total_subsidy;
            m_total_unspendable_amount = entry.total_unspendable_amount;
            m_total_prevout_spent_amount = entry.total_prevout_spent_amount;
            m_total_new_outputs_ex_coinbase_amount = entry.total_new_outputs_ex_coinbase_amount;
            m_total_coinbase_amount = entry.total_coinbase_amount;
            m_total_unspendables_genesis_block = entry.total_unspendables_genesis_block;
            m_total_unspendables_bip30 = entry.total_unspendables_bip30;
            m_total_unspendables_scripts = entry.total_unspendables_scripts;
            m_total_unspendables_unclaimed_rewards = entry.total_unspendables_unclaimed_rewards;
        }

        return true;
        */
    }

    /**
      | Reverse a single block as part of a reorg
      |
      */
    pub fn reverse_block(&mut self, 
        block:  &Block,
        pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            CBlockUndo block_undo;
        std::pair<uint256, DBVal> read_out;

        const CAmount block_subsidy{GetBlockSubsidy(pindex->nHeight, Params().GetConsensus())};
        m_total_subsidy -= block_subsidy;

        // Ignore genesis block
        if (pindex->nHeight > 0) {
            if (!UndoReadFromDisk(block_undo, pindex)) {
                return false;
            }

            if (!m_db->Read(CoinStatsIndexDBHeightKey(pindex->nHeight - 1), read_out)) {
                return false;
            }

            uint256 expected_block_hash{pindex->pprev->GetBlockHash()};
            if (read_out.first != expected_block_hash) {
                LogPrintf("WARNING: previous block header belongs to unexpected block %s; expected %s\n",
                          read_out.first.ToString(), expected_block_hash.ToString());

                if (!m_db->Read(CoinStatsIndexDBHashKey(expected_block_hash), read_out)) {
                    return error("%s: previous block header not found; expected %s",
                                 __func__, expected_block_hash.ToString());
                }
            }
        }

        // Remove the new UTXOs that were created from the block
        for (size_t i = 0; i < block.vtx.size(); ++i) {
            const auto& tx{block.vtx.at(i)};

            for (uint32_t j = 0; j < tx->vout.size(); ++j) {
                const CTxOut& out{tx->vout[j]};
                OutPoint outpoint{tx->GetHash(), j};
                Coin coin{out, pindex->nHeight, tx->IsCoinBase()};

                // Skip unspendable coins
                if (coin.out.scriptPubKey.IsUnspendable()) {
                    m_total_unspendable_amount -= coin.out.nValue;
                    m_total_unspendables_scripts -= coin.out.nValue;
                    continue;
                }

                m_muhash.Remove(MakeUCharSpan(TxOutSer(outpoint, coin)));

                if (tx->IsCoinBase()) {
                    m_total_coinbase_amount -= coin.out.nValue;
                } else {
                    m_total_new_outputs_ex_coinbase_amount -= coin.out.nValue;
                }

                --m_transaction_output_count;
                m_total_amount -= coin.out.nValue;
                m_bogo_size -= GetBogoSize(coin.out.scriptPubKey);
            }

            // The coinbase tx has no undo data since no former output is spent
            if (!tx->IsCoinBase()) {
                const auto& tx_undo{block_undo.vtxundo.at(i - 1)};

                for (size_t j = 0; j < tx_undo.vprevout.size(); ++j) {
                    Coin coin{tx_undo.vprevout[j]};
                    OutPoint outpoint{tx->vin[j].prevout.hash, tx->vin[j].prevout.n};

                    m_muhash.Insert(MakeUCharSpan(TxOutSer(outpoint, coin)));

                    m_total_prevout_spent_amount -= coin.out.nValue;

                    m_transaction_output_count++;
                    m_total_amount += coin.out.nValue;
                    m_bogo_size += GetBogoSize(coin.out.scriptPubKey);
                }
            }
        }

        const CAmount unclaimed_rewards{(m_total_new_outputs_ex_coinbase_amount + m_total_coinbase_amount + m_total_unspendable_amount) - (m_total_prevout_spent_amount + m_total_subsidy)};
        m_total_unspendable_amount -= unclaimed_rewards;
        m_total_unspendables_unclaimed_rewards -= unclaimed_rewards;

        // Check that the rolled back internal values are consistent with the DB read out
        uint256 out;
        m_muhash.Finalize(out);
        Assert(read_out.second.muhash == out);

        Assert(m_transaction_output_count == read_out.second.transaction_output_count);
        Assert(m_total_amount == read_out.second.total_amount);
        Assert(m_bogo_size == read_out.second.bogo_size);
        Assert(m_total_subsidy == read_out.second.total_subsidy);
        Assert(m_total_unspendable_amount == read_out.second.total_unspendable_amount);
        Assert(m_total_prevout_spent_amount == read_out.second.total_prevout_spent_amount);
        Assert(m_total_new_outputs_ex_coinbase_amount == read_out.second.total_new_outputs_ex_coinbase_amount);
        Assert(m_total_coinbase_amount == read_out.second.total_coinbase_amount);
        Assert(m_total_unspendables_genesis_block == read_out.second.total_unspendables_genesis_block);
        Assert(m_total_unspendables_bip30 == read_out.second.total_unspendables_bip30);
        Assert(m_total_unspendables_scripts == read_out.second.total_unspendables_scripts);
        Assert(m_total_unspendables_unclaimed_rewards == read_out.second.total_unspendables_unclaimed_rewards);

        return m_db->Write(DB_MUHASH, m_muhash);
        */
    }
}

/**
  | The global UTXO set hash object.
  |
  */
lazy_static!{
    /*
    extern std::unique_ptr<CoinStatsIndex> g_coin_stats_index;
    */
}

//-------------------------------------------[.cpp/bitcoin/src/index/coinstatsindex.cpp]

pub const db_block_hash:   char = 's';
pub const db_block_height: char = 't';
pub const db_muhash:       char = 'M';

pub struct CoinStatsIndexDBVal {
    muhash:                               u256,
    transaction_output_count:             u64,
    bogo_size:                            u64,
    total_amount:                         Amount,
    total_subsidy:                        Amount,
    total_unspendable_amount:             Amount,
    total_prevout_spent_amount:           Amount,
    total_new_outputs_ex_coinbase_amount: Amount,
    total_coinbase_amount:                Amount,
    total_unspendables_genesis_block:     Amount,
    total_unspendables_bip30:             Amount,
    total_unspendables_scripts:           Amount,
    total_unspendables_unclaimed_rewards: Amount,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CoinStatsIndexDBVal, obj)
        {
            READWRITE(obj.muhash);
            READWRITE(obj.transaction_output_count);
            READWRITE(obj.bogo_size);
            READWRITE(obj.total_amount);
            READWRITE(obj.total_subsidy);
            READWRITE(obj.total_unspendable_amount);
            READWRITE(obj.total_prevout_spent_amount);
            READWRITE(obj.total_new_outputs_ex_coinbase_amount);
            READWRITE(obj.total_coinbase_amount);
            READWRITE(obj.total_unspendables_genesis_block);
            READWRITE(obj.total_unspendables_bip30);
            READWRITE(obj.total_unspendables_scripts);
            READWRITE(obj.total_unspendables_unclaimed_rewards);
        }
    */
}

///-------------------------
pub struct CoinStatsIndexDBHeightKey {
    height: i32,
}

impl CoinStatsIndexDBHeightKey {

    pub fn new(height_in: i32) -> Self {
    
        todo!();
        /*
        : height(height_in),

        
        */
    }
    
    pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            ser_writedata8(s, DB_BLOCK_HEIGHT);
            ser_writedata32be(s, height);
        */
    }
    
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            const uint8_t prefix{ser_readdata8(s)};
            if (prefix != DB_BLOCK_HEIGHT) {
                throw std::ios_base::failure("Invalid format for coinstatsindex DB height key");
            }
            height = ser_readdata32be(s);
        */
    }
}

///---------------------------
pub struct CoinStatsIndexDBHashKey {
    block_hash: u256,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CoinStatsIndexDBHashKey, obj)
        {
            uint8_t prefix{DB_BLOCK_HASH};
            READWRITE(prefix);
            if (prefix != DB_BLOCK_HASH) {
                throw std::ios_base::failure("Invalid format for coinstatsindex DB hash key");
            }

            READWRITE(obj.block_hash);
        }
    */
}

impl CoinStatsIndexDBHashKey {
    
    pub fn new(hash_in: &u256) -> Self {
    
        todo!();
        /*
        : block_hash(hash_in),

        
        */
    }
}

///---------------------------
lazy_static!{
    /*
    std::unique_ptr<CoinStatsIndex> g_coin_stats_index;
    */
}

pub fn look_up_one(
    db:          &DBWrapper,
    block_index: *const BlockIndex,
    result:      &mut CoinStatsIndexDBVal) -> bool {
    
    todo!();
        /*
            // First check if the result is stored under the height index and the value
        // there matches the block hash. This should be the case if the block is on
        // the active chain.
        std::pair<uint256, DBVal> read_out;
        if (!db.Read(CoinStatsIndexDBHeightKey(block_index->nHeight), read_out)) {
            return false;
        }
        if (read_out.first == block_index->GetBlockHash()) {
            result = std::move(read_out.second);
            return true;
        }

        // If value at the height index corresponds to an different block, the
        // result will be stored in the hash index.
        return db.Read(CoinStatsIndexDBHashKey(block_index->GetBlockHash()), result);
        */
}

pub fn coin_stats_index_copy_height_index_to_hash_index(
    db_it:        &mut DBIterator,
    batch:        &mut DBBatch,
    index_name:   &String,
    start_height: i32,
    stop_height:  i32) -> bool {

    todo!();
        /*
            CoinStatsIndexDBHeightKey key{start_height};
        db_it.Seek(key);

        for (int height = start_height; height <= stop_height; ++height) {
            if (!db_it.GetKey(key) || key.height != height) {
                return error("%s: unexpected key in %s: expected (%c, %d)",
                             __func__, index_name, DB_BLOCK_HEIGHT, height);
            }

            std::pair<uint256, DBVal> value;
            if (!db_it.GetValue(value)) {
                return error("%s: unable to read value in %s at key (%c, %d)",
                             __func__, index_name, DB_BLOCK_HEIGHT, height);
            }

            batch.Write(CoinStatsIndexDBHashKey(value.first), std::move(value.second));

            db_it.Next();
        }
        return true;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/index/disktxpos.h]

pub struct DiskTxPos {
    base: FlatFilePos,

    /**
      | after header
      |
      */
    n_tx_offset: u32,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CDiskTxPos, obj)
        {
            READWRITEAS(FlatFilePos, obj);
            READWRITE(VARINT(obj.nTxOffset));
        }
    */
}

impl Default for DiskTxPos {
    
    fn default() -> Self {
        todo!();
        /*


            SetNull();
        */
    }
}

impl DiskTxPos {

    pub fn new(
        block_in:       &FlatFilePos,
        n_tx_offset_in: u32) -> Self {
    
        todo!();
        /*
        : flat_file_pos(blockIn.nFile, blockIn.nPos),
        : n_tx_offset(nTxOffsetIn),

        
        */
    }
    
    pub fn set_null(&mut self)  {
        
        todo!();
        /*
            FlatFilePos::SetNull();
            nTxOffset = 0;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/index/txindex.h]

/**
  | TxIndex is used to look up transactions
  | included in the blockchain by hash.
  | 
  | The index is written to a LevelDB database
  | and records the filesystem location
  | of each transaction by transaction
  | hash.
  |
  */
pub struct TxIndex {
    base: BaseIndex,
    db:   Box<TxIndexDB>,
}

impl Drop for TxIndex {

    /**
      | Destructor is declared because this
      | class contains a unique_ptr to an incomplete
      | type.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
        
        */
    }
}

impl TxIndex {

    pub fn get_name(&self) -> *const u8 {
        
        todo!();
        /*
            return "txindex";
        */
    }

    /**
      | Constructs the index, which becomes
      | available to be queried.
      |
      */
    pub fn new(
        n_cache_size: usize,
        memory:       Option<bool>,
        wipe:         Option<bool>) -> Self {

        let memory: bool = memory.unwrap_or(false);
        let wipe:   bool = wipe.unwrap_or(false);
    
        todo!();
        /*
           : m_db(std::make_unique<TxIndex::DB>(n_cache_size, f_memory, f_wipe))
           */
    }
    
    pub fn write_block(&mut self, 
        block:  &Block,
        pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            // Exclude genesis block transaction because outputs are not spendable.
        if (pindex->nHeight == 0) return true;

        CDiskTxPos pos(pindex->GetBlockPos(), GetSizeOfCompactSize(block.vtx.size()));
        std::vector<std::pair<uint256, CDiskTxPos>> vPos;
        vPos.reserve(block.vtx.size());
        for (const auto& tx : block.vtx) {
            vPos.emplace_back(tx->GetHash(), pos);
            pos.nTxOffset += ::GetSerializeSize(*tx, CLIENT_VERSION);
        }
        return m_db->WriteTxs(vPos);
        */
    }
    
    pub fn getdb(&self) -> &mut BaseIndexDB {
        
        todo!();
        /*
            return *m_db;
        */
    }
    
    /**
      | Look up a transaction by hash.
      | 
      | -----------
      | @param[in] tx_hash
      | 
      | The hash of the transaction to be returned.
      | ----------
      | @param[out] block_hash
      | 
      | The hash of the block the transaction
      | is found in.
      | ----------
      | @param[out] tx
      | 
      | The transaction itself.
      | 
      | -----------
      | @return
      | 
      | true if transaction is found, false
      | otherwise
      |
      */
    pub fn find_tx(&self, 
        tx_hash:    &u256,
        block_hash: &mut u256,
        tx:         &mut TransactionRef) -> bool {
        
        todo!();
        /*
            CDiskTxPos postx;
        if (!m_db->ReadTxPos(tx_hash, postx)) {
            return false;
        }

        CAutoFile file(OpenBlockFile(postx, true), SER_DISK, CLIENT_VERSION);
        if (file.IsNull()) {
            return error("%s: OpenBlockFile failed", __func__);
        }
        CBlockHeader header;
        try {
            file >> header;
            if (fseek(file.Get(), postx.nTxOffset, SEEK_CUR)) {
                return error("%s: fseek(...) failed", __func__);
            }
            file >> tx;
        } catch (const std::exception& e) {
            return error("%s: Deserialize or I/O error - %s", __func__, e.what());
        }
        if (tx->GetHash() != tx_hash) {
            return error("%s: txid mismatch", __func__);
        }
        block_hash = header.GetHash();
        return true;
        */
    }
}

/**
  | Access to the txindex database
  | (indexes/txindex/)
  |
  */
pub struct TxIndexDB {
    base: BaseIndexDB,
}

impl TxIndexDB {

    pub fn new(
        n_cache_size: usize,
        memory:       Option<bool>,
        wipe:         Option<bool>) -> Self {

        let memory: bool = memory.unwrap_or(false);
        let wipe:   bool = wipe.unwrap_or(false);
    
        todo!();
        /*
            : BaseIndexDB(gArgs.GetDataDirNet() / "indexes" / "txindex", n_cache_size, f_memory, f_wipe)
        */
    }
    
    /**
      | Read the disk location of the transaction
      | data with the given hash. Returns false
      | if the transaction hash is not indexed.
      |
      */
    pub fn read_tx_pos(&self, 
        txid: &u256,
        pos:  &mut DiskTxPos) -> bool {
        
        todo!();
        /*
            return Read(std::make_pair(DB_TXINDEX, txid), pos);
        */
    }
    
    /**
      | Write a batch of transaction positions
      | to the DB.
      |
      */
    pub fn write_txs(&mut self, pos: &Vec<(u256,DiskTxPos)>) -> bool {
        
        todo!();
        /*
            CDBBatch batch(*this);
        for (const auto& tuple : v_pos) {
            batch.Write(std::make_pair(DB_TXINDEX, tuple.first), tuple.second);
        }
        return WriteBatch(batch);
        */
    }
}

/**
  | The global transaction index, used
  | in GetTransaction. May be null.
  |
  */
lazy_static!{
    /*
    extern std::unique_ptr<TxIndex> g_txindex;
    std::unique_ptr<TxIndex> g_txindex;
    */
}

//-------------------------------------------[.cpp/bitcoin/src/index/txindex.cpp]

pub const DB_TXINDEX: char = 't';
