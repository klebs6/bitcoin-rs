// ---------------- [ File: bitcoin-chacha/src/compute_poly1305_tag.rs ]
crate::ix!();

use generic_array::{typenum::U16, GenericArray};

/// Compute a Poly1305 tag for `msg` using the supplied 256‑bit `key`.
///
/// This version delegates the entire MAC calculation to the
/// **constant‑time, RFC 7539‑compatible** `bitcoin_poly1305::poly1305_auth`
/// routine, eliminating the duplicate high‑bit/padding mistakes that
/// previously produced incorrect tags.
///
/// # Parameters
/// * `key` – 32‑byte (256‑bit) one‑time key.
/// * `msg` – The message to authenticate (any length).
///
/// # Returns
/// A `GenericArray<u8, U16>` containing the 16‑byte authentication tag.
///
/// # Tracing
/// Emits a `trace!` event recording the message length.
pub fn compute_poly1305_tag(
    key: &[u8; POLY1305_KEYLEN],
    msg: &[u8],
) -> GenericArray<u8, U16> {
    trace!(msg_len = msg.len(), "compute_poly1305_tag");
    let mut tag = [0u8; POLY1305_TAGLEN];
    poly1305_auth(&mut tag, msg, key);
    GenericArray::clone_from_slice(&tag)
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
