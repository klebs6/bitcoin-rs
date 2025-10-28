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

    // Reference vector from the Bitcoin Wiki:
    // Base58Check("Hello World") == "3vQB7B6MrGQZaxCuFg4oh"
    let reference_b58 = "3vQB7B6MrGQZaxCuFg4oh";
    let mut out_payload = Vec::with_capacity(100);

    info!(
        target: "base58",
        "===== BEGIN_TEST: base58_decode_reference_vectors_and_edge_cases_diagnostic ====="
    );

    // Step 1: Exercise the public interface that is failing.
    let ok = decode_base_58check(reference_b58, &mut out_payload, 100);
    info!(
        target: "base58",
        input_len = reference_b58.len(),
        ok,
        out_len = out_payload.len(),
        "decode_base_58check return observed"
    );

    // If it worked, verify the decoded payload is exactly "Hello World" and return early.
    if ok {
        assert_eq!(
            out_payload,
            b"Hello World".to_vec(),
            "payload mismatch for Base58Check reference vector"
        );
        info!(target: "base58", "Reference vector decoded successfully.");
        return;
    }

    // Step 2: Decode *without* checksum to inspect raw bytes (payload || checksum).
    // We test only via public interfaces here.
    let mut raw = Vec::with_capacity(100);
    let decoded = decode_base58(reference_b58, &mut raw, 100);
    assert!(
        decoded,
        "raw base58 decode of the reference string failed; cannot diagnose checksum"
    );
    assert!(
        raw.len() >= 4,
        "decoded byte length < 4: cannot contain checksum (len = {})",
        raw.len()
    );

    let split = raw.len() - 4;
    let (payload_ref, checksum_ref) = raw.split_at(split);

    debug!(
        target: "base58",
        payload_len = payload_ref.len(),
        checksum_len = 4usize,
        payload_hex = %hex::encode(payload_ref),
        checksum_hex = %hex::encode(checksum_ref),
        "Split reference bytes into payload and checksum."
    );

    // Step 3: Ask the library to produce its own Base58Check string for *the same payload*,
    // then decode it (without check) to extract the library's checksum bytes. This pins down
    // whether the mismatch is due to byte-ordering or an entirely different checksum.
    let lib_b58 = encode_base_58check(payload_ref);
    info!(
        target: "base58",
        lib_b58_len = lib_b58.len(),
        lib_b58_preview = %lib_b58,
        "Library Base58Check encoding for the same payload."
    );

    let mut raw_lib = Vec::with_capacity(100);
    let decoded_lib = decode_base58(&lib_b58, &mut raw_lib, 100);
    assert!(
        decoded_lib && raw_lib.len() >= 4,
        "library Base58Check output failed to decode back to bytes"
    );
    let lib_split = raw_lib.len() - 4;
    let (_payload_lib, checksum_lib) = raw_lib.split_at(lib_split);

    trace!(
        target: "base58",
        lib_raw_len = raw_lib.len(),
        lib_payload_len = _payload_lib.len(),
        lib_checksum_hex = %hex::encode(checksum_lib),
        "Decoded library Base58Check back to raw to inspect checksum bytes."
    );

    // Step 4: Compare checksums in several ways to precisely identify the failure mode.
    let eq_direct = checksum_ref == checksum_lib;
    let mut lib_rev = [0u8; 4];
    lib_rev.copy_from_slice(checksum_lib);
    lib_rev.reverse();
    let eq_reversed = checksum_ref == lib_rev;

    warn!(
        target: "base58",
        ref_checksum = %hex::encode(checksum_ref),
        lib_checksum = %hex::encode(checksum_lib),
        lib_checksum_reversed = %hex::encode(lib_rev),
        eq_direct,
        eq_reversed,
        ref_b58 = %reference_b58,
        lib_b58 = %lib_b58,
        "Checksum comparison across reference and library encodings."
    );

    // Step 5: Provide a crisp assertion that explains exactly what's wrong.
    // If the reversed comparison matches, we flag a byte-order/endianness defect.
    if eq_reversed && !eq_direct {
        error!(
            target: "base58",
            "Checksum mismatch is consistent with byte-order (endianness) error: \
             reference checksum matches the library checksum when reversed."
        );
        assert!(
            eq_direct,
            "Base58Check verification failed: checksum byte-order mismatch detected \
             (reference == reverse(library))."
        );
    }

    // If neither direct nor reversed matches, the algorithm used to form the checksum
    // does not match the Base58Check spec (double-SHA256 of the payload, take the first 4 bytes as-is).
    if !eq_direct && !eq_reversed {
        // Show a short hex diff to make it easy to see where things diverge.
        let n = min(4usize, checksum_ref.len());
        let ref_bytes = &checksum_ref[..n];
        let lib_bytes = &checksum_lib[..n];
        error!(
            target: "base58",
            ref_first4 = %hex::encode(ref_bytes),
            lib_first4 = %hex::encode(lib_bytes),
            "Checksum bytes differ even after endianness check — likely wrong hashing procedure \
             (not SHA256d over payload, or extra/missing bytes hashed)."
        );
        assert!(
            eq_direct,
            "Base58Check verification failed: library checksum does not match reference, and does \
             not match in reverse order either (suspect wrong SHA256d computation or hashed region)."
        );
    }

    // Final, explicit failure mirroring the original expectation but after detailed diagnostics.
    assert!(
        ok,
        "decode_base_58check(\"{reference_b58}\") should succeed but returned false (see tracing output above for diagnostics)."
    );
}
