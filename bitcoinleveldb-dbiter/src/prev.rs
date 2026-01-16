// ---------------- [ File: bitcoinleveldb-dbiter/src/prev.rs ]
crate::ix!();

impl DBIter {

    pub fn prev(&mut self) {
        assert!(self.valid());

        if self.direction() == DBIterDirection::Forward {
            // Switch directions?
            // iter_ is pointing at the current entry.  Scan backwards until
            // the key changes so we can use the normal reverse scanning code.
            assert!(self.iter().borrow().valid()); // Otherwise valid_ would have been false

            let current_user_key: Slice = extract_user_key(&self.iter().borrow().key());

            let dst_ptr: *mut String = {
                let dst_ref: &mut String = self.saved_key__mut();
                dst_ref as *mut String
            };
            self.save_key(&current_user_key, dst_ptr);

            loop {
                self.iter().borrow_mut().prev();

                if !self.iter().borrow().valid() {
                    self.set_valid(false);
                    self.saved_key__mut().clear();
                    self.clear_saved_value();
                    return;
                }

                let iter_user_key: Slice = extract_user_key(&self.iter().borrow().key());
                let saved_key_slice: Slice = Slice::from(self.saved_key_());

                if self.user_comparator().compare(&iter_user_key, &saved_key_slice) < 0 {
                    break;
                }
            }

            debug!("DBIter::prev: switching direction Forward -> Reverse");
            self.set_direction(DBIterDirection::Reverse);
        }

        self.find_prev_user_entry();
    }
}

#[cfg(test)]
mod dbiter_prev_suite {
    use super::*;

    #[traced_test]
    fn prev_moves_backwards_over_visible_user_keys() {
        info!("prev moves backwards across visible entries");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
            make_entry(b"c", 1, ValueType::TypeValue, b"c1"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 100, entries);

        it.borrow_mut().seek_to_last();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");

        it.borrow_mut().prev();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");

        it.borrow_mut().prev();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");

        it.borrow_mut().prev();
        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn prev_switches_direction_from_forward_to_reverse_and_back_via_next() {
        info!("prev switches from forward->reverse; next switches reverse->forward");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
            make_entry(b"c", 1, ValueType::TypeValue, b"c1"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 7, entries);

        it.borrow_mut().seek_to_first();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");

        it.borrow_mut().next();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");

        it.borrow_mut().prev();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");

        it.borrow_mut().next();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");
    }

    #[traced_test]
    fn prev_skips_keys_hidden_by_deletion_markers() {
        info!("prev skips over deleted user keys");

        // b is deleted at seq 5, so visible sequence is a then c.
        let entries = vec![
            make_entry(b"a", 4, ValueType::TypeValue, b"a4"),
            make_entry(b"b", 5, ValueType::TypeDeletion, b""),
            make_entry(b"b", 3, ValueType::TypeValue, b"b3"),
            make_entry(b"c", 2, ValueType::TypeValue, b"c2"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(5, 7, entries);

        it.borrow_mut().seek_to_last();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");

        it.borrow_mut().prev();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");
    }
}
