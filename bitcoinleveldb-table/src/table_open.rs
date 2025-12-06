// ---------------- [ File: bitcoinleveldb-table/src/table_open.rs ]
crate::ix!();

impl Table {

    /// Attempt to open the table that is stored in bytes [0..file_size) of
    /// "file", and read the metadata entries necessary to allow retrieving data
    /// from the table.
    /// 
    /// If successful, returns ok and sets "*table" to the newly opened table.
    /// The client should delete "*table" when no longer needed.  If there was
    /// an error while initializing the table, sets "*table" to nullptr and
    /// returns a non-ok status.  Does not take ownership of "*source", but the
    /// client must ensure that "source" remains live for the duration of the
    /// returned table's lifetime.
    /// 
    /// *file must remain live while this Table is in use.
    ///
    pub fn open(
        &mut self,
        options: &Options,
        file:    Rc<RefCell<dyn RandomAccessFile>>,
        size:    u64,
        table:   *mut *mut Table,
    ) -> Status {
        unsafe {
            assert!(
                !table.is_null(),
                "Table::open: table out-parameter pointer is null"
            );
            *table = core::ptr::null_mut();
        }

        trace!(
            "Table::open: size={}, footer_len={}",
            size,
            FOOTER_ENCODED_LENGTH
        );

        if size < FOOTER_ENCODED_LENGTH as u64 {
            let msg = b"file is too short to be an sstable";
            let msg_slice = Slice::from(&msg[..]);
            error!(
                "Table::open: file too short to be an sstable (size={}, required_min={})",
                size,
                FOOTER_ENCODED_LENGTH
            );
            return Status::corruption(&msg_slice, None);
        }

        let mut footer_buf = vec![0u8; FOOTER_ENCODED_LENGTH];
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
            return s_read_footer;
        }

        let mut footer = Footer::default();
        let mut footer_input_mut = footer_input;
        let s_footer = footer.decode_from(&mut footer_input_mut as *mut Slice);

        if !s_footer.is_ok() {
            error!(
                "Table::open: Footer::decode_from returned non-OK status"
            );
            return s_footer;
        }

        // Read the index block
        let mut index_block_contents = BlockContents {
            data:           Slice::default(),
            cachable:       false,
            heap_allocated: false,
        };

        let mut status = Status::ok();

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

        if status.is_ok() {
            trace!("Table::open: constructing index Block and TableRep");

            // We've successfully read the footer and the index block: we're
            // ready to serve requests.
            let index_block = Box::new(Block::new(&index_block_contents));

            let cache_id = unsafe {
                let cache_ptr = options.block_cache();
                if cache_ptr.is_null() {
                    0
                } else {
                    let cache_ref = &mut *cache_ptr;
                    cache_ref.new_id()
                }
            };

            let rep = TableRep {
                options:          Options {
                    comparator:             Box::new(BytewiseComparatorImpl::default()),
                    create_if_missing:      *options.create_if_missing(),
                    error_if_exists:        *options.error_if_exists(),
                    paranoid_checks:        *options.paranoid_checks(),
                    env:                    options.env().clone(),
                    info_log:               options.info_log().clone(),
                    write_buffer_size:      *options.write_buffer_size(),
                    max_open_files:         *options.max_open_files(),
                    block_cache:            options.block_cache(),
                    block_size:             *options.block_size(),
                    block_restart_interval: *options.block_restart_interval(),
                    max_file_size:          *options.max_file_size(),
                    compression:            *options.compression(),
                    reuse_logs:             *options.reuse_logs(),
                    filter_policy:          Box::new(NullFilterPolicy::default()),
                },
                status:           Status::ok(),
                file:             file.clone(),
                cache_id,
                filter:           core::ptr::null_mut(),
                filter_data:      core::ptr::null_mut(),
                filter_data_len:  0,
                metaindex_handle: *footer.metaindex_handle(),
                index_block:      Box::into_raw(index_block),
            };

            let rep_box = Box::new(rep);
            let rep_ptr: *mut TableRep = Box::into_raw(rep_box);

            let table_box = Box::new(Table {
                rep: rep_ptr as *const TableRep,
            });
            let table_ptr: *mut Table = Box::into_raw(table_box);

            unsafe {
                *table = table_ptr;
            }

            unsafe {
                if !(*table).is_null() {
                    trace!(
                        "Table::open: invoking read_meta on newly created Table (rep={:?})",
                        rep_ptr
                    );
                    (*(*table)).read_meta(&footer);
                }
            }
        } else {
            error!(
                "Table::open: failed while reading index block; status not OK"
            );
        }

        status
    }
}
