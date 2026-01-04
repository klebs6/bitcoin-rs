// ---------------- [ File: bitcoinleveldb-slice/src/lib.rs ]
#![allow(internal_features)]
#![feature(core_intrinsics)]

#[macro_use] mod imports; use imports::*;

x!{slice}

/// A range of keys
#[derive(Default)]
pub struct Range {

    /// Included in the range
    start: Slice,

    /// Not included in the range
    limit: Slice,
}

impl Range {

    pub fn new(
        s: Slice,
        l: Slice) -> Self {
        Self { start: s, limit: l }
    }
}
