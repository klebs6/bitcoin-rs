// ---------------- [ File: bitcoinleveldb-dbiter/src/find_prev_user_entry.rs ]
crate::ix!();

impl DBIter {

    pub fn find_prev_user_entry(&mut self) {
        match self.direction() {
            DBIterDirection::Reverse => {}
            DBIterDirection::Forward => {
                panic!("assert(direction_ == kReverse);");
            }
        }

        let mut value_type: ValueType = ValueType::TypeDeletion;

        if self.iter().borrow().valid() {
            loop {
                let mut ikey: ParsedInternalKey = ParsedInternalKey::default();

                if self.parse_key(&mut ikey as *mut ParsedInternalKey) && (*ikey.sequence()) <= self.sequence() {
                    if (value_type != ValueType::TypeDeletion)
                        && (self
                            .user_comparator()
                            .compare(ikey.user_key(), &Slice::from(self.saved_key_()))
                            < 0)
                    {
                        // We encountered a non-deleted value in entries for previous keys,
                        break;
                    }

                    value_type = *ikey.ty();

                    if value_type == ValueType::TypeDeletion {
                        self.saved_key__mut().clear();
                        self.clear_saved_value();
                    } else {
                        let raw_value: Slice = self.iter().borrow().value();

                        if self.saved_value().capacity() > (raw_value.as_bytes().len() + 1048576usize) {
                            let mut empty: String = String::new();
                            core::mem::swap(&mut empty, self.saved_value_mut());
                        }

                        let user_key: Slice = extract_user_key(&self.iter().borrow().key());

                        let dst_ptr: *mut String = {
                            let dst_ref: &mut String = self.saved_key__mut();
                            dst_ref as *mut String
                        };
                        self.save_key(&user_key, dst_ptr);

                        let dst_val: &mut String = self.saved_value_mut();
                        unsafe {
                            let v: &mut Vec<u8> = dst_val.as_mut_vec();
                            v.clear();
                            v.extend_from_slice(raw_value.as_bytes());
                        }
                    }
                }

                self.iter().borrow_mut().prev();

                if !self.iter().borrow().valid() {
                    break;
                }
            }
        }

        if value_type == ValueType::TypeDeletion {
            // End
            self.set_valid(false);
            self.saved_key__mut().clear();
            self.clear_saved_value();
            self.set_direction(DBIterDirection::Forward);
        } else {
            self.set_valid(true);
        }
    }
}

#[cfg(test)]
mod dbiter_find_prev_user_entry_suite {
    use super::*;

    #[traced_test]
    fn find_prev_user_entry_yields_last_visible_key_value_pair() {
        info!("find_prev_user_entry returns last visible entry when positioned at end");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
            make_entry(b"c", 1, ValueType::TypeValue, b"c1"),
        ];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 11, entries);

        dbiter.set_direction(DBIterDirection::Reverse);
        dbiter.clear_saved_value();
        dbiter.iter().borrow_mut().seek_to_last();

        dbiter.find_prev_user_entry();

        assert!(dbiter.valid());
        assert_eq!(dbiter.key().as_bytes(), b"c");
        assert_eq!(dbiter.value().as_bytes(), b"c1");
    }

    #[traced_test]
    fn find_prev_user_entry_selects_newest_version_at_or_below_snapshot() {
        info!("find_prev_user_entry chooses highest seq <= snapshot for last key");

        let entries = vec![
            make_entry(b"c", 9, ValueType::TypeValue, b"c9"),
            make_entry(b"c", 7, ValueType::TypeValue, b"c7"),
            make_entry(b"c", 3, ValueType::TypeValue, b"c3"),
        ];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(7, 5, entries);

        dbiter.set_direction(DBIterDirection::Reverse);
        dbiter.clear_saved_value();
        dbiter.iter().borrow_mut().seek_to_last();

        dbiter.find_prev_user_entry();

        assert!(dbiter.valid());
        assert_eq!(dbiter.key().as_bytes(), b"c");
        assert_eq!(dbiter.value().as_bytes(), b"c7");
    }

    #[traced_test]
    fn find_prev_user_entry_skips_user_key_deleted_at_or_below_snapshot() {
        info!("find_prev_user_entry skips a user key deleted <= snapshot and yields previous visible key");

        // b is last user key but deleted at seq 5 => yields a.
        let entries = vec![
            make_entry(b"a", 4, ValueType::TypeValue, b"a4"),
            make_entry(b"b", 5, ValueType::TypeDeletion, b""),
            make_entry(b"b", 3, ValueType::TypeValue, b"b3"),
        ];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(5, 5, entries);

        dbiter.set_direction(DBIterDirection::Reverse);
        dbiter.clear_saved_value();
        dbiter.iter().borrow_mut().seek_to_last();

        dbiter.find_prev_user_entry();

        assert!(dbiter.valid());
        assert_eq!(dbiter.key().as_bytes(), b"a");
        assert_eq!(dbiter.value().as_bytes(), b"a4");
    }

    #[traced_test]
    fn find_prev_user_entry_on_empty_sets_invalid_and_resets_direction_to_forward() {
        info!("find_prev_user_entry on empty sets invalid and switches direction to Forward");

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 1, vec![]);

        dbiter.set_direction(DBIterDirection::Reverse);
        dbiter.clear_saved_value();
        dbiter.iter().borrow_mut().seek_to_last();

        dbiter.find_prev_user_entry();

        assert!(!dbiter.valid());
        match dbiter.direction() {
            DBIterDirection::Forward => {}
            DBIterDirection::Reverse => {
                assert!(false);
            }
        }
    }
}
