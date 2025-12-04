// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/seek.rs ]
crate::ix!();

impl LevelDBIteratorWrapper {
    pub fn seek(&mut self, k: &Slice) {
        trace!(
            "LevelDBIteratorWrapper::seek: seeking to target={:?}, iter={:?}",
            k,
            self.iter()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "LevelDBIteratorWrapper::seek: underlying iterator pointer is null"
            );
            (*self.iter()).seek(k);
        }

        self.update();
    }

    pub fn seek_to_first(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::seek_to_first: iter={:?}",
            self.iter()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "LevelDBIteratorWrapper::seek_to_first: underlying iterator pointer is null"
            );
            (*self.iter()).seek_to_first();
        }

        self.update();
    }

    pub fn seek_to_last(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::seek_to_last: iter={:?}",
            self.iter()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "LevelDBIteratorWrapper::seek_to_last: underlying iterator pointer is null"
            );
            (*self.iter()).seek_to_last();
        }

        self.update();
    }
}
