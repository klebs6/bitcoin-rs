// ---------------- [ File: bitcoinleveldb-versionset/src/current_version.rs ]
crate::ix!();

impl CurrentVersion for VersionSet {
    fn current(&self) -> *mut Version {
        let cur: *mut Version = VersionSet::current(self);

        trace!(
            current_ptr = %format!("{:p}", cur),
            "VersionSet::current: returning current Version pointer"
        );

        cur
    }
}

impl VersionSet {
    /// Return the current version.
    pub fn current_version(&self) -> *mut Version {
        let current: *mut Version = VersionSet::current(self);

        trace!(
            current_ptr = %format!("{:p}", current),
            "VersionSet::current_version"
        );

        current
    }
}
