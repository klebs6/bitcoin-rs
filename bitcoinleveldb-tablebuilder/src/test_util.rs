// ---------------- [ File: bitcoinleveldb-tablebuilder/src/test_util.rs ]
crate::ix!();

#[derive(Default)]
pub(crate) struct InMemoryWritableFile {
    name:    String,
    buffer:  Vec<u8>,
    closed:  bool,
    flushed: bool,
    synced:  bool,
}

impl InMemoryWritableFile {
    pub(crate) fn new_for_test(name: &str) -> Self {
        Self {
            name:    name.to_string(),
            buffer:  Vec::new(),
            closed:  false,
            flushed: false,
            synced:  false,
        }
    }

    pub(crate) fn contents(&self) -> &[u8] {
        &self.buffer
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.closed
    }

    pub(crate) fn was_flushed(&self) -> bool {
        self.flushed
    }

    pub(crate) fn was_synced(&self) -> bool {
        self.synced
    }
}

impl Named for InMemoryWritableFile {
    fn name(&self) -> Cow<'_,str> {
        Cow::Borrowed(&self.name)
    }
}

impl WritableFileAppend for InMemoryWritableFile {
    fn append(&mut self, data: &Slice) -> Status {
        unsafe {
            let ptr = *data.data();
            let len = *data.size();
            let bytes = core::slice::from_raw_parts(ptr, len);
            self.buffer.extend_from_slice(bytes);
        }

        trace!(
            "InMemoryWritableFile::append: appended {} bytes (total={})",
            self.buffer.len(),
            self.buffer.len()
        );

        Status::ok()
    }
}

impl WritableFileClose for InMemoryWritableFile {
    fn close(&mut self) -> Status {
        trace!("InMemoryWritableFile::close: closing test writable file");
        self.closed = true;
        Status::ok()
    }
}

impl WritableFileFlush for InMemoryWritableFile {
    fn flush(&mut self) -> Status {
        trace!("InMemoryWritableFile::flush: flushing test writable file");
        self.flushed = true;
        Status::ok()
    }
}

impl WritableFileSync for InMemoryWritableFile {
    fn sync(&mut self) -> Status {
        trace!("InMemoryWritableFile::sync: syncing test writable file");
        self.synced = true;
        Status::ok()
    }
}

impl WritableFile for InMemoryWritableFile {}

pub(crate) fn create_table_builder_for_test(
    test_name: &str,
) -> (TableBuilder, &'static Options, *mut InMemoryWritableFile) {
    trace!(
        "create_table_builder_for_test: constructing Options and InMemoryWritableFile for '{}'",
        test_name
    );

    let options_ref: &'static Options = Box::leak(Box::new(Options::default()));

    let file_box = Box::new(InMemoryWritableFile::new_for_test(test_name));
    let file_raw: *mut InMemoryWritableFile = Box::into_raw(file_box);
    let file_trait: *mut dyn WritableFile = file_raw as *mut dyn WritableFile;

    let builder = TableBuilder::new(options_ref, file_trait);

    (builder, options_ref, file_raw)
}
