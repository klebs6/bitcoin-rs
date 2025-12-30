// ---------------- [ File: bitcoinleveldb-versionset/src/get_options_ptr.rs ]
crate::ix!();

impl GetOptionsPtr for VersionSet {
    fn options(&self) -> *const Options {
        let p: *const Options = VersionSet::options(self);

        trace!(
            options_ptr = %format!("{:p}", p),
            "VersionSet::options: returning Options pointer"
        );

        // NOTE: The VersionSet stores a raw pointer to Options that is owned elsewhere.
        p
    }
}

impl VersionSet {
    pub fn get_options_ptr(&self) -> *const Options {
        let options_ptr: *const Options = <VersionSet as GetOptionsPtr>::options(self);

        trace!(
            options_ptr = %format!("{:p}", options_ptr),
            "VersionSet::get_options_ptr"
        );

        options_ptr
    }
}

#[cfg(test)]
mod get_options_ptr_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace};

    #[traced_test]
    fn options_ptr_accessors_round_trip_and_are_stable() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let options_ptr: *const Options = options.as_ref() as *const Options;

        let icmp = Box::new(options.internal_key_comparator());
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));

        let vs = VersionSet::new(
            &"tmp".to_string(),
            options_ptr,
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let p1 = vs.get_options_ptr();
        let p2 = <VersionSet as GetOptionsPtr>::options(&vs);

        trace!(p1 = %format!("{:p}", p1), p2 = %format!("{:p}", p2), "options ptrs");
        assert_eq!(p1, options_ptr, "get_options_ptr must return the original pointer");
        assert_eq!(p2, options_ptr, "trait options() must return the original pointer");
        assert_eq!(p1, p2, "wrapper and trait must agree");

        // Keep `options` alive for the duration of the test (it is owned externally).
        drop(options);
    }
}
