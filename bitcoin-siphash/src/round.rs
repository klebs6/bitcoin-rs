// ---------------- [ File: bitcoin-siphash/src/round.rs ]
crate::ix!();

/// Rotate‑left for 64‑bit values.  
/// Kept as a `const fn` so the compiler can evaluate
/// it at compile‑time whenever the arguments are constant.
#[inline(always)]
pub const fn rotl64(x: u64, b: u32) -> u64 {
    (x << b) | (x >> (64 - b))
}


/// Equivalent to the original C++ `ROTL` macro
/// from Bitcoin Core (wrap‑around semantics).
#[macro_export]
macro_rules! rotl {
    ($x:expr, $b:expr) => {
        ($x).rotate_left($b)
    };
}

/// SipHash compression round (`SIPROUND`) exactly as in the
/// Bitcoin Core reference implementation.
///
/// Instead of assuming `v0 … v3` are in scope, the identifiers are
/// now passed in as macro arguments, restoring proper macro hygiene.
#[macro_export]
macro_rules! sipround {
    ($v0:ident, $v1:ident, $v2:ident, $v3:ident) => {{
        ::tracing::trace!(
            "enter SIPROUND: {:016x} {:016x} {:016x} {:016x}",
            $v0, $v1, $v2, $v3
        );

        $v0 = $v0.wrapping_add($v1); $v1 = $v1.rotate_left(13); $v1 ^= $v0;
        $v0 = $v0.rotate_left(32);
        $v2 = $v2.wrapping_add($v3); $v3 = $v3.rotate_left(16); $v3 ^= $v2;
        $v0 = $v0.wrapping_add($v3); $v3 = $v3.rotate_left(21); $v3 ^= $v0;
        $v2 = $v2.wrapping_add($v1); $v1 = $v1.rotate_left(17); $v1 ^= $v2;
        $v2 = $v2.rotate_left(32);

        ::tracing::trace!(
            "exit  SIPROUND: {:016x} {:016x} {:016x} {:016x}",
            $v0, $v1, $v2, $v3
        );
    }};
}

#[cfg(test)]
mod siphash_round_tests {
    use super::*;

    /// Ensures a single round matches the known‑good state
    /// from Bitcoin Core (SipHash reference vectors).
    #[traced_test]
    fn single_round_matches_reference() {
        let mut v0: u64 = 0x736f_6d65_7073_6575;
        let mut v1: u64 = 0x646f_7261_6e64_6f6d;
        let mut v2: u64 = 0x6c79_6765_6e65_7261;
        let mut v3: u64 = 0x7465_6462_7974_6573;

        info!("initial {:016x} {:016x} {:016x} {:016x}", v0, v1, v2, v3);

        sipround!(v0, v1, v2, v3);

        info!("final   {:016x} {:016x} {:016x} {:016x}", v0, v1, v2, v3);

        assert_eq!(v0, 0x6394_8796_5a89_8377);
        assert_eq!(v1, 0xfe65_1ba6_cbca_2366);
        assert_eq!(v2, 0x3b14_5043_1a71_bdd2);
        assert_eq!(v3, 0x35e4_d2c2_2cb9_14e1);
    }
}
