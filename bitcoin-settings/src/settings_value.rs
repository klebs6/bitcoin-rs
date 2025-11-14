// ---------------- [ File: bitcoin-settings/src/settings_value.rs ]
crate::ix!();

/**
  | Settings value type
  | (string/integer/boolean/null variant).
  |
  | @note UniValue is used here for convenience
  |       and because it can be easily serialized
  |       in a readable format. But any other
  |       variant type that can be assigned
  |       strings, int64_t, and bool values and
  |       has get_str(), get_int64(), get_bool(),
  |       isNum(), isBool(), isFalse(), isTrue()
  |       and
  |       isNull() methods can be substituted if
  |       there's a need to move away from
  |       UniValue. (An implementation with
  |       boost::variant was posted at
  |       https://github.com/bitcoin/bitcoin/pull/15934/files#r337691812)
  */
#[derive(Debug,Clone)]
pub struct SettingsValue(pub UniValue);

impl SettingsValue {
    /// Returns `true` when the wrapped UniValue is a JSON boolean **false**.
    #[inline]
    pub fn is_false(&self) -> bool {
        self.0.is_bool() && !self.0.get_bool()
    }
}

pub fn sv_json(j: &str) -> SettingsValue {
    // (get_setting_behavior_spec) helper
    let mut u = UniValue::null();
    let raw = j.as_bytes();
    assert!(u.read(raw.as_ptr(), raw.len()), "Invalid JSON literal for UniValue");
    SettingsValue(u)
}

impl From<bool> for SettingsValue {
    fn from(val: bool) -> Self {
        Self(UniValue::from(val))
    }
}

impl PartialEq<SettingsValue> for SettingsValue {
    fn eq(&self, other: &SettingsValue) -> bool {
        // Equality follows the C++ semantics: compare the serialized form.
        let self_str  = self.0.write(None, None);   // ← specify both options
        let other_str = other.0.write(None, None);  // ← specify both options
        trace!(
            "Comparing SettingsValue: self='{self_str}', other='{other_str}'"
        );
        self_str == other_str
    }
}

impl Eq for SettingsValue {}

impl fmt::Display for SettingsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let written = self.0.write(None, None);  // ← specify both options
        trace!("Formatting SettingsValue: '{written}'");
        write!(f, "{written}")
    }
}


#[cfg(test)]
mod settings_value_equality_semantics_spec {
    use super::*;

    #[traced_test]
    fn equality_is_based_on_serialized_json() {
        info!("Two values with identical serialized JSON should be equal");
        let a = sv_json("1"); // parsed as numeric JSON
        let mut u = UniValue::default();
        u.set_int(1i64);
        let b = SettingsValue(u);
        assert_eq!(a, b);

        info!("Different JSON serializations are not equal, even if numerically similar");
        let one        = sv_json("1");
        let one_point0 = sv_json("1.0");
        assert_ne!(one, one_point0);

        info!("Structural equality for arrays/objects is based on write(None, None)");
        let arr1 = sv_json("[1,true,\"x\"]");
        let arr2 = sv_json("[1,true,\"x\"]");
        assert_eq!(arr1, arr2);
    }
}
