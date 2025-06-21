// ---------------- [ File: bitcoin-string/tests/base32.rs ]
use bitcoin_imports::*;
use bitcoin_string::*;

//-------------------------------------------[.cpp/bitcoin/src/test/base32_tests.cpp]
/// Reference vectors taken verbatim from the upstream C++ test‑suite.
#[traced_test]
fn base32_testvectors() {
    const VSTR_IN: [&str; 7]  = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
    const VSTR_OUT: [&str; 7] = ["", "my======", "mzxq====", "mzxw6===", "mzxw6yq=", "mzxw6ytb", "mzxw6ytboi======"];
    const VSTR_OUT_NOPAD: [&str; 7] = ["", "my", "mzxq", "mzxw6", "mzxw6yq", "mzxw6ytb", "mzxw6ytboi"];

    for i in 0..VSTR_IN.len() {
        let enc = encode_base32_bytes(VSTR_IN[i].as_bytes(), Some(true));
        assert_eq!(enc, VSTR_OUT[i], "pad = true, i = {i}");

        let enc_no_pad = encode_base32_bytes(VSTR_IN[i].as_bytes(), Some(false));
        assert_eq!(enc_no_pad, VSTR_OUT_NOPAD[i], "pad = false, i = {i}");

        let dec = decode_base32(VSTR_OUT[i], None);
        assert_eq!(dec, VSTR_IN[i], "round‑trip i = {i}");
    }

    /* embedded‑NUL / size‑check cases */
    let mut failure = false;
    let _ = decode_base32("invalid\0", Some(&mut failure as *mut bool));
    assert!(failure, "embedded‑NUL must set failure flag");

    failure = false;
    let _ = decode_base32("AWSX3VPP", Some(&mut failure as *mut bool));
    assert!(!failure, "valid input must clear failure flag");

    failure = false;
    let _ = decode_base32("AWSX3VPP\0invalid", Some(&mut failure as *mut bool));
    assert!(failure);

    failure = false;
    let _ = decode_base32("AWSX3VPPinvalid", Some(&mut failure as *mut bool));
    assert!(failure);
}
