// ---------------- [ File: bitcoin-aes/src/ctaes_test.rs ]
// Full replacement of the former C‑style stubs with a fully‑functional,
// constant‑time Rust validation harness for the FIPS‑197 and
// NIST SP 800‑38A AES test vectors.
//
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ctaes/test.c]

/// A single reference test‑vector entry (key, plaintext, ciphertext).
#[derive(Clone, Debug, Getters, Builder)]
#[builder(pattern = "owned")]
#[getset(get = "pub")]
pub struct CtAesTest {
    keysize: usize,           /* key size in bits: 128/192/256           */
    key:     &'static str,    /* lowercase hex string (no “0x”, no spaces) */
    plain:   &'static str,    /* 32‑char hex string                       */
    cipher:  &'static str,    /* 32‑char hex string                       */
}

/* -------------------------------------------------------------------------
 * Reference vectors: FIPS‑197 §C and NIST SP 800‑38A Appendix F
 * ---------------------------------------------------------------------- */
lazy_static! {
    static ref CTAES_TESTS: Vec<CtAesTest> = vec![
        /* AES‑128 – FIPS 197 */
        CtAesTestBuilder::default()
            .keysize(128)
            .key("000102030405060708090a0b0c0d0e0f")
            .plain("00112233445566778899aabbccddeeff")
            .cipher("69c4e0d86a7b0430d8cdb78070b4c55a")
            .build().unwrap(),
        /* AES‑192 – FIPS 197 */
        CtAesTestBuilder::default()
            .keysize(192)
            .key("000102030405060708090a0b0c0d0e0f1011121314151617")
            .plain("00112233445566778899aabbccddeeff")
            .cipher("dda97ca4864cdfe06eaf70a0ec0d7191")
            .build().unwrap(),
        /* AES‑256 – FIPS 197 */
        CtAesTestBuilder::default()
            .keysize(256)
            .key("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
            .plain("00112233445566778899aabbccddeeff")
            .cipher("8ea2b7ca516745bfeafc49904b496089")
            .build().unwrap(),
        /* AES‑ECB – NIST SP 800‑38A (128‑bit key) */
        CtAesTestBuilder::default()
            .keysize(128)
            .key("2b7e151628aed2a6abf7158809cf4f3c")
            .plain("6bc1bee22e409f96e93d7e117393172a")
            .cipher("3ad77bb40d7a3660a89ecaf32466ef97")
            .build().unwrap(),
        CtAesTestBuilder::default()
            .keysize(128)
            .key("2b7e151628aed2a6abf7158809cf4f3c")
            .plain("ae2d8a571e03ac9c9eb76fac45af8e51")
            .cipher("f5d3d58503b9699de785895a96fdbaaf")
            .build().unwrap(),
        CtAesTestBuilder::default()
            .keysize(128)
            .key("2b7e151628aed2a6abf7158809cf4f3c")
            .plain("30c81c46a35ce411e5fbc1191a0a52ef")
            .cipher("43b1cd7f598ece23881b00e3ed030688")
            .build().unwrap(),
        CtAesTestBuilder::default()
            .keysize(128)
            .key("2b7e151628aed2a6abf7158809cf4f3c")
            .plain("f69f2445df4f9b17ad2b417be66c3710")
            .cipher("7b0c785e27e8ad3f8223207104725dd4")
            .build().unwrap(),
        /* AES‑ECB – NIST SP 800‑38A (192‑bit key) */
        CtAesTestBuilder::default()
            .keysize(192)
            .key("8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b")
            .plain("6bc1bee22e409f96e93d7e117393172a")
            .cipher("bd334f1d6e45f25ff712a214571fa5cc")
            .build().unwrap(),
        CtAesTestBuilder::default()
            .keysize(192)
            .key("8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b")
            .plain("ae2d8a571e03ac9c9eb76fac45af8e51")
            .cipher("974104846d0ad3ad7734ecb3ecee4eef")
            .build().unwrap(),
        CtAesTestBuilder::default()
            .keysize(192)
            .key("8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b")
            .plain("30c81c46a35ce411e5fbc1191a0a52ef")
            .cipher("ef7afd2270e2e60adce0ba2face6444e")
            .build().unwrap(),
        CtAesTestBuilder::default()
            .keysize(192)
            .key("8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b")
            .plain("f69f2445df4f9b17ad2b417be66c3710")
            .cipher("9a4b41ba738d6c72fb16691603c18e0e")
            .build().unwrap(),
        /* AES‑ECB – NIST SP 800‑38A (256‑bit key) */
        CtAesTestBuilder::default()
            .keysize(256)
            .key("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4")
            .plain("6bc1bee22e409f96e93d7e117393172a")
            .cipher("f3eed1bdb5d2a03c064b5a7e3db181f8")
            .build().unwrap(),
        CtAesTestBuilder::default()
            .keysize(256)
            .key("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4")
            .plain("ae2d8a571e03ac9c9eb76fac45af8e51")
            .cipher("591ccb10d410ed26dc5ba74a31362870")
            .build().unwrap(),
        CtAesTestBuilder::default()
            .keysize(256)
            .key("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4")
            .plain("30c81c46a35ce411e5fbc1191a0a52ef")
            .cipher("b6ed21b99ca6f4f9f153e7b1beafed1d")
            .build().unwrap(),
        CtAesTestBuilder::default()
            .keysize(256)
            .key("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4")
            .plain("f69f2445df4f9b17ad2b417be66c3710")
            .cipher("23304b7a39f9f3ff067d8d8f9e24ecc7")
            .build().unwrap(),
    ];
}

/* -------------------------------------------------------------------------
 * Helper: hex decoding (ASCII lower‑case only, no spaces)
 * ---------------------------------------------------------------------- */
#[inline(always)]
fn decode_hex(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0, "hex strings must have an even length");
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("valid hex digit"))
        .collect()
}

/* -------------------------------------------------------------------------
 * C‑compatible “fill buffer from C‑string” adapter (rarely used internally)
 * ---------------------------------------------------------------------- */
#[inline(always)]
pub unsafe fn from_hex(data: *mut u8, len: i32, hex: *const u8) {
    trace!(target: "aes", "from_hex – len = {len}");
    assert!(!data.is_null());
    assert!(!hex.is_null());

    for p in 0..len as isize {
        let hi = *hex.offset(2 * p) as char;
        let lo = *hex.offset(2 * p + 1) as char;

        let hv = hi.to_digit(16).expect("hi‑digit") as u8;
        let lv = lo.to_digit(16).expect("lo‑digit") as u8;

        *data.offset(p) = (hv << 4) | lv;
    }

    /* require NUL‑termination exactly after the expected digits */
    assert!(*hex.offset((len * 2) as isize) == 0);
    trace!(target: "aes", "from_hex – exit");
}

#[cfg(test)]
mod fips_nist_vector_validation {
    use super::*;
    use tracing::{debug, info};

    /* -------------------------------------------------------------------
     *  Helper: execute exactly one reference vector by index
     * ---------------------------------------------------------------- */
    fn exec_vector(idx: usize) {
        let tv = &CTAES_TESTS[idx];
        debug!(target: "test",
               idx,
               key_size = tv.keysize(),
               key = tv.key(),
               plain = tv.plain(),
               cipher = tv.cipher(),
               "running reference vector");

        /* ---- decode hex strings ------------------------------------ */
        let key_bytes   = decode_hex(tv.key());
        let plain_vec   = decode_hex(tv.plain());
        let cipher_vec  = decode_hex(tv.cipher());

        /* ---- runtimes expect fixed 16‑byte blocks ------------------- */
        let plain:  [u8; 16] = plain_vec .try_into().unwrap();
        let cipher: [u8; 16] = cipher_vec.try_into().unwrap();

        match tv.keysize() {
            /* ---------------- AES‑128 ---------------- */
            128 => {
                let key: [u8; 16] = key_bytes.try_into().unwrap();
                let mut ctx = AES128_ctx::default();
                aes128_init(&mut ctx as *mut _, key.as_ptr());

                let mut enc = [0u8; 16];
                let mut dec = [0u8; 16];

                aes128_encrypt(&ctx as *const _, 1, enc.as_mut_ptr(),  plain .as_ptr());
                aes128_decrypt(&ctx as *const _, 1, dec.as_mut_ptr(), cipher.as_ptr());

                assert_eq!(enc, cipher, "AES‑128 encrypt mismatch (vector #{idx})");
                assert_eq!(dec, plain , "AES‑128 decrypt mismatch (vector #{idx})");
            }
            /* ---------------- AES‑192 ---------------- */
            192 => {
                let key: [u8; 24] = key_bytes.try_into().unwrap();
                let mut ctx = AES192_ctx::default();
                aes192_init(&mut ctx as *mut _, key.as_ptr());

                let mut enc = [0u8; 16];
                let mut dec = [0u8; 16];

                aes192_encrypt(&ctx as *const _, 1, enc.as_mut_ptr(),  plain .as_ptr());
                aes192_decrypt(&ctx as *const _, 1, dec.as_mut_ptr(), cipher.as_ptr());

                assert_eq!(enc, cipher, "AES‑192 encrypt mismatch (vector #{idx})");
                assert_eq!(dec, plain , "AES‑192 decrypt mismatch (vector #{idx})");
            }
            /* ---------------- AES‑256 ---------------- */
            256 => {
                let key: [u8; 32] = key_bytes.try_into().unwrap();
                let mut ctx = AES256_ctx::default();
                aes256_init(&mut ctx as *mut _, key.as_ptr());

                let mut enc = [0u8; 16];
                let mut dec = [0u8; 16];

                aes256_encrypt(&ctx as *const _, 1, enc.as_mut_ptr(),  plain .as_ptr());
                aes256_decrypt(&ctx as *const _, 1, dec.as_mut_ptr(), cipher.as_ptr());

                assert_eq!(enc, cipher, "AES‑256 encrypt mismatch (vector #{idx})");
                assert_eq!(dec, plain , "AES‑256 decrypt mismatch (vector #{idx})");
            }
            k => unreachable!("unrecognised key size: {k} bits"),
        }

        info!(target: "test", idx, "reference vector passed ✓");
    }

    /* -------------------------------------------------------------------
     *  Macro: generate one cargo‑test per reference vector
     * ---------------------------------------------------------------- */
    macro_rules! gen_ctaes_vector_tests {
        ($( $fn_name:ident => $index:expr ),* $(,)?) => {$(
            #[traced_test]
            fn $fn_name() {
                exec_vector($index);
            }
        )*};
    }

    /* -------------------------------------------------------------------
     *  Instantiate 15 individual tests (3 FIPS + 12 NIST)
     * ---------------------------------------------------------------- */
    gen_ctaes_vector_tests! {
        fips_197_aes128          =>  0,
        fips_197_aes192          =>  1,
        fips_197_aes256          =>  2,
        nist_ecb_aes128_vec0     =>  3,
        nist_ecb_aes128_vec1     =>  4,
        nist_ecb_aes128_vec2     =>  5,
        nist_ecb_aes128_vec3     =>  6,
        nist_ecb_aes192_vec0     =>  7,
        nist_ecb_aes192_vec1     =>  8,
        nist_ecb_aes192_vec2     =>  9,
        nist_ecb_aes192_vec3     => 10,
        nist_ecb_aes256_vec0     => 11,
        nist_ecb_aes256_vec1     => 12,
        nist_ecb_aes256_vec2     => 13,
        nist_ecb_aes256_vec3     => 14,
    }
}
