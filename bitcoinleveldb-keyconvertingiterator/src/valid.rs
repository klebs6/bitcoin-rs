// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/valid.rs ]
crate::ix!();

impl Valid for KeyConvertingIterator {

    fn valid(&self) -> bool {
        trace!(
            "KeyConvertingIterator::valid: delegating to underlying iter={:?}",
            self.iter()
        );

        unsafe {
            if self.iter().is_null() {
                trace!(
                    "KeyConvertingIterator::valid: underlying iterator pointer is null -> false"
                );
                false
            } else {
                let v = (*(*self.iter())).valid();
                trace!(
                    "KeyConvertingIterator::valid: underlying valid={}",
                    v
                );
                v
            }
        }
    }
}
