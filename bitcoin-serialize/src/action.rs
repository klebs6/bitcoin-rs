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
    /// `BtcUnserialize`‐phase marker – always **true** because we are _reading_.
    #[inline]
    pub fn for_read(&self) -> bool {
        trace!("SerActionUnserialize::for_read -> true");
        true
    }
}

#[cfg(test)]
mod action_tests {
    use super::*;
    use std::mem;

    /// `SerActionSerialize::for_read()` must always be **false** and the
    /// type must be ZST (zero‑sized).
    #[traced_test]
    fn serialize_marker_properties() {
        let marker = SerActionSerialize {};
        assert!(!marker.for_read(), "Serialize marker incorrectly reports read phase");
        assert_eq!(mem::size_of::<SerActionSerialize>(), 0, "Serialize marker should be ZST");
    }

    /// `SerActionUnserialize::for_read()` must always be **true** and the
    /// type must be ZST.
    #[traced_test]
    fn unserialize_marker_properties() {
        let marker = SerActionUnserialize {};
        assert!(marker.for_read(), "BtcUnserialize marker incorrectly reports write phase");
        assert_eq!(mem::size_of::<SerActionUnserialize>(), 0, "BtcUnserialize marker should be ZST");
    }
}
