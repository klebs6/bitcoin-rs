// ---------------- [ File: bitcoinleveldb-harness/src/test.rs ]
crate::ix!();

impl Harness {

    pub fn test(&mut self, rnd: *mut Random) {
        trace!(target: "bitcoinleveldb_harness", label = "bitcoinleveldb_harness.harness.test.entry");

        const TAG_MASK: usize = BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        assert!(!raw.is_null());

        let mut keys: Vec<String> = Vec::new();
        let mut data: KVMap = KVMap::default();

        unsafe {
            (&mut *raw).finish(&self.options, &mut keys as *mut Vec<String>, &mut data as *mut KVMap);

            match tag {
                0 => {
                    let s: Status = (&mut *(raw as *mut TableConstructor)).finish_impl(&self.options, &data);
                    assert!(s.is_ok());
                }
                1 => {
                    let s: Status = (&mut *(raw as *mut BlockConstructor)).finish_impl(&self.options, &data);
                    assert!(s.is_ok());
                }
                2 => {
                    let s: Status = (&mut *(raw as *mut MemTableConstructor)).finish_impl(&self.options, &data);
                    assert!(s.is_ok());
                }
                3 => {
                    let s: Status = (&mut *(raw as *mut DBConstructor)).finish_impl(&self.options, &data);
                    assert!(s.is_ok());
                }
                _ => {
                    panic!();
                }
            }
        }

        self.test_forward_scan(&keys, &data);
        self.test_backward_scan(&keys, &data);
        self.test_random_access(rnd, &keys, &data);

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.test.exit",
            num_keys = keys.len(),
        );
    }
}
