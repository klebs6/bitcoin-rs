// ---------------- [ File: bitcoin-poly1305/src/add_pad_serialize.rs ]
crate::ix!();

/// Constant‑time “add with carry” for 64‑bit words.
///
/// Returns the sum and a boolean carry‑out.
/// (Identical logic is already used elsewhere in the crate;
/// shown here in case you need a local copy.)
#[inline(always)]
fn adc64(x: u64, y: u64, carry: bool) -> (u64, bool) {
    let (z, c1) = x.overflowing_add(y);
    let (z, c2) = z.overflowing_add(carry as u64);
    (z, c1 | c2)
}

/// After `final_carry_and_sub_p` we have five 26‑bit limbs `h[0..=4]`,
/// each < 2²⁶.  This function
///
/// 1. Packs them into the four 32‑bit words of the reference
///    implementation (truncating → u32 immediately, *exactly* as Donna did).
/// 2. Adds the 128‑bit pad `s`, propagating carries word‑wise.
/// 3. Writes the resulting 16‑byte tag (little‑endian).
///
/// Extra **trace‑level** instrumentation has been added so that every
/// intermediate value, carry and final tag can be inspected when tests are
/// executed with `RUST_LOG=trace`.
#[inline(always)]
pub(crate) fn add_pad_serialize(h: [u32; 5], key: &[u8; 32]) -> [u8; 16] {
    //--------------------------------------------------------------------
    // 0.  Trace inputs
    //--------------------------------------------------------------------
    tracing::trace!(
        ?h,
        pad_hi = ?&key[16..],
        "add_pad_serialize: start"
    );

    //--------------------------------------------------------------------
    // 1.  Radix‑26 → radix‑32 packing (immediate u32 truncation)
    //--------------------------------------------------------------------
    let mut f0: u32 = (h[0])          | (h[1] << 26);
    let mut f1: u32 = (h[1] >> 6)     | (h[2] << 20);
    let mut f2: u32 = (h[2] >> 12)    | (h[3] << 14);
    let mut f3: u32 = (h[3] >> 18)    | (h[4] <<  8);

    tracing::debug!(
        f0_raw = f0,
        f1_raw = f1,
        f2_raw = f2,
        f3_raw = f3,
        "add_pad_serialize: packed (truncated) words"
    );

    //--------------------------------------------------------------------
    // 2.  Add the 128‑bit pad – carry‑propagate word‑wise (constant‑time)
    //--------------------------------------------------------------------
    let s0 = u32::from_le_bytes(key[16..20].try_into().unwrap()) as u64;
    let s1 = u32::from_le_bytes(key[20..24].try_into().unwrap()) as u64;
    let s2 = u32::from_le_bytes(key[24..28].try_into().unwrap()) as u64;
    let s3 = u32::from_le_bytes(key[28..32].try_into().unwrap()) as u64;

    tracing::debug!(s0, s1, s2, s3, "add_pad_serialize: pad words");

    let mut acc: u64 = f0 as u64 + s0;
    f0 = acc as u32;
    let mut c = acc >> 32;
    tracing::trace!(step = "add_pad", acc, carry_out = c, f0);

    acc = f1 as u64 + s1 + c;
    f1 = acc as u32;
    c   = acc >> 32;
    tracing::trace!(step = "add_pad", acc, carry_out = c, f1);

    acc = f2 as u64 + s2 + c;
    f2 = acc as u32;
    c   = acc >> 32;
    tracing::trace!(step = "add_pad", acc, carry_out = c, f2);

    acc = f3 as u64 + s3 + c;
    f3 = acc as u32;
    tracing::trace!(step = "add_pad", acc, carry_out_final = (acc >> 32), f3);

    //--------------------------------------------------------------------
    // 3.  Serialise tag
    //--------------------------------------------------------------------
    let mut tag = [0u8; 16];
    tag[ 0.. 4].copy_from_slice(&f0.to_le_bytes());
    tag[ 4.. 8].copy_from_slice(&f1.to_le_bytes());
    tag[ 8..12].copy_from_slice(&f2.to_le_bytes());
    tag[12..16].copy_from_slice(&f3.to_le_bytes());

    tracing::debug!(?tag, "add_pad_serialize: finished");
    tag
}

#[cfg(test)]
mod tests_pad_serialize {
    use super::*;
    use hex_literal::hex;
    use proptest::prelude::*;

    // ------------------------------------------------ deterministic -----
    #[traced_test]
    fn zero_h_tag_equals_pad() {
        let h   = [0u32; 5];
        let key = hex!(
            "00000000000000000000000000000000 \
             d0d1d2d3d4d5d6d7d8d9dadbdcdddedf"
        );

        let tag = add_pad_serialize(h, &key);
        assert_eq!(tag, key[16..], "all‑zero limbs ⇒ tag == pad‑high");
    }

    // ------------------------------------------------ property test -----
    ///
    /// Re‑implements the *exact* Donna algorithm step‑for‑step, including the
    /// **32‑bit truncation _before_ the pad addition** that the real code
    /// performs.  The previous version of this test forgot that early mask,
    /// so it occasionally disagreed with the production routine when the
    /// packed word `f₂` or `f₃` overflowed 32 bits.
    ///
    /// Keeping both implementations bit‑for‑bit identical guarantees that the
    /// randomly‑generated inputs exercise the same corner‑cases.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]

        #[traced_test]
        fn add_pad_matches_reference(
            h   in proptest::array::uniform5(0u32..(1u32 << 26)),
            key in proptest::array::uniform32(any::<u8>()),
        ) {
            //----------------------------------------------------------------
            // library implementation
            //----------------------------------------------------------------
            let tag_lib = add_pad_serialize(h, &key);

            //----------------------------------------------------------------
            // independent re‑implementation (mirrors Donna verbatim)
            //----------------------------------------------------------------
            let mut f0: u64 = ((h[0] as u64)            | ((h[1] as u64) << 26)) & 0xffff_ffff;
            let mut f1: u64 = (((h[1] >>  6) as u64)    | ((h[2] as u64) << 20)) & 0xffff_ffff;
            let mut f2: u64 = (((h[2] >> 12) as u64)    | ((h[3] as u64) << 14)) & 0xffff_ffff;
            let mut f3: u64 = (((h[3] >> 18) as u64)    | ((h[4] as u64) <<  8)) & 0xffff_ffff;

            let s0 = u32::from_le_bytes(key[16..20].try_into().unwrap()) as u64;
            let s1 = u32::from_le_bytes(key[20..24].try_into().unwrap()) as u64;
            let s2 = u32::from_le_bytes(key[24..28].try_into().unwrap()) as u64;
            let s3 = u32::from_le_bytes(key[28..32].try_into().unwrap()) as u64;

            // word‑wise addition with carry propagation
            f0 = f0.wrapping_add(s0);
            let mut c = f0 >> 32;
            f0 &= 0xffff_ffff;

            f1 = f1.wrapping_add(s1).wrapping_add(c);
            c   = f1 >> 32;
            f1 &= 0xffff_ffff;

            f2 = f2.wrapping_add(s2).wrapping_add(c);
            c   = f2 >> 32;
            f2 &= 0xffff_ffff;

            f3 = f3.wrapping_add(s3).wrapping_add(c);
            f3 &= 0xffff_ffff;           // carry beyond 128 bits is discarded

            // serialise reference tag
            let mut tag_ref = [0u8; POLY1305_TAGLEN];
            tag_ref[ 0.. 4].copy_from_slice(&(f0 as u32).to_le_bytes());
            tag_ref[ 4.. 8].copy_from_slice(&(f1 as u32).to_le_bytes());
            tag_ref[ 8..12].copy_from_slice(&(f2 as u32).to_le_bytes());
            tag_ref[12..16].copy_from_slice(&(f3 as u32).to_le_bytes());

            //----------------------------------------------------------------
            // compare
            //----------------------------------------------------------------
            prop_assert_eq!(
                tag_lib, tag_ref,
                "packed‑and‑padded result must match reference algorithm"
            );
        }
    }
}
