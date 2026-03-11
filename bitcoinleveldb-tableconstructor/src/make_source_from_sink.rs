// ---------------- [ File: bitcoinleveldb-tableconstructor/src/make_source_from_sink.rs ]
crate::ix!();

impl TableConstructor {

    /// From a completed `StringSink`, create a `StringSource`, keep a non‑owning raw pointer to it
    /// in `self.source`, and return the owning `Rc<RefCell<StringSource>>`.
    ///
    pub(crate) fn make_source_from_sink(&mut self, sink: &StringSink) -> Rc<RefCell<StringSource>> {
        // Preserve an owned backing copy inside the constructor so any RandomAccessFile
        // implementation that borrows underlying bytes remains valid for the full table lifetime.
        self.source_bytes_mut().clear();
        self.source_bytes_mut().extend_from_slice(sink.contents().as_slice());

        let contents_slice: Slice = Slice::from(self.source_bytes().as_slice());

        let source: StringSource = StringSource::new(&contents_slice);

        // Own the source via Rc<RefCell<...>> so it can be shared with the
        // TableRep while we keep a non‑owning raw pointer for diagnostics.
        let source_rc: Rc<RefCell<StringSource>> =
            Rc::new(RefCell::new(source));

        // Grab a raw pointer into the underlying StringSource for diagnostics,
        // but DO NOT free it via this pointer (reset() only clears it).
        let raw_ptr: *mut StringSource = {
            let mut borrow = source_rc.borrow_mut();
            &mut *borrow as *mut StringSource
        };

        self.set_source(raw_ptr);
        self.set_source_owner(Some(source_rc.clone()));

        trace!(
            "TableConstructor::make_source_from_sink: created StringSource @ {:?}, len={}",
            raw_ptr,
            self.source_bytes().len()
        );

        source_rc
    }
}
