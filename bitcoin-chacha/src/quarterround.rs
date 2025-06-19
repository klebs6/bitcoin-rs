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
