// ---------------- [ File: bitcoin-hmac-sha512/tests/rfc4231.rs ]
use bitcoin_hmac_sha512::*;
use bitcoin_imports::*;
use hex::decode;

fn mac(key: &[u8], data: &[u8]) -> [u8; 64] {
    let mut h = HmacSha512::new(key.as_ptr(), key.len());
    h.write(data.as_ptr(), data.len());
    let mut out = [0u8; 64];
    h.finalize(&mut out);
    out
}

fn hex(s: &str) -> Vec<u8> {
    let s = s.split_whitespace().collect::<String>();
    decode(s).expect("hex")
}

// --- RFC 4231 Test Case 1
#[traced_test]
fn rfc4231_tc1() {
    let key = hex("0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b 0b");
    let data = hex("4869205468657265"); // "Hi There"
    let expect = hex(concat!(
        "87aa7cdea5ef619d4ff0b4241a1d6cb0",
        "2379f4e2ce4ec2787ad0b30545e17cde",
        "daa833b7d6b8a702038b274eaea3f4e4",
        "be9d914eeb61f1702e696c203a126854"
    ));
    assert_eq!(mac(&key, &data), expect.as_slice());
}

// --- RFC 4231 Test Case 2 ("Jefe")
#[traced_test]
fn rfc4231_tc2() {
    let key = hex("4a656665");
    let data = hex("7768617420646f2079612077616e7420\
                    666f72206e6f7468696e673f");
    let expect = hex(concat!(
        "164b7a7bfcf819e2e395fbe73b56e0a3",
        "87bd64222e831fd610270cd7ea250554",
        "9758bf75c05a994a6d034f65f8f0e6fd",
        "caeab1a34d4a6b4b636e070a38bce737"
    ));
    assert_eq!(mac(&key, &data), expect.as_slice());
}

// --- RFC 4231 Test Case 3 (50 x 0xdd with 20 x 0xaa key)
#[traced_test]
fn rfc4231_tc3() {
    let key = vec![0xaa; 20];
    let data = vec![0xdd; 50];
    let expect = hex(concat!(
        "fa73b0089d56a284efb0f0756c890be9",
        "b1b5dbdd8ee81a3655f83e33b2279d39",
        "bf3e848279a722c806b485a47e67c807",
        "b946a337bee8942674278859e13292fb"
    ));
    assert_eq!(mac(&key, &data), expect.as_slice());
}

// --- RFC 4231 Test Case 4
#[traced_test]
fn rfc4231_tc4() {
    let key = hex("0102030405060708090a0b0c0d0e0f10\
                   111213141516171819");
    let data = vec![0xcd; 50];
    let expect = hex(concat!(
        "b0ba465637458c6990e5a8c5f61d4af7",
        "e576d97ff94b872de76f8050361ee3db",
        "a91ca5c11aa25eb4d679275cc5788063",
        "a5f19741120c4f2de2adebeb10a298dd"
    ));
    assert_eq!(mac(&key, &data), expect.as_slice());
}

// --- RFC 4231 Test Case 5 (truncate to 128 bits -> compare prefix)
#[traced_test]
fn rfc4231_tc5_truncated128() {
    let key = vec![0x0c; 20];
    let data = b"Test With Truncation";
    let full = mac(&key, data);
    let expect_128 = hex("415fad6271580a531d4179bc891d87a6");
    assert_eq!(&full[..16], expect_128.as_slice());
}

// --- RFC 4231 Test Case 6 (key > 128 bytes)
#[traced_test]
fn rfc4231_tc6() {
    let key = vec![0xaa; 131];
    let data = b"Test Using Larger Than Block-Size Key - Hash Key First";
    let expect = hex(concat!(
        "80b24263c7c1a3ebb71493c1dd7be8b4",
        "9b46d1f41b4aeec1121b013783f8f352",
        "6b56d037e05f2598bd0fd2215d6a1e52",
        "95e64f73f63f0aec8b915a985d786598"
    ));
    assert_eq!(mac(&key, data), expect.as_slice());
}

// --- RFC 4231 Test Case 7 (key+data > 128)
#[traced_test]
fn rfc4231_tc7() {
    let key = vec![0xaa; 131];

    let data = concat!(
        "This is a test using a larger than block-size key and a larger than block-size data.",
        " The key needs to be hashed before being used by the HMAC algorithm."
    ).as_bytes();

    assert!(data.windows(b". The".len()).any(|w| w == b". The"), "TC7 message must contain \". The\" (space after the period)");

    let expect = hex(concat!(
        "e37b6a775dc87dbaa4dfa9f96e5e3ffd",
        "debd71f8867289865df5a32d20cdc944",
        "b6022cac3c4982b10d5eeb55c3e4de15",
        "134676fb6de0446065c97440fa8c6a58"
    ));
    assert_eq!(mac(&key, data), expect.as_slice());
}
