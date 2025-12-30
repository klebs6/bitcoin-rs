// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_ptr.rs ]
crate::ix!();

#[derive(Copy, Clone, Debug)]
pub struct VersionSetPtr(*mut VersionSet);

impl VersionSetPtr {
    pub fn new(ptr: *mut VersionSet) -> Self {
        trace!(ptr = ?ptr, "VersionSetPtr::new");
        Self(ptr)
    }

    pub fn as_mut_ptr(&self) -> *mut VersionSet {
        trace!(ptr = ?self.0, "VersionSetPtr::as_mut_ptr");
        self.0
    }
}

impl From<*mut VersionSet> for VersionSetPtr {
    fn from(p: *mut VersionSet) -> Self {
        VersionSetPtr::new(p)
    }
}

impl From<VersionSetPtr> for Version {
    fn from(p: VersionSetPtr) -> Version {
        let vset_raw = p.as_mut_ptr();

        trace!(
            vset_ptr = ?vset_raw,
            "VersionSetPtr->Version: constructing a new Version"
        );

        let vset_iface_ptr: *mut dyn VersionSetInterface = unsafe {
            if vset_raw.is_null() {
                VersionSet::null_versionset_interface_ptr()
            } else {
                (&mut *vset_raw as &mut dyn VersionSetInterface) as *mut dyn VersionSetInterface
            }
        };

        let files: [Vec<*mut FileMetaData>; NUM_LEVELS] = core::array::from_fn(|_| Vec::new());

        VersionBuilder::default()
            .vset(vset_iface_ptr)
            .next(core::ptr::null_mut())
            .prev(core::ptr::null_mut())
            .refs(0)
            .files(files)
            .file_to_compact(core::ptr::null_mut())
            .file_to_compact_level(-1)
            .compaction_score(-1.0)
            .compaction_level(-1)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod version_set_ptr_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace};

    #[traced_test]
    fn version_set_ptr_round_trips_raw_pointer_and_constructs_versions_with_expected_defaults() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));

        let icmp = Box::new(options.internal_key_comparator());
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));

        let mut vs = VersionSet::new(
            &"tmp".to_string(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let vs_ptr: *mut VersionSet = &mut vs as *mut VersionSet;
        let p = VersionSetPtr::new(vs_ptr);
        assert_eq!(p.as_mut_ptr() as *mut (), vs_ptr as *mut (), "VersionSetPtr must preserve the raw pointer");

        let v = Version::from(p);
        debug!(
            refs = unsafe { *v.refs() },
            compaction_score = unsafe { *v.compaction_score() },
            compaction_level = unsafe { *v.compaction_level() },
            "constructed Version defaults"
        );

        assert_eq!(unsafe { *v.refs() }, 0, "new Version from VersionSetPtr must have refs=0");
        assert_eq!(unsafe { *v.compaction_level() }, -1, "new Version must default compaction_level=-1");
        assert_eq!(unsafe { *v.compaction_score() }, -1.0, "new Version must default compaction_score=-1.0");

        for (lvl, files) in v.files().iter().enumerate() {
            assert!(
                files.is_empty(),
                "new Version must start with empty file list at level {}",
                lvl
            );
        }

        // Null pointer case must not crash and must yield a Version with a null vset interface.
        let null_vs: *mut VersionSet = core::ptr::null_mut();
        let null_p = VersionSetPtr::new(null_vs);
        let vnull = Version::from(null_p);

        let vset_iface = vnull.vset();
        debug!(
            vset_iface_is_null = vset_iface.is_null(),
            "constructed Version from null VersionSetPtr"
        );
        assert!(vset_iface.is_null(), "vset interface pointer must be null when VersionSetPtr is null");
    }
}
