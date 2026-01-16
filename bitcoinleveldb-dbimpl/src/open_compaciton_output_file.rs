// ---------------- [ File: bitcoinleveldb-dbimpl/src/open_compaciton_output_file.rs ]
crate::ix!();

impl DBImpl {
    pub fn open_compaction_output_file(&mut self, compact: *mut CompactionState) -> crate::Status { 
        assert!(!compact.is_null());
        assert!(unsafe { (*compact).builder() }.is_null());

        let file_number: u64;

        self.mutex.lock();
        file_number = unsafe { (*self.versions).new_file_number() };
        self.pending_outputs.insert(file_number);

        unsafe {

            let out = CompactionStateOutputBuilder::default()
                .number(file_number)
                .smallest(InternalKey::new_empty())
                .largest(InternalKey::new_empty())
                .file_size(0)
                .build()
                .unwrap();

            (*compact).outputs_mut().push(out);
        }

        unsafe { self.mutex.unlock() };

        // Make the output file
        let fname: String = table_file_name(&self.dbname, file_number);

        let mut outfile_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

        let mut s: Status = self
            .env
            .as_mut()
            .new_writable_file(&fname, &mut outfile_ptr);

        if s.is_ok() {
            unsafe {
                let outfile_box: Box<dyn WritableFile> = *Box::from_raw(outfile_ptr);
                let outfile_rc: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(outfile_box));
                (*compact).set_outfile(outfile_rc);

                let file_ptr: *mut dyn WritableFile = {
                    let mut borrow = (*compact).outfile().borrow_mut();
                    &mut *borrow as &mut dyn WritableFile as *mut dyn WritableFile
                };

                (*compact).set_builder(
                    Box::into_raw(Box::new(TableBuilder::new(&self.options, file_ptr)))
                );
            }
        }

        s
    }
}
