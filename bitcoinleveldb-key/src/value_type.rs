// ---------------- [ File: bitcoinleveldb-key/src/value_type.rs ]
crate::ix!();

/**
  | Value types encoded as the last component of
  | internal keys.
  |
  | DO NOT CHANGE THESE ENUM VALUES: they are
  | embedded in the on-disk data structures.
  */
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueType {
    TypeDeletion = 0x0,
    TypeValue    = 0x1,
}

impl ValueType {
    #[inline]
    pub fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0x0 => Some(ValueType::TypeDeletion),
            0x1 => Some(ValueType::TypeValue),
            _ => None,
        }
    }
}

/**
  | VALUE_TYPE_FOR_SEEK defines the ValueType that
  | should be passed when constructing
  | a ParsedInternalKey object for seeking to
  | a particular sequence number (since we sort
  | sequence numbers in decreasing order and the
  | value type is embedded as the low 8 bits in the
  | sequence number in internal keys, we need to
  | use the highest-numbered ValueType, not the
  | lowest).
  */
pub const VALUE_TYPE_FOR_SEEK: ValueType = ValueType::TypeValue;
