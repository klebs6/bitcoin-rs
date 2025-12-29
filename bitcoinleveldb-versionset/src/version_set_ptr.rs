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
