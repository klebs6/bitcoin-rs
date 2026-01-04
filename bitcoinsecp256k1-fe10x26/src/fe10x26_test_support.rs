// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe10x26_test_support.rs ]
#![cfg(test)]

crate::ix!();

pub(super) const FIELD_PRIME_WORDS_LE: [u32; 8] = [
    0xFFFFFC2Fu32,
    0xFFFFFFFEu32,
    0xFFFFFFFFu32,
    0xFFFFFFFFu32,
    0xFFFFFFFFu32,
    0xFFFFFFFFu32,
    0xFFFFFFFFu32,
    0xFFFFFFFFu32,
];

pub(super) const BYTES_ZERO: [u8; 32] = [0u8; 32];

pub(super) const BYTES_ONE: [u8; 32] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
];

pub(super) const BYTES_TWO: [u8; 32] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8,
];

pub(super) const BYTES_THREE: [u8; 32] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8,
];

pub(super) const BYTES_FOUR: [u8; 32] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8,
];

pub(super) const BYTES_FIVE: [u8; 32] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8,
];

pub(super) const BYTES_2_POW_255: [u8; 32] = [
    0x80u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
];

pub(super) const BYTES_LOW_32_ONES: [u8; 32] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
];

pub(super) const BYTES_PATTERN_A: [u8; 32] = [
    0x12u8, 0x34u8, 0x56u8, 0x78u8, 0x90u8, 0xABu8, 0xCDu8, 0xEFu8,
    0x01u8, 0x23u8, 0x45u8, 0x67u8, 0x89u8, 0xABu8, 0xCDu8, 0xEFu8,
    0x10u8, 0x32u8, 0x54u8, 0x76u8, 0x98u8, 0xBAu8, 0xDCu8, 0xFEu8,
    0x00u8, 0x11u8, 0x22u8, 0x33u8, 0x44u8, 0x55u8, 0x66u8, 0x77u8,
];

pub(super) const BYTES_PATTERN_B: [u8; 32] = [
    0xFEu8, 0xDCu8, 0xBAu8, 0x98u8, 0x76u8, 0x54u8, 0x32u8, 0x10u8,
    0xFFu8, 0xEEu8, 0xDDu8, 0xCCu8, 0xBBu8, 0xAAu8, 0x99u8, 0x88u8,
    0x77u8, 0x66u8, 0x55u8, 0x44u8, 0x33u8, 0x22u8, 0x11u8, 0x00u8,
    0x10u8, 0x20u8, 0x30u8, 0x40u8, 0x50u8, 0x60u8, 0x70u8, 0x80u8,
];

pub(super) const FIELD_PRIME_BYTES_BE: [u8; 32] = [
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFEu8, 0xFFu8, 0xFFu8, 0xFCu8, 0x2Fu8,
];

pub(super) const FIELD_PRIME_PLUS_ONE_BYTES_BE: [u8; 32] = [
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFEu8, 0xFFu8, 0xFFu8, 0xFCu8, 0x30u8,
];

pub(super) const FIELD_PRIME_MINUS_ONE_BYTES_BE: [u8; 32] = [
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFEu8, 0xFFu8, 0xFFu8, 0xFCu8, 0x2Eu8,
];

pub(super) const FIELD_PRIME_MINUS_TWO_BYTES_BE: [u8; 32] = [
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    0xFFu8, 0xFFu8, 0xFFu8, 0xFEu8, 0xFFu8, 0xFFu8, 0xFCu8, 0x2Du8,
];

pub(super) const BYTES_MAX: [u8; 32] = [0xFFu8; 32];

pub(super) fn words_le_from_be_bytes(bytes: &[u8; 32]) -> [u32; 8] {
    let mut out = [0u32; 8];
    for i in 0_usize..8_usize {
        let idx = 32usize - 4usize.wrapping_mul(i.wrapping_add(1));
        out[i] = ((bytes[idx] as u32) << 24)
            | ((bytes[idx + 1] as u32) << 16)
            | ((bytes[idx + 2] as u32) << 8)
            | (bytes[idx + 3] as u32);
    }
    out
}

pub(super) fn be_bytes_from_words_le(words: &[u32; 8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for i in 0..8 {
        let w = words[7usize.wrapping_sub(i)];
        let idx = i.wrapping_mul(4);
        out[idx] = (w >> 24) as u8;
        out[idx + 1] = (w >> 16) as u8;
        out[idx + 2] = (w >> 8) as u8;
        out[idx + 3] = w as u8;
    }
    out
}

pub(super) fn cmp_words_le(a: &[u32; 8], b: &[u32; 8]) -> Ordering {
    for i in (0..8).rev() {
        if a[i] < b[i] {
            return Ordering::Less;
        }
        if a[i] > b[i] {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

pub(super) fn sub_words_le(a: &[u32; 8], b: &[u32; 8]) -> [u32; 8] {
    let mut out = [0u32; 8];
    let mut borrow: u64 = 0;
    for i in 0..8 {
        let ai = a[i] as u64;
        let bi = b[i] as u64;
        let sub = bi.wrapping_add(borrow);
        out[i] = ai.wrapping_sub(sub) as u32;
        borrow = ((ai < sub) as u32) as u64;
    }
    out
}

pub(super) fn add_mod_p(a: &[u32; 8], b: &[u32; 8]) -> [u32; 8] {
    let mut t = [0u128; 24];
    let mut carry: u128 = 0;
    for i in 0..8 {
        let sum = (a[i] as u128).wrapping_add(b[i] as u128).wrapping_add(carry);
        t[i] = sum & 0xFFFF_FFFFu128;
        carry = sum >> 32;
    }
    t[8] = carry;
    reduce_u128_limbs_mod_p(t)
}

pub(super) fn neg_mod_p(a: &[u32; 8]) -> [u32; 8] {
    if a.iter().all(|&w| w == 0) {
        return [0u32; 8];
    }
    sub_words_le(&FIELD_PRIME_WORDS_LE, a)
}

pub(super) fn mul_mod_p(a: &[u32; 8], b: &[u32; 8]) -> [u32; 8] {
    let mut prod = [0u128; 17];
    for i in 0..8 {
        for j in 0..8 {
            prod[i + j] = prod[i + j].wrapping_add((a[i] as u128).wrapping_mul(b[j] as u128));
        }
    }
    for i in 0..16 {
        let carry = prod[i] >> 32;
        prod[i] &= 0xFFFF_FFFFu128;
        prod[i + 1] = prod[i + 1].wrapping_add(carry);
    }

    let mut t = [0u128; 24];
    for i in 0..17 {
        t[i] = prod[i];
    }
    reduce_u128_limbs_mod_p(t)
}

pub(super) fn sqr_mod_p(a: &[u32; 8]) -> [u32; 8] {
    mul_mod_p(a, a)
}

pub(super) fn fe_from_be_bytes_ret(bytes: &[u8; 32]) -> (Fe10x26, i32) {
    let mut fe = Fe10x26::new();
    let ret = unsafe { fe_set_b32(&mut fe as *mut Fe10x26, bytes.as_ptr()) };
    (fe, ret)
}

pub(super) fn fe_from_be_bytes_checked(bytes: &[u8; 32]) -> Fe10x26 {
    let (fe, ret) = fe_from_be_bytes_ret(bytes);
    assert!(ret != 0, "fe_set_b32 rejected an expected-in-range test vector");
    fe
}

pub(super) fn fe_from_u32(v: u32) -> Fe10x26 {
    let mut fe = Fe10x26::new();
    unsafe { fe_set_int(&mut fe as *mut Fe10x26, v as i32) };
    fe
}

pub(super) fn fe_clone_value(src: &Fe10x26) -> Fe10x26 {
    unsafe { core::ptr::read(src as *const Fe10x26) }
}

pub(super) fn fe_normalize_in_place(fe: &mut Fe10x26) {
    unsafe { fe_normalize(fe as *mut Fe10x26) };
}

pub(super) fn fe_to_be_bytes_normalized(fe: &mut Fe10x26) -> [u8; 32] {
    fe_normalize_in_place(fe);
    let mut out = [0u8; 32];
    unsafe { fe_get_b32(out.as_mut_ptr(), fe as *const Fe10x26) };
    out
}

pub(super) fn fe_to_words_le_normalized(fe: &mut Fe10x26) -> [u32; 8] {
    let b = fe_to_be_bytes_normalized(fe);
    words_le_from_be_bytes(&b)
}

pub(super) fn fe_add_in_place(r: &mut Fe10x26, a: &Fe10x26) {
    unsafe { fe_add(r as *mut Fe10x26, a as *const Fe10x26) };
}

pub(super) fn fe_mul_to_words_le_normalized(a: &Fe10x26, b: &Fe10x26) -> [u32; 8] {
    let mut r = Fe10x26::new();
    let mut b_mut = fe_clone_value(b);
    unsafe { fe_mul(&mut r as *mut Fe10x26, a as *const Fe10x26, &mut b_mut as *mut Fe10x26) };
    fe_to_words_le_normalized(&mut r)
}

pub(super) fn fe_sqr_to_words_le_normalized(a: &Fe10x26) -> [u32; 8] {
    let mut r = Fe10x26::new();
    unsafe { fe_sqr(&mut r as *mut Fe10x26, a as *const Fe10x26) };
    fe_to_words_le_normalized(&mut r)
}

pub(super) fn fe_negate_to_words_le_normalized(a: &Fe10x26, m: i32) -> [u32; 8] {
    let mut r = Fe10x26::new();
    unsafe { fe_negate(&mut r as *mut Fe10x26, a as *const Fe10x26, m) };
    fe_to_words_le_normalized(&mut r)
}

fn reduce_u128_limbs_mod_p(mut t: [u128; 24]) -> [u32; 8] {
    const MASK32: u128 = 0xFFFF_FFFFu128;

    loop {
        for k in (8..t.len()).rev() {
            let v = t[k];
            if v != 0 {
                t[k] = 0;
                t[k - 8] = t[k - 8].wrapping_add(v.wrapping_mul(977u128));
                t[k - 7] = t[k - 7].wrapping_add(v);
            }
        }

        for i in 0..t.len().wrapping_sub(1) {
            let carry = t[i] >> 32;
            t[i] &= MASK32;
            t[i + 1] = t[i + 1].wrapping_add(carry);
        }

        if t[8..].iter().all(|&x| x == 0) {
            break;
        }
    }

    let mut res = [0u32; 8];
    for i in 0..8 {
        res[i] = t[i] as u32;
    }

    while cmp_words_le(&res, &FIELD_PRIME_WORDS_LE) != Ordering::Less {
        res = sub_words_le(&res, &FIELD_PRIME_WORDS_LE);
    }

    res
}
