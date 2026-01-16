// ---------------- [ File: bitcoinleveldb-dbiter/src/next.rs ]
crate::ix!();

impl DBIter {
    pub fn next(&mut self) {
        assert!(self.valid());

        match self.direction() {
            DBIterDirection::Reverse => {
                // Switch directions?
                debug!("DBIter::next: switching direction Reverse -> Forward");

                self.set_direction(DBIterDirection::Forward);

                // iter_ is pointing just before the entries for this->key(),
                // so advance into the range of entries for this->key() and then
                // use the normal skipping code below.
                if !self.iter().borrow().valid() {
                    self.iter().borrow_mut().seek_to_first();
                } else {
                    self.iter().borrow_mut().next();
                }

                if !self.iter().borrow().valid() {
                    self.set_valid(false);
                    self.saved_key__mut().clear();
                    return;
                }

                // saved_key_ already contains the key to skip past.
            }

            DBIterDirection::Forward => {
                // Store in saved_key_ the current key so we skip it below.
                let current_user_key: Slice = extract_user_key(&self.iter().borrow().key());

                let dst_ptr: *mut String = {
                    let dst_ref: &mut String = self.saved_key__mut();
                    dst_ref as *mut String
                };
                self.save_key(&current_user_key, dst_ptr);

                // iter_ is pointing to current key. We can now safely move to the next to
                // avoid checking current key.
                self.iter().borrow_mut().next();

                if !self.iter().borrow().valid() {
                    self.set_valid(false);
                    self.saved_key__mut().clear();
                    return;
                }
            }
        }

        let skip_ptr: *mut String = {
            let skip_ref: &mut String = self.saved_key__mut();
            skip_ref as *mut String
        };
        self.find_next_user_entry(true, skip_ptr);
    }
}

#[cfg(test)]
mod dbiter_next_suite {
    use super::*;

    #[traced_test]
    fn next_iterates_forward_over_visible_entries() {
        info!("next iterates forward and skips hidden/deleted entries");

        // a visible at seq 5
        // b deleted at seq 6 -> b hidden
        // c visible at seq 2
        let entries = vec![
            make_entry(b"a", 5, ValueType::TypeValue, b"a5"),
            make_entry(b"b", 6, ValueType::TypeDeletion, b""),
            make_entry(b"b", 3, ValueType::TypeValue, b"b3"),
            make_entry(b"c", 2, ValueType::TypeValue, b"c2"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(6, 9, entries);

        it.borrow_mut().seek_to_first();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"a");

        it.borrow_mut().next();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");
        assert_eq!(it.borrow().value().as_bytes(), b"c2");

        it.borrow_mut().next();
        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn next_switches_from_reverse_to_forward_and_yields_next_key() {
        info!("next switches direction reverse->forward and yields next user key");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
            make_entry(b"c", 1, ValueType::TypeValue, b"c1"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 44, entries);

        it.borrow_mut().seek_to_last();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");

        it.borrow_mut().prev();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");

        it.borrow_mut().next();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"c");
    }

    #[traced_test]
    fn next_from_last_entry_in_reverse_becomes_invalid() {
        info!("next called while in reverse at last user key becomes invalid");

        let entries = vec![
            make_entry(b"a", 3, ValueType::TypeValue, b"a3"),
            make_entry(b"b", 2, ValueType::TypeValue, b"b2"),
        ];

        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 55, entries);

        it.borrow_mut().seek_to_last();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");

        it.borrow_mut().next();
        assert!(!it.borrow().valid());
    }
}

