// ---------------- [ File: bitcoinleveldb-harness/src/test_backward_scan.rs ]
crate::ix!();

impl Harness {

    pub fn test_backward_scan(&mut self, keys: &[String], data: &KVMap) {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.test_backward_scan.entry",
            num_keys = keys.len(),
        );

        const TAG_MASK: usize = BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        assert!(!raw.is_null());

        let iter: *mut LevelDBIterator = unsafe {
            match tag {
                0 => (&*(raw as *mut TableConstructor)).new_iterator(),
                1 => (&*(raw as *mut BlockConstructor)).new_iterator(),
                2 => (&*(raw as *mut MemTableConstructor)).new_iterator(),
                3 => (&*(raw as *mut DBConstructor)).new_iterator(),
                _ => {
                    panic!();
                }
            }
        };

        unsafe {
            assert!(!(&*iter).valid());
            (&mut *iter).seek_to_last();

            for k in keys.iter().rev() {
                let v: &String = match data.get(k) {
                    Some(v) => v,
                    None => {
                        error!(
                            target: "bitcoinleveldb_harness",
                            label = "bitcoinleveldb_harness.harness.test_backward_scan.missing_model_value",
                            key_len = k.len(),
                        );
                        panic!();
                    }
                };

                assert_eq!(
                    self.to_string_rev(data, Some((k, v))),
                    self.to_string(iter as *const LevelDBIterator)
                );
                (&mut *iter).prev();
            }

            assert!(!(&*iter).valid());
            drop(Box::from_raw(iter));
        }

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.test_backward_scan.exit",
        );
    }
}
