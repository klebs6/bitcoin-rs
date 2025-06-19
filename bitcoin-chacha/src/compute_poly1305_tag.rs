// ---------------- [ File: bitcoin-chacha/src/compute_poly1305_tag.rs ]
crate::ix!();

use generic_array::{typenum::U16, GenericArray};
use poly1305::Key;

/// Compute a Poly1305 tag for an arbitrary‑length message.
///
/// The algorithm processes each 16‑byte block **as‑is**; for the
/// final partial block (if any) we zero‑pad to 16 bytes and append a
/// single `1` byte as specified in RFC 8439 §2.5.
pub fn compute_poly1305_tag(
    key: &[u8; POLY1305_KEYLEN],
    msg: &[u8],
) -> GenericArray<u8, U16> {
    let key = Key::from_slice(key);
    let mut mac = Poly1305::new(key);

    // --- full 16‑byte blocks -------------------------------------------
    for chunk in msg.chunks_exact(16) {
        // each call to `update` expects a slice of `Block`s
        let block = GenericArray::<u8, U16>::clone_from_slice(chunk);
        mac.update(core::slice::from_ref(&block));
    }

    // --- trailing partial block (≤15 bytes) -----------------------------
    let rem = msg.len() % 16;
    if rem != 0 {
        let mut last = [0u8; 16];
        last[..rem].copy_from_slice(&msg[msg.len() - rem..]);
        last[rem] = 1; // Poly1305 padding bit
        let block = GenericArray::<u8, U16>::from(last);
        mac.update(core::slice::from_ref(&block));
    }

    mac.finalize()
}

#[cfg(test)]
mod compute_poly1305_tag_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn empty_and_single_byte_messages() {
        let key = [7u8; POLY1305_KEYLEN];

        // empty message
        let tag0 = compute_poly1305_tag(&key, &[]);
        let tag0_ref = compute_poly1305_tag(&key, &[0u8; 0]);
        assert_eq!(tag0, tag0_ref, "empty message tag must be deterministic");

        // 1‑byte message
        let tag1 = compute_poly1305_tag(&key, &[0xAF]);
        let tag1_repeat = compute_poly1305_tag(&key, &[0xAF]);
        assert_eq!(tag1, tag1_repeat, "same key/msg ⇒ same tag");
        assert_ne!(tag0, tag1, "different msgs ⇒ different tags");
    }

    #[traced_test]
    fn multiple_of_16_boundary() {
        let key = [0x42u8; POLY1305_KEYLEN];
        let msg_16 = [0x11u8; 16];
        let msg_15 = [0x11u8; 15];

        let tag_16 = compute_poly1305_tag(&key, &msg_16);
        let tag_15 = compute_poly1305_tag(&key, &msg_15);
        assert_ne!(tag_16, tag_15, "padding changes tag");
    }
}
