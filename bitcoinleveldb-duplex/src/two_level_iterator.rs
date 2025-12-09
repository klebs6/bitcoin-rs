// ---------------- [ File: bitcoinleveldb-duplex/src/two_level_iterator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/two_level_iterator.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/two_level_iterator.cc]

/// BlockFunction corresponds to the C++
///   `typedef Iterator* (*BlockFunction)(void*, const ReadOptions&, const Slice&);`
///
/// We model `nullptr` by returning `None`.
pub type BlockFunction = fn(
    arg: *mut c_void,
    options: &ReadOptions,
    index_value: &Slice,
) -> Option<Box<dyn LevelDBIteratorInterface>>;

#[repr(C)]
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct TwoLevelIterator {
    /// Embedded base iterator object (mirrors C++ inheritance from `Iterator`).
    base:              LevelDBIterator,
    block_function:    BlockFunction,
    arg:               *mut c_void,
    options:           ReadOptions,

    /// Cached internal status used when both the index iterator
    /// and the data iterator report OK.
    ///
    /// This corresponds to the `status_` field in the original C++
    /// implementation.
    internal_status:   Status,

    index_iter:        LevelDBIterator,

    /**
       May be nullptr (represented as `None` inside the wrapper)
    */
    data_iter:         LevelDBIterator,

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

impl LevelDBIteratorInterface for TwoLevelIterator {}

impl TwoLevelIterator {

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

        let base = LevelDBIterator::default();

        let index_iter_wrapper = LevelDBIterator::new(Some(index_iter));
        let data_iter_wrapper  = LevelDBIterator::default();

        let mut me = TwoLevelIterator {
            base,
            block_function,
            arg,
            options,
            internal_status: Status::ok(),
            index_iter: index_iter_wrapper,
            data_iter:  data_iter_wrapper,
            data_block_handle: Vec::new(),
        };

        // Initial positioning matches the C++ constructor behavior:
        // constructor does not perform any seek/init by itself.
        trace!("TwoLevelIterator::new: constructed; initial_valid={}", me.valid());

        me
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
    options:        &ReadOptions,
) -> Box<dyn LevelDBIteratorInterface> {
    trace!("new_two_level_iterator: constructing composite iterator");
    Box::new(TwoLevelIterator::new(index_iter, block_function, arg, options.clone()))
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

    impl LevelDBIteratorValid for ErrorStatusIterator {
        fn valid(&self) -> bool {
            false
        }
    }

    impl LevelDBIteratorSeekToFirst for ErrorStatusIterator {
        fn seek_to_first(&mut self) {}
    }

    impl LevelDBIteratorSeekToLast for ErrorStatusIterator {
        fn seek_to_last(&mut self) {}
    }

    impl LevelDBIteratorSeek for ErrorStatusIterator {
        fn seek(&mut self, _target: &Slice) {}
    }

    impl LevelDBIteratorNext for ErrorStatusIterator {
        fn next(&mut self) {}
    }

    impl LevelDBIteratorPrev for ErrorStatusIterator {
        fn prev(&mut self) {}
    }

    impl LevelDBIteratorStatus for ErrorStatusIterator {
        fn status(&self) -> Status {
            Status::new_from_other_copy(&self.status)
        }
    }

    impl LevelDBIteratorKey for ErrorStatusIterator {
        fn key(&self) -> Slice {
            panic!("ErrorStatusIterator::key should not be called");
        }
    }

    impl LevelDBIteratorValue for ErrorStatusIterator {
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
        let mut two = TwoLevelIterator::new(
            index_iter,
            unused_block_function,
            std::ptr::null_mut(),
            options,
        );

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

        let mut it_box = new_two_level_iterator(index_iter, test_block_function, arg, &options);

        // Exercise purely through the iterator interface (no downcast).
        it_box.seek_to_first();
        assert!(it_box.valid(), "iterator from helper should be valid");

        let k1 = it_box.key().to_string();
        let v1 = it_box.value().to_string();

        assert_eq!(k1, "a");
        assert_eq!(v1, "1");
    }
}
