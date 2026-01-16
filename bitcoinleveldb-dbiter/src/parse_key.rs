// ---------------- [ File: bitcoinleveldb-dbiter/src/parse_key.rs ]
crate::ix!();

impl DBIter {
    #[inline]
    pub fn parse_key(&mut self, ikey_: *mut ParsedInternalKey) -> bool {
        let (k, bytes_read): (Slice, usize) = {
            let it = self.iter().borrow();
            let k = it.key();
            let v = it.value();
            let bytes_read = k.as_bytes().len() + v.as_bytes().len();
            (k, bytes_read)
        };

        while self.bytes_until_read_sampling() < bytes_read {
            let next_budget = self.bytes_until_read_sampling() + self.random_compaction_period();
            self.set_bytes_until_read_sampling(next_budget);

            // C++: db_->RecordReadSample(k);
            // Re-read key to avoid moving `k` regardless of DB signature.
            let k_sample: Slice = self.iter().borrow().key();
            self.db().borrow_mut().record_read_sample(k_sample);
        }

        assert!(self.bytes_until_read_sampling() >= bytes_read);

        self.set_bytes_until_read_sampling(self.bytes_until_read_sampling() - bytes_read);

        if !parse_internal_key(&k, ikey_) {
            let msg = Slice::from_str("corrupted internal key in DBIter");
            self.set_internal_status(Status::corruption(&msg, None));
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod dbiter_parse_key_suite {
    use super::*;

    use std::sync::atomic::Ordering;

    #[traced_test]
    fn parse_key_decodes_valid_internal_key() {
        info!("parse_key returns true and populates ParsedInternalKey for valid internal key");

        let entries = vec![make_entry(b"k", 7, ValueType::TypeValue, b"vk7")];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(7, 1, entries);

        dbiter.iter().borrow_mut().seek_to_first();
        assert!(dbiter.iter().borrow().valid());

        let mut ikey = ParsedInternalKey::default();
        let ok = dbiter.parse_key(&mut ikey as *mut ParsedInternalKey);

        assert!(ok);
        assert_eq!(ikey.user_key().as_bytes(), b"k");
        assert_eq!(*ikey.sequence(), 7);
        assert_eq!(*ikey.ty(), ValueType::TypeValue);

        let st = dbiter.status();
        assert_eq!(status_code(&st), StatusCode::Ok);
    }

    #[traced_test]
    fn parse_key_sets_corruption_status_on_invalid_internal_key() {
        info!("parse_key returns false and sets Status::Corruption on invalid internal key");

        let entries = vec![make_corrupt_entry(b"bad", b"v")];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(100, 2, entries);

        dbiter.iter().borrow_mut().seek_to_first();
        assert!(dbiter.iter().borrow().valid());

        let mut ikey = ParsedInternalKey::default();
        let ok = dbiter.parse_key(&mut ikey as *mut ParsedInternalKey);

        assert!(!ok);

        let st = dbiter.status();
        assert_eq!(status_code(&st), StatusCode::Corruption);
    }

    #[traced_test]
    fn parse_key_decrements_bytes_until_read_sampling_when_sufficient() {
        info!("parse_key decrements bytes_until_read_sampling_ by bytes_read when sufficient");

        let entries = vec![make_entry(b"k", 7, ValueType::TypeValue, b"v")];

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(100, 3, entries);

        dbiter.iter().borrow_mut().seek_to_first();
        assert!(dbiter.iter().borrow().valid());

        let (k, v) = {
            let it = dbiter.iter().borrow();
            (it.key(), it.value())
        };
        let bytes_read = k.as_bytes().len() + v.as_bytes().len();

        dbiter.set_bytes_until_read_sampling(bytes_read + 10);

        let mut ikey = ParsedInternalKey::default();
        let ok = dbiter.parse_key(&mut ikey as *mut ParsedInternalKey);

        assert!(ok);
        assert_eq!(dbiter.bytes_until_read_sampling(), 10);
    }

    #[traced_test]
    fn parse_key_triggers_read_sampling_when_insufficient_budget() {
        info!("parse_key triggers db.RecordReadSample() when bytes_until_read_sampling_ < bytes_read");

        let entries = vec![make_entry(b"k", 7, ValueType::TypeValue, b"v")];

        let (mut dbiter, calls, last_len) = build_dbiter_direct(100, 4, entries);

        dbiter.iter().borrow_mut().seek_to_first();
        assert!(dbiter.iter().borrow().valid());

        dbiter.set_bytes_until_read_sampling(0);

        let mut ikey = ParsedInternalKey::default();
        let ok = dbiter.parse_key(&mut ikey as *mut ParsedInternalKey);

        assert!(ok);

        let read_samples = calls.load(Ordering::SeqCst);
        assert!(read_samples >= 1);

        let key_len = last_len.load(Ordering::SeqCst);
        assert!(key_len >= 8);
    }
}
