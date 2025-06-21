// ---------------- [ File: bitcoin-poly1305/src/read_write.rs ]
crate::ix!();

/// 32‑bit × 32‑bit → 64‑bit multiply (matches the C `((uint64_t)(a)*(b))`).
#[macro_export]
macro_rules! mul32x32_64 {
    ($a:expr, $b:expr) => {
        (($a as u64).wrapping_mul($b as u64))
    };
}

#[inline(always)]
pub fn read_le32(input: &[u8]) -> u32 {
    debug_assert!(input.len() >= 4);
    let val = u32::from_le_bytes(input[0..4].try_into().expect("slice len == 4"));
    tracing::trace!(bytes = ?&input[0..4], val, "read_le32");
    val
}

#[inline(always)]
pub fn write_le32(dst: &mut [u8], v: u64) {
    debug_assert!(dst.len() >= 4);
    dst[..4].copy_from_slice(&(v as u32).to_le_bytes());
    tracing::trace!(bytes = ?&dst[..4], val = (v as u32), "write_le32");
}

#[cfg(test)]
mod tests_read_write {
    use super::*;
    use proptest::prelude::*;

    #[traced_test]
    fn roundtrip_extremes_and_midpoints() {
        const CASES: &[u32] = &[0, 1, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff];

        for &v in CASES {
            let mut buf = [0u8; 4];
            write_le32(&mut buf[..], v as u64);
            let back = read_le32(&buf[..]);
            assert_eq!(back, v, "value {:08x} round‑trips through le32 helpers", v);
        }
    }

    /// Fuzz the helpers with 32‑bit space.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(4000))]

        #[traced_test]
        fn le32_roundtrip_random(v in any::<u32>()) {
            let mut buf = [0u8; 4];
            write_le32(&mut buf[..], v as u64);
            prop_assert_eq!(read_le32(&buf[..]), v);
        }
    }
}
