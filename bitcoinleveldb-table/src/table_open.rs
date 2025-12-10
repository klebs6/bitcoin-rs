// ---------------- [ File: bitcoinleveldb-table/src/table_open.rs ]
crate::ix!();

impl Table {

    pub fn open(
        options: &Options,
        file:    Rc<RefCell<dyn RandomAccessFile>>,
        size:    u64,
    ) -> Result<Box<Table>, Status> {
        trace!(
            "Table::open: size={}, footer_len={}",
            size,
            FOOTER_ENCODED_LENGTH
        );

        if size < FOOTER_ENCODED_LENGTH as u64 {
            let msg       = b"file is too short to be an sstable";
            let msg_slice = Slice::from(&msg[..]);
            error!(
                "Table::open: file too short to be an sstable (size={}, required_min={})",
                size,
                FOOTER_ENCODED_LENGTH
            );
            return Err(Status::corruption(&msg_slice, None));
        }

        let mut footer_buf   = vec![0u8; FOOTER_ENCODED_LENGTH];
        let mut footer_input = Slice::default();

        let read_offset = size - FOOTER_ENCODED_LENGTH as u64;

        let s_read_footer = {
            use bitcoinleveldb_file::RandomAccessFileRead;

            let file_ref = file.borrow();
            trace!(
                "Table::open: reading footer from file='{}' at offset={} size={}",
                file_ref.name(),
                read_offset,
                FOOTER_ENCODED_LENGTH
            );

            RandomAccessFileRead::read(
                &*file_ref,
                read_offset,
                FOOTER_ENCODED_LENGTH,
                &mut footer_input,
                footer_buf.as_mut_ptr(),
            )
        };

        if !s_read_footer.is_ok() {
            error!(
                "Table::open: RandomAccessFile::read for footer returned non-OK status"
            );
            return Err(s_read_footer);
        }

        let mut footer           = Footer::default();
        let mut footer_input_mut = footer_input;
        let s_footer             = footer.decode_from(&mut footer_input_mut as *mut Slice);

        if !s_footer.is_ok() {
            error!(
                "Table::open: Footer::decode_from returned non-OK status"
            );
            return Err(s_footer);
        }

        let mut index_block_contents = BlockContents::default();
        let mut status               = Status::ok();

        if status.is_ok() {
            let mut opt = ReadOptions::default();
            if *options.paranoid_checks() {
                *opt.verify_checksums_mut() = true;
            }

            trace!(
                "Table::open: reading index block at offset={}, size={}",
                footer.index_handle().offset(),
                footer.index_handle().size()
            );

            status = read_block(
                file.clone(),
                &opt,
                footer.index_handle(),
                &mut index_block_contents as *mut BlockContents,
            );
        }

        if !status.is_ok() {
            error!(
                "Table::open: failed while reading index block; status not OK"
            );
            return Err(status);
        }

        trace!("Table::open: constructing index Block and TableRep");

        let index_block = Box::new(Block::new(&index_block_contents));

        let cache_id = unsafe {
            let cache_ptr_ref = options.block_cache();
            let cache_ptr: *mut Cache = *cache_ptr_ref;
            if cache_ptr.is_null() {
                0
            } else {
                let cache_ref: &mut Cache = &mut *cache_ptr;
                cache_ref.new_id()
            }
        };

        let mut rep_options = options.clone();
        rep_options.set_comparator(Arc::new(BytewiseComparatorImpl::default()));
        rep_options.set_filter_policy(Arc::new(NullFilterPolicy::default()));

        let rep = TableRep::new(
            rep_options.clone(),
            file.clone(),
            cache_id,
            *footer.metaindex_handle(),
            Box::into_raw(index_block),
        );

        let rep_box: Box<TableRep> = Box::new(rep);
        let rep_ptr: *mut TableRep  = Box::into_raw(rep_box);

        // Preserve the original heap/ptr semantics: allocate Table on the heap,
        // convert to a raw pointer for read_meta, then rewrap into Box for return.
        let table_box: Box<Table> = Box::new(Table::new(rep_ptr));
        let table_ptr: *mut Table = Box::into_raw(table_box);

        unsafe {
            trace!(
                "Table::open: invoking read_meta on newly created Table (rep={:?})",
                rep_ptr
            );
            (*table_ptr).read_meta(&footer);
            Ok(Box::from_raw(table_ptr))
        }
    }
}

#[cfg(test)]
mod table_open_file_size_behavior {
    use super::*;
    use bitcoin_imports::Named;
    use bitcoinleveldb_file::RandomAccessFileRead;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct DummyRandomAccessFile {
        name:        &'static str,
        read_status: Status,
    }

    impl DummyRandomAccessFile {
        fn new_ok() -> Self {
            DummyRandomAccessFile {
                name:        "DummyRandomAccessFile(ok)",
                read_status: Status::ok(),
            }
        }

        fn new_error(status: Status) -> Self {
            DummyRandomAccessFile {
                name:        "DummyRandomAccessFile(error)",
                read_status: status,
            }
        }
    }

    impl Named for DummyRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed(self.name)
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
                "DummyRandomAccessFile::read(open): offset={}, n={}, scratch={:?}",
                offset,
                n,
                scratch
            );
            unsafe {
                *result = Slice::default();
            }
            self.read_status.clone()
        }
    }

    impl RandomAccessFile for DummyRandomAccessFile {}

    #[traced_test]
    fn open_returns_corruption_for_too_small_file() {
        let mut out_table: *mut Table = core::ptr::null_mut();

        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(DummyRandomAccessFile::new_ok()));
        let opts = Options::default();

        let result =  Table::open(
            &opts, 
            file.clone(), 
            (FOOTER_ENCODED_LENGTH as u64) - 1
        );

        trace!(
            "open_returns_corruption_for_too_small_file: result={:?}",
            result,
        );

        match result {
            Ok(tbl) => {
                // On success, tbl is a Box<Table>, and by design cannot be null.
                // This branch should never happen for this test.
                panic!("expected Err(Status), but got Ok({:?})", tbl);
            }
            Err(status) => {
                assert!(!status.is_ok());
            }
        }
    }

    #[traced_test]
    fn open_propagates_footer_read_error() {
        let msg = b"forced read error";
        let msg_slice = Slice::from(&msg[..]);
        let forced_status = Status::corruption(&msg_slice, None);

        let mut out_table: *mut Table = core::ptr::null_mut();

        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(DummyRandomAccessFile::new_error(
                forced_status.clone(),
            )));
        let opts = Options::default();

        let result = Table::open(
            &opts,
            file.clone(),
            FOOTER_ENCODED_LENGTH as u64,
        );

        match result {
            Ok(_) => panic!("expected footer read error, got Ok"),
            Err(status) => {
                assert!(!status.is_ok());
                assert!(out_table.is_null());
            }
        }
    }

    #[traced_test]
    fn open_returns_err_for_too_small_file() {
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(DummyRandomAccessFile::new_ok()));
        let opts = Options::default();

        let result = Table::open(
            &opts,
            file.clone(),
            (FOOTER_ENCODED_LENGTH as u64) - 1,
        );

        trace!("open_returns_err_for_too_small_file: result={:?}", result);

        // The correct behavior: return Err(Status::corruption), NOT panic.
        assert!(result.is_err(), "too-small file size must return Err(Status)");

        // And we also check the returned status is corruption.
        let status = result.err().unwrap();
        assert!(status.is_corruption(), "expected corruption status");
    }
}
