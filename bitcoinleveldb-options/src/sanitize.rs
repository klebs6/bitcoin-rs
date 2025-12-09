// ---------------- [ File: bitcoinleveldb-options/src/sanitize.rs ]
crate::ix!();

/**
  | Fix user-supplied options to be reasonable
  |
  */
pub fn clip_to_range<T, V>(
        ptr:      *mut T,
        minvalue: V,
        maxvalue: V)  {

    todo!();
        /*
            if (static_cast<V>(*ptr) > maxvalue) *ptr = maxvalue;
      if (static_cast<V>(*ptr) < minvalue) *ptr = minvalue;
        */
}

/**
  | Sanitize db options. The caller should
  | delete result.info_log if it is not
  | equal to src.info_log.
  |
  */
pub fn sanitize_options(
        dbname:  &String,
        icmp:    *const InternalKeyComparator,
        ipolicy: *const InternalFilterPolicy,
        src:     &Options) -> Options {
    
    todo!();
        /*
            Options result = src;
      result.comparator = icmp;
      result.filter_policy = (src.filter_policy != nullptr) ? ipolicy : nullptr;
      ClipToRange(&result.max_open_files, 64 + kNumNonTableCacheFiles, 50000);
      ClipToRange(&result.write_buffer_size, 64 << 10, 1 << 30);
      ClipToRange(&result.max_file_size, 1 << 20, 1 << 30);
      ClipToRange(&result.block_size, 1 << 10, 4 << 20);
      if (result.info_log == nullptr) {
        // Open a log file in the same directory as the db
        src.env->CreateDir(dbname);  // In case it does not exist
        src.env->RenameFile(InfoLogFileName(dbname), OldInfoLogFileName(dbname));
        Status s = src.env->NewLogger(InfoLogFileName(dbname), &result.info_log);
        if (!s.ok()) {
          // No place suitable for logging
          result.info_log = nullptr;
        }
      }
      if (result.block_cache == nullptr) {
        result.block_cache = NewLRUCache(8 << 20);
      }
      return result;
        */
}
