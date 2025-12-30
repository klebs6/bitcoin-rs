// ---------------- [ File: bitcoinleveldb-versionset/src/level_state.rs ]
crate::ix!();

#[derive(Builder,Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
#[builder(pattern="owned")]
pub struct VersionSetBuilderLevelState {
    deleted_files: HashSet<u64>,
    added_files:   *mut VersionSetBuilderFileSet,
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

#[cfg(test)]
mod level_state_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace};

    #[traced_test]
    fn level_state_default_and_mutators_behave_consistently() {
        let mut st = VersionSetBuilderLevelState::default();

        debug!(added_files_ptr = %format!("{:p}", st.added_files_ptr()), "default added_files ptr");
        assert!(st.added_files_ptr().is_null(), "default added_files pointer must be null");

        assert!(st.deleted_files_ref().is_empty(), "default deleted_files must be empty");

        st.deleted_files_mut_ref().insert(123);
        st.deleted_files_mut_ref().insert(456);

        debug!(deleted_len = st.deleted_files_ref().len(), "deleted_files after insert");
        assert!(st.deleted_files_ref().contains(&123));
        assert!(st.deleted_files_ref().contains(&456));

        let fake_ptr: *mut VersionSetBuilderFileSet = 0x1 as *mut VersionSetBuilderFileSet;
        st.set_added_files_ptr(fake_ptr);

        trace!(
            new_ptr = %format!("{:p}", st.added_files_ptr()),
            "set_added_files_ptr"
        );
        assert_eq!(st.added_files_ptr() as *mut (), fake_ptr as *mut (), "added_files_ptr must update");

        st.set_added_files_ptr(core::ptr::null_mut());
        assert!(st.added_files_ptr().is_null(), "added_files_ptr must be resettable to null");
    }
}
