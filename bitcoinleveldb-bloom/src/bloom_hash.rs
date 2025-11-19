// ---------------- [ File: bitcoinleveldb-bloom/src/bloom_hash.rs ]
crate::ix!();

pub fn bloom_hash(key_: &Slice) -> u32 {
    let data_ptr: *const u8 = *key_.data();
    let len: usize          = *key_.size();

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

#[cfg(test)]
mod bloom_hash_suite {
    use super::*;

    #[traced_test]
    fn bloom_hash_empty_slice_matches_leveldb_hash_of_null_input() {
        let empty_slice = unsafe { Slice::from_ptr_len(std::ptr::null(), 0) };

        let bloom = bloom_hash(&empty_slice);
        let reference = leveldb_hash(std::ptr::null(), 0, 0xbc9f1d34);

        info!(
            bloom_hash = bloom,
            reference_hash = reference,
            "bloom_hash_empty_slice_matches_leveldb_hash_of_null_input"
        );

        assert_eq!(bloom, reference);
    }

    #[traced_test]
    fn bloom_hash_matches_leveldb_hash_for_nonempty_key() {
        let key_bytes = b"bloom-hash-key";
        let slice = unsafe { Slice::from_ptr_len(key_bytes.as_ptr(), key_bytes.len()) };

        let bloom = bloom_hash(&slice);
        let reference = leveldb_hash(key_bytes.as_ptr(), key_bytes.len(), 0xbc9f1d34);

        info!(
            bloom_hash = bloom,
            reference_hash = reference,
            key_len = key_bytes.len(),
            "bloom_hash_matches_leveldb_hash_for_nonempty_key"
        );

        assert_eq!(bloom, reference);
    }
}
