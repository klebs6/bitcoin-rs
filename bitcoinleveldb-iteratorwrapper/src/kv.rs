// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/kv.rs ]
crate::ix!();

impl LevelDBIteratorWrapper {
    pub fn key(&self) -> Slice {
        trace!(
            "LevelDBIteratorWrapper::key: requested; cached_valid={}, iter={:?}",
            self.valid(),
            self.iter()
        );

        assert!(
            self.valid(),
            "LevelDBIteratorWrapper::key requires the iterator to be valid"
        );

        // Recreate an equivalent Slice pointing at the same underlying bytes.
        let data   = self.key_().data();
        let size   = self.key_().size();
        let result = Slice::from_ptr_len(data, size);

        trace!(
            "LevelDBIteratorWrapper::key: returning cached key slice (data={:?}, size={})",
            data,
            size
        );

        result
    }

    pub fn value(&self) -> Slice {
        trace!(
            "LevelDBIteratorWrapper::value: requested; cached_valid={}, iter={:?}",
            self.valid(),
            self.iter()
        );

        assert!(
            self.valid(),
            "LevelDBIteratorWrapper::value requires the iterator to be valid"
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "LevelDBIteratorWrapper::value: underlying iterator pointer is null"
            );
            let value = (*self.iter()).value();
            trace!(
                "LevelDBIteratorWrapper::value: delegated to underlying iterator; value={:?}",
                value
            );
            value
        }
    }
}
