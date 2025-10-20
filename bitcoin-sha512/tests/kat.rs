// ---------------- [ File: bitcoin-sha512/tests/kat.rs ]
use bitcoin_sha512::*;
use bitcoin_imports::*;
use hex::decode;

fn sha512_of(data: &[u8]) -> [u8; 64] {
    let mut h = Sha512::new();
    h.write(data.as_ptr(), data.len());
    let mut out = [0u8; 64];
    h.finalize(&mut out);
    out
}

fn hex(s: &str) -> Vec<u8> {
    let s = s.split_whitespace().collect::<String>();
    decode(s).expect("hex")
}

#[traced_test]
fn kat_empty() {
    // SHA-512("")
    let expect = hex("cf83e1357eefb8bd f1542850d66d8007 d620e4050b5715dc \
                      83f4a921d36ce9ce 47d0d13c5d85f2b0 ff8318d2877eec2f \
                      63b931bd47417a81 a538327af927da3e");
    assert_eq!(sha512_of(b""), expect.as_slice());
}

#[traced_test]
fn kat_abc() {
    // SHA-512("abc")
    let expect = hex("ddaf35a193617aba cc417349ae204131 12e6fa4e89a97ea2 \
                      0a9eeee64b55d39a 2192992a274fc1a8 36ba3c23a3feebbd \
                      454d4423643ce80e 2a9ac94fa54ca49f");
    assert_eq!(sha512_of(b"abc"), expect.as_slice());
}

#[traced_test]
fn kat_quick_brown_no_dot() {
    // "The quick brown fox jumps over the lazy dog"
    let expect = hex("07e547d9586f6a73 f73fbac0435ed769 51218fb7d0c8d788 \
                      a309d785436bbb64 2e93a252a954f239 12547d1e8a3b5ed6 \
                      e1bfd7097821233f a0538f3db854fee6");
    assert_eq!(
        sha512_of(b"The quick brown fox jumps over the lazy dog"),
        expect.as_slice()
    );
}

#[traced_test]
fn kat_quick_brown_with_dot() {
    // "The quick brown fox jumps over the lazy dog."
    let expect = hex("91ea1245f20d46ae 9a037a989f54f1f7 90f0a47607eeb8a1 \
                      4d12890cea77a1bb c6c7ed9cf205e67b 7f2b8fd4c7dfd3a7 \
                      a8617e45f3c463d4 81c7e586c39ac1ed");
    assert_eq!(
        sha512_of(b"The quick brown fox jumps over the lazy dog."),
        expect.as_slice()
    );
}

#[traced_test]
fn kat_abcdbcdecdef() {
    // "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
    let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    let expect = hex("204a8fc6dda82f0a 0ced7beb8e08a416 57c16ef468b228a8 \
                      279be331a703c335 96fd15c13b1b07f9 aa1d3bea57789ca0 \
                      31ad85c7a71dd703 54ec631238ca3445");
    assert_eq!(sha512_of(msg), expect.as_slice());
}

#[traced_test]
fn kat_long_896_bits() {
    // "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
    let msg = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmno\
                ijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
    let expect = hex("8e959b75dae313da 8cf4f72814fc143f 8f7779c6eb9f7fa1 \
                      7299aeadb6889018 501d289e4900f7e4 331b99dec4b5433a \
                      c7d329eeb6dd2654 5e96e55b874be909");
    assert_eq!(sha512_of(msg), expect.as_slice());
}

#[traced_test]
fn kat_million_a() {
    // one million 'a' (0x61)
    let msg = vec![0x61u8; 1_000_000];
    let expect = hex("e718483d0ce76964 4e2e42c7bc15b463 8e1f98b13b204428 \
                      5632a803afa973eb de0ff244877ea60a 4cb0432ce577c31b \
                      eb009c5c2c49aa2e 4eadb217ad8cc09b");
    assert_eq!(sha512_of(&msg), expect.as_slice());
}
