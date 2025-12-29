// ---------------- [ File: bitcoinleveldb-versionset/src/get_table_cache.rs ]
crate::ix!();

impl GetTableCache for VersionSet {
    fn table_cache(&self) -> *mut TableCache {
        let p: *const TableCache = VersionSet::table_cache(self);

        trace!(
            table_cache_ptr = %format!("{:p}", p),
            "VersionSet::table_cache: returning TableCache pointer"
        );

        // NOTE: The VersionSet stores a raw pointer to a TableCache that is owned elsewhere.
        p as *mut TableCache
    }
}

impl VersionSet {
    pub fn get_table_cache(&self) -> *mut TableCache {
        let table_cache_ptr: *mut TableCache = <VersionSet as GetTableCache>::table_cache(self);

        trace!(
            table_cache_ptr = %format!("{:p}", table_cache_ptr),
            "VersionSet::get_table_cache"
        );

        table_cache_ptr
    }
}
