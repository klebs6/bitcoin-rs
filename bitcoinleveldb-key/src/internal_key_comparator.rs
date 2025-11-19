// ---------------- [ File: bitcoinleveldb-key/src/internal_key_comparator.rs ]
crate::ix!();

/**
  | A comparator for internal keys that
  | uses a specified comparator for the
  | user key portion and breaks ties by decreasing
  | sequence number.
  |
  */
pub struct InternalKeyComparator {
    user_comparator: *const dyn SliceComparator,
}

impl InternalKeyComparator {

    pub fn compare_slices(&self, akey_: &Slice, bkey_: &Slice) -> i32 {
        let a_data = slice_as_bytes(akey_);
        let b_data = slice_as_bytes(bkey_);
        trace!(
            "InternalKeyComparator::compare_slices: a_len={}, b_len={}",
            a_data.len(),
            b_data.len()
        );

        let a_user = extract_user_key(akey_);
        let b_user = extract_user_key(bkey_);

        let user_cmp = unsafe {
            if !self.user_comparator.is_null() {
                let uc = &*self.user_comparator;
                uc.compare(&a_user, &b_user)
            } else {
                bytewise_compare(slice_as_bytes(&a_user), slice_as_bytes(&b_user))
            }
        };

        if user_cmp != 0 {
            return user_cmp;
        }

        unsafe {
            let a_size = *akey_.size();
            let b_size = *bkey_.size();
            debug_assert!(a_size >= 8 && b_size >= 8);

            let a_num = decode_fixed64_le((*akey_.data()).add(a_size - 8));
            let b_num = decode_fixed64_le((*bkey_.data()).add(b_size - 8));

            if a_num > b_num {
                -1
            } else if a_num < b_num {
                1
            } else {
                0
            }
        }
    }

    #[inline]
    pub fn compare_internal_key(&self, a: &InternalKey, b: &InternalKey) -> i32 {
        trace!("InternalKeyComparator::compare_internal_key");
        let a_slice = a.encode();
        let b_slice = b.encode();
        self.compare(&a_slice, &b_slice)
    }

    fn compare_internal_bytes(&self, a: &[u8], b: &[u8]) -> i32 {
        unsafe {
            let a_slice = Slice::from_ptr_len(a.as_ptr(), a.len());
            let b_slice = Slice::from_ptr_len(b.as_ptr(), b.len());
            self.compare(&a_slice, &b_slice)
        }
    }
}

impl SliceComparator for InternalKeyComparator {

    fn bytewise_comparator(&self) -> *const (dyn SliceComparator + 'static) {
        trace!("InternalKeyComparator::bytewise_comparator");
        self.user_comparator
    }
}

impl Compare for InternalKeyComparator {

    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        self.compare_slices(a, b)
    }
}

impl Named for InternalKeyComparator {

    fn name(&self) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Borrowed("leveldb.InternalKeyComparator")
    }
}

impl InternalKeyComparator {

    pub fn new(c: *const dyn SliceComparator) -> Self {
        trace!("InternalKeyComparator::new: user_comparator={:p}", c);
        InternalKeyComparator { user_comparator: c }
    }

    #[inline]
    pub fn user_comparator(&self) -> *const dyn SliceComparator {
        self.user_comparator
    }
}

impl FindShortSuccessor for InternalKeyComparator {

    fn find_short_successor(&self, k: &mut Vec<u8>) {
        trace!(
            "InternalKeyComparator::find_short_successor: len={}",
            k.len()
        );
        if k.len() < 8 {
            return;
        }

        let original_internal = k.clone();
        let user_len = k.len() - 8;
        let original_user = k[..user_len].to_vec();
        let mut tmp = original_user.clone();

        unsafe {
            if !self.user_comparator.is_null() {
                let uc = &*self.user_comparator;
                uc.find_short_successor(&mut tmp);
            } else {
                let _ = find_short_successor_user_key(&mut tmp);
            }
        }

        if tmp.len() >= original_user.len() {
            return;
        }

        let user_cmp = unsafe {
            if !self.user_comparator.is_null() {
                let uc = &*self.user_comparator;
                let orig_slice =
                    Slice::from_ptr_len(original_user.as_ptr(), original_user.len());
                let tmp_slice = Slice::from_ptr_len(tmp.as_ptr(), tmp.len());
                uc.compare(&orig_slice, &tmp_slice)
            } else {
                bytewise_compare(&original_user, &tmp)
            }
        };

        if user_cmp >= 0 {
            return;
        }

        let packed = pack_sequence_and_type(MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
        let tag = encode_fixed64_le(packed);
        tmp.extend_from_slice(&tag);

        debug_assert!(
            self.compare_internal_bytes(&original_internal, &tmp) < 0,
            "InternalKeyComparator::find_short_successor: new key is not > old key"
        );

        k.clear();
        k.extend_from_slice(&tmp);
    }
}

impl FindShortestSeparator for InternalKeyComparator {
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
        trace!(
            "InternalKeyComparator::find_shortest_separator: start_len={}, limit_len={}",
            start.len(),
            limit.len()
        );
        if start.len() < 8 || limit.len() < 8 {
            return;
        }

        let original_start = start.clone();

        let start_user = &start[..start.len() - 8];
        let limit_user_slice = unsafe {
            let limit_slice = Slice::from_ptr_len(limit.as_ptr(), limit.len());
            extract_user_key(&limit_slice)
        };
        let limit_user_bytes = slice_as_bytes(&limit_user_slice);

        let mut tmp = start_user.to_vec();

        unsafe {
            if !self.user_comparator.is_null() {
                let uc = &*self.user_comparator;
                uc.find_shortest_separator(&mut tmp, limit_user_bytes);
            } else if let Some(shortened) =
                shorten_separator_user_keys(start_user, limit_user_bytes)
            {
                tmp = shortened;
            }
        }

        if tmp.len() >= start_user.len() {
            return;
        }

        let user_cmp_ok = unsafe {
            if !self.user_comparator.is_null() {
                let uc = &*self.user_comparator;
                let start_slice =
                    Slice::from_ptr_len(start_user.as_ptr(), start_user.len());
                let tmp_slice = Slice::from_ptr_len(tmp.as_ptr(), tmp.len());
                uc.compare(&start_slice, &tmp_slice) < 0
            } else {
                bytewise_compare(start_user, &tmp) < 0
            }
        };

        if !user_cmp_ok {
            return;
        }

        let mut internal_tmp = tmp;
        let packed = pack_sequence_and_type(MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
        let tag = encode_fixed64_le(packed);
        internal_tmp.extend_from_slice(&tag);

        debug_assert!(
            self.compare_internal_bytes(&original_start, &internal_tmp) < 0,
            "InternalKeyComparator::find_shortest_separator: new start is not > original"
        );
        debug_assert!(
            self.compare_internal_bytes(&internal_tmp, limit) < 0,
            "InternalKeyComparator::find_shortest_separator: new start is not < limit"
        );

        start.clear();
        start.extend_from_slice(&internal_tmp);
    }
}

#[cfg(test)]
mod internal_key_comparator_tests {
    use super::*;

    fn build_internal_key_bytes(user: &str, seq: SequenceNumber) -> Vec<u8> {
        let mut s = String::new();
        unsafe {
            let user_slice = Slice::from_ptr_len(user.as_ptr(), user.len());
            let parsed =
                ParsedInternalKey::new(&user_slice, &seq, ValueType::TypeValue);
            append_internal_key(&mut s as *mut String, &parsed);
        }
        s.as_bytes().to_vec()
    }

    #[traced_test]
    fn internal_key_comparator_orders_by_user_key_then_sequence() {
        let seq_low: SequenceNumber = 100;
        let seq_high: SequenceNumber = 200;

        let a1 = build_internal_key_bytes("a", seq_low);
        let b1 = build_internal_key_bytes("b", seq_low);

        let x_low = build_internal_key_bytes("x", seq_low);
        let x_high = build_internal_key_bytes("x", seq_high);

        let icmp = InternalKeyComparator::new(null_slice_comparator());

        unsafe {
            let a_slice =
                Slice::from_ptr_len(a1.as_ptr(), a1.len());
            let b_slice =
                Slice::from_ptr_len(b1.as_ptr(), b1.len());

            let x_low_slice =
                Slice::from_ptr_len(x_low.as_ptr(), x_low.len());
            let x_high_slice =
                Slice::from_ptr_len(x_high.as_ptr(), x_high.len());

            // User keys: "a" < "b"
            let r1 = icmp.compare(&a_slice, &b_slice);
            let r2 = icmp.compare(&b_slice, &a_slice);
            assert!(
                r1 < 0 && r2 > 0,
                "comparator must order by user key when different"
            );

            // Same user key "x", different sequence numbers:
            // higher sequence must sort *before* lower sequence.
            let r3 = icmp.compare(&x_high_slice, &x_low_slice);
            let r4 = icmp.compare(&x_low_slice, &x_high_slice);
            assert!(
                r3 < 0 && r4 > 0,
                "comparator must order by decreasing sequence for same user key"
            );
        }
    }

    #[traced_test]
    fn internal_key_comparator_with_non_null_user_comparator_behaves_consistently() {

        /// Comparator that operates on *user keys* (not internal keys).
        /// It uses the same bytewise ordering and shortening helpers as the main crate.
        struct UserKeyComparatorForTests;

        impl Named for UserKeyComparatorForTests {
            fn name(&self) -> std::borrow::Cow<'_, str> {
                std::borrow::Cow::Borrowed("test.UserKeyComparatorForTests")
            }
        }

        impl Compare for UserKeyComparatorForTests {
            fn compare(&self, a: &Slice, b: &Slice) -> i32 {
                let a_bytes = slice_as_bytes(a);
                let b_bytes = slice_as_bytes(b);
                bytewise_compare(a_bytes, b_bytes)
            }
        }

        impl FindShortestSeparator for UserKeyComparatorForTests {
            fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
                if let Some(shortened) = shorten_separator_user_keys(start, limit) {
                    *start = shortened;
                }
            }
        }

        impl FindShortSuccessor for UserKeyComparatorForTests {
            fn find_short_successor(&self, key: &mut Vec<u8>) {
                let _ = find_short_successor_user_key(key);
            }
        }

        impl SliceComparator for UserKeyComparatorForTests {
            fn bytewise_comparator(&self) -> *const (dyn SliceComparator + 'static) {
                self as *const _ as *const (dyn SliceComparator + 'static)
            }
        }

        /// Build a *valid internal key* (user || tag) using the same machinery as the main code.
        fn make_internal_key_for_tests(
            user: &[u8],
            seq: SequenceNumber,
            vt: ValueType,
        ) -> Vec<u8> {
            trace!(
                "make_internal_key_for_tests: user={:?}, seq={}, vt={:?}",
                String::from_utf8_lossy(user),
                seq,
                vt
            );

            let mut encoded = String::new();

            unsafe {
                let user_slice = Slice::from_ptr_len(user.as_ptr(), user.len());
                let parsed = ParsedInternalKey::new(&user_slice, &seq, vt);
                append_internal_key(&mut encoded as *mut String, &parsed);
            }

            let bytes = encoded.as_bytes().to_vec();
            debug!(
                "make_internal_key_for_tests: internal_len={}, bytes={:?}",
                bytes.len(),
                bytes
            );
            bytes
        }


        trace!("BEGIN internal_key_comparator_with_non_null_user_comparator_behaves_consistently");

        // User-level comparator (operates on *user* keys).
        let user_cmp = UserKeyComparatorForTests;
        let user_cmp_ptr: *const dyn SliceComparator = &user_cmp;

        // InternalKeyComparator that delegates user-key comparisons to our user comparator.
        let icmp = InternalKeyComparator::new(user_cmp_ptr);

        //
        // 1. User ordering must be preserved when sequences are equal.
        //
        let user_a = b"foo";
        let user_b = b"foz";
        let seq_equal: SequenceNumber = 100;

        unsafe {
            let ua_slice = Slice::from_ptr_len(user_a.as_ptr(), user_a.len());
            let ub_slice = Slice::from_ptr_len(user_b.as_ptr(), user_b.len());

            let user_order = user_cmp.compare(&ua_slice, &ub_slice);
            debug!(
                "user comparator order for 'foo' vs 'foz' = {}",
                user_order
            );
            assert!(
                user_order < 0,
                "expected 'foo' < 'foz' at user comparator level"
            );
        }

        let internal_a = make_internal_key_for_tests(user_a, seq_equal, ValueType::TypeValue);
        let internal_b = make_internal_key_for_tests(user_b, seq_equal, ValueType::TypeValue);

        unsafe {
            let ia_slice = Slice::from_ptr_len(internal_a.as_ptr(), internal_a.len());
            let ib_slice = Slice::from_ptr_len(internal_b.as_ptr(), internal_b.len());

            let internal_order = icmp.compare(&ia_slice, &ib_slice);
            debug!(
                "internal comparator order for 'foo' vs 'foz' (same seq) = {}",
                internal_order
            );

            assert!(
                internal_order < 0,
                "internal comparator must respect user comparator ordering when sequences are equal"
            );
        }

        //
        // 2. For identical user keys, sequence number must break ties in *descending* order.
        //
        let user_same = b"foo";
        let seq_high: SequenceNumber = 200;
        let seq_low: SequenceNumber = 100;

        let internal_high =
            make_internal_key_for_tests(user_same, seq_high, ValueType::TypeValue);
        let internal_low =
            make_internal_key_for_tests(user_same, seq_low, ValueType::TypeValue);

        unsafe {
            let ih_slice = Slice::from_ptr_len(internal_high.as_ptr(), internal_high.len());
            let il_slice = Slice::from_ptr_len(internal_low.as_ptr(), internal_low.len());

            let seq_order = icmp.compare(&ih_slice, &il_slice);
            debug!(
                "internal comparator order for same user, seq_high={} vs seq_low={} = {}",
                seq_high,
                seq_low,
                seq_order
            );

            assert!(
                seq_order < 0,
                "higher sequence must sort before lower sequence for the same user key"
            );

            let seq_order_reverse = icmp.compare(&il_slice, &ih_slice);
            assert!(
                seq_order_reverse > 0,
                "lower sequence must sort after higher sequence for the same user key"
            );
        }

        trace!("END internal_key_comparator_with_non_null_user_comparator_behaves_consistently");
    }
}
