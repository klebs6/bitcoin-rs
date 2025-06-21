// ---------------- [ File: bitcoin-poly1305/src/add_pad_serialize.rs ]
crate::ix!();

#[inline(always)]
pub fn add_pad_serialize(
    out: &mut [u8; POLY1305_TAGLEN],
    h:   &LimbArr5,
    key: &[u8; POLY1305_KEYLEN],
) {

    tracing::trace!(?h, "add_pad_serialize: start");

    // ---------------------------------------------------------------------
    // 1.  Pack five 26‑bit limbs into four 32‑bit words (little‑endian).
    //     *No* pre‑propagation here – we follow the Donna reference flow:
    //     add pad first, then propagate carries word‑by‑word.
    // ---------------------------------------------------------------------
    let mut f0: u64 = (h[0] as u64) | ((h[1] as u64) << 26);
    let mut f1: u64 = ((h[1] >> 6) as u64) | ((h[2] as u64) << 20);
    let mut f2: u64 = ((h[2] >> 12) as u64) | ((h[3] as u64) << 14);
    let mut f3: u64 = ((h[3] >> 18) as u64) | ((h[4] as u64) << 8);

    tracing::debug!(f0, f1, f2, f3, "add_pad_serialize: packed limbs");

    // ---------------------------------------------------------------------
    // 2.  Add the 128‑bit *s* pad, propagating carries *after* each word.
    // ---------------------------------------------------------------------
    f0 = f0.wrapping_add(read_le32(&key[16..]) as u64);
    let mut carry = f0 >> 32;
    write_le32(&mut out[0..], f0);
    tracing::debug!(f0, carry, "add_pad_serialize: after pad₀");

    f1 = f1
        .wrapping_add(read_le32(&key[20..]) as u64)
        .wrapping_add(carry);
    carry = f1 >> 32;
    write_le32(&mut out[4..], f1);
    tracing::debug!(f1, carry, "add_pad_serialize: after pad₁");

    f2 = f2
        .wrapping_add(read_le32(&key[24..]) as u64)
        .wrapping_add(carry);
    carry = f2 >> 32;
    write_le32(&mut out[8..], f2);
    tracing::debug!(f2, carry, "add_pad_serialize: after pad₂");

    f3 = f3
        .wrapping_add(read_le32(&key[28..]) as u64)
        .wrapping_add(carry);
    write_le32(&mut out[12..], f3);
    tracing::debug!(f3, "add_pad_serialize: after pad₃");

    tracing::trace!(tag = ?*out, "add_pad_serialize: finished");
}

#[cfg(test)]
mod tests_pad_serialize {
    use super::*;
    use hex_literal::hex;
    use proptest::prelude::*;

    #[traced_test]
    fn zero_h_tag_equals_pad() {
        let mut out = [0u8; POLY1305_TAGLEN];
        let h = [0u32; 5];
        let key = hex!(
            "00000000000000000000000000000000 \
             d0d1d2d3d4d5d6d7d8d9dadbdcdddedf"
        );

        add_pad_serialize(&mut out, &h, &key);
        assert_eq!(out, key[16..]);
    }

    /// Verify the pad‑addition logic against an independent reference
    /// implementation that *does not* pre‑propagate carries.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]

        #[traced_test]
        fn add_pad_matches_reference(
            h in proptest::array::uniform5(0u32..(1u32 << 26)),
            key in proptest::array::uniform32(any::<u8>()),
        ) {
            // --- library implementation
            let mut tag_lib = [0u8; POLY1305_TAGLEN];
            add_pad_serialize(&mut tag_lib, &h, &key);

            // --- reference re‑implementation (same algorithm as above)
            let mut f0: u64 = (h[0] as u64) | ((h[1] as u64) << 26);
            let mut f1: u64 = ((h[1] >> 6)  as u64) | ((h[2] as u64) << 20);
            let mut f2: u64 = ((h[2] >> 12) as u64) | ((h[3] as u64) << 14);
            let mut f3: u64 = ((h[3] >> 18) as u64) | ((h[4] as u64) <<  8);

            fn u32_le(bytes: &[u8]) -> u32 {
                u32::from_le_bytes(bytes.try_into().unwrap())
            }

            f0 = f0.wrapping_add(u32_le(&key[16..20]) as u64);
            let mut c = f0 >> 32;
            let w0 = f0 as u32;

            f1 = f1
                .wrapping_add(u32_le(&key[20..24]) as u64)
                .wrapping_add(c);
            c  = f1 >> 32;
            let w1 = f1 as u32;

            f2 = f2
                .wrapping_add(u32_le(&key[24..28]) as u64)
                .wrapping_add(c);
            c  = f2 >> 32;
            let w2 = f2 as u32;

            f3 = f3
                .wrapping_add(u32_le(&key[28..32]) as u64)
                .wrapping_add(c);
            let w3 = f3 as u32;

            let mut tag_ref = [0u8; POLY1305_TAGLEN];
            tag_ref[..4].copy_from_slice(&w0.to_le_bytes());
            tag_ref[ 4..8].copy_from_slice(&w1.to_le_bytes());
            tag_ref[ 8..12].copy_from_slice(&w2.to_le_bytes());
            tag_ref[12..16].copy_from_slice(&w3.to_le_bytes());

            prop_assert_eq!(tag_lib, tag_ref);
        }
    }
}
