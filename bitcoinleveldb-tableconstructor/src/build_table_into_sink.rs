// ---------------- [ File: bitcoinleveldb-tableconstructor/src/build_table_into_sink.rs ]
crate::ix!();

/// Build an sstable into an in‑memory `StringSink` using a `TableBuilder`.
///
/// Returns `(status, sink, file_size_bytes)`.
pub(crate) fn build_table_into_sink(
    options: &Options,
    data:    &KVMap,
) -> (crate::Status, StringSink, u64) {
    // Build table into an in‑memory StringSink.
    let mut sink = StringSink::new();

    let file_ptr: *mut dyn WritableFile = &mut sink;

    let mut builder = TableBuilder::new(options, file_ptr);

    // Ensure keys are presented to TableBuilder in strictly increasing order.
    // This is required by the underlying LevelDB table builder.
    let mut entries: Vec<(&String, &String)> = data.iter().collect();
    entries.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

    for (k, v) in entries.into_iter() {
        let key_slice   = Slice::from(k.as_bytes());
        let value_slice = Slice::from(v.as_bytes());

        trace!(
            "build_table_into_sink: adding entry key_len={}, value_len={}",
            k.len(),
            v.len()
        );

        builder.add(&key_slice, &value_slice);

        let st = builder.status();
        if !st.is_ok() {
            error!(
                "build_table_into_sink: builder status became non-OK while adding key='{}'",
                k
            );

            // Ensure the underlying TableBuilderRep is marked closed so Drop
            // does not panic about an unterminated builder.
            builder.abandon();

            return (st, sink, 0);
        }
    }

    let mut status = builder.finish();

    trace!(
        "build_table_into_sink: builder.finish status_ok={}, file_size={}",
        status.is_ok(),
        builder.file_size()
    );

    if !status.is_ok() {
        error!(
            "build_table_into_sink: builder.finish returned non-OK status"
        );
        return (status, sink, 0);
    }

    // Logical table size as reported by the TableBuilder.
    let file_size_bytes = builder.file_size();

    // Physical backing buffer size.
    let sink_size = sink.contents().len() as u64;

    // In practice, StringSink may contain pre‑existing data or additional
    // bookkeeping bytes. The TableBuilder's file_size tracks the logical
    // table image we just wrote, which must not exceed the underlying sink
    // length.
    debug_assert!(
        sink_size >= file_size_bytes,
        "build_table_into_sink: builder.file_size ({}) larger than sink length ({})",
        file_size_bytes,
        sink_size
    );

    trace!(
        "build_table_into_sink: sink_size={} bytes, builder.file_size={}",
        sink_size,
        file_size_bytes
    );

    // For callers, return the *logical* SSTable size as seen by TableBuilder.
    // The StringSink may contain extra bytes (e.g., instrumentation) beyond this.
    (status, sink, file_size_bytes)
}

#[cfg(test)]
mod table_constructor_finish_impl_behavior {
    use super::*;
    use std::sync::Arc;

    fn default_options() -> Options {
        let mut opts = Options::default();
        opts.set_comparator(Arc::new(BytewiseComparatorImpl::default()));
        opts
    }

    fn small_kv_map() -> KVMap {
        let mut map = KVMap::default();
        map.insert("a".to_string(), "one".to_string());
        map.insert("b".to_string(), "two".to_string());
        map
    }

    #[traced_test]
    fn build_table_into_sink_with_empty_map_produces_non_zero_sstable() {
        let opts = default_options();
        let data = KVMap::default();

        let (status, sink, file_size) =
            build_table_into_sink(&opts, &data);

        let sink_len = sink.contents().len() as u64;

        assert!(status.is_ok());
        assert!(file_size > 0, "file_size must be non‑zero");
        assert!(
            sink_len >= file_size,
            "sink length ({}) must be at least the reported file_size ({})",
            sink_len,
            file_size
        );
    }

    #[traced_test]
    fn make_source_from_sink_records_non_null_pointer_and_is_readable() {
        use bitcoinleveldb_file::RandomAccessFileRead;

        let mut ctor = TableConstructor::new(
            Box::new(BytewiseComparatorImpl::default()),
        );

        let sink = StringSink::with("abc123");

        let source_rc = ctor.make_source_from_sink(&sink);

        assert!(
            !ctor.source().is_null(),
            "source pointer must be non-null after make_source_from_sink"
        );

        // Smoke‑test that the created StringSource can be read via the
        // RandomAccessFileRead trait.
        let mut result = Slice::default();
        let mut scratch = vec![0u8; sink.contents().len()];

        let status = {
            let file_ref = source_rc.borrow();
            RandomAccessFileRead::read(
                &*file_ref,
                0,
                sink.contents().len(),
                &mut result as *mut Slice,
                scratch.as_mut_ptr(),
            )
        };

        assert!(status.is_ok());
        assert_eq!(
            *result.size(),
            sink.contents().len(),
            "read length must match sink length"
        );
    }

    #[traced_test]
    fn open_table_from_source_creates_table_for_simple_map() {
        let mut ctor = TableConstructor::new(
            Box::new(BytewiseComparatorImpl::default()),
        );
        let opts = default_options();
        let data = small_kv_map();

        let (status_build, sink, file_size) =
            build_table_into_sink(&opts, &data);
        assert!(status_build.is_ok());

        let source_rc = ctor.make_source_from_sink(&sink);

        let status_open =
            ctor.open_table_from_source(&opts, source_rc, file_size);

        assert!(status_open.is_ok());
        assert!(
            !ctor.table().is_null(),
            "table pointer must be non-null after successful open"
        );

        // Smoke‑check: new_iterator should not panic with a valid table.
        let _iter = ctor.new_iterator();

        ctor.reset();
        assert!(ctor.table().is_null());
    }

    #[traced_test]
    fn open_table_from_source_reports_error_for_too_small_file_size() {
        let mut ctor = TableConstructor::new(
            Box::new(BytewiseComparatorImpl::default()),
        );
        let opts = default_options();
        let data = small_kv_map();

        let (status_build, sink, _real_size) =
            build_table_into_sink(&opts, &data);
        assert!(status_build.is_ok());

        let source_rc = ctor.make_source_from_sink(&sink);

        // Force a size smaller than the encoded footer length so
        // Table::open will reject it.
        let truncated_size = (FOOTER_ENCODED_LENGTH as u64) - 1;

        let status_open =
            ctor.open_table_from_source(&opts, source_rc, truncated_size);

        assert!(
            !status_open.is_ok(),
            "expected non-OK status for too-small file size"
        );
        assert!(
            ctor.table().is_null(),
            "table pointer must remain null on open error"
        );
    }

    #[traced_test]
    fn finish_impl_builds_and_opens_table_for_simple_map() {
        let mut ctor = TableConstructor::new(
            Box::new(BytewiseComparatorImpl::default()),
        );
        let opts = default_options();
        let data = small_kv_map();

        let status = ctor.finish_impl(&opts, &data);

        assert!(status.is_ok());
        assert!(
            !ctor.table().is_null(),
            "finish_impl must leave a non-null table pointer on success"
        );

        // Calling finish_impl again should safely replace the table.
        let status2 = ctor.finish_impl(&opts, &data);
        assert!(status2.is_ok());
        assert!(!ctor.table().is_null());

        ctor.reset();
        assert!(ctor.table().is_null());
    }
}
