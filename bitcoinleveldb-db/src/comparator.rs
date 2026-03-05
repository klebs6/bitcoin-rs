// ---------------- [ File: bitcoinleveldb-db/src/comparator.rs ]
crate::ix!();

pub struct LevelDBComparator {
    state:      *mut c_void,
    destructor: fn(_0: *mut c_void),
    compare:    fn(
        _0:   *mut c_void,
        a:    *const u8,
        alen: usize,
        b:    *const u8,
        blen: usize,
    ) -> i32,
    name:       fn(_0: *mut c_void) -> *const u8,
}

impl Comparator<Slice> for LevelDBComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> core::cmp::Ordering {
        trace!(
            target: "bitcoinleveldb_db::c_api",
            alen = *a.size(),
            blen = *b.size(),
            "LevelDBComparator::compare entry"
        );

        let r = (self.compare)(
            self.state,
            *a.data(),
            *a.size(),
            *b.data(),
            *b.size(),
        );

        let ord = if r < 0 {
            core::cmp::Ordering::Less
        } else if r > 0 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        };

        trace!(
            target: "bitcoinleveldb_db::c_api",
            result = r,
            ordering = ?ord,
            "LevelDBComparator::compare exit"
        );

        ord
    }

}

impl FindShortestSeparator for LevelDBComparator {
    fn find_shortest_separator(&self, _0: &mut Vec<u8>, _1: &[u8]) {
        trace!(
            target: "bitcoinleveldb_db::c_api",
            "LevelDBComparator::find_shortest_separator noop"
        );
    }
}

impl Drop for LevelDBComparator {
    fn drop(&mut self) {
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBComparator::drop entry");
        (self.destructor)(self.state);
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBComparator::drop exit");
    }
}

impl Named for LevelDBComparator {
    fn name(&self) -> Cow<'_, str> {
        trace!(target: "bitcoinleveldb_db::c_api", "LevelDBComparator::name entry");
        let p = (self.name)(self.state);
        trace!(
            target: "bitcoinleveldb_db::c_api",
            ptr_is_null = p.is_null(),
            "LevelDBComparator::name exit"
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

impl SliceComparator for LevelDBComparator {}
