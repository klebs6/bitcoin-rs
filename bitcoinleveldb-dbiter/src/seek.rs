// ---------------- [ File: bitcoinleveldb-dbiter/src/seek.rs ]
crate::ix!();

impl DBIter {

    pub fn seek(&mut self, target: &Slice) {
        self.set_direction(DBIterDirection::Forward);
        self.clear_saved_value();
        self.saved_key__mut().clear();

        let seq = self.sequence();
        let parsed = ParsedInternalKey::new(target, &seq, ValueType::TypeValue);

        let dst_ptr: *mut String = {
            let dst_ref: &mut String = self.saved_key__mut();
            dst_ref as *mut String
        };
        append_internal_key(dst_ptr, &parsed);

        let internal_target: Slice = Slice::from(self.saved_key_());
        self.iter().borrow_mut().seek(&internal_target);

        if self.iter().borrow().valid() {
            let tmp_ptr: *mut String = {
                let tmp_ref: &mut String = self.saved_key__mut();
                tmp_ref as *mut String
            };
            self.find_next_user_entry(false, tmp_ptr /* temporary storage */);
        } else {
            self.set_valid(false);
        }
    }

    pub fn seek_to_first(&mut self) {
        self.set_direction(DBIterDirection::Forward);
        self.clear_saved_value();

        self.iter().borrow_mut().seek_to_first();

        if self.iter().borrow().valid() {
            let tmp_ptr: *mut String = {
                let tmp_ref: &mut String = self.saved_key__mut();
                tmp_ref as *mut String
            };
            self.find_next_user_entry(false, tmp_ptr /* temporary storage */);
        } else {
            self.set_valid(false);
        }
    }

    pub fn seek_to_last(&mut self) {
        self.set_direction(DBIterDirection::Reverse);
        self.clear_saved_value();

        self.iter().borrow_mut().seek_to_last();
        self.find_prev_user_entry();
    }
}

#[cfg(test)]
mod dbiter_seek_to_first_suite {
    use super::*;

    #[traced_test]
    fn seek_to_first_on_empty_sets_invalid() {
        info!("seek_to_first on empty iterator yields invalid");

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(100, 1, vec![]);
        it.borrow_mut().seek_to_first();

        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn seek_to_first_skips_deleted_first_user_key() {
        info!("seek_to_first skips a user key deleted at or below snapshot");

        // a deleted at seq 5; a older value exists but should be hidden.
        // b visible.
        let entries = vec![
            make_entry(b"a", 5, ValueType::TypeDeletion, b""),
            make_entry(b"a", 4, ValueType::TypeValue, b"a4"),
            make_entry(b"b", 3, ValueType::TypeValue, b"b3"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(5, 77, entries);

        it.borrow_mut().seek_to_first();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");
        assert_eq!(it.borrow().value().as_bytes(), b"b3");
    }

    #[traced_test]
    fn seek_to_first_skips_corrupt_internal_key_but_sets_corruption_status() {
        info!("seek_to_first skips corrupt internal key; iterator remains usable but status is corruption");

        let entries = vec![
            make_corrupt_entry(b"bad", b"x"),
            make_entry(b"a", 1, ValueType::TypeValue, b"va"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 1234, entries);

        it.borrow_mut().seek_to_first();

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");
        assert_eq!(it.borrow().value().as_bytes(), b"va");

        let st = it.borrow().status();
        assert_eq!(status_code(&st), StatusCode::Corruption);
    }

    #[traced_test]
    fn seek_to_first_clears_reverse_saved_state() {
        info!("seek_to_first clears reverse saved value/state and iterates forward from beginning");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
            make_entry(b"c", 1, ValueType::TypeValue, b"c1"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(3, 9, entries);

        it.borrow_mut().seek_to_last();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");
        assert_eq!(it.borrow().value().as_bytes(), b"c1");

        it.borrow_mut().seek_to_first();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");
        assert_eq!(it.borrow().value().as_bytes(), b"a3");
    }
}

#[cfg(test)]
mod dbiter_seek_to_last_suite {
    use super::*;

    #[traced_test]
    fn seek_to_last_on_empty_sets_invalid() {
        info!("seek_to_last on empty iterator yields invalid");

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(100, 1, vec![]);
        it.borrow_mut().seek_to_last();

        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn seek_to_last_yields_last_visible_user_key() {
        info!("seek_to_last yields last visible user key");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
            make_entry(b"c", 1, ValueType::TypeValue, b"c1"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 2, entries);
        it.borrow_mut().seek_to_last();

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");
        assert_eq!(it.borrow().value().as_bytes(), b"c1");
    }

    #[traced_test]
    fn seek_to_last_skips_last_key_deleted_at_or_below_snapshot() {
        info!("seek_to_last skips last user key if deleted at/below snapshot");

        // b is last user key but deleted at seq 5 => should yield a.
        let entries = vec![
            make_entry(b"a", 4, ValueType::TypeValue, b"a4"),
            make_entry(b"b", 5, ValueType::TypeDeletion, b""),
            make_entry(b"b", 3, ValueType::TypeValue, b"b3"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(5, 88, entries);
        it.borrow_mut().seek_to_last();

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");
        assert_eq!(it.borrow().value().as_bytes(), b"a4");
    }

    #[traced_test]
    fn seek_to_last_selects_highest_sequence_at_or_below_snapshot_for_last_key() {
        info!("seek_to_last chooses newest version <= snapshot");

        // last key 'c' has seq 9 (hidden by snapshot=7), seq 7 (visible)
        let entries = vec![
            make_entry(b"b", 1, ValueType::TypeValue, b"b1"),
            make_entry(b"c", 9, ValueType::TypeValue, b"c9"),
            make_entry(b"c", 7, ValueType::TypeValue, b"c7"),
            make_entry(b"c", 3, ValueType::TypeValue, b"c3"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(7, 314, entries);
        it.borrow_mut().seek_to_last();

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");
        assert_eq!(it.borrow().value().as_bytes(), b"c7");
    }
}

#[cfg(test)]
mod dbiter_seek_suite {
    use super::*;

    #[traced_test]
    fn seek_target_before_first_yields_first_visible() {
        info!("seek(target < first) yields first visible entry");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 1, entries);

        let target = Slice::from_bytes(b"0");
        it.borrow_mut().seek(&target);

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");
        assert_eq!(it.borrow().value().as_bytes(), b"a3");
    }

    #[traced_test]
    fn seek_to_existing_key_yields_that_key() {
        info!("seek(existing key) yields that key if visible");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 1, entries);

        let target = Slice::from_bytes(b"b");
        it.borrow_mut().seek(&target);

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");
        assert_eq!(it.borrow().value().as_bytes(), b"b2");
    }

    #[traced_test]
    fn seek_between_keys_yields_next_key() {
        info!("seek(between keys) yields next user key");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"c", 2, ValueType::TypeValue, b"c2"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 1, entries);

        let target = Slice::from_bytes(b"b");
        it.borrow_mut().seek(&target);

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");
        assert_eq!(it.borrow().value().as_bytes(), b"c2");
    }

    #[traced_test]
    fn seek_beyond_last_yields_invalid() {
        info!("seek(target > last) yields invalid");

        let entries = vec![make_entry(b"a", 1, ValueType::TypeValue, b"a1")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 1, entries);

        let target = Slice::from_bytes(b"z");
        it.borrow_mut().seek(&target);

        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn seek_skips_deleted_key_and_yields_next_visible() {
        info!("seek(deleted key) yields next visible user key");

        let entries = vec![
            make_entry(b"a", 5, ValueType::TypeValue, b"a5"),
            make_entry(b"b", 6, ValueType::TypeDeletion, b""),
            make_entry(b"b", 4, ValueType::TypeValue, b"b4"),
            make_entry(b"c", 3, ValueType::TypeValue, b"c3"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(6, 2, entries);

        let target = Slice::from_bytes(b"b");
        it.borrow_mut().seek(&target);

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");
        assert_eq!(it.borrow().value().as_bytes(), b"c3");
    }

    #[traced_test]
    fn seek_respects_snapshot_and_may_skip_key_with_only_newer_entries() {
        info!("seek respects snapshot; if key only has newer entries, it yields the next key");

        // b has seq 10 only, snapshot=5 => b not visible; next is c.
        let entries = vec![
            make_entry(b"a", 4, ValueType::TypeValue, b"a4"),
            make_entry(b"b", 10, ValueType::TypeValue, b"b10"),
            make_entry(b"c", 3, ValueType::TypeValue, b"c3"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(5, 3, entries);

        let target = Slice::from_bytes(b"b");
        it.borrow_mut().seek(&target);

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");
        assert_eq!(it.borrow().value().as_bytes(), b"c3");
    }
}
