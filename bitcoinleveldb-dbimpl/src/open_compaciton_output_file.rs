// ---------------- [ File: bitcoinleveldb-dbimpl/src/open_compaciton_output_file.rs ]
crate::ix!();

impl DBImpl {
    pub fn open_compaction_output_file(&mut self, compact: *mut CompactionState) -> crate::Status {
        assert!(!compact.is_null());
        assert!(unsafe { (*compact).builder() }.is_null());

        let file_number: u64;

        self.mutex.lock();
        file_number = unsafe { (*self.versions).new_file_number() };
        self.pending_outputs_mut().insert(file_number);

        unsafe {
            let out = CompactionStateOutput {
                number: file_number,
                smallest: InternalKey::new_empty(),
                largest: InternalKey::new_empty(),
                file_size: 0,
            };
            (*compact).outputs_mut().push(out);
        }

        self.mutex.unlock();

        // Make the output file
        let fname: String = table_file_name(&self.dbname, file_number);

        let mut s: Status = self
            .env
            .borrow_mut()
            .new_writable_file(&fname, unsafe { &mut (*compact).outfile() });

        if s.is_ok() {
            unsafe {
                (*compact).set_builder(
                    Box::into_raw(Box::new(TableBuilder::new(&self.options, (*compact).outfile())))
                );
            }
        }

        s
    }
}
