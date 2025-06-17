// ---------------- [ File: bitcoin-serialize/src/meta.rs ]
crate::ix!();

pub struct If<const B: bool>;

pub trait True { }

impl True for If<true> { }

pub const fn inclusive_range_1_to_8<const Bytes: i32>() -> bool {
    Bytes > 0 && Bytes <= 8 
}
