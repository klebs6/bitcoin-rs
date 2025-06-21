// ---------------- [ File: bitcoin-poly1305/src/multiply_and_reduce.rs ]
crate::ix!();

// -----------------------------------------------------------------------------
// [poly1305] multiply‑and‑carry (h *= r mod 2¹³⁰ − 5)
// -----------------------------------------------------------------------------
#[inline(always)]
pub fn multiply_and_reduce(h: &mut LimbArr5, r: &LimbArr5, s: &LimbArr4) {
    let h_in = *h;
    tracing::trace!(h_in = ?h_in, ?r, ?s, "multiply_and_reduce: start");

    // 64‑bit products before any carries
    let mut t = [0u64; 5];
    t[0] = mul32x32_64!(h[0], r[0])
        + mul32x32_64!(h[1], s[3])
        + mul32x32_64!(h[2], s[2])
        + mul32x32_64!(h[3], s[1])
        + mul32x32_64!(h[4], s[0]);
    trace_step!("mul‑raw‑prod", { t = ?t });
    t[1] = mul32x32_64!(h[0], r[1])
        + mul32x32_64!(h[1], r[0])
        + mul32x32_64!(h[2], s[3])
        + mul32x32_64!(h[3], s[2])
        + mul32x32_64!(h[4], s[1]);
    trace_step!("mul‑raw‑prod", { t = ?t });
    t[2] = mul32x32_64!(h[0], r[2])
        + mul32x32_64!(h[1], r[1])
        + mul32x32_64!(h[2], r[0])
        + mul32x32_64!(h[3], s[3])
        + mul32x32_64!(h[4], s[2]);
    trace_step!("mul‑raw‑prod", { t = ?t });
    t[3] = mul32x32_64!(h[0], r[3])
        + mul32x32_64!(h[1], r[2])
        + mul32x32_64!(h[2], r[1])
        + mul32x32_64!(h[3], r[0])
        + mul32x32_64!(h[4], s[3]);
    trace_step!("mul‑raw‑prod", { t = ?t });
    t[4] = mul32x32_64!(h[0], r[4])
        + mul32x32_64!(h[1], r[3])
        + mul32x32_64!(h[2], r[2])
        + mul32x32_64!(h[3], r[1])
        + mul32x32_64!(h[4], r[0]);
    trace_step!("mul‑raw‑prod", { t = ?t });

    tracing::debug!(?t, "multiply_and_reduce: raw 64‑bit products");

    // propagate carries & fold back into h  (Donna reference flow)
    h[0] = (t[0] as u32) & 0x3ffffff;
    trace_step!("acc‑update",  { h = ?*h });
    let mut c = t[0] >> 26;
    t[1] += c;
    trace_step!("mul‑raw‑prod", { t = ?t });
    h[1] = (t[1] as u32) & 0x3ffffff;
    trace_step!("acc‑update",  { h = ?*h });
    c = t[1] >> 26;
    t[2] += c;
    trace_step!("mul‑raw‑prod", { t = ?t });
    h[2] = (t[2] as u32) & 0x3ffffff;
    trace_step!("acc‑update",  { h = ?*h });
    c = t[2] >> 26;
    t[3] += c;
    trace_step!("mul‑raw‑prod", { t = ?t });
    h[3] = (t[3] as u32) & 0x3ffffff;
    trace_step!("acc‑update",  { h = ?*h });
    c = t[3] >> 26;
    t[4] += c;
    trace_step!("mul‑raw‑prod", { t = ?t });
    h[4] = (t[4] as u32) & 0x3ffffff;
    trace_step!("acc‑update",  { h = ?*h });
    c = t[4] >> 26;
    h[0] = h[0].wrapping_add((c as u32) * 5);
    trace_step!("acc‑update",  { h = ?*h });

    // **Do *not* mask h[1] here** — the original algorithm leaves h₁
    // potentially above 2²⁶ so that the next round can fold carries
    // naturally.  Masking early was the root cause of the RFC 7539
    // vector mismatch.
    if h[0] >= (1 << 26) {
        let carry_out = h[0] >> 26;
        h[0] &= 0x3ffffff;
        trace_step!("acc‑update",  { h = ?*h });
        h[1] = h[1].wrapping_add(carry_out);
        trace_step!("acc‑update",  { h = ?*h });
    }

    tracing::debug!(h_out = ?*h, "multiply_and_reduce: finished");
}

#[cfg(test)]
mod tests_multiply_reduce {
    use super::*;
    use hex_literal::hex;
    use proptest::prelude::*;

    #[traced_test]
    fn identity_when_r_is_one() {
        let r = [1, 0, 0, 0, 0];
        let s = [0, 0, 0, 0];
        let mut h = [3, 4, 5, 6, 7];
        let h_before = h;
        multiply_and_reduce(&mut h, &r, &s);
        assert_eq!(h, h_before, "h should remain unchanged when r == 1");
    }

    /// When *r* ≡ 0, the state must collapse to zero.
    #[traced_test]
    fn r_zero_annihilates_state() {
        let r = [0u32; 5];
        let s = [0u32; 4];
        let mut h = [42, 13, 7, 5, 9];
        multiply_and_reduce(&mut h, &r, &s);
        assert_eq!(h, [0u32; 5]);
    }

    /// Property‑based: output limbs always remain below 2²⁶.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(600))]

        #[traced_test]
        fn limb_range_after_mul_reduce(
            mut h in proptest::array::uniform5(0u32..(1u32 << 26)),
            r in proptest::array::uniform5(0u32..(1u32 << 26)),
        ) {
            let s = [r[1] * 5, r[2] * 5, r[3] * 5, r[4] * 5];
            multiply_and_reduce(&mut h, &r, &s);

            for &limb in &h {
                prop_assert!(limb < (1 << 26));
            }
        }
    }
}
