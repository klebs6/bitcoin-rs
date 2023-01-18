crate::ix!();

/**
  | A convenience class for constructing
  | the CCoinsView* hierarchy used to facilitate
  | access to the UTXO set.
  | 
  | This class consists of an arrangement
  | of layered CCoinsView objects, preferring
  | to store and retrieve coins in memory
  | via `m_cacheview` but ultimately falling
  | back on cache misses to the canonical
  | store of UTXOs on disk, `m_dbview`.
  |
  */
//TODO: #[GUARDED_BY(cs_main)]
pub struct CoinsViews {

    /**
      | The lowest level of the CoinsViews cache
      | hierarchy sits in a leveldb database on
      | disk.
      |
      | All unspent coins reside in this store.
      */
    dbview:      CoinsViewDB,

    /**
      | This view wraps access to the leveldb
      | instance and handles read errors
      | gracefully.
      |
      */
    catcherview: CoinsViewErrorCatcher,

    /**
      | This is the top layer of the cache hierarchy
      | - it keeps as many coins in memory as can
      | fit per the dbcache setting.
      |
      */
    cacheview:   Box<CoinsViewCache>,
}

impl CoinsViews {
    
    /**
      | This constructor initializes CCoinsViewDB
      | and CCoinsViewErrorCatcher instances, but
      | it *does not* create a CCoinsViewCache
      | instance by default. This is done
      | separately because the presence of the
      | cache has implications on whether or not
      | we're allowed to flush the cache's state
      | to disk, which should not be done until
      | the health of the database is verified.
      |
      | All arguments forwarded onto CCoinsViewDB.
      */
    pub fn new(
        ldb_name:         String,
        cache_size_bytes: usize,
        in_memory:        bool,
        should_wipe:      bool) -> Self {
    
        todo!();
        /*
           : m_dbview(
           gArgs.GetDataDirNet() / ldb_name, cache_size_bytes, in_memory, should_wipe),
           m_catcherview(&m_dbview)
           */
    }
    
    /**
      | Initialize the CCoinsViewCache member.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn init_cache(&mut self)  {
        
        todo!();
        /*
            m_cacheview = std::make_unique<CCoinsViewCache>(&m_catcherview);
        */
    }
}
