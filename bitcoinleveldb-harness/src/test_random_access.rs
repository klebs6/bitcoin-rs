// ---------------- [ File: bitcoinleveldb-harness/src/test_random_access.rs ]
crate::ix!();

impl Harness {

    pub fn test_random_access(&mut self, rnd: *mut Random, keys: &[String], data: &KVMap) {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.test_random_access.entry",
            num_keys = keys.len(),
        );

        const BITCOINLEVELDB_HARNESS_RANDOM_ACCESS_VERBOSE: bool = false;

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
        }

        let mut model_index: usize = 0;

        if BITCOINLEVELDB_HARNESS_RANDOM_ACCESS_VERBOSE {
            debug!(
                target: "bitcoinleveldb_harness",
                label = "bitcoinleveldb_harness.harness.test_random_access.verbose_separator",
            );
        }

        for _i in 0..200 {
            let toss: i32 = unsafe { (*rnd).uniform(5) as i32 };

            match toss {
                0 => {
                    if unsafe { (&*iter).valid() } {
                        if BITCOINLEVELDB_HARNESS_RANDOM_ACCESS_VERBOSE {
                            debug!(
                                target: "bitcoinleveldb_harness",
                                label = "bitcoinleveldb_harness.harness.test_random_access.next",
                            );
                        }
                        unsafe {
                            (&mut *iter).next();
                        }
                        model_index += 1;

                        let expected: String = if model_index == keys.len() {
                            self.to_string_with_data(data, None)
                        } else {
                            let k: &String = &keys[model_index];
                            let v: &String = match data.get(k) {
                                Some(v) => v,
                                None => {
                                    error!(
                                        target: "bitcoinleveldb_harness",
                                        label = "bitcoinleveldb_harness.harness.test_random_access.missing_model_value",
                                        key_len = k.len(),
                                    );
                                    panic!();
                                }
                            };
                            self.to_string_with_data(data, Some((k, v)))
                        };

                        assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                    }
                }

                1 => {
                    if BITCOINLEVELDB_HARNESS_RANDOM_ACCESS_VERBOSE {
                        debug!(
                            target: "bitcoinleveldb_harness",
                            label = "bitcoinleveldb_harness.harness.test_random_access.seek_to_first",
                        );
                    }
                    unsafe {
                        (&mut *iter).seek_to_first();
                    }
                    model_index = 0;

                    let expected: String = if model_index == keys.len() {
                        self.to_string_with_data(data, None)
                    } else {
                        let k: &String = &keys[model_index];
                        let v: &String = match data.get(k) {
                            Some(v) => v,
                            None => {
                                error!(
                                    target: "bitcoinleveldb_harness",
                                    label = "bitcoinleveldb_harness.harness.test_random_access.missing_model_value",
                                    key_len = k.len(),
                                );
                                panic!();
                            }
                        };
                        self.to_string_with_data(data, Some((k, v)))
                    };

                    assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                }

                2 => {
                    let key: String = self.pick_random_key(rnd, keys);

                    let mut left: usize = 0;
                    let mut right: usize = keys.len();
                    while left < right {
                        let mid: usize = left + ((right - left) / 2);

                        let a: Slice = Slice::from(keys[mid].as_bytes());
                        let b: Slice = Slice::from(key.as_bytes());

                        let c: i32 = self.options.comparator().compare(&a, &b);
                        if c < 0 {
                            left = mid + 1;
                        } else {
                            right = mid;
                        }
                    }
                    model_index = left;

                    if BITCOINLEVELDB_HARNESS_RANDOM_ACCESS_VERBOSE {
                        debug!(
                            target: "bitcoinleveldb_harness",
                            label = "bitcoinleveldb_harness.harness.test_random_access.seek",
                            key_len = key.len(),
                        );
                    }

                    let target: Slice = Slice::from(key.as_bytes());
                    unsafe {
                        (&mut *iter).seek(&target);
                    }

                    let expected: String = if model_index == keys.len() {
                        self.to_string_with_data(data, None)
                    } else {
                        let k: &String = &keys[model_index];
                        let v: &String = match data.get(k) {
                            Some(v) => v,
                            None => {
                                error!(
                                    target: "bitcoinleveldb_harness",
                                    label = "bitcoinleveldb_harness.harness.test_random_access.missing_model_value",
                                    key_len = k.len(),
                                );
                                panic!();
                            }
                        };
                        self.to_string_with_data(data, Some((k, v)))
                    };

                    assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                }

                3 => {
                    if unsafe { (&*iter).valid() } {
                        if BITCOINLEVELDB_HARNESS_RANDOM_ACCESS_VERBOSE {
                            debug!(
                                target: "bitcoinleveldb_harness",
                                label = "bitcoinleveldb_harness.harness.test_random_access.prev",
                            );
                        }
                        unsafe {
                            (&mut *iter).prev();
                        }
                        if model_index == 0 {
                            model_index = keys.len(); // Wrap around to invalid value
                        } else {
                            model_index -= 1;
                        }

                        let expected: String = if model_index == keys.len() {
                            self.to_string_with_data(data, None)
                        } else {
                            let k: &String = &keys[model_index];
                            let v: &String = match data.get(k) {
                                Some(v) => v,
                                None => {
                                    error!(
                                        target: "bitcoinleveldb_harness",
                                        label = "bitcoinleveldb_harness.harness.test_random_access.missing_model_value",
                                        key_len = k.len(),
                                    );
                                    panic!();
                                }
                            };
                            self.to_string_with_data(data, Some((k, v)))
                        };

                        assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                    }
                }

                4 => {
                    if BITCOINLEVELDB_HARNESS_RANDOM_ACCESS_VERBOSE {
                        debug!(
                            target: "bitcoinleveldb_harness",
                            label = "bitcoinleveldb_harness.harness.test_random_access.seek_to_last",
                        );
                    }
                    unsafe {
                        (&mut *iter).seek_to_last();
                    }
                    if keys.is_empty() {
                        model_index = keys.len();
                    } else {
                        let last: &String = &keys[keys.len() - 1];

                        let mut left: usize = 0;
                        let mut right: usize = keys.len();
                        while left < right {
                            let mid: usize = left + ((right - left) / 2);

                            let a: Slice = Slice::from(keys[mid].as_bytes());
                            let b: Slice = Slice::from(last.as_bytes());

                            let c: i32 = self.options.comparator().compare(&a, &b);
                            if c < 0 {
                                left = mid + 1;
                            } else {
                                right = mid;
                            }
                        }
                        model_index = left;
                    }

                    let expected: String = if model_index == keys.len() {
                        self.to_string_with_data(data, None)
                    } else {
                        let k: &String = &keys[model_index];
                        let v: &String = match data.get(k) {
                            Some(v) => v,
                            None => {
                                error!(
                                    target: "bitcoinleveldb_harness",
                                    label = "bitcoinleveldb_harness.harness.test_random_access.missing_model_value",
                                    key_len = k.len(),
                                );
                                panic!();
                            }
                        };
                        self.to_string_with_data(data, Some((k, v)))
                    };

                    assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                }

                _ => {
                    panic!();
                }
            }
        }

        unsafe {
            drop(Box::from_raw(iter));
        }

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.test_random_access.exit",
        );
    }
}
