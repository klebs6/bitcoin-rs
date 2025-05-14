// ---------------- [ File: bitcoin-blockfilter/src/index.rs ]
crate::ix!();

/**
  | BlockFilterIndex is used to store and
  | retrieve block filters, hashes, and
  | headers for a range of blocks by height.
  | An index is constructed for each supported
  | filter type with its own database (ie.
  | filter data for different types are
  | stored in separate databases).
  | 
  | This index is used to serve BIP 157 net
  | requests.
  |
  */
pub struct BlockFilterIndex {
    base:             BaseIndex,
    filter_type:      BlockFilterType,
    name:             String,
    db:               Box<BaseIndexDB>,
    next_filter_pos:  FlatFilePos,
    filter_fileseq:   Box<FlatFileSeq>,
    cs_headers_cache: std::sync::Mutex<BlockFilterIndexInner>,
}

pub struct BlockFilterIndexInner {

    /**
      | cache of block hash to filter header,
      | to avoid disk access when responding
      | to getcfcheckpt.
      |
      */
    headers_cache: HashMap<u256,u256,FilterHeaderHasher>,
}

impl BlockFilterIndex {

    pub fn getdb(&self) -> &mut BaseIndexDB {
        
        todo!();
        /*
            return *m_db;
        */
    }
    
    pub fn get_name(&self) -> *const u8 {
        
        todo!();
        /*
            return m_name.c_str();
        */
    }

    pub fn get_filter_type(&self) -> BlockFilterType {
        
        todo!();
        /*
            return m_filter_type;
        */
    }

    /**
      | Constructs the index, which becomes
      | available to be queried.
      |
      */
    pub fn new(
        filter_type:  BlockFilterType,
        n_cache_size: usize,
        memory:       Option<bool>,
        wipe:         Option<bool>) -> Self {

        let memory: bool = memory.unwrap_or(false);
        let wipe:   bool = wipe.unwrap_or(false);
    
        todo!();
        /*
        : filter_type(filter_type),

            const std::string& filter_name = BlockFilterTypeName(filter_type);
        if (filter_name.empty()) throw std::invalid_argument("unknown filter_type");

        fs::path path = gArgs.GetDataDirNet() / "indexes" / "blockfilter" / filter_name;
        fs::create_directories(path);

        m_name = filter_name + " block filter index";
        m_db = std::make_unique<BaseIndexDB>(path / "db", n_cache_size, f_memory, f_wipe);
        m_filter_fileseq = std::make_unique<FlatFileSeq>(std::move(path), "fltr", FLTR_FILE_CHUNK_SIZE);
        */
    }
    
    pub fn init(&mut self) -> bool {
        
        todo!();
        /*
            if (!m_db->Read(DB_FILTER_POS, m_next_filter_pos)) {
            // Check that the cause of the read failure is that the key does not exist. Any other errors
            // indicate database corruption or a disk failure, and starting the index would cause
            // further corruption.
            if (m_db->Exists(DB_FILTER_POS)) {
                return error("%s: Cannot read current %s state; index may be corrupted",
                             __func__, GetName());
            }

            // If the DB_FILTER_POS is not set, then initialize to the first location.
            m_next_filter_pos.nFile = 0;
            m_next_filter_pos.nPos = 0;
        }
        return BaseIndex::Init();
        */
    }
    
    pub fn commit_internal(&mut self, batch: &mut DBBatch) -> bool {
        
        todo!();
        /*
            const FlatFilePos& pos = m_next_filter_pos;

        // Flush current filter file to disk.
        CAutoFile file(m_filter_fileseq->Open(pos), SER_DISK, CLIENT_VERSION);
        if (file.IsNull()) {
            return error("%s: Failed to open filter file %d", __func__, pos.nFile);
        }
        if (!FileCommit(file.Get())) {
            return error("%s: Failed to commit filter file %d", __func__, pos.nFile);
        }

        batch.Write(DB_FILTER_POS, pos);
        return BaseIndex::CommitInternal(batch);
        */
    }
    
    pub fn read_filter_from_disk(&self, 
        pos:    &FlatFilePos,
        filter: &mut BlockFilter) -> bool {
        
        todo!();
        /*
            CAutoFile filein(m_filter_fileseq->Open(pos, true), SER_DISK, CLIENT_VERSION);
        if (filein.IsNull()) {
            return false;
        }

        uint256 block_hash;
        std::vector<uint8_t> encoded_filter;
        try {
            filein >> block_hash >> encoded_filter;
            filter = BlockFilter(GetFilterType(), block_hash, std::move(encoded_filter));
        }
        catch (const std::exception& e) {
            return error("%s: Failed to deserialize block filter from disk: %s", __func__, e.what());
        }

        return true;
        */
    }
    
    pub fn write_filter_to_disk(&mut self, 
        pos:    &mut FlatFilePos,
        filter: &BlockFilter) -> usize {
        
        todo!();
        /*
            assert(filter.GetFilterType() == GetFilterType());

        size_t data_size =
            GetSerializeSize(filter.GetBlockHash(), CLIENT_VERSION) +
            GetSerializeSize(filter.GetEncodedFilter(), CLIENT_VERSION);

        // If writing the filter would overflow the file, flush and move to the next one.
        if (pos.nPos + data_size > MAX_FLTR_FILE_SIZE) {
            CAutoFile last_file(m_filter_fileseq->Open(pos), SER_DISK, CLIENT_VERSION);
            if (last_file.IsNull()) {
                LogPrintf("%s: Failed to open filter file %d\n", __func__, pos.nFile);
                return 0;
            }
            if (!TruncateFile(last_file.Get(), pos.nPos)) {
                LogPrintf("%s: Failed to truncate filter file %d\n", __func__, pos.nFile);
                return 0;
            }
            if (!FileCommit(last_file.Get())) {
                LogPrintf("%s: Failed to commit filter file %d\n", __func__, pos.nFile);
                return 0;
            }

            pos.nFile++;
            pos.nPos = 0;
        }

        // Pre-allocate sufficient space for filter data.
        bool out_of_space;
        m_filter_fileseq->Allocate(pos, data_size, out_of_space);
        if (out_of_space) {
            LogPrintf("%s: out of disk space\n", __func__);
            return 0;
        }

        CAutoFile fileout(m_filter_fileseq->Open(pos), SER_DISK, CLIENT_VERSION);
        if (fileout.IsNull()) {
            LogPrintf("%s: Failed to open filter file %d\n", __func__, pos.nFile);
            return 0;
        }

        fileout << filter.GetBlockHash() << filter.GetEncodedFilter();
        return data_size;
        */
    }
    
    pub fn write_block(&mut self, 
        block:  &Block,
        pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            CBlockUndo block_undo;
        uint256 prev_header;

        if (pindex->nHeight > 0) {
            if (!UndoReadFromDisk(block_undo, pindex)) {
                return false;
            }

            std::pair<uint256, DBVal> read_out;
            if (!m_db->Read(BlockFilterIndexDBHeightKey(pindex->nHeight - 1), read_out)) {
                return false;
            }

            uint256 expected_block_hash = pindex->pprev->GetBlockHash();
            if (read_out.first != expected_block_hash) {
                return error("%s: previous block header belongs to unexpected block %s; expected %s",
                             __func__, read_out.first.ToString(), expected_block_hash.ToString());
            }

            prev_header = read_out.second.header;
        }

        BlockFilter filter(m_filter_type, block, block_undo);

        size_t bytes_written = WriteFilterToDisk(m_next_filter_pos, filter);
        if (bytes_written == 0) return false;

        std::pair<uint256, DBVal> value;
        value.first = pindex->GetBlockHash();
        value.second.hash = filter.GetHash();
        value.second.header = filter.ComputeHeader(prev_header);
        value.second.pos = m_next_filter_pos;

        if (!m_db->Write(BlockFilterIndexDBHeightKey(pindex->nHeight), value)) {
            return false;
        }

        m_next_filter_pos.nPos += bytes_written;
        return true;
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

        // During a reorg, we need to copy all filters for blocks that are getting disconnected from the
        // height index to the hash index so we can still find them when the height index entries are
        // overwritten.
        if (!CopyHeightIndexToHashIndex(*db_it, batch, m_name, new_tip->nHeight, current_tip->nHeight)) {
            return false;
        }

        // The latest filter position gets written in Commit by the call to the BaseIndex::Rewind.
        // But since this creates new references to the filter, the position should get updated here
        // atomically as well in case Commit fails.
        batch.Write(DB_FILTER_POS, m_next_filter_pos);
        if (!m_db->WriteBatch(batch)) return false;

        return BaseIndex::Rewind(current_tip, new_tip);
        */
    }
    
    /**
      | Get a single filter by block.
      |
      */
    pub fn lookup_filter(&self, 
        block_index: Arc<BlockIndex>,
        filter_out:  &mut BlockFilter) -> bool {
        
        todo!();
        /*
            DBVal entry;
        if (!LookupOne(*m_db, block_index, entry)) {
            return false;
        }

        return ReadFilterFromDisk(entry.pos, filter_out);
        */
    }
    
    /**
      | Get a single filter header by block.
      |
      */
    pub fn lookup_filter_header(&mut self, 
        block_index: Option<Arc<BlockIndex>>,
        header_out:  &mut u256) -> bool {
        
        todo!();
        /*
            LOCK(m_cs_headers_cache);

        bool is_checkpoint{block_index->nHeight % CFCHECKPT_INTERVAL == 0};

        if (is_checkpoint) {
            // Try to find the block in the headers cache if this is a checkpoint height.
            auto header = m_headers_cache.find(block_index->GetBlockHash());
            if (header != m_headers_cache.end()) {
                header_out = header->second;
                return true;
            }
        }

        DBVal entry;
        if (!LookupOne(*m_db, block_index, entry)) {
            return false;
        }

        if (is_checkpoint &&
            m_headers_cache.size() < CF_HEADERS_CACHE_MAX_SZ) {
            // Add to the headers cache if this is a checkpoint height.
            m_headers_cache.emplace(block_index->GetBlockHash(), entry.header);
        }

        header_out = entry.header;
        return true;
        */
    }
    
    /**
      | Get a range of filters between two heights
      | on a chain.
      |
      */
    pub fn lookup_filter_range(&self, 
        start_height: i32,
        stop_index:   Option<Arc<BlockIndex>>,
        filters_out:  &mut Vec<BlockFilter>) -> bool {
        
        todo!();
        /*
            std::vector<DBVal> entries;
        if (!LookupRange(*m_db, m_name, start_height, stop_index, entries)) {
            return false;
        }

        filters_out.resize(entries.size());
        auto filter_pos_it = filters_out.begin();
        for (const auto& entry : entries) {
            if (!ReadFilterFromDisk(entry.pos, *filter_pos_it)) {
                return false;
            }
            ++filter_pos_it;
        }

        return true;
        */
    }
    
    /**
      | Get a range of filter hashes between
      | two heights on a chain.
      |
      */
    pub fn lookup_filter_hash_range(&self, 
        start_height: i32,
        stop_index:   Option<Arc<BlockIndex>>,
        hashes_out:   &mut Vec<u256>) -> bool {
        
        todo!();
        /*
            std::vector<DBVal> entries;
        if (!LookupRange(*m_db, m_name, start_height, stop_index, entries)) {
            return false;
        }

        hashes_out.clear();
        hashes_out.reserve(entries.size());
        for (const auto& entry : entries) {
            hashes_out.push_back(entry.hash);
        }
        return true;
        */
    }
}
