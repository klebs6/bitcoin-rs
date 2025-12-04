// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/update.rs ]
crate::ix!();

impl LevelDBIteratorWrapper {
    pub fn update(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::update: refreshing cached valid/key; iter={:?}",
            self.iter()
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "LevelDBIteratorWrapper::update: underlying iterator pointer is null"
            );

            self.set_valid((*self.iter()).valid());
            trace!(
                "LevelDBIteratorWrapper::update: underlying valid={}",
                self.valid()
            );

            if self.valid() {
                let k    = (*self.iter()).key();
                let data = k.data();
                let size = k.size();

                self.set_key(Slice::from_ptr_len(data, size));

                trace!(
                    "LevelDBIteratorWrapper::update: cached key from underlying iterator (data={:?}, size={})",
                    data,
                    size
                );
            }
        }
    }
}
