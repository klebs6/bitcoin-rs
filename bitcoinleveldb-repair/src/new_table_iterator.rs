// ---------------- [ File: bitcoinleveldb-repair/src/new_table_iterator.rs ]
crate::ix!();

impl Repairer {

    pub fn new_table_iterator(&mut self, meta: &FileMetaData) -> *mut LevelDBIterator {
        // Same as compaction iterators: if paranoid_checks are on, turn
        // on checksum verification.
        let mut r = ReadOptions::default();
        if *self.options.paranoid_checks() {
            *r.verify_checksums_mut() = true;
        }

        unsafe {
            if self.table_cache.is_null() {
                error!("Repairer::new_table_iterator: table_cache is null");
                return core::ptr::null_mut();
            }
            (*self.table_cache).new_iterator(r, *meta.number(), *meta.file_size())
        }
    }
}
