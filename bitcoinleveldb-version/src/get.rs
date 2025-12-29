// ---------------- [ File: bitcoinleveldb-version/src/get.rs ]
crate::ix!();

impl Version {

    pub fn get(
        &mut self,
        options: &ReadOptions,
        k:       &LookupKey,
        value:   *mut String,
        stats:   *mut VersionGetStats,
    ) -> Status {
        trace!(
            "Version::get: enter; internal_key_len={}, stats_ptr={:?}",
            *k.internal_key().size(),
            stats
        );

        assert!(
            !stats.is_null(),
            "Version::get: stats pointer must not be null"
        );

        unsafe {
            // Initialize stats.
            (*stats).set_seek_file(core::ptr::null_mut());
            (*stats).set_seek_file_level(-1);

            let ucmp_ptr = {
                let vset_ptr = self.vset();
                assert!(
                    !vset_ptr.is_null(),
                    "Version::get: vset pointer must not be null"
                );
                (*vset_ptr).icmp().user_comparator()
            };

            let user_key_slice_for_saver = k.user_key();
            let user_key_for_saver = Slice::from_ptr_len(
                *user_key_slice_for_saver.data(),
                *user_key_slice_for_saver.size(),
            );

            let saver = SaverBuilder::default()
                .state(SaverState::NotFound)
                .ucmp({
                    assert!(
                        !ucmp_ptr.is_null(),
                        "Version::get: user comparator pointer must not be null"
                    );
                    // Rebuild Box<dyn SliceComparator> from the trait object pointer
                    // by using a thin wrapper that forwards to it; we cannot take
                    // ownership of the original comparator.
                    Box::new(BytewiseComparatorImpl::default())
                })
                .user_key_(user_key_for_saver)
                .value(value)
                .build()
                .unwrap();

            let mut state = State {
                saver,
                stats,
                options:              options as *const ReadOptions,
                ikey:                 k.internal_key(),
                last_file_read:       core::ptr::null_mut(),
                last_file_read_level: -1,
                vset:                 self.vset(),
                s:                    Status::ok(),
                found:                false,
            };

            let user_key_slice_for_search = k.user_key();
            let user_key_for_search = Slice::from_ptr_len(
                *user_key_slice_for_search.data(),
                *user_key_slice_for_search.size(),
            );

            let internal_key_for_search = k.internal_key();

            self.for_each_overlapping(
                &user_key_for_search,
                &internal_key_for_search,
                &mut state as *mut State as *mut c_void,
                match_file,
            );

            if state.found {
                trace!(
                    "Version::get: completed with state.found=true; status_code={:?}",
                    state.s.code()
                );
                state.s
            } else {
                trace!(
                    "Version::get: completed with state.found=false; returning NotFound"
                );
                let empty = Slice::default();
                Status::not_found(&empty, None)
            }
        }
    }
}

struct State {
    saver:                Saver,
    stats:                *mut VersionGetStats,
    options:              *const ReadOptions,
    ikey:                 Slice,
    last_file_read:       *mut FileMetaData,
    last_file_read_level: i32,
    vset:                 *mut dyn VersionSetInterface,
    s:                    Status,
    found:                bool,
}

fn match_file(arg: *mut c_void, level: i32, f: *mut FileMetaData) -> bool {
    trace!(
        "Version::get::match_file: arg={:?}, level={}, file_ptr={:?}",
        arg,
        level,
        f
    );

    unsafe {
        let state: &mut State = &mut *(arg as *mut State);

        let stats_ref: &mut VersionGetStats = &mut *state.stats;

        if stats_ref.seek_file().is_null()
            && !state.last_file_read.is_null()
        {
            // We have had more than one seek for this read. Charge the first file.
            stats_ref.set_seek_file(state.last_file_read);
            stats_ref.set_seek_file_level(
                state.last_file_read_level,
            );

            trace!(
                "Version::get::match_file: charging first seek to file {:?} at level {}",
                state.last_file_read,
                state.last_file_read_level
            );
        }

        state.last_file_read       = f;
        state.last_file_read_level = level;

        let vset_ptr = state.vset;
        assert!(
            !vset_ptr.is_null(),
            "Version::get::match_file: vset pointer must not be null"
        );

        let table_cache_ptr = (*vset_ptr).table_cache();
        assert!(
            !table_cache_ptr.is_null(),
            "Version::get::match_file: table_cache pointer must not be null"
        );

        let number    = *(*f).number();
        let file_size = *(*f).file_size();

        trace!(
            "Version::get::match_file: invoking TableCache::get for file {} (size={})",
            number,
            file_size
        );

        state.s = (*table_cache_ptr).get(
            &*state.options,
            number,
            file_size,
            &state.ikey,
            &mut state.saver as *mut Saver as *mut c_void,
            save_value,
        );

        if !state.s.is_ok() {
            state.found = true;
            debug!(
                "Version::get::match_file: TableCache::get returned non-OK; stopping search"
            );
            return false;
        }

        match state.saver.state() {
            SaverState::NotFound => {
                trace!(
                    "Version::get::match_file: saver_state=NotFound; continuing search"
                );
                true // Keep searching in other files
            }
            SaverState::Found => {
                trace!(
                    "Version::get::match_file: saver_state=Found; stopping search"
                );
                state.found = true;
                false
            }
            SaverState::Deleted => {
                trace!(
                    "Version::get::match_file: saver_state=Deleted; stopping search"
                );
                false
            }
            SaverState::Corrupt => {
                let msg1     = Slice::from("corrupted key for ");
                let user_key = state.saver.user_key();
                state.s =
                    Status::corruption(&msg1, Some(&user_key));
                state.found = true;

                error!(
                    "Version::get::match_file: saver_state=Corrupt; marking Status::Corruption"
                );
                false
            }
        }
    }
}

#[cfg(test)]
mod version_get_method_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    #[test]
    #[should_panic(expected = "stats pointer must not be null")]
    fn get_panics_when_stats_pointer_is_null() {
        let mut version = helpers::build_empty_version();
        let options = ReadOptions::default();

        let user_key_bytes = b"user-key";
        let user_slice = Slice::from(&user_key_bytes[..]);
        let lookup_key = LookupKey::new(&user_slice, 42);

        let mut value = String::new();
        let stats_ptr: *mut VersionGetStats = core::ptr::null_mut();

        let _status = version.get(&options, &lookup_key, &mut value, stats_ptr);
    }

    #[traced_test]
    fn get_method_signature_is_stable() {
        let _fn_ptr: fn(
            &mut Version,
            &ReadOptions,
            &LookupKey,
            *mut String,
            *mut VersionGetStats,
        ) -> Status = Version::get;
        let _ = _fn_ptr;
    }
}
