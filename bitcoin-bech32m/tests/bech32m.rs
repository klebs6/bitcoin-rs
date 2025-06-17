use bitcoin_bech32m::*;
use bitcoin_imports::*;

// ---------------- [ File: bitcoin-bech32m/tests/bech32m.rs ]

#[traced_test]
fn bech32m_testvectors_valid() {
    static CASES: &[&str] = &[
        "A1LQFN3A",
        "a1lqfn3a",
        "an83characterlonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11sg7hg6",
        "abcdef1l7aum6echk45nj3s0wdvt2fg8x9yrzpqzd3ryx",
        "11llllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllludsr8",
        "split1checkupstagehandshakeupstreamerranterredcaperredlc445v",
        "?1v759aa",
    ];

    for &case in CASES {
        let dec = decode(&s(case));
        assert_eq!(
            *dec.encoding(),
            Encoding::BECH32M,
            "Expected BECH32M for '{}'",
            case
        );

        let recode = encode(Encoding::BECH32M, dec.hrp(), dec.data());
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
fn s(s: &str) -> String {
    s.to_owned()
}

fn case_insensitive_equal(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

// --------------------------------------------------------------------
//  Invalid Bech32m vectors (corrected UTF‑8 escapes)
// --------------------------------------------------------------------
#[traced_test]
fn bech32m_testvectors_invalid() {
    static CASES: &[&str] = &[
        " 1xj0phk",
        "\u{007f}1g6xzxy",
        "\u{0080}1vctc34",
        "an84characterslonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11d6pts4",
        "qyrz8wqd2c9m",
        "1qyrz8wqd2c9m",
        "y1b0jsk6g",
        "lt1igcx5c0",
        "in1muywd",
        "mm1crxm3i",
        "au1s5cgom",
        "M1VUXWEZ",
        "16plkw9",
        "1p2gdwpf",
    ];

    for &case in CASES {
        let dec = decode(&s(case));
        assert_eq!(
            *dec.encoding(),
            Encoding::INVALID,
            "Expected INVALID for '{}'",
            case
        );
    }
}
