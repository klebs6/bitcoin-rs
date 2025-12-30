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

#[cfg(test)]
mod get_table_cache_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace};

    #[traced_test]
    fn table_cache_accessors_round_trip_and_are_stable() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));

        let icmp = Box::new(options.internal_key_comparator());
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));
        let tc_ptr: *mut TableCache = table_cache.as_mut() as *mut TableCache;

        let vs = VersionSet::new(
            &"tmp".to_string(),
            options.as_ref(),
            tc_ptr,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let p1 = vs.get_table_cache();
        let p2 = <VersionSet as GetTableCache>::table_cache(&vs);

        trace!(
            tc_expected = %format!("{:p}", tc_ptr),
            p1 = %format!("{:p}", p1),
            p2 = %format!("{:p}", p2),
            "table_cache pointers"
        );

        assert_eq!(p1 as *mut (), tc_ptr as *mut (), "get_table_cache must return the original pointer");
        assert_eq!(p2 as *mut (), tc_ptr as *mut (), "trait table_cache must return the original pointer");
        assert_eq!(p1 as *mut (), p2 as *mut (), "wrapper and trait must agree");

        drop(table_cache);
    }
}
