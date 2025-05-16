crate::ix!();

/// Convert from `ArithU256` to the opaque `u256` by writing each 32-bit limb
/// in **little-endian** order into the 32 bytes of `u256`.
///
/// This matches the C++ code: 
/// ```cpp
/// for x in 0..a.WIDTH:
///   writele32(b.as_ptr().offset(x * 4), a.pn[x]);
/// ```
pub fn arith_to_uint256(a: &ArithU256) -> u256 {
    trace!("arith_to_uint256 => converting ArithU256 into u256 by LE limb writes.");

    let mut out = u256::default();
    let limb_count = 256 / 32; // => 8
    for i in 0..limb_count {
        let limb = a.base.get_limb(i); 
        let le = limb.to_le_bytes(); 
        // Copy these 4 bytes into out's slice at offset i*4
        out.as_slice_mut()[(i * 4)..(i * 4 + 4)].copy_from_slice(&le);
    }
    out
}

/// Convert from `u256` to `ArithU256` by **reading** each set of 4 bytes
/// in little-endian order. 
///
/// This matches the C++:
/// ```cpp
/// for x in 0..b.WIDTH:
///   b.base.pn[x] = readle32(a.as_ptr().offset(x * 4));
/// ```
pub fn uint_to_arith256(a: &u256) -> ArithU256 {
    trace!("uint_to_arith256 => converting u256 into ArithU256 by LE limb reads.");

    let mut b = ArithU256::default();
    let limb_count = 256 / 32; // => 8
    for i in 0..limb_count {
        let le_slice = &a.as_slice()[(i * 4)..(i * 4 + 4)];
        let val = u32::from_le_bytes(le_slice.try_into().unwrap());
        b.base.set_limb(i, val);
    }
    b
}
