// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/prev_next.rs ]
crate::ix!();

impl LevelDBIteratorWrapper {
    pub fn prev(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::prev: moving backwards; iter={:?}, before_valid={}",
            self.iter(),
            self.valid()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "LevelDBIteratorWrapper::prev: underlying iterator pointer is null"
            );
            (*self.iter()).prev();
        }

        self.update();
    }

    pub fn next(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::next: advancing; iter={:?}, before_valid={}",
            self.iter(),
            self.valid()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "LevelDBIteratorWrapper::next: underlying iterator pointer is null"
            );
            (*self.iter()).next();
        }

        self.update();
    }
}
