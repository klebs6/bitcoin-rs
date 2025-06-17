// ---------------- [ File: bitcoin-serialize/src/action.rs ]
crate::ix!();

/**
  | Support for SERIALIZE_METHODS and
  | READWRITE macro.
  |
  */
pub struct SerActionSerialize { }

impl SerActionSerialize {
    /// `Serialize`‐phase marker – always **false** because we are _writing_.
    #[inline]
    pub fn for_read(&self) -> bool {
        trace!("SerActionSerialize::for_read -> false");
        false
    }
}

pub struct SerActionUnserialize { }

impl SerActionUnserialize {
    /// `Unserialize`‐phase marker – always **true** because we are _reading_.
    #[inline]
    pub fn for_read(&self) -> bool {
        trace!("SerActionUnserialize::for_read -> true");
        true
    }
}
