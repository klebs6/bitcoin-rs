// ---------------- [ File: bitcoinleveldb-dbiter/src/find_next_user_entry.rs ]
crate::ix!();

impl DBIter {
    pub fn find_next_user_entry(&mut self, mut skipping: bool, skip: *mut String) {
        // Loop until we hit an acceptable entry to yield
        assert!(self.iter().borrow().valid());

        match self.direction() {
            DBIterDirection::Forward => {}
            DBIterDirection::Reverse => {
                panic!("assert(direction_ == kForward);");
            }
        }

        loop {
            let mut ikey: ParsedInternalKey = ParsedInternalKey::default();

            if self.parse_key(&mut ikey as *mut ParsedInternalKey) && (*ikey.sequence()) <= self.sequence() {
                match *ikey.ty() {
                    ValueType::TypeDeletion => {
                        // Arrange to skip all upcoming entries for this key since
                        // they are hidden by this deletion.
                        let user_key: &Slice = ikey.user_key();
                        self.save_key(user_key, skip);

                        skipping = true;
                    }

                    ValueType::TypeValue => {
                        if skipping {
                            let skip_slice: Slice = unsafe { Slice::from(&*skip) };

                            if self.user_comparator().compare(ikey.user_key(), &skip_slice) <= 0 {
                                // Entry hidden
                            } else {
                                self.set_valid(true);
                                self.saved_key__mut().clear();
                                return;
                            }
                        } else {
                            self.set_valid(true);
                            self.saved_key__mut().clear();
                            return;
                        }
                    }
                }
            }

            self.iter().borrow_mut().next();

            if !self.iter().borrow().valid() {
                break;
            }
        }

        self.saved_key__mut().clear();
        self.set_valid(false);
    }
}

#[cfg(test)]
mod dbiter_find_next_user_entry_suite {
    use super::*;

    #[traced_test]
    fn find_next_user_entry_yields_first_visible_value_when_not_skipping() {
        info!("find_next_user_entry yields first visible user entry when skipping=false");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
        ];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 1, entries);

        dbiter.set_direction(DBIterDirection::Forward);
        dbiter.iter().borrow_mut().seek_to_first();

        let mut tmp = String::new();
        dbiter.find_next_user_entry(false, &mut tmp as *mut String);

        assert!(dbiter.valid());
        assert_eq!(dbiter.key().as_bytes(), b"a");
        assert_eq!(dbiter.value().as_bytes(), b"a3");
    }

    #[traced_test]
    fn find_next_user_entry_respects_deletion_and_skips_hidden_values() {
        info!("find_next_user_entry sets skipping on deletion and skips values for that key");

        // a deleted at seq 5 => hide a4; yield b3
        let entries = vec![
            make_entry(b"a", 5, ValueType::TypeDeletion, b""),
            make_entry(b"a", 4, ValueType::TypeValue, b"a4"),
            make_entry(b"b", 3, ValueType::TypeValue, b"b3"),
        ];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(5, 2, entries);

        dbiter.set_direction(DBIterDirection::Forward);
        dbiter.iter().borrow_mut().seek_to_first();

        let mut skip = String::new();
        dbiter.find_next_user_entry(false, &mut skip as *mut String);

        assert!(dbiter.valid());
        assert_eq!(dbiter.key().as_bytes(), b"b");
        assert_eq!(dbiter.value().as_bytes(), b"b3");
    }

    #[traced_test]
    fn find_next_user_entry_skips_current_key_when_skipping_true() {
        info!("find_next_user_entry skips entries for user keys <= skip when skipping=true");

        let entries = vec![
            make_entry(b"a", 2, ValueType::TypeValue, b"a2"),
            make_entry(b"b", 1, ValueType::TypeValue, b"b1"),
        ];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 3, entries);

        dbiter.set_direction(DBIterDirection::Forward);
        dbiter.iter().borrow_mut().seek_to_first();

        let mut skip = String::from("a");
        dbiter.find_next_user_entry(true, &mut skip as *mut String);

        assert!(dbiter.valid());
        assert_eq!(dbiter.key().as_bytes(), b"b");
        assert_eq!(dbiter.value().as_bytes(), b"b1");
    }

    #[traced_test]
    fn find_next_user_entry_ignores_entries_newer_than_snapshot() {
        info!("find_next_user_entry ignores entries with sequence > snapshot");

        // a has seq 9 (hidden), seq 5 (visible) -> yields a5
        let entries = vec![
            make_entry(b"a", 9, ValueType::TypeValue, b"a9"),
            make_entry(b"a", 5, ValueType::TypeValue, b"a5"),
        ];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(5, 4, entries);

        dbiter.set_direction(DBIterDirection::Forward);
        dbiter.iter().borrow_mut().seek_to_first();

        let mut tmp = String::new();
        dbiter.find_next_user_entry(false, &mut tmp as *mut String);

        assert!(dbiter.valid());
        assert_eq!(dbiter.key().as_bytes(), b"a");
        assert_eq!(dbiter.value().as_bytes(), b"a5");
    }

    #[traced_test]
    fn find_next_user_entry_skips_corrupt_key_sets_corruption_status() {
        info!("find_next_user_entry skips corrupt key and sets status to corruption");

        let entries = vec![
            make_corrupt_entry(b"bad", b"oops"),
            make_entry(b"a", 1, ValueType::TypeValue, b"a1"),
        ];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 5, entries);

        dbiter.set_direction(DBIterDirection::Forward);
        dbiter.iter().borrow_mut().seek_to_first();

        let mut tmp = String::new();
        dbiter.find_next_user_entry(false, &mut tmp as *mut String);

        assert!(dbiter.valid());
        assert_eq!(dbiter.key().as_bytes(), b"a");

        let st = dbiter.status();
        assert_eq!(status_code(&st), StatusCode::Corruption);
    }

    #[traced_test]
    fn find_next_user_entry_reaches_end_sets_invalid_and_clears_saved_key() {
        info!("find_next_user_entry sets invalid and clears saved_key_ at end");

        let entries = vec![make_entry(b"a", 1, ValueType::TypeDeletion, b"")];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 6, entries);

        dbiter.set_direction(DBIterDirection::Forward);
        dbiter.iter().borrow_mut().seek_to_first();

        let mut tmp = String::new();
        dbiter.find_next_user_entry(false, &mut tmp as *mut String);

        assert!(!dbiter.valid());
        assert!(dbiter.saved_key_().is_empty());
    }
}
