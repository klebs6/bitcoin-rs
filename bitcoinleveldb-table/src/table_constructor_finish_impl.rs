// ---------------- [ File: bitcoinleveldb-table/src/table_constructor_finish_impl.rs ]
crate::ix!();

impl TableConstructor {
   
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

        // Build table into an in-memory StringSink.
        let mut sink = StringSink {
            contents: String::new(),
        };

        let file_ptr: *mut dyn WritableFile = &mut sink;

        let mut builder = TableBuilder::new(options, file_ptr);

        for (k, v) in data.iter() {
            let key_slice   = Slice::from(k.as_bytes());
            let value_slice = Slice::from(v.as_bytes());

            trace!(
                "TableConstructor::finish_impl: adding entry key_len={}, value_len={}",
                k.len(),
                v.len()
            );

            builder.add(&key_slice, &value_slice);

            let st = builder.status();
            if !st.is_ok() {
                error!(
                    "TableConstructor::finish_impl: builder status became non-OK while adding key='{}'",
                    k
                );
                assert!(
                    st.is_ok(),
                    "TableConstructor::finish_impl: builder status not OK during Add"
                );
                return st;
            }
        }

        let mut status = builder.finish();

        trace!(
            "TableConstructor::finish_impl: builder.finish status_ok={}, file_size={}",
            status.is_ok(),
            builder.file_size()
        );

        if !status.is_ok() {
            error!(
                "TableConstructor::finish_impl: builder.finish returned non-OK status"
            );
            return status;
        }

        let sink_size = sink.contents().len() as u64;
        let file_size = builder.file_size();

        debug_assert_eq!(
            sink_size,
            file_size,
            "TableConstructor::finish_impl: sink size and builder file_size mismatch"
        );

        trace!(
            "TableConstructor::finish_impl: sink_size={} bytes matches builder.file_size",
            sink_size
        );

        // Open the table over a StringSource wrapping the built contents.
        let contents_slice =
            Slice::from(sink.contents().as_bytes());
        let source = StringSource::new(&contents_slice);
        let source_box = Box::new(source);
        let source_ptr: *mut StringSource = Box::into_raw(source_box);
        self.source = source_ptr;

        let mut table_options = options.clone();
        table_options.comparator = options.comparator;

        trace!(
            "TableConstructor::finish_impl: opening Table on in-memory StringSource @ {:?}, file_size={}",
            self.source,
            sink_size
        );

        status = crate::table::Table::open(
            &table_options,
            self.source as *mut dyn RandomAccessFile,
            sink_size,
            &mut self.table,
        );

        if !status.is_ok() {
            error!(
                "TableConstructor::finish_impl: Table::open returned non-OK status"
            );
        } else {
            trace!(
                "TableConstructor::finish_impl: Table opened successfully @ {:?}",
                self.table
            );
        }

        status
    }
}
