// ---------------- [ File: bitcoin-string/src/timing_resistant_equal.rs ]
crate::ix!();

/// Timing‑attack‑resistant comparison.
///
/// The running time is proportional to the length of **`a`** only,
/// which thwarts simple timing side‑channel attacks.
///
/// The function accepts anything that can be borrowed as a byte‑slice
/// (`&[u8]`) so it works for `Vec<u8>`, `&[u8]`, fixed‑size arrays,
/// `String`, `&str`, etc.
pub fn timing_resistant_equal<T>(a: &T, b: &T) -> bool
where
    T: AsRef<[u8]> + ?Sized,
{
    let a_bytes = a.as_ref();
    let b_bytes = b.as_ref();

    // Short‑circuit for the most divergent case.
    if b_bytes.is_empty() {
        trace!(
            target: "timing_resistant_equal",
            a_len = a_bytes.len(),
            b_len = 0,
            "b is empty – result = {}",
            a_bytes.is_empty()
        );
        return a_bytes.is_empty();
    }

    // Step one: accumulate the length difference.
    let mut acc: usize = a_bytes.len() ^ b_bytes.len();

    // Step two: walk *only* over `a` so timing is length‑dependent on `a`.
    for (i, &byte_a) in a_bytes.iter().enumerate() {
        let byte_b = b_bytes[i % b_bytes.len()];
        acc |= (byte_a ^ byte_b) as usize;
    }

    let result = acc == 0;
    debug!(
        target: "timing_resistant_equal",
        a_len = a_bytes.len(),
        b_len = b_bytes.len(),
        equal = result,
        "comparison complete"
    );
    result
}

#[cfg(test)]
mod timing_and_integral_tests {
    use super::*;

    // ---------- timing_resistant_equal ----------

    #[traced_test]
    fn equal_byte_slices_match() {
        let lhs = b"bitcoin";
        let rhs = b"bitcoin";
        assert!(timing_resistant_equal(&lhs[..], &rhs[..]));
    }

    #[traced_test]
    fn differing_byte_slices_do_not_match() {
        let lhs = b"bitcoin";
        let rhs = b"bitco1n";
        assert!(!timing_resistant_equal(&lhs[..], &rhs[..]));
    }

    #[traced_test]
    fn unequal_lengths_do_not_match() {
        let lhs = b"abcde";
        let rhs = b"abc";
        assert!(!timing_resistant_equal(&lhs[..], &rhs[..]));
    }

    #[traced_test]
    fn empty_vs_empty_match() {
        let empty: &[u8] = b"";
        assert!(timing_resistant_equal(&empty, &empty));
    }

    #[traced_test]
    fn nonempty_vs_empty_do_not_match() {
        let lhs = b"non-empty";
        let rhs: &[u8] = b"";
        assert!(!timing_resistant_equal(&lhs[..], &rhs));
    }

    // ---------- to_integral ----------

    #[traced_test]
    fn signed_positive_parses() {
        assert_eq!(to_integral::<i32>("42"), Some(42));
    }

    #[traced_test]
    fn signed_negative_parses() {
        assert_eq!(to_integral::<i32>("-42"), Some(-42));
    }

    #[traced_test]
    fn unsigned_negative_rejected() {
        assert_eq!(to_integral::<u32>("-1"), None);
    }

    #[traced_test]
    fn leading_plus_rejected() {
        assert_eq!(to_integral::<i64>("+5"), None);
    }

    #[traced_test]
    fn leading_whitespace_rejected() {
        assert_eq!(to_integral::<i32>(" 7"), None);
    }

    #[traced_test]
    fn trailing_whitespace_rejected() {
        assert_eq!(to_integral::<i32>("7 "), None);
    }

    #[traced_test]
    fn trailing_garbage_rejected() {
        assert_eq!(to_integral::<i32>("99bottles"), None);
    }

    #[traced_test]
    fn out_of_range_rejected() {
        // 2^63 does not fit into i64.
        let gigantic = "9223372036854775808";
        assert_eq!(to_integral::<i64>(gigantic), None);
    }
}
