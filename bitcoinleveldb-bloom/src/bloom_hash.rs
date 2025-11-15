// ---------------- [ File: bitcoinleveldb-bloom/src/bloom_hash.rs ]
crate::ix!();

pub fn bloom_hash(key_: &Slice) -> u32 {
    let data_ptr = key_.data();
    let len = key_.size();

    tracing::debug!(
        key_length = len,
        "bloom_hash: computing Bloom hash for key Slice"
    );

    if data_ptr.is_null() || len == 0 {
        tracing::debug!("bloom_hash: key is empty, hashing empty slice");
        return leveldb_hash(&[], 0xbc9f_1d34);
    }

    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    let result = leveldb_hash(bytes, 0xbc9f_1d34);

    tracing::trace!(hash = result, "bloom_hash: returning hash value");
    result
}
