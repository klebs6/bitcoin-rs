// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_create.rs ]
crate::ix!();

impl VersionSet {
    
    pub fn new(
        dbname:      &String,
        options:     *const Options,
        table_cache: *mut TableCache,
        cmp:         *const InternalKeyComparator) -> Self {
    
        todo!();
        /*


            : env_(options->env),
          dbname_(dbname),
          options_(options),
          table_cache_(table_cache),
          icmp_(*cmp),
          next_file_number_(2),
          manifest_file_number_(0),  // Filled by Recover()
          last_sequence_(0),
          log_number_(0),
          prev_log_number_(0),
          descriptor_file_(nullptr),
          descriptor_log_(nullptr),
          dummy_versions_(this),
          current_(nullptr) 
      AppendVersion(new Version(this));
        */
    }
}
