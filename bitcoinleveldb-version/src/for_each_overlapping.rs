// ---------------- [ File: bitcoinleveldb-version/src/for_each_overlapping.rs ]
crate::ix!();

impl Version {

    /// Call func(arg, level, f) for every file that overlaps user_key in order
    /// from newest to oldest.  
    ///
    /// If an invocation of func returns false, makes no more calls.
    /// 
    /// REQUIRES: user portion of internal_key == user_key.
    ///
    pub fn for_each_overlapping(
        &mut self,
        user_key_:     &Slice,
        internal_key_: &Slice,
        arg:           *mut c_void,
        func:          fn(
            _0: *mut c_void,
            _1: i32,
            _2: *mut FileMetaData,
        ) -> bool,
    ) {
        trace!(
            "Version::for_each_overlapping: start; user_key_len={}, internal_key_len={}",
            user_key_.size(),
            internal_key_.size()
        );

        let ucmp = unsafe { (*self.vset()).icmp().user_comparator() };

        // Search level-0 in order from newest to oldest.
        let mut tmp: Vec<*mut FileMetaData> = Vec::with_capacity(self.files()[0].len());
        let level0_files = &self.files()[0];

        for &fptr in level0_files.iter() {
            if fptr.is_null() {
                warn!("Version::for_each_overlapping: null FileMetaData pointer at level 0");
                continue;
            }

            unsafe {
                let f: &FileMetaData = &*fptr;
                let smallest_user = f.smallest().user_key();
                let largest_user  = f.largest().user_key();

                if (*ucmp).compare(user_key_, &smallest_user) >= 0
                    && (*ucmp).compare(user_key_, &largest_user) <= 0
                {
                    tmp.push(fptr);
                }
            }
        }

        if !tmp.is_empty() {
            // NewestFirst: larger file number is newer.
            tmp.sort_unstable_by(|&a_ptr, &b_ptr| unsafe {
                let a = &*a_ptr;
                let b = &*b_ptr;
                (*b.number()).cmp(a.number())
            });

            for &fptr in tmp.iter() {
                if !func(arg, 0, fptr) {
                    trace!(
                        "Version::for_each_overlapping: callback requested stop at level 0"
                    );
                    return;
                }
            }
        }

        // Search other levels.
        let icmp = InternalKeyComparator::new(null_slice_comparator());

        for level in 1..(NUM_LEVELS as i32) {
            let files_level = &self.files()[level as usize];
            let num_files   = files_level.len();
            if num_files == 0 {
                continue;
            }

            // Binary search to find earliest index whose largest key >= internal_key.
            let index = find_file(&icmp, files_level, internal_key_) as usize;
            if index < num_files {
                let fptr = files_level[index];
                if fptr.is_null() {
                    warn!(
                        "Version::for_each_overlapping: null FileMetaData pointer at level {}",
                        level
                    );
                    continue;
                }

                unsafe {
                    let f = &*fptr;
                    let smallest_user = f.smallest().user_key();

                    if (*ucmp).compare(user_key_, &smallest_user) < 0 {
                        // All of "f" is past any data for user_key
                    } else if !func(arg, level, fptr) {
                        trace!(
                            "Version::for_each_overlapping: callback requested stop at level {}",
                            level
                        );
                        return;
                    }
                }
            }
        }

        trace!(
            "Version::for_each_overlapping: completed without early termination"
        );
    }
}

#[cfg(test)]
mod version_for_each_overlapping_signature_tests {
    use super::*;
    use std::ffi::c_void;

    #[traced_test]
    fn for_each_overlapping_signature_is_stable() {
        let _fn_ptr: fn(
            &mut Version,
            &Slice,
            &Slice,
            *mut c_void,
            fn(*mut c_void, i32, *mut FileMetaData) -> bool,
        ) = Version::for_each_overlapping;
        let _ = _fn_ptr;
    }
}


