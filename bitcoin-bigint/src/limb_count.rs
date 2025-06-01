crate::ix!();

#[inline]
pub const fn base_uint_limb_count(bits: usize) -> usize {
    // If you only ever do 160 or 256, match them:
    match bits {
        160 => 5,
        256 => 8,
        _ => panic!("Unsupported bits: must be 160 or 256"),
    }
}
