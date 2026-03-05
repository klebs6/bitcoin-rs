// ---------------- [ File: bitcoinleveldb-db/src/filter_policy.rs ]
crate::ix!();

#[derive(Builder,Getters)]
#[getset(get = "pub")]
#[builder(setter(into))]
pub struct LevelDBFilterPolicy {
    state:      *mut c_void,
    destructor: fn(_0: *mut c_void),
    name:       fn(_0: *mut c_void) -> *const u8,
    create:     fn(
        _0:               *mut c_void,
        key_array:        *const *const u8,
        key_length_array: *const usize,
        num_keys:         i32,
        filter_length:    *mut usize,
    ) -> *mut u8,
    key_match:  fn(
        _0:            *mut c_void,
        key_:          *const u8,
        length:        usize,
        filter:        *const u8,
        filter_length: usize,
    ) -> u8,
}


impl FilterPolicy for LevelDBFilterPolicy {}

impl Drop for LevelDBFilterPolicy {
    fn drop(&mut self) {
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBFilterPolicy::drop entry");
        (self.destructor)(self.state);
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBFilterPolicy::drop exit");
    }
}

impl Named for LevelDBFilterPolicy {
    fn name(&self) -> Cow<'_, str> {
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBFilterPolicy::name entry");
        let p = (self.name)(self.state);
        trace!(
            target: "bitcoinleveldb_db::c_api",
            ptr_is_null = p.is_null(),
            "LevelDBFilterPolicy::name exit"
        );

        if p.is_null() {
            return Cow::Borrowed("");
        }

        unsafe {
            let cstr = std::ffi::CStr::from_ptr(p as *const core::ffi::c_char);
            Cow::Owned(cstr.to_string_lossy().into_owned())
        }
    }

}

impl CreateFilter for LevelDBFilterPolicy {
    fn create_filter(&self, keys: *const Slice, n: i32, dst: &mut Vec<u8>) {
        trace!(
            target: "bitcoinleveldb_db::c_api",
            n = n,
            "LevelDBFilterPolicy::create_filter entry"
        );

        if n <= 0 {
            trace!(
                target: "bitcoinleveldb_db::c_api",
                "LevelDBFilterPolicy::create_filter early-exit (n<=0)"
            );
            return;
        }

        let n_usize: usize = n as usize;

        // Mirror the C++ allocation/loop structure.
        let mut key_pointers: Vec<*const u8> = vec![core::ptr::null(); n_usize];
        let mut key_sizes: Vec<usize> = vec![0; n_usize];

        unsafe {
            for i in 0..n_usize {
                let kref: &Slice = &*keys.add(i);
                key_pointers[i] = *kref.data();
                key_sizes[i] = *kref.size();
            }

            let mut len: usize = 0;
            let filter: *mut u8 = (self.create)(
                self.state,
                key_pointers.as_ptr(),
                key_sizes.as_ptr(),
                n,
                (&mut len) as *mut usize,
            );

            trace!(
                target: "bitcoinleveldb_db::c_api",
                len = len,
                filter_is_null = filter.is_null(),
                "LevelDBFilterPolicy::create_filter callback returned"
            );

            if len > 0 {
                if filter.is_null() {
                    error!(
                        target: "bitcoinleveldb_db::c_api",
                        len = len,
                        "LevelDBFilterPolicy::create_filter callback returned null with nonzero length"
                    );
                } else {
                    let bytes: &[u8] = core::slice::from_raw_parts(filter as *const u8, len);
                    dst.extend_from_slice(bytes);
                }
            }

            if !filter.is_null() {
                libc::free(filter as *mut core::ffi::c_void);
            }
        }

        trace!(
            target: "bitcoinleveldb_db::c_api",
            dst_len = dst.len(),
            "LevelDBFilterPolicy::create_filter exit"
        );

        /*
            std::vector<const char*> key_pointers(n);
            std::vector<size_t> key_sizes(n);
            for (int i = 0; i < n; i++) {
              key_pointers[i] = keys[i].data();
              key_sizes[i] = keys[i].size();
            }
            size_t len;
            char* filter = (*create_)(state_, &key_pointers[0], &key_sizes[0], n, &len);
            dst->append(filter, len);
            free(filter);
        */
    }
}

impl KeyMayMatch for LevelDBFilterPolicy {
    fn key_may_match(&self, key_: &Slice, filter: &Slice) -> bool {
        trace!(
            target: "bitcoinleveldb_db::c_api",
            key_len = *key_.size(),
            filter_len = *filter.size(),
            "LevelDBFilterPolicy::key_may_match entry"
        );

        let r = (self.key_match)(
            self.state,
            *key_.data(),
            *key_.size(),
            *filter.data(),
            *filter.size(),
        );

        let ok = r != 0;

        trace!(
            target: "bitcoinleveldb_db::c_api",
            result_u8 = r,
            ok = ok,
            "LevelDBFilterPolicy::key_may_match exit"
        );

        ok
    }
}
