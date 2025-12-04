// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/value.rs ]
crate::ix!();

impl Value for KeyConvertingIterator {

    fn value(&self) -> Slice {
        trace!(
            "KeyConvertingIterator::value: delegating to iter={:?}",
            self.iter()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "KeyConvertingIterator::value: underlying iterator pointer is null"
            );
            let v = (*(*self.iter())).value();
            trace!(
                "KeyConvertingIterator::value: underlying value slice={:?}",
                v
            );
            v
        }
    }
}
