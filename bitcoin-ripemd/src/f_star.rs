// ---------------- [ File: bitcoin-ripemd/src/f_star.rs ]
crate::ix!();

/* ----- Internal RIPEMD‑160 implementation. ----- */

#[inline]
pub fn ripemd160_f1(x: u32, y: u32, z: u32) -> u32 {
    let r = x ^ y ^ z;
    trace!(
        target: "ripemd160::f",
        "f1({:#010x}, {:#010x}, {:#010x}) -> {:#010x}",
        x,
        y,
        z,
        r
    );
    r
}

#[inline]
pub fn ripemd160_f2(x: u32, y: u32, z: u32) -> u32 {
    let r = (x & y) | (!x & z);
    trace!(
        target: "ripemd160::f",
        "f2({:#010x}, {:#010x}, {:#010x}) -> {:#010x}",
        x,
        y,
        z,
        r
    );
    r
}

#[inline]
pub fn ripemd160_f3(x: u32, y: u32, z: u32) -> u32 {
    let r = (x | !y) ^ z;
    trace!(
        target: "ripemd160::f",
        "f3({:#010x}, {:#010x}, {:#010x}) -> {:#010x}",
        x,
        y,
        z,
        r
    );
    r
}

#[inline]
pub fn ripemd160_f4(x: u32, y: u32, z: u32) -> u32 {
    let r = (x & z) | (y & !z);
    trace!(
        target: "ripemd160::f",
        "f4({:#010x}, {:#010x}, {:#010x}) -> {:#010x}",
        x,
        y,
        z,
        r
    );
    r
}

#[inline]
pub fn ripemd160_f5(x: u32, y: u32, z: u32) -> u32 {
    let r = x ^ (y | !z);
    trace!(
        target: "ripemd160::f",
        "f5({:#010x}, {:#010x}, {:#010x}) -> {:#010x}",
        x,
        y,
        z,
        r
    );
    r
}

#[cfg(test)]
mod spec_f_star {
    use super::*;

    #[traced_test]
    fn verifies_known_vector() {
        let x = 0x1234_5678;
        let y = 0x9abc_def0;
        let z = 0x0f0f_0f0f;

        assert_eq!(ripemd160_f1(x, y, z), 0x8787_8787);
        assert_eq!(ripemd160_f2(x, y, z), 0x1f3f_5f77);
        assert_eq!(ripemd160_f3(x, y, z), 0x7878_7870);
        assert_eq!(ripemd160_f4(x, y, z), 0x92b4_d6f8);
        assert_eq!(ripemd160_f5(x, y, z), 0xe8c8_a888);
    }
}
