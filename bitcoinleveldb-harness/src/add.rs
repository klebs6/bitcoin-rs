// ---------------- [ File: bitcoinleveldb-harness/src/add.rs ]
crate::ix!();

impl Harness {

    pub fn add(&mut self, key_: &String, value: &String) {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.add.entry",
            key_len = key_.len(),
            value_len = value.len(),
        );

        const TAG_MASK: usize = BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK;

        let tagged: usize = self.constructor as usize;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        assert!(!raw.is_null());

        unsafe {
            let v: Slice = Slice::from(value.as_bytes());
            (&mut *raw).add(key_, &v);
        }

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.add.exit",
        );
    }
}
