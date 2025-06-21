// ---------------- [ File: bitcoin-base58/tests/base58.rs ]
use bitcoin_base58::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/test/base58_tests.cpp]

use std::sync::Once;

/// Convert an ASCII‑hex string into a `Vec<u8>`.
///
/// # Panics
///
/// * If the input length is odd.
/// * If the input contains a non‑hex digit.
fn parse_hex(src: &str) -> Vec<u8> {
    fn nibble(b: u8) -> u8 {
        match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => panic!("non‑hex digit ‘{}’", b as char),
        }
    }

    let bytes = src.as_bytes();
    assert!(
        bytes.len() % 2 == 0,
        "hex string must have an even number of digits"
    );

    (0..bytes.len())
        .step_by(2)
        .map(|i| (nibble(bytes[i]) << 4) | nibble(bytes[i + 1]))
        .collect()
}

/// Xor‑shift‑64* “insecure” PRNG used to mimic the C++
/// test‑suite’s deterministic random generator.
struct InsecureRand {
    state: u64,
}

impl InsecureRand {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        // Recommended multiplicative constant for xorshift64*
        (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }

    fn bool(&mut self) -> bool {
        self.next_u32() & 1 != 0
    }

    fn range(&mut self, upper: usize) -> usize {
        (self.next_u32() as usize) % upper
    }
}

/// Bitcoin Core reference vectors for plain Base‑58.
const ENCODE_DECODE_VECTORS: &[(&str, &str)] = &[
    ("", ""),
    ("61", "2g"),
    ("626262", "a3gV"),
    ("636363", "aPEr"),
    ("73696d706c792061206c6f6e6720737472696e67", "2cFupjhnEsSn59qHXstmK2ffpLv2"),
    ("00eb15231dfceb60925886b67d065299925915aeb172c06647", "1NS17iag9jJgTHD1VXjvLCEnZuQ3rJDE9L"),
    ("516b6fcd0f", "ABnLTmg"),
    ("bf4f89001e670274dd", "3SEo3LWLoPntC"),
    ("572e4794", "3EFU7m"),
    ("ecac89cad93923c02321", "EJDM8drfXA6uyA"),
    ("10c8511e", "Rt5zm"),
    ("00000000000000000000", "1111111111"),
];

/// Ensure **encoding** matches the official vectors.
#[traced_test]
fn base58_encode_reference_vectors() {
    for (idx, (hex, expected)) in ENCODE_DECODE_VECTORS.iter().enumerate() {
        let encoded = encode_base58(&parse_hex(hex));
        info!(idx, ?hex, ?expected, ?encoded, "encode vector");
        assert_eq!(
            encoded, *expected,
            "vector #{idx} failed: got {encoded}, expected {expected}"
        );
    }
}

/// Verify **decoding** (including edge‑cases and whitespace handling).
#[traced_test]
fn base58_decode_reference_vectors_and_edge_cases() {
    // Reference vectors
    for (idx, (hex, b58)) in ENCODE_DECODE_VECTORS.iter().enumerate() {
        let mut out = Vec::new();
        assert!(
            decode_base58(b58, &mut out, 256),
            "vector #{idx} (‘{b58}’) failed to decode"
        );
        assert_eq!(
            out,
            parse_hex(hex),
            "vector #{idx} produced incorrect payload"
        );
    }

    // Basic invalid strings
    let mut v = Vec::new();
    assert!(!decode_base58("invalid", &mut v, 100));
    assert!(!decode_base58("invalid\0", &mut v, 100));
    assert!(!decode_base58("\0invalid", &mut v, 100));

    // Forbidden characters
    assert!(decode_base58("good", &mut v, 100));
    assert!(!decode_base58("bad0IOl", &mut v, 100));
    assert!(!decode_base58("goodbad0IOl", &mut v, 100));
    assert!(!decode_base58("good\0bad0IOl", &mut v, 100));

    // Whitespace handling (note: \x0B = VT, \x0C = FF)
    assert!(!decode_base58(" \t\n\x0B\x0C\r skip \r\x0C\x0B\n\t a", &mut v, 3));
    assert!( decode_base58(" \t\n\x0B\x0C\r skip \r\x0C\x0B\n\t ", &mut v, 3));
    assert_eq!(v, parse_hex("971a55"));

    // Base‑58‑check vectors
    v.clear();
    assert!(decode_base_58check("3vQB7B6MrGQZaxCuFg4oh", &mut v, 100));
    assert!(!decode_base_58check("3vQB7B6MrGQZaxCuFg4oi", &mut v, 100));
    assert!(!decode_base_58check("3vQB7B6MrGQZaxCuFg4oh0IOl", &mut v, 100));
    assert!(!decode_base_58check(
        concat!("3vQB7B6MrGQZaxCuFg4oh", "\0", "0IOl"),
        &mut v,
        100
    ));
}

/// Randomised round‑trip tests (mirrors Bitcoin Core logic).
#[traced_test]
fn base58_random_encode_decode_roundtrip() {
    let mut rng = InsecureRand::new(0x20_25_11_05_15);
    for n in 0..1_000 {
        let len = 1 + (rng.next_u32() & 0xFF) as usize;
        let zeroes = if rng.bool() { rng.range(len + 1) } else { 0 };

        let mut data = vec![0u8; zeroes];
        for _ in zeroes..len {
            data.push(rng.next_u32() as u8);
        }

        trace!(n, len, zeroes, "generated random payload");
        let encoded = encode_base_58check(&data);

        // Too‑small max length → must fail
        let mut decoded = Vec::new();
        let too_small = rng.range(len);
        assert!(
            !decode_base_58check(&encoded, &mut decoded, too_small as i32),
            "case {n}: unexpectedly succeeded with max_len={too_small}"
        );

        // Adequate max length → succeed and round‑trip
        decoded.clear();
        let ok_limit = len + rng.range(257 - len);
        assert!(
            decode_base_58check(&encoded, &mut decoded, ok_limit as i32),
            "case {n}: failed with max_len={ok_limit}"
        );
        assert_eq!(decoded, data, "case {n}: round‑trip mismatch");
    }
}
