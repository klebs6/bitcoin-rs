// ---------------- [ File: bitcoinleveldb-versionset/src/level_state.rs ]
crate::ix!();

pub struct VersionSetBuilderLevelState {
    pub(crate) deleted_files: HashSet<u64>,
    pub(crate) added_files:   *mut VersionSetBuilderFileSet,
}

impl Default for VersionSetBuilderLevelState {
    fn default() -> Self {
        Self {
            deleted_files: HashSet::new(),
            added_files: core::ptr::null_mut(),
        }
    }
}

impl VersionSetBuilderLevelState {
    pub(crate) fn deleted_files_ref(&self) -> &HashSet<u64> {
        &self.deleted_files
    }

    pub(crate) fn deleted_files_mut_ref(&mut self) -> &mut HashSet<u64> {
        &mut self.deleted_files
    }

    pub(crate) fn added_files_ptr(&self) -> *mut VersionSetBuilderFileSet {
        self.added_files
    }

    pub(crate) fn set_added_files_ptr(&mut self, ptr: *mut VersionSetBuilderFileSet) {
        self.added_files = ptr;
    }
}
