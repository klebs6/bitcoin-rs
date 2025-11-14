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
            !decode_base_58check(&encoded, &mut decoded, (too_small as i32).try_into().unwrap()),
            "case {n}: unexpectedly succeeded with max_len={too_small}"
        );

        // Adequate max length → succeed and round‑trip
        decoded.clear();
        let ok_limit = len + rng.range(257 - len);
        assert!(
            decode_base_58check(&encoded, &mut decoded, (ok_limit as i32).try_into().unwrap()),
            "case {n}: failed with max_len={ok_limit}"
        );
        assert_eq!(decoded, data, "case {n}: round‑trip mismatch");
    }
}

#[traced_test]
fn base58_decode_reference_vectors_and_edge_cases_diagnostic() {
    use core::cmp::min;
    use tracing::{debug, error, info, trace, warn};

    // Reference vector from the Bitcoin Wiki and multiple independent implementations:
    // Base58Check("hello world") == "3vQB7B6MrGQZaxCuFg4oh"
    let reference_b58 = "3vQB7B6MrGQZaxCuFg4oh";
    let expected_payload: &[u8] = b"hello world";

    info!(
        target: "base58",
        reference_b58 = %reference_b58,
        "===== BEGIN_TEST: base58_decode_reference_vectors_and_edge_cases_diagnostic ====="
    );

    // Step 1: Exercise the public Base58Check decoding interface.
    let mut out_payload = Vec::with_capacity(100);
    let ok = decode_base_58check(reference_b58, &mut out_payload, 100);

    info!(
        target: "base58",
        input_len = reference_b58.len(),
        ok,
        out_len = out_payload.len(),
        payload_hex = %hex::encode(&out_payload),
        "decode_base_58check completed for reference vector"
    );

    // Fast path: if everything matches the spec example, assert and exit early.
    if ok && out_payload.as_slice() == expected_payload {
        info!(
            target: "base58",
            payload_hex = %hex::encode(&out_payload),
            "Base58Check reference vector decoded exactly to \"hello world\"."
        );
        info!(
            target: "base58",
            "===== END_TEST: base58_decode_reference_vectors_and_edge_cases_diagnostic ====="
        );
        assert_eq!(
            out_payload,
            expected_payload.to_vec(),
            "payload mismatch for Base58Check reference vector"
        );
        return;
    }

    // If we are here, something is off; run detailed diagnostics using only public APIs.

    warn!(
        target: "base58",
        ok,
        decoded_payload_hex = %hex::encode(&out_payload),
        expected_payload_hex = %hex::encode(expected_payload),
        "Base58Check reference vector did not decode as expected; beginning diagnostic steps."
    );

    // Step 2: Decode without checksum to inspect raw bytes (payload || checksum).
    let mut raw_ref = Vec::with_capacity(100);
    let raw_ok = decode_base58(reference_b58, &mut raw_ref, 100);
    assert!(
        raw_ok,
        "raw base58 decode of the reference string failed; cannot diagnose checksum"
    );
    assert!(
        raw_ref.len() >= 4,
        "decoded reference byte length < 4: cannot contain checksum (len = {})",
        raw_ref.len()
    );

    let split_ref = raw_ref.len() - 4;
    let (payload_ref, checksum_ref) = raw_ref.split_at(split_ref);

    debug!(
        target: "base58",
        raw_len = raw_ref.len(),
        payload_len = payload_ref.len(),
        checksum_len = checksum_ref.len(),
        payload_hex = %hex::encode(payload_ref),
        checksum_hex = %hex::encode(checksum_ref),
        "Reference Base58 decoded to payload || checksum."
    );

    // Step 3: Use the library to produce its own Base58Check string for the decoded payload,
    // then decode it (without check) to inspect the library's checksum bytes.
    let lib_b58 = encode_base_58check(payload_ref);
    info!(
        target: "base58",
        lib_b58_len = lib_b58.len(),
        lib_b58_preview = %lib_b58,
        "Library Base58Check encoding of the decoded payload."
    );

    let mut raw_lib = Vec::with_capacity(100);
    let lib_raw_ok = decode_base58(&lib_b58, &mut raw_lib, 100);
    assert!(
        lib_raw_ok && raw_lib.len() >= 4,
        "library Base58Check output could not be decoded back into payload || checksum"
    );

    let split_lib = raw_lib.len() - 4;
    let (payload_lib, checksum_lib) = raw_lib.split_at(split_lib);

    trace!(
        target: "base58",
        lib_raw_len = raw_lib.len(),
        lib_payload_len = payload_lib.len(),
        lib_checksum_len = checksum_lib.len(),
        lib_payload_hex = %hex::encode(payload_lib),
        lib_checksum_hex = %hex::encode(checksum_lib),
        "Decoded library Base58Check back into payload || checksum."
    );

    // Step 4: Compare checksums (direct and reversed) to detect byte-order issues.
    let eq_direct = checksum_ref == checksum_lib;
    let mut checksum_lib_reversed = checksum_lib.to_vec();
    checksum_lib_reversed.reverse();
    let eq_reversed = checksum_ref == checksum_lib_reversed.as_slice();

    warn!(
        target: "base58",
        ref_checksum = %hex::encode(checksum_ref),
        lib_checksum = %hex::encode(checksum_lib),
        lib_checksum_reversed = %hex::encode(&checksum_lib_reversed),
        eq_direct,
        eq_reversed,
        "Comparing reference checksum with checksum derived from library Base58Check."
    );

    if eq_reversed && !eq_direct {
        error!(
            target: "base58",
            "Checksum mismatch is consistent with byte-order (endianness) error: \
            reference checksum matches reversed library checksum."
        );
    } else if !eq_direct && !eq_reversed {
        let diff_len = min(checksum_ref.len(), checksum_lib.len());
        let mut diff_bytes = Vec::with_capacity(diff_len);
        for i in 0..diff_len {
            diff_bytes.push(checksum_ref[i] ^ checksum_lib[i]);
        }

        warn!(
            target: "base58",
            checksum_xor = %hex::encode(&diff_bytes),
            "Checksum bytes differ in a non-trivial way (not just reversal)."
        );

        error!(
            target: "base58",
            "Checksum mismatch suggests a divergence from the Base58Check specification \
            (double-SHA256 of payload, first 4 bytes)."
        );
    }

    // Step 5: Final assertion – this test should only ever fail with rich diagnostics emitted above.
    error!(
        target: "base58",
        ok,
        final_decoded_payload_hex = %hex::encode(&out_payload),
        expected_payload_hex = %hex::encode(expected_payload),
        "Base58Check reference vector did not round-trip to \"hello world\"."
    );

    info!(
        target: "base58",
        "===== END_TEST: base58_decode_reference_vectors_and_edge_cases_diagnostic ====="
    );

    assert!(
        ok && out_payload.as_slice() == expected_payload,
        "Base58Check reference vector did not decode to the expected payload \"hello world\"; \
        see tracing output for detailed diagnostics."
    );
}
