// ---------------- [ File: bitcoin-base58/src/decode.rs ]
crate::ix!();

/**
  | Decode a base58-encoded string (str)
  | into a byte vector (vchRet). return
  | true if decoding is successful.
  |
  */
pub fn decode_base58(input: &str, vch_ret: &mut Vec<u8>, max_ret_len: usize) -> bool {
    use nom::character::is_space;

    let input = input.trim();

    let mut chars = input.chars().peekable();

    // Count leading '1's as zero bytes
    let mut zeroes = 0;
    while matches!(chars.peek(), Some('1')) {
        zeroes += 1;
        chars.next();
        if zeroes > max_ret_len {
            return false;
        }
    }

    let base58_map = MAP_BASE58;

    // Start empty; seeding with 0 would make "" decode to [0] instead of [].
    let mut b256: Vec<u8> = Vec::new();

    while let Some(c) = chars.next() {

        if is_space(c as u8) {
            // Keep this variant strict: any whitespace is invalid.
            return false;
        }

        let carry = {
            let idx = c as usize;
            if idx >= 256 || base58_map[idx] == -1 {
                return false; // invalid base58 char
            }
            base58_map[idx] as u32
        };

        let mut carry = carry;
        for val in b256.iter_mut().rev() {
            let tmp = (*val as u32) * 58 + carry;
            *val = (tmp & 0xFF) as u8;
            carry = tmp >> 8;
        }

        while carry > 0 {
            b256.insert(0, (carry & 0xFF) as u8);
            carry >>= 8;
        }
    }

    let length = b256.len();
    if zeroes + length > max_ret_len {
        return false;
    }

    vch_ret.clear();
    vch_ret.resize(zeroes, 0);
    vch_ret.extend_from_slice(&b256);

    true
}

pub unsafe fn decode_base58_raw(
    mut psz:     *const u8,
    vch:         &mut Vec<u8>,
    max_ret_len: i32

) -> bool {

    // Skip leading spaces.
    while *psz != 0 && nom::character::is_space(*psz ){
        psz = psz.add(1);
    }

    // Skip and count leading '1's.
    let mut zeroes: i32 = 0;
    let mut length: i32 = 0;

    while char::from(*psz) == '1' {

        zeroes += 1;

        if zeroes > max_ret_len {
            return false;
        }

        psz = psz.add(1);
    }

    /**
      | Allocate enough space in big-endian
      | base256 representation.
      |
      */

    // log(58) / log(256), rounded up.
    let size: usize = libc::strlen(psz as *const i8) * 733 / 1000 + 1;
    // length-initialized buffer
    let mut b256: Vec<u8> = vec![0; size];

    // Process the characters.
    //
    // guarantee not out of range
    //
    // mapBase58.len() should be 256
    const_assert!(MAP_BASE58.len() == 256); 

    while *psz != 0 && !nom::character::is_space(*psz){

        //  Decode base58 character
        let mut carry: i32 = MAP_BASE58[*psz as usize] as i32;

        if carry == -1 {
            //  Invalid b58 character
            return false;
        }

        let mut i: i32 = 0;

        for val in b256.iter_mut().rev() {

            if carry == 0 && i >= length {
                break;
            }

            carry += (58 * (*val)) as i32;

            *val   = (carry % 256).try_into().unwrap();

            carry /= 256;

            i += 1;
        }

        assert!(carry == 0);

        length = i;

        if length + zeroes > max_ret_len {
            return false
        }

        psz = psz.add(1);
    }

    // Skip trailing spaces.
    while nom::character::is_space(*psz){
        psz = psz.add(1);
    }

    if *psz != 0 {
        return false
    }

    // Skip leading zeroes in b256.
    let mut it = b256.iter();

    let offset: usize = (size - length as usize).try_into().unwrap();

    it.advance_by(offset);

    // Copy result into output vector.
    vch.reserve(zeroes as usize + it.len());

    vch[0..zeroes as usize].fill(0);

    // Skip leading zeroes in b256.
    let mut idx = size - length as usize;
    while idx < b256.len() && b256[idx] == 0 {
        idx += 1;
    }
    // Copy result into output vector.
    vch.clear();
    vch.resize(zeroes as usize, 0);
    vch.extend_from_slice(&b256[idx..]);

    return true
}

#[cfg(test)]
mod decode_spec {
    use super::*;

    /// Convert an ASCII‑hex string to bytes without `unwrap`.
    fn hex_to_bytes(src: &str) -> Vec<u8> {
        fn nibble(b: u8) -> u8 {
            match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'f' => b - b'a' + 10,
                b'A'..=b'F' => b - b'A' + 10,
                _ => panic!("non‑hex digit {b:?}"),
            }
        }
        assert!(src.len() % 2 == 0, "hex string must be even length");
        src.as_bytes()
            .chunks_exact(2)
            .map(|pair| (nibble(pair[0]) << 4) | nibble(pair[1]))
            .collect()
    }

    const GOOD_VECTORS: &[(&str, &str)] = &[
        ("", ""),
        ("61", "2g"),
        ("626262", "a3gV"),
        ("000000", "111"),
    ];

    /// Happy‑path: vectors decode successfully and match reference payloads.
    #[traced_test]
    fn decode_reference_vectors() {
        for (idx, (hex, b58)) in GOOD_VECTORS.iter().enumerate() {
            let mut out = Vec::new();
            trace!(idx, ?hex, ?b58, "decoding reference vector");
            assert!(
                decode_base58(b58, &mut out, 256),
                "vector #{idx} failed to decode"
            );
            assert_eq!(out, hex_to_bytes(hex), "vector #{idx} payload mismatch");
        }
    }

    /// Error‑path: inputs containing forbidden characters must fail.
    #[traced_test]
    fn reject_forbidden_characters() {
        for bad in ["bad0", "OOPS", "Ill", "l0"] {
            let mut sink = Vec::new();
            info!(?bad, "attempting to decode forbidden input");
            assert!(
                !decode_base58(bad, &mut sink, 100),
                "input ‘{bad}’ unexpectedly decoded"
            );
        }
    }

    /// Property‑style round‑trip for **random** payloads of length ≤ 64.
    #[traced_test]
    fn random_roundtrip_property() {
        let mut seed = 0xDEC0_FFEEu64;
        for case in 0..256 {
            seed = seed.wrapping_mul(0xA5A5_9665_0000_0001).rotate_left(7);
            let len = (seed & 0x3F) as usize; // 0‑63 bytes
            let mut bytes = Vec::with_capacity(len);
            for i in 0..len {
                bytes.push(((seed >> (i % 8)) & 0xFF) as u8);
            }
            let encoded = crate::encode::encode_base58(&bytes);
            let mut decoded = Vec::<u8>::new();
            assert!(
                decode_base58(&encoded, &mut decoded, (len as i32 + 4).try_into().unwrap()),
                "case {case}: failed to decode"
            );
            assert_eq!(decoded, bytes, "case {case}: round‑trip mismatch");
        }
    }
}
