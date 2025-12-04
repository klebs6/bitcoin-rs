// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/status.rs ]
crate::ix!();

impl LevelDBIteratorStatus for KeyConvertingIterator {

    fn status(&self) -> crate::Status {
        trace!(
            "KeyConvertingIterator::status: evaluating cached vs underlying status; iter={:?}",
            self.iter()
        );

        let cached = self.status().borrow();

        if cached.is_ok() {
            unsafe {
                assert!(
                    !self.iter().is_null(),
                    "KeyConvertingIterator::status: underlying iterator pointer is null"
                );
                let st = (*(*self.iter())).status();
                trace!(
                    "KeyConvertingIterator::status: cached OK; returning underlying status_code={:?}",
                    st.code()
                );
                st
            }
        } else {
            trace!(
                "KeyConvertingIterator::status: cached non‑OK status_code={:?}; returning cached status",
                cached.code()
            );
            Status::new_from_other_copy(&*cached)
        }
    }
}
