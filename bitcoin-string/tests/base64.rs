// ---------------- [ File: bitcoin-string/tests/base64.rs ]
use bitcoin_imports::*;
use bitcoin_string::*;

//-------------------------------------------[.cpp/bitcoin/src/test/base64_tests.cpp]

/// Reference vectors for Base64 taken verbatim from the upstream C++ test‑suite.
#[traced_test]
fn base64_testvectors() {
    const VSTR_IN: [&str; 7]  = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
    const VSTR_OUT: [&str; 7] = ["", "Zg==", "Zm8=", "Zm9v", "Zm9vYg==", "Zm9vYmE=", "Zm9vYmFy"];

    for i in 0..VSTR_IN.len() {
        let enc = encode_base64_bytes(VSTR_IN[i].as_bytes());
        assert_eq!(enc, VSTR_OUT[i], "encode i = {i}");

        let dec = decode_base64(&enc, None);
        assert_eq!(dec, VSTR_IN[i], "round‑trip i = {i}");
    }

    /* embedded‑NUL / size‑check cases */
    let mut failure = false;
    let _ = decode_base64("invalid\0", Some(&mut failure));
    assert!(failure);

    failure = false;
    let _ = decode_base64("nQB/pZw=", Some(&mut failure));
    assert!(!failure);

    failure = false;
    let _ = decode_base64("nQB/pZw=\0invalid", Some(&mut failure));
    assert!(failure);

    failure = false;
    let _ = decode_base64("nQB/pZw=invalid\0", Some(&mut failure));
    assert!(failure);
}
