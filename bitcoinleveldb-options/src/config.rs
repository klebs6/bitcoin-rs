crate::ix!();

pub const NUM_NON_TABLE_CACHE_FILES: i32 = 10;

pub fn table_cache_size(sanitized_options: &Options) -> i32 {
    
    todo!();
        /*
            // Reserve ten files or so for other uses and give the rest to TableCache.
      return sanitized_options.max_open_files - kNumNonTableCacheFiles;
        */
}
