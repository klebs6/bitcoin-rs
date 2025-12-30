// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_into_version.rs ]
crate::ix!();

impl VersionSet {

    pub fn into_version(&self) -> Version {
    
        todo!();
        /*
        : vset(vset),
        : next(this),
        : prev(this),
        : refs(0),
        : file_to_compact(nullptr),
        : file_to_compact_level(-1),
        : compaction_score(-1),
        : compaction_level(-1),

        
        */
    }
}

#[cfg(test)]
mod version_set_into_version_exhaustive_test_suite {
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

    fn make_internal_key_comparator_from_options(options: &Options) -> InternalKeyComparator {
        let ucmp_ptr: *const dyn SliceComparator =
            options.comparator().as_ref() as *const dyn SliceComparator;
        InternalKeyComparator::new(ucmp_ptr)
    }

    #[traced_test]
    fn into_version_is_currently_unimplemented_and_panics() {
        let dir = make_unique_temp_db_dir("versionset_into_version_panics");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 4));

        let vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = vs.into_version();
        }));

        debug!(panicked = r.is_err(), "into_version panic check");
        assert!(r.is_err(), "into_version must panic until implemented");

        remove_dir_all_best_effort(&dir);
    }
}
