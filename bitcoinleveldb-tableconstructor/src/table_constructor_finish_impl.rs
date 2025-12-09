// ---------------- [ File: bitcoinleveldb-tableconstructor/src/table_constructor_finish_impl.rs ]
crate::ix!();

impl TableConstructor {

    /// High‑level "finish" entry point:
    ///
    /// 1. Reset any previous table/source.
    /// 2. Build an sstable image into an in‑memory `StringSink`.
    /// 3. Wrap the sink contents in a `StringSource`.
    /// 4. Open a `Table` over that in‑memory file.
    pub fn finish_impl(
        &mut self,
        options: &Options,
        data:    &KVMap,
    ) -> crate::Status {
        trace!(
            "TableConstructor::finish_impl: entries={}",
            data.len()
        );

        // Dispose of any previous table/source.
        self.reset();

        // Step 1: build the table image into an in‑memory sink.
        let (mut status, sink, sink_size) =
            Self::build_table_into_sink(options, data);

        if !status.is_ok() {
            return status;
        }

        // Step 2: create a StringSource over the sink contents and
        // stash a non‑owning raw pointer for debugging.
        let source_rc = self.make_source_from_sink(&sink);

        // Step 3: open a Table on top of that in‑memory file.
        status = self.open_table_from_source(options, source_rc, sink_size);

        status
    }
}
