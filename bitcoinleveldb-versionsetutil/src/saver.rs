// ---------------- [ File: bitcoinleveldb-versionsetutil/src/saver.rs ]
crate::ix!();

/**
  | Callback from TableCache::Get()
  |
  */
#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
pub enum SaverState {
    NotFound,
    Found,
    Deleted,
    Corrupt,
}

#[derive(Builder)]
#[builder(pattern="owned")]
pub struct Saver {
    state:    SaverState,
    ucmp:     Box<dyn SliceComparator>,
    user_key_: Slice,
    value:    *mut String,
}

impl core::fmt::Debug for Saver {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Saver")
            .field("state", &self.state)
            .field("user_key*", &self.user_key_)
            .field("value", &self.value)
            .finish()
    }
}

impl Saver {
    pub fn user_key(&self) -> &Slice {
        &self.user_key_
    }

    pub fn state(&self) -> SaverState {
        self.state
    }
}

/// Callback from TableCache::Get()
pub fn save_value(arg: *mut c_void, ikey_: &Slice, v: &Slice) -> c_void {
    unsafe {
        trace!("save_value: invoked");

        assert!(
            !arg.is_null(),
            "save_value: arg pointer must not be null"
        );

        let s: &mut Saver = &mut *(arg as *mut Saver);

        let mut parsed_key = ParsedInternalKey::default();

        let ok = parse_internal_key(ikey_, &mut parsed_key as *mut ParsedInternalKey);

        if ok {
            let user_key = parsed_key.user_key();
            let cmp      = s.ucmp.compare(user_key, &s.user_key_);

            trace!(
                parsed_user_key_len = *user_key.size(),
                saver_user_key_len  = *s.user_key_.size(),
                cmp,
                "save_value: user key comparison"
            );

            if cmp == 0 {
                let value_type = *parsed_key.ty();
                s.state = if value_type == ValueType::TypeValue {
                    SaverState::Found
                } else {
                    SaverState::Deleted
                };

                match s.state {
                    SaverState::Found => {
                        if !s.value.is_null() {
                            let out: &mut String = &mut *s.value;
                            let len              = *v.size();
                            let data_ptr         = *v.data();
                            let bytes =
                                std::slice::from_raw_parts(data_ptr, len);

                            debug!(
                                value_len = len,
                                "save_value: writing value bytes into destination string"
                            );

                            // Preserve raw bytes exactly as in C++ std::string::assign.
                            let new_string =
                                String::from_utf8_unchecked(bytes.to_vec());
                            *out = new_string;
                        } else {
                            debug!(
                                "save_value: state=Found but destination value pointer is null; skipping write"
                            );
                        }
                    }
                    SaverState::Deleted => {
                        debug!(
                            "save_value: user key matched but value marked as Deleted"
                        );
                    }
                    _ => { /* Not reachable here */ }
                }
            }
        } else {
            debug!(
                "save_value: parse_internal_key failed; marking state=Corrupt"
            );
            s.state = SaverState::Corrupt;
        }
    }
    unsafe { core::mem::zeroed::<c_void>() }
}

#[cfg(test)]
mod saver_callback_spec {
    use super::*;

    fn build_internal_key(user_key_str: &str, seq: u64, ty: ValueType) -> (InternalKey, Slice) {
        let user_slice = Slice::from(user_key_str);
        let ik = InternalKey::new(&user_slice, seq, ty);
        let encoded = ik.encode();
        (ik, encoded)
    }

    fn make_saver(user_key: &str, value_out: &mut String) -> Saver {
        let user_slice = Slice::from(user_key);
        Saver {
            state: SaverState::NotFound,
            ucmp: Box::new(BytewiseComparatorImpl::default()),
            user_key_: user_slice,
            value: value_out as *mut String,
        }
    }

    #[traced_test]
    fn verify_save_value_marks_corrupt_on_bad_internal_key_encoding() {
        let mut out = String::new();
        let mut saver = make_saver("foo", &mut out);

        // Build an ikey Slice that is too short (<8 bytes) so that parse_internal_key fails.
        let bad_bytes = [0x01u8, 0x02, 0x03, 0x04];
        let ikey = Slice::from(&bad_bytes[..]);
        let value_bytes = Slice::from("value");

        trace!("verify_save_value_marks_corrupt_on_bad_internal_key_encoding: invoking");
        save_value(&mut saver as *mut Saver as *mut c_void, &ikey, &value_bytes);

        match saver.state {
            SaverState::Corrupt => {}
            other => panic!(
                "Expected SaverState::Corrupt, got {:?}",
                other
            ),
        }
    }

    #[traced_test]
    fn verify_save_value_does_not_modify_state_for_mismatched_user_key() {
        let mut out = String::from("original");
        let mut saver = make_saver("expected", &mut out);

        let (_k, encoded) = build_internal_key("different", 1, ValueType::TypeValue);
        let v_slice = Slice::from("new-value");

        save_value(&mut saver as *mut Saver as *mut c_void, &encoded, &v_slice);

        assert!(
            matches!(saver.state, SaverState::NotFound),
            "Saver state should remain NotFound when user keys do not match"
        );
        assert_eq!(
            "original",
            out,
            "Output string should remain unchanged when keys do not match"
        );
    }

    #[traced_test]
    fn verify_save_value_stores_value_on_matching_typevalue() {
        let mut out = String::new();
        let mut saver = make_saver("user", &mut out);

        let (_k, encoded) = build_internal_key("user", 7, ValueType::TypeValue);
        let v_slice = Slice::from("stored-value");

        save_value(&mut saver as *mut Saver as *mut c_void, &encoded, &v_slice);

        match saver.state {
            SaverState::Found => {}
            other => panic!("Expected SaverState::Found, got {:?}", other),
        }

        debug!(
            stored = out.as_str(),
            "verify_save_value_stores_value_on_matching_typevalue: stored value"
        );

        assert_eq!(
            "stored-value",
            out,
            "Saver must copy the raw bytes of the supplied value"
        );
    }

    #[traced_test]
    fn verify_save_value_sets_deleted_state_without_writing_value() {
        let mut out = String::from("prior");
        let mut saver = make_saver("user", &mut out);

        let (_k, encoded) = build_internal_key("user", 7, ValueType::TypeDeletion);
        let v_slice = Slice::from("ignored-value");

        save_value(&mut saver as *mut Saver as *mut c_void, &encoded, &v_slice);

        match saver.state {
            SaverState::Deleted => {}
            other => panic!("Expected SaverState::Deleted, got {:?}", other),
        }

        assert_eq!(
            "prior",
            out,
            "Value string must not be modified when the entry represents a deletion"
        );
    }
}
