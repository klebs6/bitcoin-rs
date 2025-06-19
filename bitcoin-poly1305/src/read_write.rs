crate::ix!();

/// 32‑bit × 32‑bit → 64‑bit multiply (matches the C `((uint64_t)(a) * (b))`).
#[macro_export]
macro_rules! mul32x32_64 {
    ($a:expr, $b:expr) => {
        (($a as u64).wrapping_mul($b as u64))
    };
}

#[inline(always)]
pub fn read_le32(input: &[u8]) -> u32 {
    debug_assert!(input.len() >= 4);
    u32::from_le_bytes(input[0..4].try_into().expect("slice length == 4"))
}

#[inline(always)]
pub fn write_le32(dst: &mut [u8], v: u64) {
    debug_assert!(dst.len() >= 4);
    dst[..4].copy_from_slice(&(v as u32).to_le_bytes());
}
