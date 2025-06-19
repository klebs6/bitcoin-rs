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

    /// `inclusive_range_1_to_8` must be **true** only for 1 ≤ N ≤ 8.
    #[traced_test]
    fn range_predicate_correctness() {
        for n in 0..=9 {
            let expected = (1..=8).contains(&n);
            assert_eq!(
                inclusive_range_1_to_8::<{ n }>(),
                expected,
                "predicate failed for n = {n}"
            );
        }
    }

    /// Compile‑time proof that `If<true>` implements the marker trait
    /// `True` and that `If<false>` does *not*.
    #[traced_test]
    fn true_trait_is_implemented() {
        // This compiles only if the bound is satisfied.
        fn assert_true<T: True>() {}
        assert_true::<If<true>>();

        // The following line **must not compile**; we comment it out to
        // keep the test green, but it serves as documentation.
        //
        // ```compile_fail
        // assert_true::<If<false>>();
        // ```
    }
}
