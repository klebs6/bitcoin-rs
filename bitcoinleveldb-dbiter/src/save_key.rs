// ---------------- [ File: bitcoinleveldb-dbiter/src/save_key.rs ]
crate::ix!();

impl DBIter {
    #[inline]
    pub fn save_key(&mut self, k: &Slice, dst: *mut String) {
        // C++: dst->assign(k.data(), k.size());
        unsafe {
            let dst_ref: &mut String = &mut *dst;
            let v: &mut Vec<u8> = dst_ref.as_mut_vec();
            v.clear();
            v.extend_from_slice(k.as_bytes());
        }
    }

    #[inline]
    pub fn clear_saved_value(&mut self) {
        // C++: if (saved_value_.capacity() > 1048576) { std::string empty; swap(empty, saved_value_); }
        //      else { saved_value_.clear(); }

        if self.saved_value().capacity() > 1048576usize {
            let mut empty: String = String::new();
            core::mem::swap(&mut empty, self.saved_value_mut());
        } else {
            self.saved_value_mut().clear();
        }
    }
}

#[cfg(test)]
mod dbiter_save_key_suite {
    use super::*;

    #[traced_test]
    fn save_key_overwrites_destination_with_slice_bytes() {
        info!("save_key overwrites dst with k bytes");

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 1, vec![]);

        let k = Slice::from_bytes(b"abc");
        let mut dst = String::from("old");

        dbiter.save_key(&k, &mut dst as *mut String);

        assert_eq!(dst.as_bytes(), b"abc");
    }

    #[traced_test]
    fn save_key_writes_empty_when_slice_is_empty() {
        info!("save_key produces empty dst when k is empty");

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 2, vec![]);

        let k = Slice::from_bytes(b"");
        let mut dst = String::from("not-empty");

        dbiter.save_key(&k, &mut dst as *mut String);

        assert!(dst.is_empty());
    }

    #[traced_test]
    fn save_key_preserves_multibyte_utf8_sequences() {
        info!("save_key preserves UTF-8 byte sequences exactly");

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 3, vec![]);

        let bytes = "héllö".as_bytes();
        let k = Slice::from_bytes(bytes);
        let mut dst = String::new();

        dbiter.save_key(&k, &mut dst as *mut String);

        assert_eq!(dst.as_bytes(), bytes);
    }
}

#[cfg(test)]
mod dbiter_clear_saved_value_suite {
    use super::*;

    #[traced_test]
    fn clear_saved_value_clears_small_values_without_releasing_capacity() {
        info!("clear_saved_value clears saved_value for small capacity without swapping");

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 1, vec![]);

        *dbiter.saved_value_mut() = String::from("hello");
        let cap_before = dbiter.saved_value().capacity();

        dbiter.clear_saved_value();

        assert_eq!(dbiter.saved_value().len(), 0);
        assert_eq!(dbiter.saved_value().capacity(), cap_before);
    }

    #[traced_test]
    fn clear_saved_value_swaps_out_large_capacity_buffers() {
        info!("clear_saved_value swaps large buffers to release memory");

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 2, vec![]);

        *dbiter.saved_value_mut() = String::new();
        dbiter.saved_value_mut().reserve(2 * 1048576);
        dbiter.saved_value_mut().push_str("x");

        let cap_before = dbiter.saved_value().capacity();
        assert!(cap_before > 1048576);

        dbiter.clear_saved_value();

        let cap_after = dbiter.saved_value().capacity();
        assert_eq!(dbiter.saved_value().len(), 0);
        assert!(cap_after < cap_before);
    }

    #[traced_test]
    fn clear_saved_value_swaps_out_large_capacity_even_when_empty() {
        info!("clear_saved_value swaps out capacity even when saved_value is empty but huge");

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 3, vec![]);

        *dbiter.saved_value_mut() = String::new();
        dbiter.saved_value_mut().reserve(2 * 1048576);

        let cap_before = dbiter.saved_value().capacity();
        assert!(cap_before > 1048576);

        dbiter.clear_saved_value();

        let cap_after = dbiter.saved_value().capacity();
        assert_eq!(dbiter.saved_value().len(), 0);
        assert!(cap_after < cap_before);
    }
}
