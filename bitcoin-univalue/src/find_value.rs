// ---------------- [ File: bitcoin-univalue/src/find_value.rs ]
crate::ix!();

/// Immutable lookup in a JSON object.  
/// Returns a reference to `NULL_UNI_VALUE` when the key is
/// absent or when *obj* is not an object.
#[instrument(level = "trace", skip_all)]
pub fn find_value<'a>(obj: &'a UniValue, name: &'a str) -> &'a UniValue {
    if *obj.typ() != uni_value::VType::VOBJ {
        return &NULL_UNI_VALUE;
    }

    for (idx, key) in obj.keys().iter().enumerate() {
        if key == name {
            trace!(found = true, key);
            return &obj.values()[idx];
        }
    }

    trace!(found = false, key = name);
    &NULL_UNI_VALUE
}

/// Mutable lookup variant.  
/// Upstream returns a *const* global when the key is missing.
/// Re‑creating that semantics in Rust requires `unsafe` as we
/// must coerce an immutable static to `&mut`.  The operation is
/// gated under an `allow` so we can compile with
/// `#![deny(invalid_reference_casting)]` elsewhere.
#[instrument(level = "trace", skip_all)]
pub fn find_value_mut<'a>(obj: &'a mut UniValue, name: &'a str) -> &'a mut UniValue {
    if *obj.typ() == uni_value::VType::VOBJ {
        for (idx, key) in obj.keys().iter().enumerate() {
            if key == name {
                trace!(found = true, key);
                return &mut obj.values_mut()[idx];
            }
        }
    }

    trace!(found = false, key = name);
    // SAFETY: identical to the upstream C++ behaviour; callers
    // that mutate the returned **null** value do so knowingly.
    #[allow(invalid_reference_casting)]
    unsafe {
        &mut *(&*NULL_UNI_VALUE as *const UniValue as *mut UniValue)
    }
}

#[cfg(test)]
mod find_value_spec {
    use super::*;

    #[traced_test]
    fn finds_existing_key() {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        obj.keys_mut().push("a".into());
        obj.values_mut().push(42u64.into());

        assert_eq!(find_value(&obj, "a").get_int64(), 42);
        assert_eq!(find_value_mut(&mut obj, "a").get_int64(), 42);
    }

    #[traced_test]
    fn returns_null_for_missing_key() {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(find_value(&obj, "nope").is_null());
        assert!(find_value_mut(&mut obj, "nope").is_null());
    }
}
