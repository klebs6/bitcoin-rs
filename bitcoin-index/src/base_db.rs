crate::ix!();

pub trait GetDB {
    fn getdb(&self) -> &mut BaseIndexDB;
}

/// The database stores a block locator of the chain the database is synced to so that the index
/// can efficiently determine the point it last stopped at.
/// 
/// A locator is used instead of a simple hash of the chain tip because blocks and block index
/// entries may not be flushed to disk until after this database is updated.
/// 
pub struct BaseIndexDB {
    base: DBWrapper,
}

impl BaseIndexDB {

    pub fn new(
        path:         &Path,
        n_cache_size: usize,
        memory:       Option<bool>,
        wipe:         Option<bool>,
        obfuscate:    Option<bool>) -> Self {

        let memory:    bool = memory.unwrap_or(false);
        let wipe:      bool = wipe.unwrap_or(false);
        let obfuscate: bool = obfuscate.unwrap_or(false);
    
        todo!();
        /*
        : cdb_wrapper(path, n_cache_size, f_memory, f_wipe, f_obfuscate),
        */
    }
    
    /**
      | Read block locator of the chain that
      | the txindex is in sync with.
      |
      */
    pub fn read_best_block(&self, locator: &mut BlockLocator) -> bool {
        
        todo!();

        /*
        let success: bool = read(DB_BEST_BLOCK,locator);

        if !success {
            locator.set_null();
        }

        success
        */
    }
    
    /**
      | Write block locator of the chain that
      | the txindex is in sync with.
      |
      */
    pub fn write_best_block(&mut self, 
        batch:   &mut DBBatch,
        locator: &BlockLocator)  {
        
        todo!();

        /*
        batch.write(DB_BEST_BLOCK, locator);
        */
    }
}
