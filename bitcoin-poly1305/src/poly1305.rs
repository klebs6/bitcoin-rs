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

// bitcoin-poly1305/src/poly1305.rs  (tests only)
#[cfg(test)]
mod poly1305_tests {
    use super::*;
    use hex_literal::hex;
    use proptest::prelude::*;

    #[traced_test]
    fn rfc_7539_vector_1() {
        let key = hex!("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
        let msg = b"Cryptographic Forum Research Group";
        let expected = hex!("a8061dc1305136c6c22b8baf0c0127a9");

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, msg, &key);
        assert_eq!(tag, expected);
    }

    #[traced_test]
    fn all_zero_key_all_zero_msg() {
        let key = [0u8; POLY1305_KEYLEN];
        let msg = [0u8; 64];

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, &msg, &key);

        assert_eq!(tag, [0u8; POLY1305_TAGLEN]);
    }

    #[traced_test]
    fn zero_r_nonzero_s_arbitrary_msg() {
        let key = hex!(
            "00000000000000000000000000000000 \
             36e5f6b5c5e06070f0efca96227a863e"
        );
        let msg = b"Any submission to the IETF intended \
                    by the Contributor for publication \
                    as all or part of an IETF";
        let expected = hex!("36e5f6b5c5e06070f0efca96227a863e");

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, msg, &key);
        assert_eq!(tag, expected);
    }

    #[traced_test]
    fn rfc7539_vector_1_regression() {
        let key = hex!("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
        let msg = b"Cryptographic Forum Research Group";
        let expected = hex!("a8061dc1305136c6c22b8baf0c0127a9");

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, msg, &key);
        assert_eq!(tag, expected);
    }

    /// Property‑based: determinism & length‑safety up to 1 KiB random inputs.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        #[traced_test]
        fn determinism_and_length(
            key in proptest::array::uniform32(any::<u8>()),
            msg in proptest::collection::vec(any::<u8>(), 0..1024),
        ) {
            let mut tag1 = [0u8; POLY1305_TAGLEN];
            let mut tag2 = [0u8; POLY1305_TAGLEN];

            poly1305_auth(&mut tag1, &msg, &key);
            poly1305_auth(&mut tag2, &msg, &key);

            prop_assert_eq!(tag1, tag2, "same input must yield identical tag");
        }
    }
}
