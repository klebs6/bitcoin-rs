// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/seek_to.rs ]
crate::ix!();

impl SeekToFirst for KeyConvertingIterator {

    fn seek_to_first(&mut self) {
        trace!(
            "KeyConvertingIterator::seek_to_first: delegating to iter={:?}",
            self.iter()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "KeyConvertingIterator::seek_to_first: underlying iterator pointer is null"
            );
            (*(*self.iter())).seek_to_first();
        }
    }
}

impl SeekToLast for KeyConvertingIterator {

    fn seek_to_last(&mut self) {
        trace!(
            "KeyConvertingIterator::seek_to_last: delegating to iter={:?}",
            self.iter()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "KeyConvertingIterator::seek_to_last: underlying iterator pointer is null"
            );
            (*(*self.iter())).seek_to_last();
        }
    }
}
