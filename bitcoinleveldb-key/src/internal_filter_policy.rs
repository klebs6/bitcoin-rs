// ---------------- [ File: bitcoinleveldb-key/src/internal_filter_policy.rs ]
crate::ix!();

/**
  | Filter policy wrapper that converts
  | from internal keys to user keys
  |
  */
pub struct InternalFilterPolicy {
    user_policy: *const dyn FilterPolicy,
}

impl FilterPolicy for InternalFilterPolicy {}

impl Named for InternalFilterPolicy {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        trace!("InternalFilterPolicy::name");
        unsafe {
            if self.user_policy.is_null() {
                std::borrow::Cow::Borrowed("")
            } else {
                (*self.user_policy).name()
            }
        }
    }
}

impl CreateFilter for InternalFilterPolicy {
    fn create_filter(&self, keys: *const Slice, n: i32, dst: &mut Vec<u8>) {
        trace!(
            "InternalFilterPolicy::create_filter: n_keys={}",
            n
        );
        if n <= 0 || keys.is_null() {
            return;
        }

        unsafe {
            let mkeys = keys as *mut Slice;
            for i in 0..(n as isize) {
                let key_ptr = mkeys.offset(i);
                let orig = &*key_ptr;
                *key_ptr = extract_user_key(orig);
            }

            if !self.user_policy.is_null() {
                (*self.user_policy).create_filter(keys, n, dst);
            }
        }
    }
}

impl KeyMayMatch for InternalFilterPolicy {
    fn key_may_match(&self, k: &Slice, f: &Slice) -> bool {
        trace!("InternalFilterPolicy::key_may_match");
        unsafe {
            if self.user_policy.is_null() {
                return false;
            }
            let user_key = extract_user_key(k);
            (*self.user_policy).key_may_match(&user_key, f)
        }
    }
}

impl InternalFilterPolicy {
    pub fn new(p: *const dyn FilterPolicy) -> Self {
        trace!("InternalFilterPolicy::new: user_policy={:p}", p);
        InternalFilterPolicy { user_policy: p }
    }
}

#[cfg(test)]
mod internal_filter_policy_tests {
    use super::*;

    struct TestFilterPolicy;

    impl FilterPolicy for TestFilterPolicy {}

    impl Named for TestFilterPolicy {
        fn name(&self) -> std::borrow::Cow<'_, str> {
            std::borrow::Cow::Borrowed("test.filter")
        }
    }

    impl CreateFilter for TestFilterPolicy {
        fn create_filter(
            &self,
            keys: *const Slice,
            n: i32,
            dst: &mut Vec<u8>,
        ) {
            trace!(
                "TestFilterPolicy::create_filter: n={}",
                n
            );
            unsafe {
                for i in 0..n {
                    let s = &*keys.add(i as usize);
                    let bytes = slice_as_bytes(s);
                    dst.extend_from_slice(bytes);
                    dst.push(0xff); // delimiter to make assertions easier
                }
            }
        }
    }

    impl KeyMayMatch for TestFilterPolicy {
        fn key_may_match(
            &self,
            k: &Slice,
            f: &Slice,
        ) -> bool {
            let key_bytes = slice_as_bytes(k);
            let filter_bytes = slice_as_bytes(f);
            if key_bytes.is_empty() {
                return false;
            }
            filter_bytes
                .windows(key_bytes.len())
                .any(|w| w == key_bytes)
        }
    }

    fn build_internal_key_bytes(user: &str, seq: SequenceNumber) -> Vec<u8> {
        let mut s = String::new();
        unsafe {
            let user_slice = Slice::from_ptr_len(user.as_ptr(), user.len());
            let parsed =
                ParsedInternalKey::new(&user_slice, &seq, VALUE_TYPE_FOR_SEEK);
            append_internal_key(&mut s as *mut String, &parsed);
        }
        s.as_bytes().to_vec()
    }

    #[traced_test]
    fn internal_filter_policy_name_forwards_to_underlying_policy() {
        let policy = TestFilterPolicy;
        let policy_ref: &dyn FilterPolicy = &policy;
        let policy_ptr: *const dyn FilterPolicy =
            policy_ref as *const dyn FilterPolicy;

        let internal = InternalFilterPolicy::new(policy_ptr);
        let name = internal.name();
        assert_eq!(name, "test.filter");
    }

    #[traced_test]
    fn internal_filter_policy_converts_internal_keys_to_user_keys() {
        let user_keys = ["foo", "barbaz"];
        let seq: SequenceNumber = 42;

        let internal_data: Vec<Vec<u8>> = user_keys
            .iter()
            .map(|u| build_internal_key_bytes(u, seq))
            .collect();

        let mut slices: Vec<Slice> = internal_data
            .iter()
            .map(|v| unsafe { Slice::from_ptr_len(v.as_ptr(), v.len()) })
            .collect();

        let policy = TestFilterPolicy;
        let policy_ref: &dyn FilterPolicy = &policy;
        let policy_ptr: *const dyn FilterPolicy =
            policy_ref as *const dyn FilterPolicy;

        let internal_policy = InternalFilterPolicy::new(policy_ptr);

        let mut dst = Vec::new();
        internal_policy.create_filter(slices.as_ptr(), slices.len() as i32, &mut dst);

        // The underlying TestFilterPolicy writes user keys plus 0xff delimiters.
        let total_user_len: usize =
            user_keys.iter().map(|u| u.len()).sum::<usize>();
        assert_eq!(
            dst.len(),
            total_user_len + user_keys.len(),
            "filter length should equal sum of user key lengths plus delimiters"
        );

        // Verify that the slices in the array have been rewritten to user keys.
        for (i, s) in slices.iter().enumerate() {
            let bytes = slice_as_bytes(s);
            assert_eq!(
                bytes,
                user_keys[i].as_bytes(),
                "slice {} must be converted to user key",
                i
            );
        }
    }

    #[traced_test]
    fn internal_filter_policy_key_may_match_uses_extracted_user_key() {
        let user_keys = ["foo", "bar"];
        let seq: SequenceNumber = 7;

        let internal_data: Vec<Vec<u8>> = user_keys
            .iter()
            .map(|u| build_internal_key_bytes(u, seq))
            .collect();

        let mut slices: Vec<Slice> = internal_data
            .iter()
            .map(|v| unsafe { Slice::from_ptr_len(v.as_ptr(), v.len()) })
            .collect();

        let policy = TestFilterPolicy;
        let policy_ref: &dyn FilterPolicy = &policy;
        let policy_ptr: *const dyn FilterPolicy =
            policy_ref as *const dyn FilterPolicy;

        let internal_policy = InternalFilterPolicy::new(policy_ptr);

        let mut dst = Vec::new();
        internal_policy.create_filter(slices.as_ptr(), slices.len() as i32, &mut dst);

        unsafe {
            let filter_slice = Slice::from_ptr_len(dst.as_ptr(), dst.len());

            // Build internal key for "foo"
            let internal_foo = build_internal_key_bytes("foo", seq);
            let internal_foo_slice =
                Slice::from_ptr_len(internal_foo.as_ptr(), internal_foo.len());
            assert!(
                internal_policy.key_may_match(&internal_foo_slice, &filter_slice),
                "filter should match internal key for 'foo'"
            );

            // Build internal key for "baz" (not present)
            let internal_baz = build_internal_key_bytes("baz", seq);
            let internal_baz_slice =
                Slice::from_ptr_len(internal_baz.as_ptr(), internal_baz.len());
            assert!(
                !internal_policy.key_may_match(&internal_baz_slice, &filter_slice),
                "filter should not match internal key for 'baz'"
            );
        }
    }
}
