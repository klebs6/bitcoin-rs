// ---------------- [ File: bitcoin-base58/src/encode.rs ]
crate::ix!();

/**
  | Encode a byte span as a base58-encoded
  | string
  |
  */
pub fn encode_base58(mut input: &[u8]) -> String {
    
    // Skip & count leading zeroes.
    let mut zeroes: usize = 0;
    let mut length: i32 = 0;

    while input.len() > 0 && input[0] == 0 {
        input = &input[1..];
        zeroes += 1;
    }

    /*
      | Allocate enough space in big-endian
      | base58 representation.
      |
      */

    // log(256) / log(58), rounded up.
    let mut size: i32 = (input.len() * 138 / 100 + 1).try_into().unwrap();

    let mut b58: Vec::<u8> = Vec::<u8>::with_capacity(size.try_into().unwrap());

    // Process the bytes.
    while input.len() > 0 {

        let mut carry: i32 = input[0].into();

        let mut i: i32 = 0;

        // Apply "b58 = b58 * 256 + ch".

        for it in b58.iter_mut().rev() {

            if carry == 0 && i >= length {
                break;
            }

            carry += 256 * ((*it) as i32);

            *it = (carry % 58).try_into().unwrap();

            carry /= 58;

            i += 1;
        }

        assert!(carry == 0);

        length = i;
        input  = &input[1..];
    }

    // Skip leading zeroes in base58 result.
    let mut it = b58.iter();

    it.advance_by((size - length).try_into().unwrap());

    while it.next() == Some(&0) {}

    // Translate the result into a string.
    let mut s: String = 
        String::with_capacity((zeroes as usize) + it.len());

    s += &"1".repeat(zeroes);

    while let Some(val) = it.next() {
        s += &String::from(PSZ_BASE58.chars().nth(*val as usize).unwrap());
    }

    s
}

#[cfg(test)]
mod encode_spec {
    use super::*;

    /// Hex‑to‑bytes helper (duplicates logic locally to avoid cross‑module deps).
    fn parse_hex(src: &str) -> Vec<u8> {
        fn nibble(b: u8) -> u8 {
            match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'f' => b - b'a' + 10,
                b'A'..=b'F' => b - b'A' + 10,
                _ => panic!("non‑hex digit {b:?}"),
            }
        }
        assert!(src.len() % 2 == 0, "odd‑length hex");
        src.as_bytes()
            .chunks_exact(2)
            .map(|c| (nibble(c[0]) << 4) | nibble(c[1]))
            .collect()
    }

    const ENCODE_VECTORS: &[(&str, &str)] = &[
        ("", ""),
        ("00", "1"),
        ("61", "2g"),
        ("626262", "a3gV"),
        ("73696d706c652e", "2cFupjhnEsSn"),
    ];

    /// Ensure `encode_base58` matches known vectors.
    #[traced_test]
    fn encode_reference_vectors() {
        for (idx, (hex, expected)) in ENCODE_VECTORS.iter().enumerate() {
            let encoded = encode_base58(&parse_hex(hex));
            debug!(idx, ?hex, ?expected, ?encoded, "verifying encode vector");
            assert_eq!(
                encoded, *expected,
                "vector #{idx}: expected {expected}, got {encoded}"
            );
        }
    }

    /// Exhaustive test of **leading‑zero preservation**.
    #[traced_test]
    fn preserves_leading_zero_bytes() {
        for zeros in 0..5 {
            let mut payload = vec![0u8; zeros];
            payload.extend_from_slice(b"data");
            let encoded = encode_base58(&payload);
            info!(zeros, ?encoded, "encoded with leading zeros");
            assert!(
                encoded.starts_with(&"1".repeat(zeros)),
                "expected {zeros} leading ‘1’ symbols"
            );
        }
    }
}
