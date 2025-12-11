// ---------------- [ File: bitcoinleveldb-tableconstructor/src/make_source_from_sink.rs ]
crate::ix!();

impl TableConstructor {

    /// From a completed `StringSink`, create a `StringSource`, keep a
    /// non‑owning raw pointer to it in `self.source`, and return the
    /// owning `Rc<RefCell<StringSource>>`.
    pub(crate) fn make_source_from_sink(
        &mut self,
        sink: &StringSink,
    ) -> std::rc::Rc<std::cell::RefCell<StringSource>> {
        use std::cell::RefCell;
        use std::rc::Rc;

        let contents_slice = Slice::from(sink.contents().as_slice());

        let source = StringSource::new(&contents_slice);

        // Own the source via Rc<RefCell<...>> so it can be shared with the
        // TableRep while we keep a non‑owning raw pointer for debugging.
        let source_rc: Rc<RefCell<StringSource>> =
            Rc::new(RefCell::new(source));

        // Grab a raw pointer into the underlying StringSource for diagnostics,
        // but DO NOT free it via this pointer (reset() only clears it).
        let raw_ptr: *mut StringSource = {
            let mut borrow = source_rc.borrow_mut();
            &mut *borrow as *mut StringSource
        };

        self.set_source(raw_ptr);

        trace!(
            "TableConstructor::make_source_from_sink: created StringSource @ {:?}, len={}",
            raw_ptr,
            sink.contents().len()
        );

        source_rc
    }
}
