crate::ix!();

pub fn get_options(n_cache_size: usize) -> leveldb::Options {
    
    todo!();
        /*
        leveldb::Options options;
        options.block_cache = leveldb::NewLRUCache(nCacheSize / 2);
        options.write_buffer_size = nCacheSize / 4; // up to two write buffers may be held in memory simultaneously
        options.filter_policy = leveldb::NewBloomFilterPolicy(10);
        options.compression = leveldb::kNoCompression;

        options.info_log = new CBitcoinLevelDBLogger();
        if (leveldb::kMajorVersion > 1 || (leveldb::kMajorVersion == 1 && leveldb::kMinorVersion >= 16)) {
            // LevelDB versions before 1.16 consider short writes to be corruption. Only trigger error
            // on corruption in later versions.
            options.paranoid_checks = true;
        }
        SetMaxOpenFiles(&options);
        return options;
        */
}
