// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_append_version.rs ]
crate::ix!();

impl AppendVersion for VersionSet {

    fn append_version(&mut self, v: *mut Version) {
        let cur: *mut Version = self.current();

        trace!(
            v_ptr = %format!("{:p}", v),
            current_ptr = %format!("{:p}", cur),
            "VersionSet::append_version: enter"
        );

        assert!(
            !v.is_null(),
            "VersionSet::append_version: v must not be null"
        );

        unsafe {
            // Make "v" current
            assert!(
                *(*v).refs() == 0,
                "VersionSet::append_version: v->refs must be 0 on entry; got {}",
                *(*v).refs()
            );
            assert!(
                v != cur,
                "VersionSet::append_version: v must not equal current"
            );

            if !cur.is_null() {
                trace!(
                    old_current_ptr = %format!("{:p}", cur),
                    "VersionSet::append_version: unref old current"
                );
                (&mut *cur).unref();
            }

            self.set_current(v);
            (&mut *v).ref_();

            // Append to linked list
            let dummy_ptr: *mut Version = self.dummy_versions_mut() as *mut Version;

            let old_tail: *mut Version = *(*dummy_ptr).prev();

            (*v).set_prev(old_tail);
            (*v).set_next(dummy_ptr);

            // old_tail->next_ = v;
            if !old_tail.is_null() {
                (*old_tail).set_next(v);
            } else {
                // In a correct sentinel setup, old_tail is never null.
                warn!(
                    "VersionSet::append_version: dummy.prev was null; list may be corrupted"
                );
            }

            // dummy_versions_.prev_ = v;
            (*dummy_ptr).set_prev(v);

            debug!(
                new_current_ptr = %format!("{:p}", self.current()),
                v_refs = *(*v).refs(),
                "VersionSet::append_version: installed new current and appended to list"
            );
        }
    }
}
