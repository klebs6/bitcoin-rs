// ---------------- [ File: bitcoinleveldb-versionset/src/get_internal_key_comparator.rs ]
crate::ix!();

impl GetInternalKeyComparator for VersionSet {
    fn icmp(&self) -> &InternalKeyComparator {
        VersionSet::icmp(self)
    }
}

impl VersionSet {
    pub fn get_internal_key_comparator(&self) -> &InternalKeyComparator {
        let icmp_ref: &InternalKeyComparator = <VersionSet as GetInternalKeyComparator>::icmp(self);

        trace!(
            icmp_ptr = %format!("{:p}", icmp_ref as *const InternalKeyComparator),
            "VersionSet::get_internal_key_comparator"
        );

        icmp_ref
    }
}

#[cfg(test)]
mod get_internal_key_comparator_exhaustive_test_suite {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, error, info, trace, warn};

    fn make_unique_temp_db_dir(prefix: &str) -> PathBuf {
        let pid = std::process::id();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);

        let mut p = std::env::temp_dir();
        p.push(format!("{prefix}_{pid}_{nanos}"));
        p
    }

    fn remove_dir_all_best_effort(dir: &Path) {
        match std::fs::remove_dir_all(dir) {
            Ok(()) => trace!(dir = %dir.display(), "removed temp db dir"),
            Err(e) => warn!(dir = %dir.display(), error = ?e, "failed to remove temp db dir (best effort)"),
        }
    }

    fn make_ikey(user_key: &str, seq: u64) -> InternalKey {
        InternalKey::new(&Slice::from(user_key), seq, ValueType::TypeValue)
    }

    fn make_internal_key_comparator_from_options(options: &Options) -> InternalKeyComparator {
        let ucmp_ptr: *const dyn SliceComparator =
            options.comparator().as_ref() as *const dyn SliceComparator;
        InternalKeyComparator::new(ucmp_ptr)
    }

    #[traced_test]
    fn internal_key_comparator_accessor_is_stable_and_orders_keys() {
        let dir = make_unique_temp_db_dir("versionset_icmp_accessor");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 8));

        let vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let a = vs.get_internal_key_comparator() as *const InternalKeyComparator;
        let b = vs.get_internal_key_comparator() as *const InternalKeyComparator;
        debug!(
            a_ptr = %format!("{:p}", a),
            b_ptr = %format!("{:p}", b),
            "icmp pointer stability"
        );
        assert_eq!(a, b, "icmp reference should be stable across repeated calls");

        let k1 = make_ikey("a", 100);
        let k2 = make_ikey("b", 100);
        let r = vs
            .get_internal_key_comparator()
            .compare_internal_key(&k1, &k2);
        debug!(r, "compare_internal_key(a,b)");
        assert!(r < 0, "expected internal key for 'a' to compare less than 'b'");

        remove_dir_all_best_effort(&dir);
    }
}
