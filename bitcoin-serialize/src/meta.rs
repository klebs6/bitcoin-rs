// ---------------- [ File: bitcoin-serialize/src/meta.rs ]
crate::ix!();

pub struct If<const B: bool>;

pub trait True { }

impl True for If<true> { }

pub const fn inclusive_range_1_to_8<const Bytes: i32>() -> bool {
    Bytes > 0 && Bytes <= 8 
}

#[cfg(test)]
mod meta_tests {
    use super::*;

    #[traced_test]
    fn predicate_correct_for_0_to_9() {
        // Manual constant checks (cannot use `n` loop with const‑generics)
        assert!(!inclusive_range_1_to_8::<0>());
        assert!( inclusive_range_1_to_8::<1>());
        assert!( inclusive_range_1_to_8::<2>());
        assert!( inclusive_range_1_to_8::<3>());
        assert!( inclusive_range_1_to_8::<4>());
        assert!( inclusive_range_1_to_8::<5>());
        assert!( inclusive_range_1_to_8::<6>());
        assert!( inclusive_range_1_to_8::<7>());
        assert!( inclusive_range_1_to_8::<8>());
        assert!(!inclusive_range_1_to_8::<9>());
    }

    // Compile‑time check: `If<true>` implements `True`
    fn _compile_time_assert()
    where
        If<true>: True,
    {}
}
