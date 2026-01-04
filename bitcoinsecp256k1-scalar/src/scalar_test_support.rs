// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_test_support.rs ]
#![allow(dead_code)]
#![cfg(test)]

crate::ix!();

pub(crate) const SECP256K1_ORDER_BE: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
    0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41,
];

pub(crate) const SECP256K1_ORDER_MINUS_1_BE: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
    0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x40,
];

pub(crate) const SECP256K1_ORDER_MINUS_2_BE: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
    0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x3F,
];

pub(crate) const SECP256K1_ORDER_HALF_BE: [u8; 32] = [
    0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0x5D, 0x57, 0x6E, 0x73, 0x57, 0xA4, 0x50, 0x1D, 0xDF, 0xE9, 0x2F, 0x46, 0x68, 0x1B, 0x20, 0xA0,
];

pub(crate) const SECP256K1_LAMBDA_BE: [u8; 32] = [
    0x53, 0x63, 0xAD, 0x4C, 0xC0, 0x5C, 0x30, 0xE0, 0xA5, 0x26, 0x1C, 0x02, 0x88, 0x12, 0x64, 0x5A,
    0x12, 0x2E, 0x22, 0xEA, 0x20, 0x81, 0x66, 0x78, 0xDF, 0x02, 0x96, 0x7C, 0x1B, 0x23, 0xBD, 0x72,
];

pub(crate) const SCALAR_ZERO_BE: [u8; 32] = [0u8; 32];

pub(crate) const SCALAR_ONE_BE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];

pub(crate) const SCALAR_TWO_BE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
];

pub(crate) const SCALAR_THREE_BE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
];

pub(crate) const SCALAR_MAX_U32_BE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF,
    0xFF, 0xFF,
];

pub(crate) const SCALAR_2_127_BE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub(crate) const SCALAR_2_128_BE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub(crate) const SCALAR_2_255_BE: [u8; 32] = [
    0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub(crate) const CANONICAL_TEST_SCALARS_BE: [[u8; 32]; 10] = [
    SCALAR_ZERO_BE,
    SCALAR_ONE_BE,
    SCALAR_TWO_BE,
    SCALAR_THREE_BE,
    SCALAR_MAX_U32_BE,
    SCALAR_2_127_BE,
    SCALAR_2_128_BE,
    SCALAR_2_255_BE,
    SECP256K1_ORDER_HALF_BE,
    SECP256K1_ORDER_MINUS_1_BE,
];

pub(crate) const REDUCTION_TEST_INPUTS_BE: [[u8; 32]; 6] = [
    SCALAR_ZERO_BE,
    SCALAR_ONE_BE,
    SECP256K1_ORDER_BE,
    {
        let mut v = SECP256K1_ORDER_BE;
        v[31] = v[31].wrapping_add(1);
        v
    },
    SECP256K1_ORDER_MINUS_1_BE,
    [0xFFu8; 32],
];

    pub(crate) fn be_is_zero_32(a: &[u8; 32]) -> bool {
        a.iter().all(|&b| b == 0)
    }

    pub(crate) fn be_cmp_32(a: &[u8; 32], b: &[u8; 32]) -> Ordering {
        for i in 0..32 {
            if a[i] < b[i] {
                return Ordering::Less;
            }
            if a[i] > b[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }

    pub(crate) fn be_ge_32(a: &[u8; 32], b: &[u8; 32]) -> bool {
        be_cmp_32(a, b) != Ordering::Less
    }

    pub(crate) fn be_add_32(a: &[u8; 32], b: &[u8; 32]) -> ([u8; 32], u8) {
        let mut out = [0u8; 32];
        let mut carry: u16 = 0;
        for i in (0..32).rev() {
            let sum: u16 = a[i] as u16 + b[i] as u16 + carry;
            out[i] = (sum & 0xFF) as u8;
            carry = sum >> 8;
        }
        (out, carry as u8)
    }

    pub(crate) fn be_sub_32(a: &[u8; 32], b: &[u8; 32]) -> ([u8; 32], u8) {
        let mut out = [0u8; 32];
        let mut borrow: i16 = 0;
        for i in (0..32).rev() {
            let ai = a[i] as i16;
            let bi = b[i] as i16;
            let mut diff = ai - bi - borrow;
            if diff < 0 {
                diff += 256;
                borrow = 1;
            } else {
                borrow = 0;
            }
            out[i] = diff as u8;
        }
        (out, borrow as u8)
    }

    pub(crate) fn be_sub_assign_32(a: &mut [u8; 32], b: &[u8; 32]) {
        let mut borrow: i16 = 0;
        for i in (0..32).rev() {
            let ai = a[i] as i16;
            let bi = b[i] as i16;
            let mut diff = ai - bi - borrow;
            if diff < 0 {
                diff += 256;
                borrow = 1;
            } else {
                borrow = 0;
            }
            a[i] = diff as u8;
        }
        debug_assert!(borrow == 0, "be_sub_assign_32 called with a < b");
    }

    pub(crate) fn be_add_overflow_flag(a: &[u8; 32], b: &[u8; 32]) -> u8 {
        let (sum, carry) = be_add_32(a, b);
        if carry != 0 || be_ge_32(&sum, &SECP256K1_ORDER_BE) {
            1
        } else {
            0
        }
    }

    pub(crate) fn be_add_mod_n(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let (sum, carry) = be_add_32(a, b);
        if carry != 0 || be_ge_32(&sum, &SECP256K1_ORDER_BE) {
            let (reduced, _) = be_sub_32(&sum, &SECP256K1_ORDER_BE);
            reduced
        } else {
            sum
        }
    }

    pub(crate) fn be_reduce_256_mod_n(x: &[u8; 32]) -> ([u8; 32], u8) {
        if be_cmp_32(x, &SECP256K1_ORDER_BE) == Ordering::Less {
            (*x, 0)
        } else {
            let (reduced, borrow) = be_sub_32(x, &SECP256K1_ORDER_BE);
            debug_assert!(borrow == 0);
            (reduced, 1)
        }
    }

    pub(crate) fn be_neg_mod_n(a: &[u8; 32]) -> [u8; 32] {
        if be_is_zero_32(a) {
            return [0u8; 32];
        }
        let (out, borrow) = be_sub_32(&SECP256K1_ORDER_BE, a);
        debug_assert!(borrow == 0);
        out
    }

    pub(crate) fn be_shr1_256(a: &[u8; 32]) -> [u8; 32] {
        let mut out = [0u8; 32];
        let mut carry: u8 = 0;
        for i in 0..32 {
            let new_carry = a[i] & 1;
            out[i] = (a[i] >> 1) | (carry << 7);
            carry = new_carry;
        }
        out
    }

    pub(crate) fn be_bit_extract_u32(be: &[u8; 32], offset: u32, count: u32) -> u32 {
        debug_assert!(count <= 32);
        let mut out: u32 = 0;
        for i in 0..count {
            let bit_index = offset.wrapping_add(i);
            if bit_index >= 256 {
                break;
            }
            let byte_index = 31usize - (bit_index / 8) as usize;
            let bit_in_byte = (bit_index % 8) as u8;
            let bit = (be[byte_index] >> bit_in_byte) & 1;
            out |= (bit as u32) << i;
        }
        out
    }

    pub(crate) fn be_set_bit_256(bit: u32) -> [u8; 32] {
        debug_assert!(bit < 256);
        let mut out = [0u8; 32];
        let byte_from_lsb = (bit / 8) as usize;
        let bit_in_byte = (bit % 8) as u8;
        let idx = 31usize - byte_from_lsb;
        out[idx] = 1u8 << bit_in_byte;
        out
    }

    pub(crate) fn be_mul_256(a: &[u8; 32], b: &[u8; 32]) -> [u8; 64] {
        fn be_to_le_u32x8(be: &[u8; 32]) -> [u32; 8] {
            let mut out = [0u32; 8];
            for i in 0..8 {
                let start = 32usize - 4usize * (i + 1);
                out[i] = ((be[start] as u32) << 24)
                    | ((be[start + 1] as u32) << 16)
                    | ((be[start + 2] as u32) << 8)
                    | (be[start + 3] as u32);
            }
            out
        }

        fn le_u32x16_to_be_64(le: &[u32; 16]) -> [u8; 64] {
            let mut out = [0u8; 64];
            for i in 0..16 {
                let limb = le[15 - i];
                let base = i * 4;
                out[base] = (limb >> 24) as u8;
                out[base + 1] = (limb >> 16) as u8;
                out[base + 2] = (limb >> 8) as u8;
                out[base + 3] = limb as u8;
            }
            out
        }

        let aa = be_to_le_u32x8(a);
        let bb = be_to_le_u32x8(b);

        let mut acc = [0u128; 16];
        for i in 0..8 {
            for j in 0..8 {
                acc[i + j] = acc[i + j].wrapping_add((aa[i] as u128).wrapping_mul(bb[j] as u128));
            }
        }

        let mut out_le = [0u32; 16];
        let mut carry: u128 = 0;
        for k in 0..16 {
            let val = acc[k].wrapping_add(carry);
            out_le[k] = (val & 0xFFFF_FFFFu128) as u32;
            carry = val >> 32;
        }
        debug_assert!(carry == 0);

        le_u32x16_to_be_64(&out_le)
    }

    pub(crate) fn be_mod_512_by_order_n(prod: &[u8; 64]) -> [u8; 32] {
        fn be_shl1_assign_32(x: &mut [u8; 32]) {
            let mut carry: u8 = 0;
            for i in (0..32).rev() {
                let new_carry = (x[i] >> 7) & 1;
                x[i] = (x[i] << 1) | carry;
                carry = new_carry;
            }
        }

        let mut rem = [0u8; 32];
        for bitpos in 0..512u32 {
            let byte_index = (bitpos / 8) as usize;
            let bit_in_byte = 7 - (bitpos % 8) as u8;
            let bit = (prod[byte_index] >> bit_in_byte) & 1;

            be_shl1_assign_32(&mut rem);
            rem[31] |= bit;

            if be_ge_32(&rem, &SECP256K1_ORDER_BE) {
                be_sub_assign_32(&mut rem, &SECP256K1_ORDER_BE);
            }
        }
        rem
    }

    pub(crate) fn be_mul_mod_n(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let prod = be_mul_256(a, b);
        be_mod_512_by_order_n(&prod)
    }

    pub(crate) fn be_add_pow2_512(prod: &[u8; 64], bit_lsb: u32) -> [u8; 64] {
        debug_assert!(bit_lsb < 512);
        let mut out = *prod;

        let byte_from_lsb = (bit_lsb / 8) as usize;
        let bit_in_byte = (bit_lsb % 8) as u8;
        let mut idx = 63usize - byte_from_lsb;

        let mut carry: u16 = (1u16) << bit_in_byte;
        loop {
            let sum = out[idx] as u16 + carry;
            out[idx] = (sum & 0xFF) as u8;
            carry = sum >> 8;
            if carry == 0 || idx == 0 {
                break;
            }
            idx -= 1;
        }

        out
    }

    pub(crate) fn be_shr_512(prod: &[u8; 64], shift: u32) -> [u8; 64] {
        if shift >= 512 {
            return [0u8; 64];
        }
        if shift == 0 {
            return *prod;
        }

        let shift_bytes = (shift / 8) as usize;
        let shift_bits = (shift % 8) as u8;

        let mut out = [0u8; 64];
        for i in 0..64 {
            if i < shift_bytes {
                continue;
            }
            let src = i - shift_bytes;
            let mut v = prod[src] >> shift_bits;
            if shift_bits != 0 && src > 0 {
                v |= prod[src - 1] << (8 - shift_bits);
            }
            out[i] = v;
        }
        out
    }

    pub(crate) fn be_shr_rounded_512(prod: &[u8; 64], shift: u32) -> [u8; 32] {
        debug_assert!(shift >= 1 && shift <= 512);

        let mut shifted = be_shr_512(prod, shift);

        // Round to nearest integer by adding 1 to the shifted quotient iff bit (shift-1) of prod is set.
        let bitpos = shift - 1;
        let byte_from_lsb = (bitpos / 8) as usize;
        let bit_in_byte = (bitpos % 8) as u8;
        let idx = 63usize - byte_from_lsb;
        let round = (shifted[idx] & 0u8) | ((prod[idx] >> bit_in_byte) & 1u8);

        if round != 0 {
            let mut carry: u16 = 1;
            let mut i: usize = 63;
            while carry != 0 {
                let sum = shifted[i] as u16 + carry;
                shifted[i] = (sum & 0xFF) as u8;
                carry = sum >> 8;
                if i == 0 {
                    break;
                }
                i -= 1;
            }
            debug_assert!(carry == 0);
        }

        let mut out = [0u8; 32];
        out.copy_from_slice(&shifted[32..64]);
        out
    }

    #[cfg(any(feature = "widemul-int64", feature = "widemul-int128"))]
    pub(crate) fn scalar_zero_value() -> Scalar {
        Scalar::new()
    }

    #[cfg(feature = "exhaustive-test-order")]
    pub(crate) fn scalar_zero_value() -> Scalar {
        0u32
    }

    pub(crate) fn scalar_from_be_bytes_with_overflow(bytes: &[u8; 32]) -> (Scalar, i32) {
        let mut s = scalar_zero_value();
        let mut overflow: i32 = 0;
        unsafe {
            scalar_set_b32(
                &mut s as *mut Scalar,
                bytes.as_ptr(),
                &mut overflow as *mut i32,
            );
        }
        (s, overflow)
    }

    pub(crate) fn scalar_from_be_bytes(bytes: &[u8; 32]) -> Scalar {
        scalar_from_be_bytes_with_overflow(bytes).0
    }

    pub(crate) fn scalar_from_u32(v: u32) -> Scalar {
        let mut s = scalar_zero_value();
        unsafe {
            scalar_set_int(&mut s as *mut Scalar, v);
        }
        s
    }

    pub(crate) fn scalar_to_be_bytes(s: &Scalar) -> [u8; 32] {
        let mut out = [0u8; 32];
        unsafe {
            scalar_get_b32(out.as_mut_ptr(), s as *const Scalar);
        }
        out
    }

    pub(crate) fn scalar_clone_via_b32(s: &Scalar) -> Scalar {
        let bytes = scalar_to_be_bytes(s);
        scalar_from_be_bytes(&bytes)
    }

    pub(crate) fn scalar_is_normalized_bytes(be: &[u8; 32]) -> bool {
        be_cmp_32(be, &SECP256K1_ORDER_BE) == Ordering::Less
    }
