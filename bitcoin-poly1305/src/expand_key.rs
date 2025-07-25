// ---------------- [ File: bitcoin-poly1305/src/expand_key.rs ]
crate::ix!();

// -----------------------------------------------------------------------------
// [poly1305] key expansion & clamping
// -----------------------------------------------------------------------------
pub type LimbArr5 = [u32; 5];
pub type LimbArr4 = [u32; 4];

/// Expand the 32‑byte *r*‖*s* key into *r₀…r₄* plus the
/// “ℓ = r · 5” helpers *s₁…s₄* (Donna naming).  
/// Returns `(r, s)` where  
///  * `r` – five 26‑bit limbs  
///  * `s` – `r[1..=4] * 5`
#[inline(always)]
pub fn expand_key(key: &[u8; POLY1305_KEYLEN]) -> (LimbArr5, LimbArr4) {
    tracing::trace!(key = ?key, "expand_key: start");

    // raw little‑endian words
    let mut t0 = read_le32(&key[0..]);
    let mut t1 = read_le32(&key[4..]);
    let mut t2 = read_le32(&key[8..]);
    let mut t3 = read_le32(&key[12..]);
    tracing::debug!(t0, t1, t2, t3, "expand_key: raw 32‑bit words");

    // clamp into five 26‑bit limbs
    let mut r = [0u32; 5];
    r[0] = t0 & 0x3ffffff;
    t0 = (t0 >> 26) | (t1 << 6);
    r[1] = t0 & 0x3ffff03;
    t1 = (t1 >> 20) | (t2 << 12);
    r[2] = t1 & 0x3ffc0ff;
    t2 = (t2 >> 14) | (t3 << 18);
    r[3] = t2 & 0x3f03fff;
    t3 >>= 8;
    r[4] = t3 & 0x00fffff;
    tracing::debug!(?r, "expand_key: clamped r‑limbs");

    // multipliers r[i] * 5
    let s = [r[1] * 5, r[2] * 5, r[3] * 5, r[4] * 5];
    tracing::debug!(?s, "expand_key: s‑multipliers (r[i] * 5)");

    (r, s)
}

#[cfg(test)]
mod tests_key_expansion {
    use super::*;
    use hex_literal::hex;
    use proptest::prelude::*;

    #[traced_test]
    fn rfc7539_key_clamp_values() {
        let key = hex!("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
        let (r, s) = expand_key(&key);
        assert_eq!(r, [12506757, 55923970, 4702262, 16791881, 526037]);
        assert_eq!(s, [279_619_850, 23_511_310, 83_959_405, 2_630_185]);
    }

    /// Random‑key sanity: every limb < 2²⁶ (spec guarantee).
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(350))]

        #[traced_test]
        fn limb_range_only(key in proptest::array::uniform32(any::<u8>())) {
            let (r, _) = expand_key(&key);
            for &limb in &r {
                prop_assert!(limb < (1 << 26));
            }
        }
    }
}
