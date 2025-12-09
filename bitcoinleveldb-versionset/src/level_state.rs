// ---------------- [ File: bitcoinleveldb-versionset/src/level_state.rs ]
crate::ix!();

pub struct VersionSetBuilderLevelState {
    deleted_files: HashSet<u64>,
    added_files:   *mut VersionSetBuilderFileSet,
}
