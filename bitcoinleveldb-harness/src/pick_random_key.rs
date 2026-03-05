// ---------------- [ File: bitcoinleveldb-harness/src/pick_random_key.rs ]
crate::ix!();

impl Harness {

    pub fn pick_random_key(&mut self, rnd: *mut Random, keys: &[String]) -> String {
        if keys.is_empty() {
            "foo".to_owned()
        } else {
            let index: usize = unsafe { (*rnd).uniform(keys.len() as i32) as usize };
            let mut result: String = keys[index].clone();

            match unsafe { (*rnd).uniform(3) as i32 } {
                0 => {
                    // Return an existing key
                }

                1 => {
                    // Attempt to return something smaller than an existing key
                    if !result.is_empty() && result.is_ascii() {
                        let mut bytes: Vec<u8> = result.into_bytes();
                        if let Some(last) = bytes.last_mut() {
                            if *last > 0u8 {
                                *last = last.wrapping_sub(1);
                            }
                        }
                        result = match String::from_utf8(bytes) {
                            Ok(s) => s,
                            Err(e) => {
                                // Preserve determinism: fall back to the original bytes if UTF-8 would be violated.
                                match String::from_utf8(e.into_bytes()) {
                                    Ok(s) => s,
                                    Err(_) => {
                                        panic!();
                                    }
                                }
                            }
                        };
                    }
                }

                2 => {
                    // Return something larger than an existing key
                    const BYTEWISE: &str = "leveldb.BytewiseComparator";
                    const REVERSE: &str = "leveldb.ReverseBytewiseComparator";

                    let comparator_name: Cow<'_, str> = self.options.comparator().name();
                    let name: &str = comparator_name.as_ref();

                    if name == BYTEWISE {
                        result.push('\0');
                    } else {
                        // This is the case used by the C++ test harness.
                        // (ReverseKeyComparator compares keys by comparing their reversed bytes.)
                        debug_assert_eq!(name, REVERSE);
                        result.insert(0, '\0');
                    }
                }

                _ => {
                    panic!();
                }
            }

            result
        }
    }
}
