crate::ix!();

/**
  | Access to the block database (blocks/index/)
  |
  */
pub struct BlockTreeDB {
    base: DBWrapper,
}

impl BlockTreeDB {
    
    pub fn write_batch_sync(&mut self, 
        file_info:   &Vec<(i32,*const BlockFileInfo)>,
        n_last_file: i32,
        blockinfo:   &Vec<*const BlockIndex>) -> bool {
        
        todo!();
        /*
            CDBBatch batch(*this);
        for (std::vector<std::pair<int, const BlockFileInfo*> >::const_iterator it=fileInfo.begin(); it != fileInfo.end(); it++) {
            batch.Write(std::make_pair(DB_BLOCK_FILES, it->first), *it->second);
        }
        batch.Write(DB_LAST_BLOCK, nLastFile);
        for (std::vector<const BlockIndex*>::const_iterator it=blockinfo.begin(); it != blockinfo.end(); it++) {
            batch.Write(std::make_pair(DB_BLOCK_INDEX, (*it)->GetBlockHash()), CDiskBlockIndex(*it));
        }
        return WriteBatch(batch, true);
        */
    }
    
    pub fn write_flag(&mut self, 
        name:  &String,
        value: bool) -> bool {
        
        todo!();
        /*
            return Write(std::make_pair(DB_FLAG, name), fValue ? uint8_t{'1'} : uint8_t{'0'});
        */
    }
    
    pub fn read_flag(&mut self, 
        name:  &String,
        value: &mut bool) -> bool {
        
        todo!();
        /*
            uint8_t ch;
        if (!Read(std::make_pair(DB_FLAG, name), ch))
            return false;
        fValue = ch == uint8_t{'1'};
        return true;
        */
    }
    
    pub fn load_block_index_guts(&mut self, 
        consensus_params:   &ChainConsensusParams,
        insert_block_index: fn(_0: &u256) -> *mut BlockIndex) -> bool {
        
        todo!();
        /*
            std::unique_ptr<CDBIterator> pcursor(NewIterator());

        pcursor->Seek(std::make_pair(DB_BLOCK_INDEX, uint256()));

        // Load m_block_index
        while (pcursor->Valid()) {
            if (ShutdownRequested()) return false;
            std::pair<uint8_t, uint256> key;
            if (pcursor->GetKey(key) && key.first == DB_BLOCK_INDEX) {
                CDiskBlockIndex diskindex;
                if (pcursor->GetValue(diskindex)) {
                    // Construct block index object
                    BlockIndex* pindexNew = insertBlockIndex(diskindex.GetBlockHash());
                    pindexNew->pprev          = insertBlockIndex(diskindex.hashPrev);
                    pindexNew->nHeight        = diskindex.nHeight;
                    pindexNew->nFile          = diskindex.nFile;
                    pindexNew->nDataPos       = diskindex.nDataPos;
                    pindexNew->nUndoPos       = diskindex.nUndoPos;
                    pindexNew->nVersion       = diskindex.nVersion;
                    pindexNew->hashMerkleRoot = diskindex.hashMerkleRoot;
                    pindexNew->nTime          = diskindex.nTime;
                    pindexNew->nBits          = diskindex.nBits;
                    pindexNew->nNonce         = diskindex.nNonce;
                    pindexNew->nStatus        = diskindex.nStatus;
                    pindexNew->nTx            = diskindex.nTx;

                    if (!CheckProofOfWork(pindexNew->GetBlockHash(), pindexNew->nBits, consensusParams))
                        return error("%s: CheckProofOfWork failed: %s", __func__, pindexNew->ToString());

                    pcursor->Next();
                } else {
                    return error("%s: failed to read value", __func__);
                }
            } else {
                break;
            }
        }

        return true;
        */
    }

    pub fn new(
        n_cache_size: usize,
        memory:       Option<bool>,
        wipe:         Option<bool>) -> Self {
        let memory: bool = memory.unwrap_or(false);
        let wipe:   bool = wipe.unwrap_or(false);
    
        todo!();
        /*


            : CDBWrapper(gArgs.GetDataDirNet() / "blocks" / "index", nCacheSize, fMemory, fWipe)
        */
    }
    
    pub fn read_block_file_info(&mut self, 
        n_file: i32,
        info:   &mut BlockFileInfo) -> bool {
        
        todo!();
        /*
            return Read(std::make_pair(DB_BLOCK_FILES, nFile), info);
        */
    }
    
    pub fn write_reindexing(&mut self, reindexing: bool) -> bool {
        
        todo!();
        /*
            if (fReindexing)
            return Write(DB_REINDEX_FLAG, uint8_t{'1'});
        else
            return Erase(DB_REINDEX_FLAG);
        */
    }
    
    pub fn read_reindexing(&mut self, reindexing: &mut bool)  {
        
        todo!();
        /*
            fReindexing = Exists(DB_REINDEX_FLAG);
        */
    }
    
    pub fn read_last_block_file(&mut self, n_file: &mut i32) -> bool {
        
        todo!();
        /*
            return Read(DB_LAST_BLOCK, nFile);
        */
    }
}
