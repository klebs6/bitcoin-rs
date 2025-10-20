// ---------------- [ File: bitcoin-hmac-sha256/src/hmac.rs ]
crate::ix!();

pub fn hmac_sha256_initialize(hash: *mut HmacSha256, key: *const u8, keylen: usize) {
    unsafe {
        let out = &mut *hash;
        let key_slice = if keylen == 0 { &[] } else { std::slice::from_raw_parts(key, keylen) };
        // Move a freshly initialised context into the caller's storage.
        *out = HmacSha256::from_slice(key_slice);
    }
}

pub fn hmac_sha256_write(hash: *mut HmacSha256, data: *const u8, size: usize) {
    unsafe {
        let h = &mut *hash;
        let data_slice = if size == 0 { &[] } else { std::slice::from_raw_parts(data, size) };
        h.write_ref(data_slice);
    }
}

pub fn hmac_sha256_finalize(hash: *mut HmacSha256, out32: *mut u8) {
    unsafe {
        debug_assert!(!out32.is_null());
        // SAFETY: caller guarantees space for 32 bytes
        let out: &mut [u8; 32] = &mut *(out32 as *mut [u8; 32]);
        (&mut *hash).finalize_into(out);
    }
}

#[cfg(test)]
mod hmac_c_api_tests {
    use super::*;
    use core::{mem, ptr, slice};

    /* -------------------- helpers -------------------- */

    fn hmac_c_api(key: &[u8], msg: &[u8]) -> [u8; 32] {
        let mut ctx: HmacSha256 = unsafe { mem::zeroed() };
        unsafe {
            hmac_sha256_initialize(&mut ctx as *mut _, key.as_ptr(), key.len());
            if !msg.is_empty() {
                hmac_sha256_write(&mut ctx as *mut _, msg.as_ptr(), msg.len());
            }
            let mut out = [0u8; 32];
            hmac_sha256_finalize(&mut ctx as *mut _, out.as_mut_ptr());
            out
        }
    }

    fn hmac_safe_api(key: &[u8], msg: &[u8]) -> [u8; 32] {
        let mut h = HmacSha256::from_slice(key);
        h.write_ref(msg);
        let mut out = [0u8; 32];
        h.finalize_into(&mut out);
        out
    }

    fn xorshift64(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *state = x;
        x
    }

    /* -------------------- tests -------------------- */

    #[traced_test]
    fn c_api_matches_safe_api_for_basic_cases() {
        for (k, m) in [
            (b"" as &[u8], b"" as &[u8]),
            (b"k", b""),
            (b"", b"m"),
            (b"key", b"message"),
            (&[0u8; 65][..], b"hashed-key-path"),
        ] {
            assert_eq!(hmac_c_api(k, m), hmac_safe_api(k, m));
        }
    }

    #[traced_test]
    fn initialize_overwrites_existing_context() {
        // 1) Fill ctx with a first key, produce a tag for msg1
        let mut ctx: HmacSha256 = unsafe { mem::zeroed() };
        let key1 = b"first-key";
        let msg1 = b"message-one";
        let tag1_ref = hmac_safe_api(key1, msg1);

        unsafe {
            hmac_sha256_initialize(&mut ctx as *mut _, key1.as_ptr(), key1.len());
            hmac_sha256_write(&mut ctx as *mut _, msg1.as_ptr(), msg1.len());
            let mut out1 = [0u8; 32];
            hmac_sha256_finalize(&mut ctx as *mut _, out1.as_mut_ptr());
            assert_eq!(out1, tag1_ref, "first tag mismatch");
        }

        // 2) Re-initialise *the same* ctx memory with a different key and message
        let key2 = b"second-key with different len ...............";
        let msg2 = b"message-two";
        let tag2_ref = hmac_safe_api(key2, msg2);

        unsafe {
            hmac_sha256_initialize(&mut ctx as *mut _, key2.as_ptr(), key2.len());
            hmac_sha256_write(&mut ctx as *mut _, msg2.as_ptr(), msg2.len());
            let mut out2 = [0u8; 32];
            hmac_sha256_finalize(&mut ctx as *mut _, out2.as_mut_ptr());
            assert_eq!(out2, tag2_ref, "reinitialised ctx produced wrong tag");
        }
    }

    #[traced_test]
    fn write_null_pointer_zero_size_is_noop() {
        let mut ctx: HmacSha256 = unsafe { mem::zeroed() };
        let key = b"key";
        unsafe {
            hmac_sha256_initialize(&mut ctx as *mut _, key.as_ptr(), key.len());
            // no-op write
            hmac_sha256_write(&mut ctx as *mut _, ptr::null(), 0);
            let mut out = [0u8; 32];
            hmac_sha256_finalize(&mut ctx as *mut _, out.as_mut_ptr());
            assert_eq!(out, hmac_safe_api(key, b""), "null+zero write changed output");
        }
    }

    #[traced_test]
    fn finalize_writes_exactly_32_bytes() {
        let mut ctx: HmacSha256 = unsafe { mem::zeroed() };
        let key = b"aaa";
        let msg = b"bbb";
        unsafe {
            hmac_sha256_initialize(&mut ctx as *mut _, key.as_ptr(), key.len());
            hmac_sha256_write(&mut ctx as *mut _, msg.as_ptr(), msg.len());

            // Canary: upper half must remain 0xAA
            let mut out = [0xAAu8; 64];
            hmac_sha256_finalize(&mut ctx as *mut _, out.as_mut_ptr());
            let ref32 = hmac_safe_api(key, msg);

            assert_eq!(&out[..32], &ref32[..], "first 32 mismatch");
            assert!(out[32..].iter().all(|&b| b == 0xAA), "bytes beyond 32 were modified");
        }
    }

    #[traced_test]
    fn unaligned_input_pointer_works() {
        let key = b"unaligned";
        let msg = b"abcdefghijklmnopqrstuvwxyz0123456789";
        let mut buf = vec![0u8; msg.len() + 1];
        buf[1..1 + msg.len()].copy_from_slice(msg);

        let mut ctx: HmacSha256 = unsafe { mem::zeroed() };
        unsafe {
            hmac_sha256_initialize(&mut ctx as *mut _, key.as_ptr(), key.len());
            hmac_sha256_write(&mut ctx as *mut _, buf.as_ptr().add(1), msg.len());
            let mut out = [0u8; 32];
            hmac_sha256_finalize(&mut ctx as *mut _, out.as_mut_ptr());
            assert_eq!(out, hmac_safe_api(key, msg));
        }
    }

    #[traced_test]
    fn streaming_equivalence_c_api() {
        let key = b"stream-key";
        let msg = b"The quick brown fox jumps over the lazy dog";

        let one_shot = hmac_c_api(key, msg);

        let mut ctx: HmacSha256 = unsafe { mem::zeroed() };
        unsafe {
            hmac_sha256_initialize(&mut ctx as *mut _, key.as_ptr(), key.len());
            hmac_sha256_write(&mut ctx as *mut _, msg[..10].as_ptr(), 10);
            hmac_sha256_write(&mut ctx as *mut _, msg[10..25].as_ptr(), 15);
            hmac_sha256_write(&mut ctx as *mut _, msg[25..].as_ptr(), msg.len() - 25);
            let mut chunked = [0u8; 32];
            hmac_sha256_finalize(&mut ctx as *mut _, chunked.as_mut_ptr());
            assert_eq!(one_shot, chunked);
        }
    }

    #[traced_test]
    #[ignore = "heavy: randomized parity across many sizes"]
    fn randomized_parity_c_vs_safe() {
        let mut s = 0xDEAD_BEEF_CAFE_F00D_u64;
        for _ in 0..2000 {
            let klen = (xorshift64(&mut s) % 256) as usize;
            let dlen = (xorshift64(&mut s) % 4096) as usize;

            let mut key = vec![0u8; klen];
            let mut msg = vec![0u8; dlen];
            for b in &mut key { *b = (xorshift64(&mut s) & 0xFF) as u8; }
            for b in &mut msg { *b = (xorshift64(&mut s) & 0xFF) as u8; }

            let a = hmac_c_api(&key, &msg);
            let b = hmac_safe_api(&key, &msg);
            assert_eq!(a, b, "mismatch for klen={}, dlen={}", klen, dlen);
        }
    }
}
