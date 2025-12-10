// ---------------- [ File: bitcoinleveldb-tableconstructor/src/open_table_from_source.rs ]
crate::ix!();

impl TableConstructor {

    /// Open a `Table` over the given in‑memory file.
    ///
    /// * `source_rc` is an owning handle to the in‑memory file.
    /// * `file_size` is the logical size of the file in bytes.
    pub(crate) fn open_table_from_source(
        &mut self,
        options: &Options,
        source_rc: std::rc::Rc<std::cell::RefCell<StringSource>>,
        file_size: u64,
    ) -> crate::Status {
        use std::cell::RefCell;
        use std::rc::Rc;

        let mut table_options = options.clone();
        table_options.set_comparator(options.comparator().clone());

        // Upcast Rc<RefCell<StringSource>> to Rc<RefCell<dyn RandomAccessFile>>
        // so it can be consumed by Table::open and stored inside TableRep.
        let file_rc: Rc<RefCell<dyn RandomAccessFile>> = source_rc;

        trace!(
            "TableConstructor::open_table_from_source: opening Table on in-memory StringSource @ {:?}, file_size={}",
            self.source(),
            file_size
        );

        // We only need a receiver to call the method; `open` allocates a fresh
        // Table into `self.table`.
        let mut tmp = Table::new(core::ptr::null_mut());

        let result = Table::open(
            options,
            file_rc.clone(),
            file_size,
        );

        let status = match result {
            Ok(table_box) => {
                // Preserve original semantics: store a raw pointer into self.table
                let raw: *mut Table = Box::into_raw(table_box);
                *self.table_mut() = raw;

                Status::ok()
            }
            Err(status) => status,
        };

        status
    }
}
