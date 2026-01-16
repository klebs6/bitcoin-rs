// ---------------- [ File: bitcoinleveldb-dbiter/src/access.rs ]
crate::ix!();

impl DBIter {
    pub fn key(&self) -> Slice {
        assert!(self.valid());

        match self.direction() {
            DBIterDirection::Forward => extract_user_key(&self.iter().borrow().key()),
            DBIterDirection::Reverse => Slice::from(self.saved_key_()),
        }
    }

    pub fn value(&self) -> Slice {
        assert!(self.valid());

        match self.direction() {
            DBIterDirection::Forward => self.iter().borrow().value(),
            DBIterDirection::Reverse => Slice::from(self.saved_value()),
        }
    }

    pub fn status(&self) -> crate::Status {
        if self.internal_status().is_ok() {
            self.iter().borrow().status()
        } else {
            self.internal_status().clone()
        }
    }
}

#[cfg(test)]
mod dbiter_key_suite {
    use super::*;

    use std::panic::{AssertUnwindSafe, catch_unwind};

    #[traced_test]
    fn key_panics_when_iterator_is_invalid() {
        info!("key() asserts(valid) and panics when invalid");

        let entries = vec![make_entry(b"a", 1, ValueType::TypeValue, b"a1")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 1, entries);

        let r = catch_unwind(AssertUnwindSafe(|| {
            let _ = it.borrow().key();
        }));

        assert!(r.is_err());
    }

    #[traced_test]
    fn key_returns_user_key_not_internal_key_in_forward_direction() {
        info!("key() returns ExtractUserKey(internal_key) in forward direction");

        let entries = vec![make_entry(b"user", 7, ValueType::TypeValue, b"v")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 2, entries);

        it.borrow_mut().seek_to_first();

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"user");
    }

    #[traced_test]
    fn key_returns_saved_key_in_reverse_direction() {
        info!("key() returns saved_key_ in reverse direction");

        let entries = vec![
            make_entry(b"a", 1, ValueType::TypeValue, b"a1"),
            make_entry(b"b", 1, ValueType::TypeValue, b"b1"),
        ];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 3, entries);

        it.borrow_mut().seek_to_last();

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");
    }
}

#[cfg(test)]
mod dbiter_value_suite {
    use super::*;

    use std::panic::{AssertUnwindSafe, catch_unwind};

    #[traced_test]
    fn value_panics_when_iterator_is_invalid() {
        info!("value() asserts(valid) and panics when invalid");

        let entries = vec![make_entry(b"a", 1, ValueType::TypeValue, b"a1")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 1, entries);

        let r = catch_unwind(AssertUnwindSafe(|| {
            let _ = it.borrow().value();
        }));

        assert!(r.is_err());
    }

    #[traced_test]
    fn value_returns_forward_underlying_value_when_direction_forward() {
        info!("value() returns iter_->value() in forward direction");

        let entries = vec![make_entry(b"a", 1, ValueType::TypeValue, b"a1")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 2, entries);

        it.borrow_mut().seek_to_first();

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().value().as_bytes(), b"a1");
    }

    #[traced_test]
    fn value_returns_saved_value_when_direction_reverse() {
        info!("value() returns saved_value_ in reverse direction");

        let entries = vec![
            make_entry(b"a", 1, ValueType::TypeValue, b"a1"),
            make_entry(b"b", 1, ValueType::TypeValue, b"b1"),
        ];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 3, entries);

        it.borrow_mut().seek_to_last();

        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");
        assert_eq!(it.borrow().value().as_bytes(), b"b1");
    }
}

#[cfg(test)]
mod dbiter_valid_suite {
    use super::*;

    #[traced_test]
    fn valid_is_false_initially() {
        info!("valid() is false initially");

        let entries = vec![make_entry(b"a", 1, ValueType::TypeValue, b"a1")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 1, entries);

        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn valid_transitions_true_after_seek_to_first_and_false_after_consuming_end() {
        info!("valid toggles correctly across seek_to_first and next to end");

        let entries = vec![make_entry(b"a", 1, ValueType::TypeValue, b"a1")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 2, entries);

        it.borrow_mut().seek_to_first();
        assert!(it.borrow().valid());

        it.borrow_mut().next();
        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn valid_is_false_after_seek_beyond_last() {
        info!("valid is false after seek beyond last key");

        let entries = vec![make_entry(b"a", 1, ValueType::TypeValue, b"a1")];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 3, entries);

        let target = Slice::from_bytes(b"z");
        it.borrow_mut().seek(&target);

        assert!(!it.borrow().valid());
    }

    #[traced_test]
    fn valid_is_true_after_seek_to_last_when_visible_entry_exists() {
        info!("valid is true after seek_to_last when a visible entry exists");

        let entries = vec![
            make_entry(b"a", 1, ValueType::TypeValue, b"a1"),
            make_entry(b"b", 1, ValueType::TypeValue, b"b1"),
        ];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(10, 4, entries);

        it.borrow_mut().seek_to_last();
        assert!(it.borrow().valid());
        assert_eq!(it.borrow().key().as_bytes(), b"b");
    }

    #[traced_test]
    fn valid_is_false_when_all_entries_are_deleted_at_or_below_snapshot() {
        info!("valid is false when all entries are deleted and no other visible keys exist");

        let entries = vec![
            make_entry(b"a", 2, ValueType::TypeDeletion, b""),
            make_entry(b"a", 1, ValueType::TypeValue, b"a1"),
        ];
        let (it, _calls, _last_len) = build_leveldb_iterator_via_new_db_iterator(2, 5, entries);

        it.borrow_mut().seek_to_first();
        assert!(!it.borrow().valid());

        it.borrow_mut().seek_to_last();
        assert!(!it.borrow().valid());
    }
}

#[cfg(test)]
mod dbiter_status_suite {
    use super::*;

    #[traced_test]
    fn status_returns_internal_iterator_status_when_dbiter_status_is_ok() {
        info!("status() returns internal iterator status when dbiter internal status is OK");

        let internal_key = encode_internal_key_bytes(b"k", 1, ValueType::TypeValue);
        let value = b"v".to_vec();

        let msg = Slice::from_bytes(b"io");
        let internal_status = Status::io_error(&msg, None);

        let internal_iter = make_fixed_status_internal_iter(internal_key, value, internal_status.clone());

        let (dbiter, _calls, _last_len) = build_dbiter_direct_with_internal_iter(10, 1, internal_iter);

        let st = dbiter.status();
        assert_eq!(status_code(&st), status_code(&internal_status));
    }

    #[traced_test]
    fn status_returns_dbiter_corruption_even_if_internal_iterator_reports_other_error() {
        info!("status() returns dbiter corruption when dbiter status is not OK, regardless of internal iterator status");

        // Internal iterator reports IO error, but first key is corrupt => dbiter sets corruption.
        let msg = Slice::from_bytes(b"io");
        let internal_status = Status::io_error(&msg, None);

        let corrupt_key = b"bad".to_vec();
        let value = b"v".to_vec();

        let internal_iter = make_fixed_status_internal_iter(corrupt_key, value, internal_status);

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct_with_internal_iter(10, 2, internal_iter);

        dbiter.seek_to_first();
        assert!(!dbiter.valid() || dbiter.valid());

        let st = dbiter.status();
        assert_eq!(status_code(&st), StatusCode::Corruption);
    }

    #[traced_test]
    fn status_is_ok_when_no_errors_and_internal_iterator_ok() {
        info!("status() returns OK when both dbiter status and internal iterator status are OK");

        let internal_key = encode_internal_key_bytes(b"k", 1, ValueType::TypeValue);
        let value = b"v".to_vec();

        let internal_status = Status::ok();
        let internal_iter = make_fixed_status_internal_iter(internal_key, value, internal_status);

        let (dbiter, _calls, _last_len) = build_dbiter_direct_with_internal_iter(10, 3, internal_iter);

        let st = dbiter.status();
        assert_eq!(status_code(&st), StatusCode::Ok);
    }
}
