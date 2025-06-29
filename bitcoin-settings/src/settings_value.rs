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
pub struct SettingsValue(pub UniValue);

impl PartialEq<SettingsValue> for SettingsValue {
    fn eq(&self, other: &SettingsValue) -> bool {
        // Equality follows the C++ semantics: compare the serialized form.
        let self_str = self.0.write();
        let other_str = other.0.write();
        trace!(
            "Comparing SettingsValue: self='{}', other='{}'",
            self_str,
            other_str
        );
        self_str == other_str
    }
}

impl Eq for SettingsValue {}

impl fmt::Display for SettingsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let written = self.0.write();
        trace!("Formatting SettingsValue: '{}'", written);
        write!(f, "{written}")
    }
}

impl SettingsValue {
    /// Returns `true` when the wrapped UniValue is a JSON boolean **false**.
    #[inline]
    pub fn is_false(&self) -> bool {
        self.0.isBool() && !self.0.get_bool()
    }
}

impl From<bool> for SettingsValue {
    fn from(val: bool) -> Self {
        Self(UniValue::from(val))
    }
}
