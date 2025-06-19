// ---------------- [ File: bitcoin-chacha/src/quarterround.rs ]
crate::ix!();

/**
  | Based on the public domain implementation
  | 'merged' by D. J. Bernstein See
  | https://cr.yp.to/chacha.html.
  */
#[macro_export]
macro_rules! quarterround {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {{
        $a = $a.wrapping_add($b); $d ^= $a; $d = rotl32($d, 16);
        $c = $c.wrapping_add($d); $b ^= $c; $b = rotl32($b, 12);
        $a = $a.wrapping_add($b); $d ^= $a; $d = rotl32($d,  8);
        $c = $c.wrapping_add($d); $b ^= $c; $b = rotl32($b,  7);
    }};
}

#[inline]
pub fn rotl32(v: u32, c: i32) -> u32 {
    v.rotate_left(c as u32)
}

#[cfg(test)]
mod quarterround_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn quarterround_matches_reference() {
        // RFC 8439 §2.1.1 test
        let (mut a, mut b, mut c, mut d) =
            (0x11111111_u32, 0x01020304_u32, 0x9b8d6f43_u32, 0x01234567_u32);
        quarterround!(a, b, c, d);
        assert_eq!(a, 0xea2a_92f4);
        assert_eq!(b, 0xcb1c_f8ce);
        assert_eq!(c, 0x4581_472e);
        assert_eq!(d, 0x5881_c4bb);
    }
}
