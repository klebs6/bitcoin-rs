// ---------------- [ File: bitcoinleveldb-tableconstructor/src/table_constructor_finish_impl.rs ]
crate::ix!();

impl TableConstructor {

    /// High‑level "finish" entry point:
    ///
    /// 1. Reset any previous table/source.
    /// 2. Build an sstable image into an in‑memory `StringSink`.
    /// 3. Wrap the sink contents in a `StringSource`.
    /// 4. Open a `Table` over that in‑memory file.
    pub fn finish_impl(&mut self, options: &Options, data: &KVMap) -> crate::Status {
        trace!(
            "TableConstructor::finish_impl: entries={}",
            data.len()
        );

        // Dispose of any previous table/source.
        self.reset();

        // Step 1: build the table image into an in‑memory sink.
        let (mut status, sink, file_size_bytes): (crate::Status, StringSink, u64) =
            build_table_into_sink(options, data);

        if !status.is_ok() {
            return status;
        }

        // Step 2: create a StringSource over the sink contents, retaining both the
        // backing bytes and the source owner for the full constructor lifetime.
        let source_rc: Rc<RefCell<StringSource>> = self.make_source_from_sink(&sink);

        // Step 3: open a Table on top of that in‑memory file.
        status = self.open_table_from_source(options, source_rc, file_size_bytes);

        if !status.is_ok() {
            error!(
                "TableConstructor::finish_impl: open_table_from_source returned non-OK status"
            );
            self.reset();
            return status;
        }

        assert!(
            !self.table().is_null(),
            "TableConstructor::finish_impl: table pointer remained null after successful open"
        );

        trace!(
            "TableConstructor::finish_impl: opened table @ {:?}, source @ {:?}, source_owner_present={}, source_bytes_len={}, file_size={}",
            self.table(),
            self.source(),
            self.source_owner().is_some(),
            self.source_bytes().len(),
            file_size_bytes
        );

        status
    }
}
