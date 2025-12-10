// ---------------- [ File: bitcoinleveldb-tablerep/src/table_rep.rs ]
crate::ix!();

#[derive(MutGetters,Getters,Setters)]
#[getset(get = "pub",get_mut="pub",set="pub")]
pub struct TableRep {
    options:          Options,
    status:           Status,
    file:             Rc<RefCell<dyn RandomAccessFile>>,
    cache_id:         u64,
    filter:           Option<Box<FilterBlockReader>>,
    filter_data:      *mut u8,
    filter_data_len:  usize,

    /// Handle to metaindex_block: saved from
    /// footer
    /// 
    metaindex_handle: BlockHandle,
    index_block:      *mut Block,
}

impl core::fmt::Debug for TableRep {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Debug;

        // We cannot print the underlying RandomAccessFile safely,
        // but we *can* indicate pointer identity for debugging.
        let file_ptr = Rc::as_ptr(&self.file);

        f.debug_struct("TableRep")
            .field("options", &self.options)
            .field("status", &self.status)
            .field("file", &format_args!("{:p}", file_ptr))
            .field("cache_id", &self.cache_id)
            .field(
                "filter",
                if self.filter.is_some() {
                    &"Some(FilterBlockReader)"
                } else {
                    &"None"
                },
            )
            .field("filter_data", &format_args!("{:p}", self.filter_data))
            .field("filter_data_len", &self.filter_data_len)
            .field("metaindex_handle", &self.metaindex_handle)
            .field("index_block", &format_args!("{:p}", self.index_block))
            .finish()
    }
}

impl TableRep {
    pub fn new(
        options:          Options,
        file:             Rc<RefCell<dyn RandomAccessFile>>,
        cache_id:         u64,
        metaindex_handle: BlockHandle,
        index_block:      *mut Block,
    ) -> Self {
        trace!(
            "TableRep::new: cache_id={}, index_block={:?}",
            cache_id,
            index_block
        );

        TableRep {
            options,
            status:          Status::ok(),
            file,
            cache_id,
            filter:          None,
            filter_data:     core::ptr::null_mut(),
            filter_data_len: 0,
            metaindex_handle,
            index_block,
        }
    }
}

impl Drop for TableRep {
    fn drop(&mut self) {
        trace!(
            "TableRep::drop: cache_id={}, index_block={:?}, filter={:?}, filter_data={:?}, filter_data_len={}",
            self.cache_id,
            self.index_block,
            self.filter,
            self.filter_data,
            self.filter_data_len
        );

        unsafe {

            if !self.filter_data.is_null() {
                if self.filter_data_len > 0 {
                    trace!(
                        "TableRep::drop: freeing filter_data buffer @ {:?} (len={})",
                        self.filter_data,
                        self.filter_data_len
                    );
                    let slice =
                        core::ptr::slice_from_raw_parts_mut(self.filter_data, self.filter_data_len);
                    let _buf: Box<[u8]> = Box::from_raw(slice);
                    // drop(_buf) happens here
                } else {
                    debug!(
                        "TableRep::drop: filter_data non-null but len==0; skipping free to avoid UB"
                    );
                }
                self.filter_data = core::ptr::null_mut();
                self.filter_data_len = 0;
            } else {
                trace!("TableRep::drop: filter_data pointer is null; nothing to free");
            }

            if !self.index_block.is_null() {
                trace!(
                    "TableRep::drop: deleting index Block @ {:?}",
                    self.index_block
                );
                let _block_box: Box<Block> = Box::from_raw(self.index_block);
                self.index_block = core::ptr::null_mut();
            } else {
                trace!("TableRep::drop: index_block pointer is null; nothing to free");
            }
        }
    }
}

#[cfg(test)]
mod table_rep_drop_lifecycle_tests {
    use super::*;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct DummyRandomAccessFile;

    impl Named for DummyRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            trace!("DummyRandomAccessFile::name called");
            Cow::Borrowed("dummy-random-access-file")
        }
    }

    impl RandomAccessFileRead for DummyRandomAccessFile {
        fn read(
            &self,
            offset: u64,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            trace!(
                "DummyRandomAccessFile::read: offset={}, n={}, result_ptr={:?}, scratch_ptr={:?}",
                offset,
                n,
                result,
                scratch
            );
            let _ = (result, scratch);
            Status::ok()
        }
    }

    impl RandomAccessFile for DummyRandomAccessFile {}

    #[derive(Clone)]
    struct DropCountingFilterPolicy {
        drops: Arc<AtomicUsize>,
    }

    impl Drop for DropCountingFilterPolicy {
        fn drop(&mut self) {
            let prev = self.drops.fetch_add(1, Ordering::SeqCst);
            trace!(
                "DropCountingFilterPolicy::drop: previous_drop_count={}, new_drop_count={}",
                prev,
                prev + 1
            );
        }
    }

    impl Named for DropCountingFilterPolicy {
        fn name(&self) -> Cow<'_, str> {
            trace!("DropCountingFilterPolicy::name called");
            Cow::Borrowed("drop-counting-filter-policy")
        }
    }

    impl CreateFilter for DropCountingFilterPolicy {
        fn create_filter(
            &self,
            keys: *const Slice,
            n: i32,
            dst: &mut Vec<u8>,
        ) {
            trace!(
                "DropCountingFilterPolicy::create_filter: n={}, dst_len_before={}",
                n,
                dst.len()
            );
            let _ = keys;
        }
    }

    impl KeyMayMatch for DropCountingFilterPolicy {
        fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool {
            trace!(
                "DropCountingFilterPolicy::key_may_match: invoked; key_ptr={:?}, filter_ptr={:?}",
                key as *const Slice,
                filter as *const Slice
            );
            let _ = (key, filter);
            true
        }
    }

    impl FilterPolicy for DropCountingFilterPolicy {}

    fn make_dummy_file_handle() -> Rc<RefCell<dyn RandomAccessFile>> {
        trace!(
            "make_dummy_file_handle: constructing DummyRandomAccessFile inside Rc<RefCell<..>>"
        );
        Rc::new(RefCell::new(DummyRandomAccessFile))
    }

    fn make_default_options() -> Options {
        trace!("make_default_options: using Options::default()");
        Options::default()
    }

    fn make_ok_status() -> Status {
        trace!("make_ok_status: using Status::ok()");
        Status::ok()
    }

    fn construct_table_rep_for_test(
        cache_id: u64,
        filter_ptr: *mut FilterBlockReader,
        filter_data_ptr: *mut u8,
        filter_data_len: usize,
        index_block_ptr: *mut Block,
    ) -> TableRep {
        trace!(
            "construct_table_rep_for_test: cache_id={}, filter_ptr={:?}, filter_data_ptr={:?}, filter_data_len={}, index_block_ptr={:?}",
            cache_id,
            filter_ptr,
            filter_data_ptr,
            filter_data_len,
            index_block_ptr
        );

        TableRep {
            options: make_default_options(),
            status: make_ok_status(),
            file: make_dummy_file_handle(),
            cache_id,
            filter: filter_ptr,
            filter_data: filter_data_ptr,
            filter_data_len,
            metaindex_handle: BlockHandle::new(),
            index_block: index_block_ptr,
        }
    }

    fn create_filter_block_reader_with_drop_counter(
    ) -> (*mut FilterBlockReader, Arc<AtomicUsize>) {
        trace!(
            "create_filter_block_reader_with_drop_counter: constructing backing filter block contents"
        );

        // Minimal well‑formed filter block layout:
        // [0..4]  = u32 offset of filter array (0)
        // [4]     = base_lg (arbitrary, e.g. 11)
        let mut backing = vec![0u8; 5];
        backing[0] = 0;
        backing[1] = 0;
        backing[2] = 0;
        backing[3] = 0;
        backing[4] = 11;

        let contents_slice = Slice::from(&backing[..]);

        let drop_counter = Arc::new(AtomicUsize::new(0));
        let policy = DropCountingFilterPolicy {
            drops: drop_counter.clone(),
        };

        let reader = FilterBlockReader::new(Box::new(policy), &contents_slice);
        let boxed = Box::new(reader);
        let ptr = Box::into_raw(boxed);

        trace!(
            "create_filter_block_reader_with_drop_counter: created FilterBlockReader at {:?}",
            ptr
        );

        (ptr, drop_counter)
    }

    fn create_heap_owned_filter_buffer(len: usize) -> (*mut u8, usize) {
        assert!(
            len > 0,
            "create_heap_owned_filter_buffer requires len > 0"
        );
        trace!(
            "create_heap_owned_filter_buffer: allocating Box<[u8]> with requested_len={}",
            len
        );

        let mut owned: Box<[u8]> = vec![0u8; len].into_boxed_slice();
        let ptr = owned.as_mut_ptr();
        let actual_len = owned.len();

        trace!(
            "create_heap_owned_filter_buffer: ptr={:?}, actual_len={}",
            ptr,
            actual_len
        );

        // Leak the box; ownership is transferred to TableRep::drop via Box::from_raw.
        core::mem::forget(owned);

        (ptr, actual_len)
    }

    fn create_zeroed_block_for_index() -> *mut Block {
        trace!("create_zeroed_block_for_index: constructing zeroed Block instance");

        // SAFETY: Block is a plain old data container:
        //   * data:           *const u8
        //   * size:           usize
        //   * restart_offset: u32
        //   * owned:          bool
        //
        // Zero is a valid bit-pattern for all of these fields. The Drop impl
        // only dereferences `data` when `owned == true` and `size > 0`,
        // which will not be the case for this zeroed instance.
        let block: Block = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

        trace!(
            "create_zeroed_block_for_index: size={}, restart_offset={}, owned={}, data_ptr={:p}",
            block.size(),
            block.restart_offset(),
            block.is_owned(),
            block.data_ptr(),
        );

        let boxed = Box::new(block);
        let ptr = Box::into_raw(boxed);

        trace!(
            "create_zeroed_block_for_index: returning Block pointer {:?}",
            ptr
        );

        ptr
    }

    fn create_non_null_filter_data_with_zero_length() -> *mut u8 {
        trace!(
            "create_non_null_filter_data_with_zero_length: returning pointer to static dummy byte"
        );

        static DUMMY_FILTER_DATA: [u8; 1] = [0u8; 1];

        let ptr = DUMMY_FILTER_DATA.as_ptr() as *mut u8;

        trace!(
            "create_non_null_filter_data_with_zero_length: dummy_ptr={:?}",
            ptr
        );

        ptr
    }

    #[traced_test]
    fn drop_table_rep_with_all_null_pointers_is_safe() {
        trace!("drop_table_rep_with_all_null_pointers_is_safe: begin");

        let table_rep = construct_table_rep_for_test(
            1,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
            core::ptr::null_mut(),
        );

        drop(table_rep);

        trace!(
            "drop_table_rep_with_all_null_pointers_is_safe: completed without panic"
        );
    }

    #[traced_test]
    fn drop_table_rep_drops_filter_block_reader_exactly_once() {
        trace!("drop_table_rep_drops_filter_block_reader_exactly_once: begin");

        let (filter_ptr, drop_counter) =
            create_filter_block_reader_with_drop_counter();

        let table_rep = construct_table_rep_for_test(
            2,
            filter_ptr,
            core::ptr::null_mut(),
            0,
            core::ptr::null_mut(),
        );

        assert_eq!(
            drop_counter.load(Ordering::SeqCst),
            0,
            "FilterPolicy must not be dropped before TableRep"
        );

        drop(table_rep);

        let drops_after = drop_counter.load(Ordering::SeqCst);
        trace!(
            "drop_table_rep_drops_filter_block_reader_exactly_once: drop_counter_after_drop={}",
            drops_after
        );

        assert_eq!(
            drops_after, 1,
            "FilterPolicy must be dropped exactly once when TableRep is dropped"
        );
    }

    #[traced_test]
    fn drop_table_rep_with_heap_owned_filter_data_is_safe() {
        trace!("drop_table_rep_with_heap_owned_filter_data_is_safe: begin");

        let (filter_data_ptr, filter_data_len) =
            create_heap_owned_filter_buffer(128);

        let table_rep = construct_table_rep_for_test(
            3,
            core::ptr::null_mut(),
            filter_data_ptr,
            filter_data_len,
            core::ptr::null_mut(),
        );

        drop(table_rep);

        trace!(
            "drop_table_rep_with_heap_owned_filter_data_is_safe: completed without panic"
        );
    }

    #[traced_test]
    fn drop_table_rep_with_non_owned_index_block_is_safe() {
        trace!("drop_table_rep_with_non_owned_index_block_is_safe: begin");

        let index_block_ptr = create_zeroed_block_for_index();

        let table_rep = construct_table_rep_for_test(
            4,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
            index_block_ptr,
        );

        drop(table_rep);

        trace!(
            "drop_table_rep_with_non_owned_index_block_is_safe: completed without panic"
        );
    }

    #[traced_test]
    fn drop_table_rep_with_all_resources_set_is_safe_and_drops_filter_once() {
        trace!(
            "drop_table_rep_with_all_resources_set_is_safe_and_drops_filter_once: begin"
        );

        let (filter_ptr, drop_counter) =
            create_filter_block_reader_with_drop_counter();
        let (filter_data_ptr, filter_data_len) =
            create_heap_owned_filter_buffer(64);
        let index_block_ptr = create_zeroed_block_for_index();

        let table_rep = construct_table_rep_for_test(
            5,
            filter_ptr,
            filter_data_ptr,
            filter_data_len,
            index_block_ptr,
        );

        assert_eq!(
            drop_counter.load(Ordering::SeqCst),
            0,
            "FilterPolicy must not be dropped before TableRep"
        );

        drop(table_rep);

        let drops_after = drop_counter.load(Ordering::SeqCst);
        trace!(
            "drop_table_rep_with_all_resources_set_is_safe_and_drops_filter_once: drop_counter_after_drop={}",
            drops_after
        );

        assert_eq!(
            drops_after, 1,
            "FilterPolicy must be dropped exactly once when TableRep owns filter, filter data, and index block"
        );
    }

    #[traced_test]
    fn drop_table_rep_with_non_null_zero_length_filter_data_is_safe() {
        trace!(
            "drop_table_rep_with_non_null_zero_length_filter_data_is_safe: begin"
        );

        let filter_data_ptr = create_non_null_filter_data_with_zero_length();

        let table_rep = construct_table_rep_for_test(
            6,
            core::ptr::null_mut(),
            filter_data_ptr,
            0,
            core::ptr::null_mut(),
        );

        drop(table_rep);

        trace!(
            "drop_table_rep_with_non_null_zero_length_filter_data_is_safe: completed without panic"
        );
    }
}
