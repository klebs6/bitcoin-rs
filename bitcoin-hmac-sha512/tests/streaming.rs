// ---------------- [ File: bitcoin-hmac-sha512/tests/streaming.rs ]
use bitcoin_hmac_sha512::*;
use bitcoin_imports::*;
use proptest::prelude::*;
use hmac::{Mac, Hmac};
use sha2::Sha512 as RefSha512;

fn ours(key: &[u8], data: &[u8]) -> [u8; 64] {
    let mut h = HmacSha512::new(key.as_ptr(), key.len());
    h.write(data.as_ptr(), data.len());
    let mut out = [0u8; 64];
    h.finalize(&mut out);
    out
}

fn ours_chunked(key: &[u8], data: &[u8], chunks: &[usize]) -> [u8; 64] {
    let mut h = HmacSha512::new(key.as_ptr(), key.len());
    let mut pos = 0usize;
    for &len in chunks {
        let end = (pos + len).min(data.len());
        if end > pos {
            let part = &data[pos..end];
            h.write(part.as_ptr(), part.len());
        }
        pos = end;
        if pos == data.len() { break; }
    }
    if pos < data.len() {
        let part = &data[pos..];
        h.write(part.as_ptr(), part.len());
    }
    let mut out = [0u8; 64];
    h.finalize(&mut out);
    out
}

fn ref_hmac(key: &[u8], data: &[u8]) -> [u8; 64] {
    let mut m = <Hmac<RefSha512>>::new_from_slice(key).unwrap();
    m.update(data);
    let tag = m.finalize().into_bytes();
    let mut out = [0u8; 64];
    out.copy_from_slice(&tag);
    out
}

proptest! {
    // keys up to ~200 bytes, messages up to 4 KiB
    #[test]
    fn prop_matches_reference(key in proptest::collection::vec(any::<u8>(), 0..200),
                              data in proptest::collection::vec(any::<u8>(), 0..4096)) {
        prop_assert_eq!(ours(&key, &data), ref_hmac(&key, &data));
    }

    #[test]
    fn prop_chunking_invariant(key in proptest::collection::vec(any::<u8>(), 0..200),
                               data in proptest::collection::vec(any::<u8>(), 0..4096),
                               chunk_lens in proptest::collection::vec(1usize..200, 0..80)) {
        let a = ours(&key, &data);
        let b = ours_chunked(&key, &data, &chunk_lens);
        prop_assert_eq!(a, b);
    }

    // Emphasize tricky sizes around the 128-byte HMAC block.
    #[test]
    fn prop_key_size_boundaries(
        // keys around 128B
        key_len in prop_oneof![Just(0usize), Just(16), Just(64), Just(127), Just(128), Just(129), Just(200)],
        data in proptest::collection::vec(any::<u8>(), 0..4096)
    ) {
        let key = vec![0xA5u8; key_len];
        prop_assert_eq!(ours(&key, &data), ref_hmac(&key, &data));
    }
}
