// ---------------- [ File: bitcoinleveldb-dbimpl/src/create.rs ]
crate::ix!();

impl DBImpl {
    
    /*
      | Extra methods (for testing) that are
      | not in the public DB interface
      |
      */

    pub fn user_comparator(&self) -> Box<dyn SliceComparator> {
        
        todo!();
        /*
            return internal_comparator_.user_comparator();
        */
    }
    
    pub fn new(
        raw_options: &Options,
        dbname:      &String) -> Self {
    
        todo!();
        /*


            : env_(raw_options.env),
          internal_comparator_(raw_options.comparator),
          internal_filter_policy_(raw_options.filter_policy),
          options_(SanitizeOptions(dbname, &internal_comparator_,
                                   &internal_filter_policy_, raw_options)),
          owns_info_log_(options_.info_log != raw_options.info_log),
          owns_cache_(options_.block_cache != raw_options.block_cache),
          dbname_(dbname),
          table_cache_(new TableCache(dbname_, options_, TableCacheSize(options_))),
          db_lock_(nullptr),
          shutting_down_(false),
          background_work_finished_signal_(&mutex_),
          mem_(nullptr),
          imm_(nullptr),
          has_imm_(false),
          logfile_(nullptr),
          logfile_number_(0),
          log_(nullptr),
          seed_(0),
          tmp_batch_(new WriteBatch),
          background_compaction_scheduled_(false),
          manual_compaction_(nullptr),
          versions_(new VersionSet(dbname_, &options_, table_cache_,
                                   &internal_comparator_))
        */
    }
}
