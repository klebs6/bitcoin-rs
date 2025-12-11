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

#[cfg(test)]
mod version_drop_linked_list_tests {
    use super::*;
    use super::version_test_helpers as helpers;
    use core::ptr;

    #[traced_test]
    fn drop_unlinks_version_from_doubly_linked_list() {
        let v1 = helpers::build_empty_version();
        let v2 = helpers::build_empty_version();

        let mut v1_box = Box::new(v1);
        let mut v2_box = Box::new(v2);

        let v1_ptr: *mut Version = &mut *v1_box;
        let v2_ptr: *mut Version = &mut *v2_box;

        v1_box.set_next(v2_ptr);
        v2_box.set_prev(v1_ptr);

        assert_eq!(
            *v2_box.prev(),
            v1_ptr,
            "Second version must initially point back to first via prev"
        );
        assert_eq!(
            *v1_box.next(),
            v2_ptr,
            "First version must initially point to second via next"
        );
        assert!(
            v1_box.prev().is_null(),
            "Head of list must have null prev pointer"
        );
        assert!(
            v2_box.next().is_null(),
            "Tail of list must have null next pointer"
        );

        drop(v1_box);

        assert_eq!(
            *v2_box.prev(),
            ptr::null_mut(),
            "After dropping head, remaining node's prev pointer must be cleared"
        );
    }
}
