// ---------------- [ File: bitcoin-sha512/tests/streaming.rs ]
use bitcoin_sha512::*;
use bitcoin_imports::*;
use proptest::prelude::*;
use sha2::{Digest, Sha512 as Ref};

fn ours(data: &[u8]) -> [u8; 64] {
    let mut h = Sha512::new();
    h.write(data.as_ptr(), data.len());
    let mut out = [0u8; 64];
    h.finalize(&mut out);
    out
}

fn ours_chunked(data: &[u8], chunks: &[usize]) -> [u8; 64] {
    let mut pos = 0usize;
    let mut h = Sha512::new();
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

proptest! {
    #[test]
    fn prop_matches_reference(bytes in proptest::collection::vec(any::<u8>(), 0..4096)) {
        let mut r = Ref::new();
        r.update(&bytes);
        let expect = r.finalize();

        let x = ours(&bytes);
        prop_assert_eq!(x.as_slice(), expect.as_slice());
    }

    #[test]
    fn prop_chunking_invariant(bytes in proptest::collection::vec(any::<u8>(), 0..4096),
                               chunk_lens in proptest::collection::vec(1usize..200, 0..80)) {
        let a = ours(&bytes);
        let b = ours_chunked(&bytes, &chunk_lens);
        prop_assert_eq!(a, b);
    }
}

#[traced_test]
fn exhaustive_small_chunkings_up_to_64_bytes() {
    // Exhaustive partitions for lengths up to 64 bytes across adversarial chunk sizes.
    let adversarial = [1,2,3,4,7,8,15,16,31,32,63,64,65,127,128,129];
    for n in 0..=64 {
        let msg: Vec<u8> = (0..n as u8).collect();
        let golden = ours(&msg);
        for &c in &adversarial {
            // feed in equal-sized steps of 'c'
            let mut h = Sha512::new();
            let mut off = 0usize;
            while off < msg.len() {
                let end = (off + c).min(msg.len());
                let chunk = &msg[off..end];
                h.write(chunk.as_ptr(), chunk.len());
                off = end;
            }
            let mut out = [0u8; 64];
            h.finalize(&mut out);
            assert_eq!(out, golden, "n={n}, chunk={c}");
        }
    }
}

#[traced_test]
fn reset_reproducible() {
    let mut h = Sha512::new();
    let m = b"abc";
    h.write(m.as_ptr(), m.len());
    let mut out1 = [0u8; 64];
    h.finalize(&mut out1);

    h.reset().write(m.as_ptr(), m.len());
    let mut out2 = [0u8; 64];
    h.finalize(&mut out2);

    assert_eq!(out1, out2);
}

#[traced_test]
fn size_tracks_prepad_bytes() {
    let mut h = Sha512::new();
    let a = [1u8; 7];
    let b = [2u8; 123];
    h.write(a.as_ptr(), a.len());
    assert_eq!(h.size(), 7);
    h.write(b.as_ptr(), b.len());
    assert_eq!(h.size(), 7 + 123);
    // Note: calling finalize() mutates internal 'bytes' by padding; don't assert size() after finalize.
}
