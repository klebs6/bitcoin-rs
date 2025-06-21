// ---------------- [ File: bitcoin-string/src/check.rs ]
crate::ix!();

/// Tests if the given character byte represents an ASCII decimal digit (0‑9).
#[inline]
pub fn is_digit(c: u8) -> bool {
    let res = c >= b'0' && c <= b'9';
    trace!("is_digit({}) = {}", c, res);
    res
}

/// Tests if the given character byte is one of the ASCII whitespace characters:
/// space, form‑feed (`\f`), newline (`\n`), carriage return (`\r`),
/// horizontal tab (`\t`), or vertical tab (`\v`).
#[inline]
pub fn is_space(c: u8) -> bool {
    let res = matches!(c, b' ' | b'\n' | b'\r' | b'\t' | 0x0C /* '\f' */ | 0x0B /* '\v' */);
    trace!("is_space({}) = {}", c, res);
    res
}

#[cfg(test)]
mod tests_check {
    use super::*;

    #[traced_test]
    fn non_digits_are_rejected() {
        for c in [b'/', b':', b'A', b'z', b' '] {
            assert!(!is_digit(c));
        }
    }

    #[traced_test]
    fn whitespace_detection() {
        for &c in [b' ', b'\n', b'\r', b'\t', 0x0C, 0x0B].iter() {
            assert!(is_space(c));
        }
        for c in [b'a', b'0', b'#'] {
            assert!(!is_space(c));
        }
    }
}

