// ---------------- [ File: bitcoinleveldb-log/src/log_reader_create.rs ]
crate::ix!();

impl LogReader {

    /**
      | Create a reader that will return log records
      | from "*file".  "*file" must remain live while
      | this Reader is in use.
      |
      | If "reporter" is non-null, it is notified
      | whenever some data is dropped due to
      | a detected corruption.  "*reporter" must
      | remain live while this Reader is in use.
      |
      | If "checksum" is true, verify checksums if
      | available.
      |
      | The Reader will start reading at the first
      | record located at physical position >=
      | initial_offset within the file.
      */
    pub fn new(
        file: Rc<RefCell<dyn SequentialFile>>,
        reporter: Rc<RefCell<dyn LogReaderReporter>>,
        checksum: bool,
        initial_offset: u64,
    ) -> Self {
        info!(
            "LogReader::new: checksum={} initial_offset={}",
            checksum, initial_offset
        );

        let backing_box: Box<[u8; LOG_BLOCK_SIZE as usize]> =
            Box::new([0u8; LOG_BLOCK_SIZE as usize]);
        let backing_store = Box::into_raw(backing_box) as *const u8;

        LogReader {
            file,
            reporter,
            checksum,
            backing_store,
            buffer: Slice::default(),
            eof: false,
            last_record_offset: 0,
            end_of_buffer_offset: 0,
            initial_offset,
            resyncing: initial_offset > 0,
        }
    }
}
