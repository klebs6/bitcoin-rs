// ---------------- [ File: bitcoin-hmac-sha256/src/bitcoin_hmac_sha256.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/hmac_sha256.h]

/**
  | A hasher class for HMAC-SHA-256.
  |
  */
#[derive(Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct HmacSha256 {
    outer: Sha256,
    inner: Sha256,
}

pub const HMAC_SHA256_OUTPUT_SIZE: usize = 32;

//-------------------------------------------[.cpp/bitcoin/src/crypto/hmac_sha256.cpp]
impl HmacSha256 {

    pub fn new(key: *const u8, keylen: usize) -> Self {
        // Delegate to the safe constructor
        let slice = unsafe { if keylen == 0 { &[] } else { std::slice::from_raw_parts(key, keylen) } };
        Self::from_slice(slice)
    }

    #[inline]
    pub fn new_with_key(key: &[u8]) -> Self { 
        Self::from_slice(key) 
    }

    pub fn write(&mut self, data: *const u8, len: usize) -> &mut HmacSha256 {
        // Delegate to the safe method
        let slice = unsafe { if len == 0 { &[] } else { std::slice::from_raw_parts(data, len) } };
        self.write_ref(slice)
    }

    /// Feed message bytes (safe slice).
    #[inline]
    pub fn write_ref(&mut self, data: &[u8]) -> &mut HmacSha256 {
        if !data.is_empty() {
            self.inner_mut().write_ptr(data.as_ptr(), data.len());
        }
        self
    }

    /// Construct an HMAC-SHA256 context from a key slice.
    pub fn from_slice(key: &[u8]) -> Self {
        let mut rkey = [0u8; 64];

        if key.len() <= 64 {
            if !key.is_empty() {
                rkey[..key.len()].copy_from_slice(key);
            }
            if key.len() < 64 {
                // trailing bytes already zeroed
            }
        } else {
            // rkey = SHA256(key), then zero-pad to 64
            let mut sh = Sha256::new();
            sh.write_ptr(key.as_ptr(), key.len());
            sh.finalize((&mut rkey[..32]).try_into().unwrap());
            // rkey[32..] already zero
        }

        // ipad/opad init
        let mut outer = Sha256::new();
        let mut inner = Sha256::new();

        // opad = 0x5c
        for b in &mut rkey { *b ^= 0x5c; }
        outer.write_ptr(rkey.as_ptr(), rkey.len());

        // ipdad: toggle ^0x5c back and ^0x36 → net ^= 0x36
        for b in &mut rkey { *b ^= 0x5c ^ 0x36; }
        inner.write_ptr(rkey.as_ptr(), rkey.len());

        HmacSha256 { outer, inner }
    }

    /// Finalize into `out[32]` (safe slice).
    pub fn finalize_into(&mut self, out: &mut [u8; HMAC_SHA256_OUTPUT_SIZE]) {
        let mut temp = [0u8; 32];
        self.inner_mut().finalize(&mut temp);
        self.outer_mut().write_ptr(temp.as_ptr(), temp.len());
        self.outer_mut().finalize(out);
    }

    /// Legacy signature (kept for symmetry with the C++ layout).
    /// Note: takes `hash` **by value**; prefer `finalize_into(&mut [u8;32])`.
    pub fn finalize(&mut self, mut hash: [u8; HMAC_SHA256_OUTPUT_SIZE]) {
        self.finalize_into(&mut hash);
    }
}

#[cfg(test)]
mod hmac_sha256_impl_tests {
    use super::*;
    use core::{ptr, slice};

    /* -------------------- helpers -------------------- */

    /// Reference HMAC‑SHA‑256 using only the `Sha256` API (no HmacSha256).
    fn hmac_ref(key: &[u8], msg: &[u8]) -> [u8; 32] {
        // 1) Normalize key to rkey (64 bytes)
        let mut rkey = [0u8; 64];
        if key.len() <= 64 {
            if !key.is_empty() {
                rkey[..key.len()].copy_from_slice(key);
            }
        } else {
            let mut sh = Sha256::new();
            sh.write_ptr(key.as_ptr(), key.len());
            sh.finalize((&mut rkey[..32]).try_into().unwrap());
            // rkey[32..] are already 0
        }

        // 2) Inner = SHA256((rkey^ipad) || msg)
        let mut kipad = rkey;
        for b in &mut kipad { *b ^= 0x36; }
        let mut inner = Sha256::new();
        inner.write_ptr(kipad.as_ptr(), kipad.len());
        if !msg.is_empty() {
            inner.write_ptr(msg.as_ptr(), msg.len());
        }
        let mut inner_tag = [0u8; 32];
        inner.finalize(&mut inner_tag);

        // 3) Outer = SHA256((rkey^opad) || inner_tag)
        let mut kopad = rkey;
        for b in &mut kopad { *b ^= 0x5c; }
        let mut outer = Sha256::new();
        outer.write_ptr(kopad.as_ptr(), kopad.len());
        outer.write_ptr(inner_tag.as_ptr(), inner_tag.len());
        let mut out = [0u8; 32];
        outer.finalize(&mut out);
        out
    }

    /// Simple deterministic xorshift for test vectors.
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
    fn empty_key_empty_msg_matches_reference() {
        let k = [];
        let m = [];
        let ref_tag = hmac_ref(&k, &m);

        let mut h = HmacSha256::new_with_key(&k);
        let mut got = [0u8; HMAC_SHA256_OUTPUT_SIZE];
        h.finalize_into(&mut got);

        assert_eq!(got, ref_tag, "HMAC('', '') mismatch vs reference");
    }

    #[traced_test]
    fn small_keys_varied_lengths_match_reference() {
        for klen in [0usize, 1, 2, 7, 31, 63, 64] {
            let key: Vec<u8> = (0..klen as u8).collect();
            let msg: Vec<u8> = b"sample-message".to_vec();

            let ref_tag = hmac_ref(&key, &msg);

            let mut h = HmacSha256::from_slice(&key);
            h.write_ref(&msg);
            let mut got = [0u8; 32];
            h.finalize_into(&mut got);

            assert_eq!(got, ref_tag, "klen={} mismatch", klen);
        }
    }

    #[traced_test]
    fn large_keys_hashed_then_padded_match_reference() {
        for klen in [65usize, 90, 128, 200] {
            let mut key = vec![0u8; klen];
            for i in 0..klen { key[i] = (i as u8).wrapping_mul(7).wrapping_add(3); }
            let msg = b"larger message to exercise hashed-key path".to_vec();

            let ref_tag = hmac_ref(&key, &msg);

            let mut h = HmacSha256::new_with_key(&key);
            h.write_ref(&msg);
            let mut got = [0u8; 32];
            h.finalize_into(&mut got);

            assert_eq!(got, ref_tag, "hashed-key path mismatch (klen={})", klen);
        }
    }

    #[traced_test]
    fn chunking_equivalence_via_write_ref() {
        // Same message in different chunk splits must yield identical tag.
        let key = b"keykeykey";
        let msg = b"The quick brown fox jumps over the lazy dog";

        let mut h1 = HmacSha256::new_with_key(key);
        h1.write_ref(msg);
        let mut tag1 = [0u8; 32];
        h1.finalize_into(&mut tag1);

        let mut h2 = HmacSha256::new_with_key(key);
        h2.write_ref(&msg[..10]);
        h2.write_ref(&msg[10..25]);
        h2.write_ref(&msg[25..]);
        let mut tag2 = [0u8; 32];
        h2.finalize_into(&mut tag2);

        assert_eq!(tag1, tag2);
    }

    #[traced_test]
    fn raw_pointer_wrappers_equal_safe_api() {
        let key = b"pointer-wrapper-key-with-odd-length!";
        let msg = b"ptr-wrapper message";

        // Safe API
        let mut hs = HmacSha256::new_with_key(key);
        hs.write_ref(msg);
        let mut tag_safe = [0u8; 32];
        hs.finalize_into(&mut tag_safe);

        // Ptr API
        let mut hp = unsafe { HmacSha256::new(key.as_ptr(), key.len()) };
        unsafe { hp.write(msg.as_ptr(), msg.len()); }
        let mut tag_ptr = [0u8; 32];
        hp.finalize_into(&mut tag_ptr);

        assert_eq!(tag_safe, tag_ptr, "ptr wrappers != safe API");
    }

    #[traced_test]
    fn null_pointer_zero_length_is_ok() {
        // new(nullptr, 0) => empty key
        let mut h = unsafe { HmacSha256::new(ptr::null(), 0) };
        // write(nullptr, 0) => no-op
        let _ = unsafe { h.write(ptr::null(), 0) };
        let mut tag = [0u8; 32];
        h.finalize_into(&mut tag);

        let ref_tag = hmac_ref(&[], &[]);
        assert_eq!(tag, ref_tag, "null+zero did not behave like empty");
    }

    #[traced_test]
    fn unaligned_input_pointer_is_handled() {
        let key = b"unaligned-key";
        let msg = b"0123456789abcdefghijklmnopqrstuvwxyz";

        // Make an intentionally unaligned base pointer for message.
        let mut buf = vec![0u8; msg.len() + 3];
        buf[1..1 + msg.len()].copy_from_slice(msg);

        let mut h = HmacSha256::new_with_key(key);
        unsafe { h.write(buf.as_ptr().add(1), msg.len()); }
        let mut tag = [0u8; 32];
        h.finalize_into(&mut tag);

        // Reference via safe path
        let mut ref_h = HmacSha256::new_with_key(key);
        ref_h.write_ref(msg);
        let mut ref_tag = [0u8; 32];
        ref_h.finalize_into(&mut ref_tag);

        assert_eq!(tag, ref_tag, "unaligned input pointer produced wrong tag");
    }

    #[traced_test]
    fn ipad_opad_initial_states_match_expected() {
        // Probe internal state immediately after `from_slice`, before any message bytes.
        let key = b"ipad-opad-state-probe-key...............(<= 40 bytes)";
        let h = HmacSha256::from_slice(key);

        // Rebuild rkey
        let mut rkey = [0u8; 64];
        rkey[..key.len()].copy_from_slice(key);

        let mut kopad = rkey;
        for b in &mut kopad { *b ^= 0x5c; }
        let mut exp_outer = Sha256::new();
        exp_outer.write_ptr(kopad.as_ptr(), kopad.len());

        let mut kipad = rkey;
        for b in &mut kipad { *b ^= 0x36; }
        let mut exp_inner = Sha256::new();
        exp_inner.write_ptr(kipad.as_ptr(), kipad.len());

        assert_eq!(*h.outer().s(), *exp_outer.s(), "outer state after opad differ");
        assert_eq!(*h.inner().s(), *exp_inner.s(), "inner state after ipad differ");
    }

    #[traced_test]
    fn randomized_parity_safe_vs_reference() {
        let mut s = 0x1234_5678_90ab_cdef_u64;
        for _ in 0..128 {
            let klen = (xorshift64(&mut s) % 100) as usize;
            let dlen = (xorshift64(&mut s) % 200) as usize;

            let mut key = vec![0u8; klen];
            let mut msg = vec![0u8; dlen];
            for b in &mut key { *b = (xorshift64(&mut s) & 0xFF) as u8; }
            for b in &mut msg { *b = (xorshift64(&mut s) & 0xFF) as u8; }

            let ref_tag = hmac_ref(&key, &msg);

            let mut h = HmacSha256::new_with_key(&key);
            h.write_ref(&msg);
            let mut got = [0u8; 32];
            h.finalize_into(&mut got);

            assert_eq!(got, ref_tag, "randomized parity mismatch");
        }
    }
}
