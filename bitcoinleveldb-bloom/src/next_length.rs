// ---------------- [ File: bitcoinleveldb-bloom/src/next_length.rs ]
crate::ix!();

pub fn next_length(length: i32) -> i32 {
    let mut len = length;

    if len < 10 {
        len += 1;
    } else if len < 100 {
        len += 10;
    } else if len < 1_000 {
        len += 100;
    } else {
        len += 1_000;
    }

    len
}

#[cfg(test)]
mod next_length_suite {
    use super::*;

    #[traced_test]
    fn next_length_increments_small_values_by_one() {
        assert_eq!(next_length(1), 2);
        assert_eq!(next_length(9), 10);
    }

    #[traced_test]
    fn next_length_increments_medium_values_by_ten() {
        assert_eq!(next_length(10), 20);
        assert_eq!(next_length(90), 100);
    }

    #[traced_test]
    fn next_length_increments_large_values_by_hundred_or_thousand() {
        assert_eq!(next_length(100), 200);
        assert_eq!(next_length(900), 1000);
        assert_eq!(next_length(1000), 2000);
    }
}
