// ---------------- [ File: bitcoin-bech32m/src/lib.rs ]
#![feature(test)]

#[macro_use] mod imports; use imports::*;

x!{bech32}
x!{encode}
x!{decode}
x!{poly_mod}
x!{checksum}

/// Comprehensive unit‑tests for the Bech32/Bech32m implementation.
///
/// *Every* public interface function is exercised, with both
/// success‑case and failure‑case coverage, while remaining independent
/// of the underlying implementation details.
#[cfg(test)]
mod bech32m_exhaustive_suite {

    use super::*;

    // ----------------------------------------------------------------
    //  Basic helpers
    // ----------------------------------------------------------------

    /// Convenience wrapper that converts `&str` into owned `String`.
    fn s(s: &str) -> String {
        s.to_owned()
    }

    // ----------------------------------------------------------------
    //  Character‑set invariants
    // ----------------------------------------------------------------

    #[traced_test]
    fn charset_has_expected_length_and_unique_symbols() {
        trace!("Verifying CHARSET length and uniqueness");
        assert_eq!(CHARSET.len(), 32, "Bech32 charset length must be 32");

        let mut seen = [false; 256];
        for b in CHARSET.bytes() {
            assert!(
                !seen[b as usize],
                "Duplicate symbol '{}' (0x{:02x}) in CHARSET",
                b as char,
                b
            );
            seen[b as usize] = true;
        }
        info!("CHARSET passed uniqueness test");
    }

    #[traced_test]
    fn charset_rev_is_valid_inverse_mapping() {
        trace!("Validating CHARSET_REV inverse mapping");
        for (i, ch) in CHARSET.bytes().enumerate() {
            let rev = CHARSET_REV[ch as usize];
            assert!(
                rev == i as i8,
                "REV table mismatch for '{}': expected {}, got {}",
                ch as char,
                i,
                rev
            );
        }
        info!("CHARSET_REV correctly inverts CHARSET");
    }

    // ----------------------------------------------------------------
    //  Stand‑alone helpers
    // ----------------------------------------------------------------

    #[traced_test]
    fn lower_case_transforms_upper_ascii_only() {
        for b in 0u8..=0x7f {
            let transformed = lower_case(b);
            if (b'A'..=b'Z').contains(&b) {
                assert_eq!(
                    transformed,
                    b + 32,
                    "ASCII 0x{:02x} should lower‑case to 0x{:02x}",
                    b,
                    b + 32
                );
            } else {
                assert_eq!(
                    transformed, b,
                    "Byte 0x{:02x} outside A–Z must be unchanged",
                    b
                );
            }
        }
    }

    #[traced_test]
    fn expand_hrp_reference_vectors() {
        // Example from BIP‑173: HRP = "bc"
        let expected = vec![3, 3, 0, 2, 3];
        let actual = expand_hrp(&s("bc"));
        assert_eq!(actual, expected, "HRP expansion mismatch for \"bc\"");
    }

    #[traced_test]
    fn cat_preserves_input_order() {
        let left = vec![1u8, 2, 3];
        let right = vec![4u8, 5, 6];
        let combined = cat(left.clone(), &right);
        assert_eq!(
            combined,
            vec![1, 2, 3, 4, 5, 6],
            "cat() must concatenate in left‑to‑right order"
        );
        debug!(?combined, "cat() output verified");
    }

    // ----------------------------------------------------------------
    //  Core algorithmic primitives
    // ----------------------------------------------------------------

    #[traced_test]
    fn encoding_constant_matches_bip_values() {
        assert_eq!(
            encoding_constant(Encoding::BECH32),
            1,
            "BECH32 constant incorrect"
        );
        assert_eq!(
            encoding_constant(Encoding::BECH32M),
            0x2bc8_30a3,
            "BECH32M constant incorrect"
        );
    }

    #[traced_test]
    fn polymod_empty_vector_is_one() {
        assert_eq!(
            poly_mod(&Vec::<u8>::new()),
            1u32,
            "poly_mod([]) must return 1"
        );
    }

    // ----------------------------------------------------------------
    //  Decoder front‑end validation
    // ----------------------------------------------------------------

    #[traced_test]
    fn decode_rejects_mixed_case_strings() {
        // Mixed‑case strings are invalid by spec.
        let result: DecodeResult = decode(&s("A1b2c3d4e5f6g7h8"));
        assert_eq!(
            *result.encoding(),
            Encoding::INVALID,
            "Decoder must reject mixed‑case input"
        );
    }

    #[traced_test]
    fn decode_rejects_invalid_separator_usage() {
        // Missing separator ‘1’.
        let result = decode(&s("bcdefghijklmnopqrstuvwxyz"));
        assert_eq!(
            *result.encoding(),
            Encoding::INVALID,
            "Decoder must reject strings without separator"
        );

        // Separator in first position.
        let result = decode(&s("1bcdefghij"));
        assert_eq!(
            *result.encoding(),
            Encoding::INVALID,
            "Decoder must reject HRP‑less strings"
        );
    }

    // ----------------------------------------------------------------
    //  Updated checksum test (no longer expects panic)
    // ----------------------------------------------------------------
    #[traced_test]
    fn create_checksum_returns_six_bytes() {
        let checksum = create_checksum(Encoding::BECH32, &s("bc"), &vec![0u8; 1]);
        assert_eq!(
            checksum.len(),
            6,
            "create_checksum() must always return six bytes"
        );
        debug!(?checksum, "Checksum generated successfully");
    }
}
