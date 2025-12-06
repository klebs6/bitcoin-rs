// ---------------- [ File: bitcoinleveldb-duplex/src/two_level_iterator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/two_level_iterator.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/two_level_iterator.cc]

/// BlockFunction corresponds to the C++
///   `typedef Iterator* (*BlockFunction)(void*, const ReadOptions&, const Slice&);`
///
/// We model `nullptr` by returning `None`.
pub type BlockFunction =
    fn(arg: *mut c_void, options: &ReadOptions, index_value: &Slice)
        -> Option<Box<dyn LevelDBIteratorInterface>>;

#[repr(C)]
pub struct TwoLevelIterator {
    /// Embedded base iterator object (mirrors C++ inheritance from `Iterator`).
    base:              LevelDBIterator,
    block_function:    BlockFunction,
    arg:               *mut c_void,
    options:           ReadOptions,
    status:            Status,
    index_iter:        LevelDBIteratorWrapper,

    /**
       May be nullptr (represented as `None` inside the wrapper)
    */
    data_iter:         LevelDBIteratorWrapper,

    /**
      | If data_iter_ is non-null, then
      | "data_block_handle_" holds the
      | "index_value" passed to block_function_ to
      | create the data_iter_.
      |
      | In C++ this is a `std::string` (arbitrary bytes);
      | here we store the raw bytes explicitly.
      */
    data_block_handle: Vec<u8>,
}

impl TwoLevelIterator {
    
    pub fn valid(&self) -> bool {
        let is_valid = self.data_iter.valid();
        trace!(
            "TwoLevelIterator::valid: data_iter_has_iter={}, data_iter_valid={}",
            self.data_iter.has_iterator(),
            is_valid,
        );
        is_valid
    }
   
    pub fn key(&self) -> Slice {
        trace!(
            "TwoLevelIterator::key: requested; data_iter_has_iter={}, data_iter_valid={}",
            self.data_iter.has_iterator(),
            self.data_iter.valid(),
        );

        assert!(
            self.valid(),
            "TwoLevelIterator::key requires iterator to be valid"
        );

        let k = self.data_iter.key();

        trace!(
            "TwoLevelIterator::key: delegated to data_iter; key_size={}",
            k.size()
        );

        k
    }
   
    pub fn value(&self) -> Slice {
        trace!(
            "TwoLevelIterator::value: requested; data_iter_has_iter={}, data_iter_valid={}",
            self.data_iter.has_iterator(),
            self.data_iter.valid(),
        );

        assert!(
            self.valid(),
            "TwoLevelIterator::value requires iterator to be valid"
        );

        let v = self.data_iter.value();

        trace!(
            "TwoLevelIterator::value: delegated to data_iter; value_size={}",
            v.size()
        );

        v
    }

    pub fn status(&self) -> Status {
        trace!("TwoLevelIterator::status: aggregating status");

        // It'd be nice if status() returned a const Status& instead of a Status
        let index_status = self.index_iter.status();
        if !index_status.is_ok() {
            trace!(
                "TwoLevelIterator::status: returning index iterator status; code={:?}",
                index_status.code()
            );
            return index_status;
        }

        if self.data_iter.iter().is_some() {
            let data_status = self.data_iter.status();
            if !data_status.is_ok() {
                trace!(
                    "TwoLevelIterator::status: returning data iterator status; code={:?}",
                    data_status.code()
                );
                return data_status;
            }
        }

        trace!(
            "TwoLevelIterator::status: returning internal status; code={:?}",
            self.status.code()
        );

        Status::new_from_other_copy(&self.status)
    }

    pub fn save_error(&mut self, s: &Status) {
        trace!(
            "TwoLevelIterator::save_error: current_ok={}, incoming_ok={}",
            self.status.is_ok(),
            s.is_ok()
        );

        if self.status.is_ok() && !s.is_ok() {
            trace!(
                "TwoLevelIterator::save_error: capturing first error; code={:?}",
                s.code()
            );
            self.status.assign_from_other_copy(s);
        } else {
            trace!(
                "TwoLevelIterator::save_error: keeping existing status; \
                 current_code={:?}, incoming_code={:?}",
                self.status.code(),
                s.code()
            );
        }
    }

    /// Constructor closely mirrors the C++:
    ///
    /// `TwoLevelIterator(Iterator* index_iter, BlockFunction block_function,
    ///                   void* arg, const ReadOptions& options);`
    ///
    /// We take ownership of the `index_iter` via `Box<dyn LevelDBIteratorInterface>`.
    pub fn new(
        index_iter:     Box<dyn LevelDBIteratorInterface>,
        block_function: BlockFunction,
        arg:            *mut c_void,
        options:        ReadOptions,
    ) -> Self {
        let raw_index: *const dyn LevelDBIteratorInterface = &*index_iter;

        trace!(
            "TwoLevelIterator::new: constructing; index_iter={:p}, \
             verify_checksums={}, fill_cache={}",
            raw_index,
            options.verify_checksums(),
            options.fill_cache(),
        );

        let base = LevelDBIterator::new();

        let index_iter_wrapper = LevelDBIteratorWrapper::new(Some(index_iter));
        let data_iter_wrapper  = LevelDBIteratorWrapper::default();

        let mut me = TwoLevelIterator {
            base,
            block_function,
            arg,
            options,
            status: Status::ok(),
            index_iter: index_iter_wrapper,
            data_iter:  data_iter_wrapper,
            data_block_handle: Vec::new(),
        };

        // Initial positioning matches the C++ constructor behavior:
        // constructor does not perform any seek/init by itself.
        trace!("TwoLevelIterator::new: constructed; initial_valid={}", me.valid());

        me
    }

    pub fn seek(&mut self, target: &Slice) {
        trace!(
            "TwoLevelIterator::seek: target_size={}, index_valid_before={}, data_valid_before={}",
            target.size(),
            self.index_iter.valid(),
            self.data_iter.valid(),
        );

        self.index_iter.seek(target);
        self.init_data_block();

        if self.data_iter.iter().is_some() {
            self.data_iter.seek(target);
        }

        self.skip_empty_data_blocks_forward();

        trace!(
            "TwoLevelIterator::seek: after; index_valid={}, data_valid={}",
            self.index_iter.valid(),
            self.data_iter.valid(),
        );
    }
   
    pub fn seek_to_first(&mut self) {
        trace!(
            "TwoLevelIterator::seek_to_first: index_valid_before={}, data_valid_before={}",
            self.index_iter.valid(),
            self.data_iter.valid(),
        );

        self.index_iter.seek_to_first();
        self.init_data_block();

        if self.data_iter.iter().is_some() {
            self.data_iter.seek_to_first();
        }

        self.skip_empty_data_blocks_forward();

        trace!(
            "TwoLevelIterator::seek_to_first: after; index_valid={}, data_valid={}",
            self.index_iter.valid(),
            self.data_iter.valid(),
        );
    }
   
    pub fn seek_to_last(&mut self) {
        trace!(
            "TwoLevelIterator::seek_to_last: index_valid_before={}, data_valid_before={}",
            self.index_iter.valid(),
            self.data_iter.valid(),
        );

        self.index_iter.seek_to_last();
        self.init_data_block();

        if self.data_iter.iter().is_some() {
            self.data_iter.seek_to_last();
        }

        self.skip_empty_data_blocks_backward();

        trace!(
            "TwoLevelIterator::seek_to_last: after; index_valid={}, data_valid={}",
            self.index_iter.valid(),
            self.data_iter.valid(),
        );
    }

    pub fn next(&mut self) {
        trace!(
            "TwoLevelIterator::next: begin; data_valid_before={}",
            self.valid()
        );

        assert!(
            self.valid(),
            "TwoLevelIterator::next requires iterator to be valid"
        );

        self.data_iter.next();
        self.skip_empty_data_blocks_forward();

        trace!(
            "TwoLevelIterator::next: end; data_valid_after={}",
            self.valid()
        );
    }

    pub fn prev(&mut self) {
        trace!(
            "TwoLevelIterator::prev: begin; data_valid_before={}",
            self.valid()
        );

        assert!(
            self.valid(),
            "TwoLevelIterator::prev requires iterator to be valid"
        );

        self.data_iter.prev();
        self.skip_empty_data_blocks_backward();

        trace!(
            "TwoLevelIterator::prev: end; data_valid_after={}",
            self.valid()
        );
    }

    pub fn skip_empty_data_blocks_forward(&mut self) {
        trace!(
            "TwoLevelIterator::skip_empty_data_blocks_forward: \
             start; index_valid={}, data_valid={}, data_has_iter={}",
            self.index_iter.valid(),
            self.data_iter.valid(),
            self.data_iter.has_iterator(),
        );

        while self.data_iter.iter().is_none() || !self.data_iter.valid() {
            // Move to next block
            if !self.index_iter.valid() {
                trace!(
                    "TwoLevelIterator::skip_empty_data_blocks_forward: \
                     index iterator exhausted; clearing data iterator"
                );
                self.set_data_iterator(None);
                return;
            }

            self.index_iter.next();
            self.init_data_block();

            if self.data_iter.iter().is_some() {
                self.data_iter.seek_to_first();
            }

            trace!(
                "TwoLevelIterator::skip_empty_data_blocks_forward: \
                 advanced block; index_valid={}, data_valid={}, data_has_iter={}",
                self.index_iter.valid(),
                self.data_iter.valid(),
                self.data_iter.has_iterator(),
            );
        }

        trace!(
            "TwoLevelIterator::skip_empty_data_blocks_forward: done; data_valid={}",
            self.data_iter.valid()
        );
    }

    pub fn skip_empty_data_blocks_backward(&mut self) {
        trace!(
            "TwoLevelIterator::skip_empty_data_blocks_backward: \
             start; index_valid={}, data_valid={}, data_has_iter={}",
            self.index_iter.valid(),
            self.data_iter.valid(),
            self.data_iter.has_iterator(),
        );

        while self.data_iter.iter().is_none() || !self.data_iter.valid() {
            // Move to previous block
            if !self.index_iter.valid() {
                trace!(
                    "TwoLevelIterator::skip_empty_data_blocks_backward: \
                     index iterator exhausted; clearing data iterator"
                );
                self.set_data_iterator(None);
                return;
            }

            self.index_iter.prev();
            self.init_data_block();

            if self.data_iter.iter().is_some() {
                self.data_iter.seek_to_last();
            }

            trace!(
                "TwoLevelIterator::skip_empty_data_blocks_backward: \
                 moved block; index_valid={}, data_valid={}, data_has_iter={}",
                self.index_iter.valid(),
                self.data_iter.valid(),
                self.data_iter.has_iterator(),
            );
        }

        trace!(
            "TwoLevelIterator::skip_empty_data_blocks_backward: done; data_valid={}",
            self.data_iter.valid()
        );
    }
   
    /// Set the data iterator, preserving the first non-OK status we observe.
    pub fn set_data_iterator(
        &mut self,
        data_iter: Option<Box<dyn LevelDBIteratorInterface>>,
    ) {
        trace!(
            "TwoLevelIterator::set_data_iterator: \
             existing_has_iter={}, new_has_iter={}",
            self.data_iter.has_iterator(),
            data_iter.is_some(),
        );

        if self.data_iter.iter().is_some() {
            let s = self.data_iter.status();
            if !s.is_ok() {
                trace!(
                    "TwoLevelIterator::set_data_iterator: \
                     existing data_iter has error; code={:?}",
                    s.code()
                );
            }
            self.save_error(&s);
        }

        self.data_iter.set(data_iter);

        trace!(
            "TwoLevelIterator::set_data_iterator: \
             now_has_iter={}, now_valid={}",
            self.data_iter.has_iterator(),
            self.data_iter.valid(),
        );
    }
   
    pub fn init_data_block(&mut self) {
        trace!(
            "TwoLevelIterator::init_data_block: index_valid={}, \
             current_data_has_iter={}, current_data_valid={}",
            self.index_iter.valid(),
            self.data_iter.has_iterator(),
            self.data_iter.valid(),
        );

        if !self.index_iter.valid() {
            trace!(
                "TwoLevelIterator::init_data_block: index iterator invalid; \
                 clearing data iterator"
            );
            self.set_data_iterator(None);
            self.data_block_handle.clear();
            return;
        }

        let handle = self.index_iter.value();
        let handle_len = *handle.size();

        let reuse_existing = if self.data_iter.iter().is_some() && !self.data_block_handle.is_empty() {
            let saved = Slice::from(self.data_block_handle.as_slice());
            let cmp = handle.compare(&saved);
            trace!(
                "TwoLevelIterator::init_data_block: comparing handle vs cached; \
                 handle_len={}, cached_len={}, cmp={}",
                handle_len,
                self.data_block_handle.len(),
                cmp,
            );
            cmp == 0
        } else {
            false
        };

        if reuse_existing {
            trace!(
                "TwoLevelIterator::init_data_block: \
                 reusing existing data iterator for same block handle"
            );
            return;
        }

        trace!(
            "TwoLevelIterator::init_data_block: \
             constructing new data iterator via block_function; handle_len={}",
            handle_len
        );

        let iter_opt = (self.block_function)(self.arg, &self.options, &handle);

        match iter_opt {
            None => {
                trace!(
                    "TwoLevelIterator::init_data_block: block_function returned None; \
                     clearing data iterator"
                );
                self.data_block_handle.clear();
                self.set_data_iterator(None);
            }
            Some(iter_box) => {
                // Copy handle bytes into `data_block_handle_`.
                self.data_block_handle.clear();

                if handle_len > 0 {
                    unsafe {
                        let data_ptr = *handle.data();
                        let slice = std::slice::from_raw_parts(data_ptr, handle_len);
                        self.data_block_handle.extend_from_slice(slice);
                    }
                }

                let raw_iter: *const dyn LevelDBIteratorInterface = &*iter_box;
                trace!(
                    "TwoLevelIterator::init_data_block: new data iterator={:p}, \
                     cached_handle_len={}",
                    raw_iter,
                    self.data_block_handle.len(),
                );

                self.set_data_iterator(Some(iter_box));
            }
        }
    }
}

impl LevelDBIteratorInterface for TwoLevelIterator {}

impl Valid for TwoLevelIterator {
    fn valid(&self) -> bool {
        TwoLevelIterator::valid(self)
    }
}

impl SeekToFirst for TwoLevelIterator {
    fn seek_to_first(&mut self) {
        TwoLevelIterator::seek_to_first(self)
    }
}

impl SeekToLast for TwoLevelIterator {
    fn seek_to_last(&mut self) {
        TwoLevelIterator::seek_to_last(self)
    }
}

impl Seek for TwoLevelIterator {
    fn seek(&mut self, target: &Slice) {
        TwoLevelIterator::seek(self, target)
    }
}

impl Next for TwoLevelIterator {
    fn next(&mut self) {
        TwoLevelIterator::next(self)
    }
}

impl Prev for TwoLevelIterator {
    fn prev(&mut self) {
        TwoLevelIterator::prev(self)
    }
}

impl LevelDBIteratorStatus for TwoLevelIterator {
    fn status(&self) -> Status {
        TwoLevelIterator::status(self)
    }
}

impl Key for TwoLevelIterator {
    fn key(&self) -> Slice {
        TwoLevelIterator::key(self)
    }
}

impl Value for TwoLevelIterator {
    fn value(&self) -> Slice {
        TwoLevelIterator::value(self)
    }
}

/**
  | Return a new two level iterator.  A two-level
  | iterator contains an index iterator whose
  | values point to a sequence of blocks where each
  | block is itself a sequence of key,value pairs.
  | The returned two-level iterator yields the
  | concatenation of all key/value pairs in the
  | sequence of blocks.  Takes ownership of
  | `index_iter` and will delete it when no longer
  | needed.
  |
  | Uses a supplied function to convert an
  | index_iter value into an iterator over the
  | contents of the corresponding block.
  */
pub fn new_two_level_iterator(
    index_iter:     Box<dyn LevelDBIteratorInterface>,
    block_function: BlockFunction,
    arg:            *mut c_void,
    options:        ReadOptions,
) -> Box<dyn LevelDBIteratorInterface> {
    trace!("new_two_level_iterator: constructing composite iterator");
    Box::new(TwoLevelIterator::new(index_iter, block_function, arg, options))
}

// ======================================================================
// Tests
// ======================================================================

#[cfg(test)]
mod two_level_iterator_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    /// A simple block-function for tests:
    /// - `arg`: points to `Vec<Vec<(Vec<u8>, Vec<u8>)>>`, one inner Vec per block.
    /// - `handle`: ASCII decimal string with the block index.
    fn test_block_function(
        arg: *mut c_void,
        _options: &ReadOptions,
        handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        let handle_str = handle.to_string();
        let block_index: usize = handle_str
            .parse()
            .expect("test_block_function: handle should be an ASCII usize");

        let blocks_ptr = arg as *mut Vec<Vec<(Vec<u8>, Vec<u8>)>>;
        assert!(
            !blocks_ptr.is_null(),
            "test_block_function: arg must not be null"
        );

        let blocks: &mut Vec<Vec<(Vec<u8>, Vec<u8>)>> = unsafe { &mut *blocks_ptr };

        if block_index >= blocks.len() {
            return None;
        }

        let entries_ref = &blocks[block_index];
        if entries_ref.is_empty() {
            // Empty block: use MockStubIterator::new_empty
            let iter = MockStubIterator::new_empty();
            return Some(Box::new(iter));
        }

        // Non-empty block: build MockStubIterator with entries.
        let mut pairs: Vec<(&[u8], &[u8])> = Vec::with_capacity(entries_ref.len());
        for (k, v) in entries_ref.iter() {
            pairs.push((k.as_slice(), v.as_slice()));
        }

        let iter = MockStubIterator::new_with_entries(&pairs);
        Some(Box::new(iter))
    }

    fn make_index_iterator(num_blocks: usize) -> Box<dyn LevelDBIteratorInterface> {
        // Index iterator entries: key="k{idx}", value="{idx}"
        let mut pairs: Vec<(Vec<u8>, Vec<u8>)> = Vec::with_capacity(num_blocks);
        for i in 0..num_blocks {
            let key_str = format!("k{}", i);
            let val_str = format!("{}", i);
            pairs.push((key_str.into_bytes(), val_str.into_bytes()));
        }

        let pairs_slice: Vec<(&[u8], &[u8])> = pairs
            .iter()
            .map(|(k, v)| (k.as_slice(), v.as_slice()))
            .collect();

        let mut iter = MockStubIterator::new_with_entries(&pairs_slice);
        iter.seek_to_first();

        Box::new(iter)
    }

    fn collect_all_kv(it: &mut TwoLevelIterator) -> Vec<(String, String)> {
        let mut out = Vec::new();

        if !it.valid() {
            return out;
        }

        loop {
            let k = it.key().to_string();
            let v = it.value().to_string();
            out.push((k, v));

            it.next();
            if !it.valid() {
                break;
            }
        }

        out
    }

    #[traced_test]
    fn two_level_iterator_basic_forward_across_blocks() {
        // Blocks:
        //  block 0: (a0 -> v0), (a1 -> v1)
        //  block 1: (b0 -> v2)
        //  block 2: (c0 -> v3, c1 -> v4)
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = Vec::new();
        blocks.push(vec![
            (b"a0".to_vec(), b"v0".to_vec()),
            (b"a1".to_vec(), b"v1".to_vec()),
        ]);
        blocks.push(vec![(b"b0".to_vec(), b"v2".to_vec())]);
        blocks.push(vec![
            (b"c0".to_vec(), b"v3".to_vec()),
            (b"c1".to_vec(), b"v4".to_vec()),
        ]);

        let mut blocks_holder = blocks;
        let arg: *mut c_void = &mut blocks_holder as *mut _ as *mut c_void;

        let index_iter = make_index_iterator(blocks_holder.len());
        let options = ReadOptions::default();

        let mut two = TwoLevelIterator::new(index_iter, test_block_function, arg, options);

        two.seek_to_first();
        assert!(two.valid(), "iterator should be valid after seek_to_first");

        let entries = collect_all_kv(&mut two);

        let expected = vec![
            ("a0".to_string(), "v0".to_string()),
            ("a1".to_string(), "v1".to_string()),
            ("b0".to_string(), "v2".to_string()),
            ("c0".to_string(), "v3".to_string()),
            ("c1".to_string(), "v4".to_string()),
        ];

        assert_eq!(entries, expected);
        assert!(two.status().is_ok(), "status should remain OK");
    }

    #[traced_test]
    fn two_level_iterator_skips_empty_blocks_forward() {
        // Blocks:
        //  block 0: empty
        //  block 1: (k1 -> v1)
        //  block 2: empty
        //  block 3: (k3 -> v3)
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = Vec::new();
        blocks.push(vec![]);
        blocks.push(vec![(b"k1".to_vec(), b"v1".to_vec())]);
        blocks.push(vec![]);
        blocks.push(vec![(b"k3".to_vec(), b"v3".to_vec())]);

        let mut blocks_holder = blocks;
        let arg: *mut c_void = &mut blocks_holder as *mut _ as *mut c_void;

        let index_iter = make_index_iterator(blocks_holder.len());
        let options = ReadOptions::default();

        let mut two = TwoLevelIterator::new(index_iter, test_block_function, arg, options);

        two.seek_to_first();
        assert!(two.valid(), "iterator should be valid after seek_to_first");

        let entries = collect_all_kv(&mut two);

        let expected = vec![
            ("k1".to_string(), "v1".to_string()),
            ("k3".to_string(), "v3".to_string()),
        ];

        assert_eq!(entries, expected);
        assert!(two.status().is_ok(), "status should remain OK");
    }

    #[traced_test]
    fn two_level_iterator_seek_and_prev_across_blocks() {
        // Blocks:
        //  block 0: (a0 -> v0)
        //  block 1: (b0 -> v1, b1 -> v2)
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = Vec::new();
        blocks.push(vec![(b"a0".to_vec(), b"v0".to_vec())]);
        blocks.push(vec![
            (b"b0".to_vec(), b"v1".to_vec()),
            (b"b1".to_vec(), b"v2".to_vec()),
        ]);

        let mut blocks_holder = blocks;
        let arg: *mut c_void = &mut blocks_holder as *mut _ as *mut c_void;

        let index_iter = make_index_iterator(blocks_holder.len());
        let options = ReadOptions::default();

        let mut two = TwoLevelIterator::new(index_iter, test_block_function, arg, options);

        two.seek_to_last();
        assert!(two.valid(), "iterator should be valid after seek_to_last");

        // Expect to see entries in reverse: b1, b0, a0
        let mut seen = Vec::new();
        loop {
            let k = two.key().to_string();
            let v = two.value().to_string();
            seen.push((k, v));
            two.prev();
            if !two.valid() {
                break;
            }
        }

        let expected = vec![
            ("b1".to_string(), "v2".to_string()),
            ("b0".to_string(), "v1".to_string()),
            ("a0".to_string(), "v0".to_string()),
        ];

        assert_eq!(seen, expected);
        assert!(two.status().is_ok(), "status should remain OK");
    }

    /// Iterator that always reports an error status.
    struct ErrorStatusIterator {
        status: Status,
    }

    impl ErrorStatusIterator {
        fn new_error() -> Self {
            let msg = Slice::from("error");
            ErrorStatusIterator {
                status: Status::io_error(&msg, None),
            }
        }
    }

    impl LevelDBIteratorInterface for ErrorStatusIterator {}

    impl Valid for ErrorStatusIterator {
        fn valid(&self) -> bool {
            false
        }
    }

    impl SeekToFirst for ErrorStatusIterator {
        fn seek_to_first(&mut self) {}
    }

    impl SeekToLast for ErrorStatusIterator {
        fn seek_to_last(&mut self) {}
    }

    impl Seek for ErrorStatusIterator {
        fn seek(&mut self, _target: &Slice) {}
    }

    impl Next for ErrorStatusIterator {
        fn next(&mut self) {}
    }

    impl Prev for ErrorStatusIterator {
        fn prev(&mut self) {}
    }

    impl LevelDBIteratorStatus for ErrorStatusIterator {
        fn status(&self) -> Status {
            Status::new_from_other_copy(&self.status)
        }
    }

    impl Key for ErrorStatusIterator {
        fn key(&self) -> Slice {
            panic!("ErrorStatusIterator::key should not be called");
        }
    }

    impl Value for ErrorStatusIterator {
        fn value(&self) -> Slice {
            panic!("ErrorStatusIterator::value should not be called");
        }
    }

    /// Block function that is never actually invoked; used for error-status tests.
    fn unused_block_function(
        _arg: *mut c_void,
        _options: &ReadOptions,
        _handle: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        None
    }

    #[traced_test]
    fn two_level_iterator_status_prefers_index_error() {
        let index_iter: Box<dyn LevelDBIteratorInterface> =
            Box::new(ErrorStatusIterator::new_error());
        let options = ReadOptions::default();
        let mut two = TwoLevelIterator::new(index_iter, unused_block_function, std::ptr::null_mut(), options);

        let st = two.status();
        assert!(!st.is_ok(), "status should reflect index error");
        assert!(st.is_io_error(), "expected IO error code");
    }

    #[traced_test]
    fn new_two_level_iterator_helper_produces_working_iterator() {
        let mut blocks: Vec<Vec<(Vec<u8>, Vec<u8>)>> = Vec::new();
        blocks.push(vec![
            (b"a".to_vec(), b"1".to_vec()),
            (b"b".to_vec(), b"2".to_vec()),
        ]);

        let mut blocks_holder = blocks;
        let arg: *mut c_void = &mut blocks_holder as *mut _ as *mut c_void;

        let index_iter = make_index_iterator(blocks_holder.len());
        let options = ReadOptions::default();

        let mut it_box = new_two_level_iterator(index_iter, test_block_function, arg, options);
        let two: &mut TwoLevelIterator = it_box
            .as_any_mut()
            .downcast_mut::<TwoLevelIterator>()
            .expect("new_two_level_iterator should return a TwoLevelIterator");

        two.seek_to_first();
        assert!(two.valid());
        let k1 = two.key().to_string();
        let v1 = two.value().to_string();
        assert_eq!(k1, "a");
        assert_eq!(v1, "1");
    }

    trait AsAny {
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
    }

    impl AsAny for dyn LevelDBIteratorInterface {
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
            self
        }
    }

    impl AsAny for TwoLevelIterator {
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
            self
        }
    }
}
