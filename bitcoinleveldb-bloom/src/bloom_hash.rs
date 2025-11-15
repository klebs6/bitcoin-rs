// ---------------- [ File: bitcoinleveldb-bloom/src/bloom_hash.rs ]
crate::ix!();

pub fn bloom_hash(key_: &Slice) -> u32 {
    let data_ptr: *const u8 = key_.data();
    let len: usize          = key_.size();

    debug!(
        key_length = len,
        "bloom_hash: computing LevelDB Bloom hash for key Slice"
    );

    if data_ptr.is_null() || len == 0 {
        debug!("bloom_hash: key is empty, hashing empty slice");
        return leveldb_hash(std::ptr::null(), 0, 0xbc9f1d34);
    }

    let result = leveldb_hash(data_ptr, len, 0xbc9f1d34);

    trace!(hash = result, "bloom_hash: returning hash value");
    result
}
