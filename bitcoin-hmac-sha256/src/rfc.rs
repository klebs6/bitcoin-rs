// ---------------- [ File: bitcoin-hmac-sha256/src/rfc.rs ]
crate::ix!();

#[derive(Builder,Getters,Setters,MutGetters)]
#[getset(get="pub",set="pub",get_mut="pub")]
#[builder(setter(into))]
pub struct Rfc6979HmacSha256 {
    v:     [u8; 32],
    k:     [u8; 32],
    retry: i32,
}

pub fn rfc6979_hmac_sha256_initialize(
    rng: *mut Rfc6979HmacSha256,
    key: *const u8,
    keylen: usize,
) {
    // C version preserved, step-for-step with RFC6979 §3.2.
    unsafe {
        let rng = &mut *rng;
        const ZERO: [u8; 1] = [0x00];
        const ONE: [u8; 1] = [0x01];

        // RFC6979 3.2.b and 3.2.c
        rng.v.fill(0x01);
        rng.k.fill(0x00);

        // RFC6979 3.2.d
        let mut hmac: HmacSha256 = core::mem::zeroed();
        hmac_sha256_initialize(&mut hmac, rng.k.as_ptr(), 32);
        hmac_sha256_write(&mut hmac, rng.v.as_ptr(), 32);
        hmac_sha256_write(&mut hmac, ZERO.as_ptr(), 1);
        hmac_sha256_write(&mut hmac, key, keylen);
        hmac_sha256_finalize(&mut hmac, rng.k.as_mut_ptr());

        hmac_sha256_initialize(&mut hmac, rng.k.as_ptr(), 32);
        hmac_sha256_write(&mut hmac, rng.v.as_ptr(), 32);
        hmac_sha256_finalize(&mut hmac, rng.v.as_mut_ptr());

        // RFC6979 3.2.f
        hmac_sha256_initialize(&mut hmac, rng.k.as_ptr(), 32);
        hmac_sha256_write(&mut hmac, rng.v.as_ptr(), 32);
        hmac_sha256_write(&mut hmac, ONE.as_ptr(), 1);
        hmac_sha256_write(&mut hmac, key, keylen);
        hmac_sha256_finalize(&mut hmac, rng.k.as_mut_ptr());

        hmac_sha256_initialize(&mut hmac, rng.k.as_ptr(), 32);
        hmac_sha256_write(&mut hmac, rng.v.as_ptr(), 32);
        hmac_sha256_finalize(&mut hmac, rng.v.as_mut_ptr());

        rng.retry = 0;
    }
}

pub fn rfc6979_hmac_sha256_generate(
    rng: *mut Rfc6979HmacSha256,
    mut out: *mut u8,
    mut outlen: usize,
) {
    // C version preserved (RFC6979 3.2.h).
    unsafe {
        let rng = &mut *rng;
        const ZERO: [u8; 1] = [0x00];

        if rng.retry != 0 {
            let mut hmac: HmacSha256 = core::mem::zeroed();
            hmac_sha256_initialize(&mut hmac, rng.k.as_ptr(), 32);
            hmac_sha256_write(&mut hmac, rng.v.as_ptr(), 32);
            hmac_sha256_write(&mut hmac, ZERO.as_ptr(), 1);
            hmac_sha256_finalize(&mut hmac, rng.k.as_mut_ptr());

            hmac_sha256_initialize(&mut hmac, rng.k.as_ptr(), 32);
            hmac_sha256_write(&mut hmac, rng.v.as_ptr(), 32);
            hmac_sha256_finalize(&mut hmac, rng.v.as_mut_ptr());
        }

        while outlen > 0 {
            let mut hmac: HmacSha256 = core::mem::zeroed();
            hmac_sha256_initialize(&mut hmac, rng.k.as_ptr(), 32);
            hmac_sha256_write(&mut hmac, rng.v.as_ptr(), 32);
            hmac_sha256_finalize(&mut hmac, rng.v.as_mut_ptr());

            let now = if outlen > 32 { 32 } else { outlen };
            ptr::copy_nonoverlapping(rng.v.as_ptr(), out, now);
            out = out.add(now);
            outlen -= now;
        }

        rng.retry = 1;
    }
}

pub fn rfc6979_hmac_sha256_finalize(rng: *mut Rfc6979HmacSha256) {
    // C version preserved.
    unsafe {
        let rng = &mut *rng;
        ptr::write_bytes(rng.k.as_mut_ptr(), 0, 32);
        ptr::write_bytes(rng.v.as_mut_ptr(), 0, 32);
        rng.retry = 0;
    }
}

#[cfg(test)]
mod rfc6979_core_tests {
    use super::*;
    use core::{ptr, fmt::Write as _};

    /* -------------------- small helpers -------------------- */

    fn hex_to_bytes(s: &str) -> Vec<u8> {
        let mut out = Vec::new();
        let mut nibble = None::<u8>;
        for ch in s.chars().filter(|c| !c.is_whitespace()) {
            let v = match ch {
                '0'..='9' => ch as u8 - b'0',
                'a'..='f' => ch as u8 - b'a' + 10,
                'A'..='F' => ch as u8 - b'A' + 10,
                _ => panic!("invalid hex char: {ch}"),
            };
            if let Some(h) = nibble {
                out.push((h << 4) | v);
                nibble = None;
            } else {
                nibble = Some(v);
            }
        }
        assert!(nibble.is_none(), "odd number of hex digits");
        out
    }

    fn bytes_to_upper_hex(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            let _ = write!(&mut s, "{:02X}", b);
        }
        s
    }

    // ---- RFC 6979 §A.2.5 helpers (P-256, SHA-256) ----

    fn ge_be(a: &[u8; 32], b: &[u8; 32]) -> bool {
        for i in 0..32 { if a[i] != b[i] { return a[i] > b[i]; } }
        true
    }

    fn sub_assign_be(a: &mut [u8; 32], b: &[u8; 32]) {
        let mut borrow: u16 = 0;
        for i in (0..32).rev() {
            let (ai, bi) = (a[i] as i32, b[i] as i32);
            let tmp = ai - bi - (borrow as i32);
            if tmp < 0 { a[i] = (tmp + 256) as u8; borrow = 1; }
            else       { a[i] = tmp as u8;        borrow = 0; }
        }
    }

    fn bits2octets_p256(h1: [u8; 32]) -> [u8; 32] {
        let q: [u8; 32] = hex_to_bytes(
            "FFFFFFFF00000000FFFFFFFFFFFFFFFF\
             BCE6FAADA7179E84F3B9CAC2FC632551"
        ).try_into().unwrap();
        let mut x = h1;
        if ge_be(&x, &q) { sub_assign_be(&mut x, &q); }
        x
    }

    fn int2octets_p256(x_hex: &str) -> [u8; 32] {
        let v = hex_to_bytes(x_hex);
        let mut out = [0u8; 32];
        out.copy_from_slice(&v);
        out
    }

    // SHA256(msg) via our backend
    fn sha256_bytes(msg: &[u8]) -> [u8; 32] {
        let mut sh = crate::Sha256::new();
        if !msg.is_empty() {
            sh.write_ptr(msg.as_ptr(), msg.len());
        }
        let mut out = [0u8; 32];
        sh.finalize(&mut out);
        out
    }

    unsafe fn make_k_p256_sha256(x_hex: &str, msg_ascii: &[u8]) -> [u8; 32] {
        let h1 = sha256_bytes(msg_ascii);
        let mut key = [0u8; 64];
        key[..32].copy_from_slice(&int2octets_p256(x_hex));
        key[32..].copy_from_slice(&bits2octets_p256(h1));
        let mut rng = Rfc6979HmacSha256 { v: [0u8; 32], k: [0u8; 32], retry: 0 };
        rfc6979_hmac_sha256_initialize(&mut rng as *mut _, key.as_ptr(), key.len());
        let mut out = [0u8; 32];
        rfc6979_hmac_sha256_generate(&mut rng as *mut _, out.as_mut_ptr(), 32);
        out
    }

    /* -------------------- tests -------------------- */

    #[traced_test]
    fn rfc6979_p256_sha256_k_vectors() {
        unsafe {
            // A.2.5 "sample"
            let x = "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721";
            let want = "A6E3C57DD01ABE90086538398355DD4C3B17AA873382B0F24D6129493D8AAD60";
            let k = make_k_p256_sha256(x, b"sample");
            assert_eq!(bytes_to_upper_hex(&k), want);

            // A.2.5 "test"
            let want2 = "D16B6AE827F17175E040871A1C7EC3500192C4C92677336EC2537ACAEE0008E0";
            let k2 = make_k_p256_sha256(x, b"test");
            assert_eq!(bytes_to_upper_hex(&k2), want2);
        }
    }

    #[traced_test]
    fn retry_semantics_and_64_vs_32_plus_32() {
        unsafe {
            // Build a seed key as in RFC 6979, but any key works to test semantics.
            let x = "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721";
            let h1 = sha256_bytes(b"sample");
            let mut key = [0u8; 64];
            key[..32].copy_from_slice(&int2octets_p256(x));
            key[32..].copy_from_slice(&bits2octets_p256(h1));

            let mut rng1 = Rfc6979HmacSha256 { v: [0; 32], k: [0; 32], retry: 0 };
            rfc6979_hmac_sha256_initialize(&mut rng1 as *mut _, key.as_ptr(), 64);
            let mut t64 = [0u8; 64];
            rfc6979_hmac_sha256_generate(&mut rng1 as *mut _, t64.as_mut_ptr(), 64);

            let mut rng2 = Rfc6979HmacSha256 { v: [0; 32], k: [0; 32], retry: 0 };
            rfc6979_hmac_sha256_initialize(&mut rng2 as *mut _, key.as_ptr(), 64);
            let mut a = [0u8; 32];
            let mut b = [0u8; 32];
            rfc6979_hmac_sha256_generate(&mut rng2 as *mut _, a.as_mut_ptr(), 32);
            rfc6979_hmac_sha256_generate(&mut rng2 as *mut _, b.as_mut_ptr(), 32);

            assert_eq!(&t64[..32], &a[..], "first half must match");
            assert_ne!(&t64[32..], &b[..], "second half must differ due to step h");
        }
    }

    #[traced_test]
    fn generate_zero_length_changes_retry_but_writes_nothing() {
        unsafe {
            let seed = [0x11u8; 48];
            let mut rng = Rfc6979HmacSha256 { v: [0; 32], k: [0; 32], retry: 0 };
            rfc6979_hmac_sha256_initialize(&mut rng as *mut _, seed.as_ptr(), seed.len());

            let mut canary = [0xAAu8; 16];
            // outlen=0: must not touch canary; implementation sets retry=1 even when outlen==0.
            rfc6979_hmac_sha256_generate(&mut rng as *mut _, canary.as_mut_ptr(), 0);

            assert!(canary.iter().all(|&b| b == 0xAA), "zero-length generate modified output");
            assert_eq!(rng.retry, 1, "retry flag not set after zero-length generate");
        }
    }

    #[traced_test]
    fn generate_writes_exactly_requested_bytes() {
        unsafe {
            let seed = [0x22u8; 32];
            let mut rng = Rfc6979HmacSha256 { v: [0; 32], k: [0; 32], retry: 0 };
            rfc6979_hmac_sha256_initialize(&mut rng as *mut _, seed.as_ptr(), seed.len());

            let mut out = [0xAAu8; 64];
            rfc6979_hmac_sha256_generate(&mut rng as *mut _, out.as_mut_ptr(), 7);

            // Only 7 bytes may change.
            assert!(&out[7..].iter().all(|&b| b == 0xAA), "bytes past requested length were written");
        }
    }

    #[traced_test]
    fn finalize_zeros_state_and_resets_retry() {
        unsafe {
            let key = [0x33u8; 40];
            let mut rng = Rfc6979HmacSha256 { v: [0; 32], k: [0; 32], retry: 0 };
            rfc6979_hmac_sha256_initialize(&mut rng as *mut _, key.as_ptr(), key.len());
            let mut out = [0u8; 1];
            rfc6979_hmac_sha256_generate(&mut rng as *mut _, out.as_mut_ptr(), 1);
            assert_ne!(rng.v, [0u8; 32], "V must have been updated");
            assert_ne!(rng.k, [0u8; 32], "K must have been updated");

            rfc6979_hmac_sha256_finalize(&mut rng as *mut _);
            assert_eq!(rng.v, [0u8; 32], "V not zeroed by finalize");
            assert_eq!(rng.k, [0u8; 32], "K not zeroed by finalize");
            assert_eq!(rng.retry, 0, "retry not reset by finalize");
        }
    }
}
