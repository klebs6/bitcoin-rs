// ---------------- [ File: bitcoinleveldb-iterator/src/version_iterator.rs ]
crate::ix!();

/**
  | An internal iterator.  For a given
  | version/level pair, yields information about
  | the files in the level.  For a given entry,
  | key() is the largest key that occurs in the
  | file, and value() is an 16-byte value
  | containing the file number and file size, both
  | encoded using EncodeFixed64.
  */
pub struct VersionLevelFileNumIterator {
    base:      LevelDBIterator,
    icmp:      InternalKeyComparator,
    flist:     *const Vec<*mut FileMetaData>,
    index:     u32,
    /**
      | Backing store for value(). Holds the
      | file number and size.
      |
      */
    value_buf: RefCell<[u8; 16]>,
}

impl VersionLevelFileNumIterator {
    
    pub fn new(
        icmp:  &InternalKeyComparator,
        flist: *const Vec<*mut FileMetaData>,
    ) -> Self {
        trace!(
            "VersionLevelFileNumIterator::new: icmp_user_comparator={:?}, flist_ptr={:?}",
            icmp.user_comparator(),
            flist
        );

        assert!(
            !flist.is_null(),
            "VersionLevelFileNumIterator::new: flist pointer must not be null"
        );

        let initial_index: u32 = unsafe { (*flist).len() as u32 };

        VersionLevelFileNumIterator {
            base:      LevelDBIterator::new(),
            icmp:      InternalKeyComparator::new(icmp.user_comparator()),
            flist,
            index:     initial_index, // Marks as invalid
            value_buf: RefCell::new([0u8; 16]),
        }
    }
   
    pub fn valid(&self) -> bool {
        trace!(
            "VersionLevelFileNumIterator::valid: index={}, flist_ptr={:?}",
            self.index,
            self.flist
        );

        assert!(
            !self.flist.is_null(),
            "VersionLevelFileNumIterator::valid: flist pointer must not be null"
        );

        unsafe {
            let len = (*self.flist).len() as u32;
            let v = self.index < len;
            trace!(
                "VersionLevelFileNumIterator::valid: flist_len={}, is_valid={}",
                len,
                v
            );
            v
        }
    }
   
    pub fn seek(&mut self, target: &Slice) {
        trace!(
            "VersionLevelFileNumIterator::seek: target={:?}, flist_ptr={:?}",
            target,
            self.flist
        );

        assert!(
            !self.flist.is_null(),
            "VersionLevelFileNumIterator::seek: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> = &*self.flist;
            let idx = find_file(&self.icmp, files_ref, target);
            self.index = idx as u32;

            trace!(
                "VersionLevelFileNumIterator::seek: FindFile returned index={}, flist_len={}",
                self.index,
                files_ref.len()
            );
        }
    }
 
    pub fn seek_to_first(&mut self) {
        trace!(
            "VersionLevelFileNumIterator::seek_to_first: flist_ptr={:?}",
            self.flist
        );
        self.index = 0;
        trace!(
            "VersionLevelFileNumIterator::seek_to_first: index set to {}",
            self.index
        );
    }

    pub fn seek_to_last(&mut self) {
        trace!(
            "VersionLevelFileNumIterator::seek_to_last: flist_ptr={:?}",
            self.flist
        );

        assert!(
            !self.flist.is_null(),
            "VersionLevelFileNumIterator::seek_to_last: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> = &*self.flist;
            if files_ref.is_empty() {
                self.index = 0;
                trace!(
                    "VersionLevelFileNumIterator::seek_to_last: empty file list; index set to 0 (invalid)"
                );
            } else {
                self.index = (files_ref.len() - 1) as u32;
                trace!(
                    "VersionLevelFileNumIterator::seek_to_last: flist_len={}, index set to {}",
                    files_ref.len(),
                    self.index
                );
            }
        }
    }
   
    pub fn next(&mut self) {
        trace!(
            "VersionLevelFileNumIterator::next: current index={}, flist_ptr={:?}",
            self.index,
            self.flist
        );

        assert!(
            self.valid(),
            "VersionLevelFileNumIterator::next requires iterator to be valid"
        );

        self.index = self.index.wrapping_add(1);

        trace!(
            "VersionLevelFileNumIterator::next: advanced to index={}",
            self.index
        );
    }

    pub fn prev(&mut self) {
        trace!(
            "VersionLevelFileNumIterator::prev: current index={}, flist_ptr={:?}",
            self.index,
            self.flist
        );

        assert!(
            self.valid(),
            "VersionLevelFileNumIterator::prev requires iterator to be valid"
        );
        assert!(
            !self.flist.is_null(),
            "VersionLevelFileNumIterator::prev: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> = &*self.flist;
            let len = files_ref.len() as u32;

            if self.index == 0 {
                // Marks as invalid (index == len)
                self.index = len;
                trace!(
                    "VersionLevelFileNumIterator::prev: moved before first; index set to {} (invalid)",
                    self.index
                );
            } else {
                self.index -= 1;
                trace!(
                    "VersionLevelFileNumIterator::prev: decremented index to {}",
                    self.index
                );
            }
        }
    }
   
    pub fn key(&self) -> Slice {
        trace!(
            "VersionLevelFileNumIterator::key: called; index={}, flist_ptr={:?}",
            self.index,
            self.flist
        );

        assert!(
            self.valid(),
            "VersionLevelFileNumIterator::key requires iterator to be valid"
        );
        assert!(
            !self.flist.is_null(),
            "VersionLevelFileNumIterator::key: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> = &*self.flist;
            let idx = self.index as usize;

            let meta_ptr: *mut FileMetaData = *files_ref
                .get(idx)
                .expect("VersionLevelFileNumIterator::key: index out of range");
            let meta: &FileMetaData = &*meta_ptr;

            let largest_internal: &InternalKey = meta.largest();
            let encoded: Slice = largest_internal.encode();

            trace!(
                "VersionLevelFileNumIterator::key: returning largest key for file_number={}, file_size={}",
                meta.number(),
                meta.file_size()
            );

            encoded
        }
    }

    pub fn value(&self) -> Slice {
        trace!(
            "VersionLevelFileNumIterator::value: called; index={}, flist_ptr={:?}",
            self.index,
            self.flist
        );

        assert!(
            self.valid(),
            "VersionLevelFileNumIterator::value requires iterator to be valid"
        );
        assert!(
            !self.flist.is_null(),
            "VersionLevelFileNumIterator::value: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> = &*self.flist;
            let idx = self.index as usize;

            let meta_ptr: *mut FileMetaData = *files_ref
                .get(idx)
                .expect("VersionLevelFileNumIterator::value: index out of range");
            let meta: &FileMetaData = &*meta_ptr;

            let mut buf = self.value_buf.borrow_mut();
            let ptr = buf.as_mut_ptr();

            encode_fixed64(ptr, meta.number());
            encode_fixed64(ptr.add(8), meta.file_size());

            let result = Slice::from_ptr_len(buf.as_ptr(), buf.len());

            trace!(
                "VersionLevelFileNumIterator::value: encoded (number={}, size={}) into 16-byte buffer",
                meta.number(),
                meta.file_size()
            );

            result
        }
    }
   
    pub fn status(&self) -> Status {
        trace!(
            "VersionLevelFileNumIterator::status: returning OK (iterator itself does not track errors)"
        );
        Status::ok()
    }
}
