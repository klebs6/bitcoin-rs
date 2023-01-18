crate::ix!();

/**
  | Unload database information
  |
  | May NOT be used after any connections
  | are up as much of the peer-processing
  | logic assumes a consistent block index
  | state
  |
  */
pub fn unload_block_index(
        mempool:  Arc<Mutex<TxMemPool>>,
        chainman: &mut ChainstateManager)  {
    
    todo!();
        /*
            LOCK(cs_main);
        chainman.Unload();
        pindexBestInvalid = nullptr;
        pindexBestHeader = nullptr;
        if (mempool) mempool->clear();
        vinfoBlockFile.clear();
        nLastBlockFile = 0;
        setDirtyBlockIndex.clear();
        setDirtyFileInfo.clear();
        g_versionbitscache.Clear();
        for (int b = 0; b < VERSIONBITS_NUM_BITS; b++) {
            warningcache[b].clear();
        }
        fHavePruned = false;
        */
}


