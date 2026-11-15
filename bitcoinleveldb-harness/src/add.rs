// ---------------- [ File: bitcoinleveldb-harness/src/add.rs ]
crate::ix!();

impl Harness {

    pub fn add(&mut self, key_: &String, value: &String) {
        self.add_bytes(key_.as_bytes(), value.as_bytes());
    }

    pub fn add_bytes(&mut self, key: &[u8], value: &[u8]) {
        const TAG_MASK: usize = BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK;

        let tagged: usize = self.constructor as usize;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        assert!(!raw.is_null());

        unsafe {
            (&mut *raw).add_bytes(key, value);
        }

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.add.exit",
        );
    }
}
