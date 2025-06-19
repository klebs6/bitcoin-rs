// ---------------- [ File: bitcoin-poly1305/src/poly1305.rs ]
/*!
   | Based on the public domain implementation by
   | Andrew Moon poly1305-donna-unrolled.c from
   | https://github.com/floodyberry/poly1305-donna
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/poly1305.h]

pub const POLY1305_KEYLEN: usize = 32;
pub const POLY1305_TAGLEN: usize = 16;

//-------------------------------------------[.cpp/bitcoin/src/crypto/poly1305.cpp]

// -----------------------------------------------------------------------------
// [poly1305] tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod poly1305_tests {
    use super::*;
    use hex_literal::hex;

    #[traced_test]
    fn rfc_7539_vector_1() {
        // RFC‑7539 §2.5.2 test vector
        let key = hex!("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
        let msg = b"Cryptographic Forum Research Group";
        let expected = hex!("a8061dc1305136c6c22b8baf0c0127a9");

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, msg, &key);

        info!(?tag, "computed tag");
        assert_eq!(tag, expected);
    }

    /// RFC 7539 § A.3 Test Vector #1 — all‑zero key & all‑zero message.
    ///
    /// When both *r* and *s* are zero the authenticator must also be all zero,
    /// regardless of the message length.
    #[traced_test]
    fn all_zero_key_all_zero_msg() {
        let key = [0u8; POLY1305_KEYLEN];
        let msg = [0u8; 64]; // 4 full blocks of zeros

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, &msg, &key);

        let expected = [0u8; POLY1305_TAGLEN];
        info!(?tag, "computed tag (zero‑key/zero‑msg)");
        assert_eq!(tag, expected);
    }

    /// RFC 7539 § A.3 Test Vector #2 — *r* ≡ 0, arbitrary text, non‑zero *s*.
    ///
    /// With *r* clamped to zero, the final tag **must equal the 128‑bit *s***
    /// irrespective of the message content.
    #[traced_test]
    fn zero_r_nonzero_s_arbitrary_msg() {
        // r = 0 (first 16 B), s = 0x36e5…863e (last 16 B from the RFC)
        let key = hex!(
            "00000000000000000000000000000000 \
             36e5f6b5c5e06070f0efca96227a863e"
        );
        let msg = b"Any submission to the IETF intended \
                    by the Contributor for publication \
                    as all or part of an IETF";

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, msg, &key);

        let expected = hex!("36e5f6b5c5e06070f0efca96227a863e");
        info!(?tag, "computed tag (r=0, arbitrary msg)");
        assert_eq!(tag, expected);
    }

    /// Quick sanity check: round‑trip a short, non‑aligned message
    /// against the published RFC 7539 vector #1 **again** to make sure the
    /// expanded suite still detects the current regression.
    #[traced_test]
    fn rfc7539_vector_1_regression() {
        let key = hex!("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
        let msg = b"Cryptographic Forum Research Group";
        let expected = hex!("a8061dc1305136c6c22b8baf0c0127a9");

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, msg, &key);

        info!(?tag, "computed tag (RFC 7539 #1 – regression)");
        assert_eq!(tag, expected);
    }
}
