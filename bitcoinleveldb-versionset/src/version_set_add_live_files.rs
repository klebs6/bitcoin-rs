// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_add_live_files.rs ]
crate::ix!();

impl AddLiveFiles for VersionSet {

    /// Add all files listed in any live version to *live.
    fn add_live_files(&mut self, live: *mut HashSet<u64>) {
        trace!(
            live_ptr = %format!("{:p}", live),
            "VersionSet::add_live_files: enter"
        );

        if live.is_null() {
            error!(
                "VersionSet::add_live_files: live out-parameter is null; nothing to do"
            );
            return;
        }

        let dummy_ptr: *mut Version = self.dummy_versions_mut() as *mut Version;

        unsafe {
            let live_set: &mut HashSet<u64> = &mut *live;

            let mut vptr: *mut Version = *(*dummy_ptr).next();

            while vptr != dummy_ptr {
                if vptr.is_null() {
                    error!(
                        "VersionSet::add_live_files: encountered null Version pointer in version list; aborting traversal"
                    );
                    break;
                }

                let v: &Version = &*vptr;

                for level in 0..NUM_LEVELS {
                    let files_level: &Vec<*mut FileMetaData> = &v.files()[level];

                    for (idx, fptr_ref) in files_level.iter().enumerate() {
                        let fptr: *mut FileMetaData = *fptr_ref;

                        if fptr.is_null() {
                            warn!(
                                level,
                                idx,
                                "VersionSet::add_live_files: null FileMetaData pointer encountered"
                            );
                            continue;
                        }

                        let f: &FileMetaData = &*fptr;
                        let number = *f.number();

                        let inserted = live_set.insert(number);

                        trace!(
                            level,
                            idx,
                            file_number = number,
                            inserted,
                            "VersionSet::add_live_files: inserted live file number"
                        );
                    }
                }

                vptr = *v.next();
            }

            debug!(
                live_len = live_set.len(),
                "VersionSet::add_live_files: completed"
            );
        }
    }
}
