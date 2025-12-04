// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/prev_next.rs ]
crate::ix!();

impl Next for KeyConvertingIterator {

    fn next(&mut self) {
        trace!(
            "KeyConvertingIterator::next: delegating to iter={:?}",
            self.iter()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "KeyConvertingIterator::next: underlying iterator pointer is null"
            );
            (*(*self.iter())).next();
        }
    }
}

impl Prev for KeyConvertingIterator {

    fn prev(&mut self) {
        trace!(
            "KeyConvertingIterator::prev: delegating to iter={:?}",
            self.iter()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "KeyConvertingIterator::prev: underlying iterator pointer is null"
            );
            (*(*self.iter())).prev();
        }
    }
}
