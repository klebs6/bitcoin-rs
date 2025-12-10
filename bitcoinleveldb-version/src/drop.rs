// ---------------- [ File: bitcoinleveldb-version/src/drop.rs ]
crate::ix!();

impl Drop for Version {

    fn drop(&mut self) {
        trace!(
            "Version::drop: start; self={:p}, refs={}",
            self as *mut Version,
            self.refs()
        );

        assert!(
            *self.refs() == 0,
            "Version::drop: refs_ must be 0, got {}",
            self.refs()
        );

        unsafe {
            // Remove from linked list
            if !self.prev().is_null() {
                (*(*self.prev())).set_next(*self.next_mut());
            }
            if !self.next().is_null() {
                (*(*self.next())).set_prev(*self.prev_mut());
            }
        }

        // Drop references to files
        for level in 0..NUM_LEVELS {
            let mut files_at_level = &mut self.files_mut()[level];
            trace!(
                "Version::drop: processing level {} with {} files",
                level,
                files_at_level.len()
            );

            for &mut fptr in files_at_level.iter_mut() {
                if fptr.is_null() {
                    warn!(
                        "Version::drop: encountered null FileMetaData pointer at level {}",
                        level
                    );
                    continue;
                }

                unsafe {
                    let f: &mut FileMetaData = &mut *fptr;
                    let fnumber = *f.number();
                    let refs = f.refs_mut();
                    assert!(
                        *refs > 0,
                        "Version::drop: FileMetaData.refs must be > 0 before decrement"
                    );
                    *refs -= 1;

                    debug!(
                        "Version::drop: decremented FileMetaData {} refs to {} at level {}",
                        fnumber,
                        *refs,
                        level
                    );

                    if *refs <= 0 {
                        trace!(
                            "Version::drop: freeing FileMetaData {} at level {}",
                            fnumber,
                            level
                        );
                        drop(Box::from_raw(fptr));
                    }
                }
            }
        }

        trace!(
            "Version::drop: completed for self={:p}",
            self as *mut Version
        );
    }
}
