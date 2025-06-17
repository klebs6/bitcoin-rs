crate::ix!();

#[inline]
pub fn align_up(x: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two(), "align must be a power of two");
    (x + align - 1) & !(align - 1)
}

// -----------------------------------------------------------------------------
// [bitcoin-support/src/lockedpool.rs] – unit tests for `align_up`
// -----------------------------------------------------------------------------
#[cfg(test)]
mod align_tests {
    use super::*;

    #[traced_test]
    fn test_align_up_powers_of_two() {
        assert_eq!(align_up(0, 8), 0);
        assert_eq!(align_up(1, 8), 8);
        assert_eq!(align_up(8, 8), 8);
        assert_eq!(align_up(9, 8), 16);

        assert_eq!(align_up(15, 16), 16);
        assert_eq!(align_up(17, 16), 32);
    }
}
