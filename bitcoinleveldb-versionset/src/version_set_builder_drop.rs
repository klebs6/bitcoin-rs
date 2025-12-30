// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder_drop.rs ]
crate::ix!();

impl Drop for VersionSetBuilder {

    fn drop(&mut self) {
        trace!(
            vset_ptr = ?self.vset_ptr(),
            base_ptr = ?self.base_ptr(),
            "VersionSetBuilder::drop: releasing builder resources"
        );

        unsafe {
            for level in 0..NUM_LEVELS {
                let added_ptr = self.level_state_ref(level).added_files_ptr();
                self.level_state_mut_ref(level)
                    .set_added_files_ptr(core::ptr::null_mut());

                if added_ptr.is_null() {
                    continue;
                }

                let added_box: Box<VersionSetBuilderFileSet> = Box::from_raw(added_ptr);

                let mut to_unref: Vec<*mut FileMetaData> = Vec::with_capacity(added_box.len());
                for f in added_box.iter() {
                    to_unref.push(*f);
                }

                drop(added_box);

                for fptr in to_unref {
                    debug_assert!(!fptr.is_null());

                    let refs = (*fptr).refs_mut();
                    *refs -= 1;

                    if *refs <= 0 {
                        debug_assert_eq!(
                            *refs, 0,
                            "VersionSetBuilder::drop: FileMetaData refs went negative"
                        );

                        drop(Box::from_raw(fptr));

                        trace!(
                            level,
                            "VersionSetBuilder::drop: freed FileMetaData that was only owned by builder"
                        );
                    }
                }
            }

            let base_ptr = self.take_base_ptr();
            if !base_ptr.is_null() {
                (*base_ptr).unref();
            }
        }

        trace!("VersionSetBuilder::drop: complete");
    }
}
