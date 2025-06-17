// ---------------- [ File: bitcoin-bech32m/tests/bech32.rs ]
use bitcoin_bech32m::*;
use bitcoin_imports::*;


//-------------------------------------------[.cpp/bitcoin/src/test/bech32_tests.cpp]
// ----------------------------------------------------------------
//  Canonical BIP‑173 / BIP‑350 test vectors
// ----------------------------------------------------------------
//  NOTE: These mirror the upstream Core vectors exactly.  When the
//  implementation is fully completed the vectors below *must* pass
//  without panicking.  Any panic indicates a regression.
// ----------------------------------------------------------------

#[traced_test]
fn bech32_testvectors_valid() {
    static CASES: &[&str] = &[
        "A12UEL5L",
        "a12uel5l",
        "an83characterlonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1tt5tgs",
        "abcdef1qpzry9x8gf2tvdw0s3jn54khce6mua7lmqqqxw",
        "11qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqc8247j",
        "split1checkupstagehandshakeupstreamerranterredcaperred2y9e3w",
        "?1ezyfcl",
    ];

    for &case in CASES {
        let dec = decode(&case);
        assert_eq!(
            *dec.encoding(),
            Encoding::BECH32,
            "Expected BECH32 for '{}'",
            case
        );

        let recode = encode(Encoding::BECH32, dec.hrp(), dec.data());
        assert!(
            !recode.is_empty(),
            "Round‑trip encode produced empty string for '{}'",
            case
        );
        assert!(
            case_insensitive_equal(case, &recode),
            "Case‑insensitive mismatch: original '{}' vs recode '{}'",
            case,
            recode
        );
    }
}

// --------------------------------------------------------------------
//  Local helpers (needed only in this file)
// --------------------------------------------------------------------
fn case_insensitive_equal(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

// --------------------------------------------------------------------
//  Invalid Bech32 vectors (corrected UTF‑8 escapes)
// --------------------------------------------------------------------
#[traced_test]
fn bech32_testvectors_invalid() {
    static CASES: &[&str] = &[
        " 1nwldj5",
        "\u{007f}1axkwrx",
        "\u{0080}1eym55h",
        "an84characterslonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1569pvx",
        "pzry9x0s0muk",
        "1pzry9x0s0muk",
        "x1b4n0q5v",
        "li1dgmt3",
        "de1lg7wt\u{00ff}",
        "A1G7SGD8",
        "10a06t8",
        "1qzzfhee",
        "a12UEL5L",
        "A12uEL5L",
    ];

    for &case in CASES {
        let dec = decode(&case);
        assert_eq!(
            *dec.encoding(),
            Encoding::INVALID,
            "Expected INVALID for '{}'",
            case
        );
    }
}
