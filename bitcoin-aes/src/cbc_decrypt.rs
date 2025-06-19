// ---------------- [ File: bitcoin-aes/src/cbc_decrypt.rs ]
crate::ix!();

/// Decrypt CBC‑mode data, optionally removing PKCS‑7 padding.
///
/// * Returns `0` on any error (null pointers, size == 0, ciphertext not
///   block‑aligned, or invalid padding).
/// * Otherwise returns the exact number of plaintext bytes written.
pub fn cbc_decrypt<T: Decrypt>(
    dec:      &T,
    iv:       [u8; AES_BLOCKSIZE],
    mut data: *const u8,
    size:     i32,
    pad:      bool,
    out:      *mut u8,
) -> i32 {
    use tracing::{debug, trace};
    use impl_::*;

    if data.is_null() || out.is_null() || size == 0 || size % AES_BLOCKSIZE as i32 != 0 {
        debug!(target: "bitcoin_aes::cbc_decrypt", size, "bad args"); return 0;
    }

    let mut mixed = iv;
    let mut read  = 0;

    while read < size {
        let cipher_block        = copy_ct_block(data);
        let prev_cipher_block   = cipher_block;        // <── keeps CT safe
        unsafe { data = data.add(AES_BLOCKSIZE); }

        let mut plain_block = [0u8; AES_BLOCKSIZE];
        dec.decrypt(&mut plain_block, &cipher_block);

        xor_in_place(&mut plain_block, &mixed);        // CBC unchain

        unsafe {
            core::ptr::copy_nonoverlapping(
                plain_block.as_ptr(),
                out.add(read as usize),
                AES_BLOCKSIZE
            );
        }

        mixed = prev_cipher_block;                    // correct IV for next round
        read  += AES_BLOCKSIZE as i32;
        trace!(target: "bitcoin_aes::cbc_decrypt", block = read / 16);
    }

    if pad {
        match strip_pkcs7(unsafe { core::slice::from_raw_parts_mut(out, read as usize) }, read) {
            Ok(n)  => n,
            Err(_) => { debug!(target: "bitcoin_aes::cbc_decrypt", "bad padding"); 0 }
        }
    } else {
        read
    }
}

mod impl_ {
    use super::*;

    #[inline]
    pub fn copy_ct_block(src: *const u8) -> [u8; AES_BLOCKSIZE] {
        let mut buf = [0u8; AES_BLOCKSIZE];
        unsafe { core::ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), AES_BLOCKSIZE); }
        buf
    }

    #[inline]
    pub fn xor_in_place(dst: &mut [u8; AES_BLOCKSIZE],
                        src: &[u8; AES_BLOCKSIZE]) {
        for i in 0..AES_BLOCKSIZE { dst[i] ^= src[i]; }
    }

    #[inline]
    /// Returns Ok(new_length_without_pad) or Err(()) on bad PKCS‑7
    pub fn strip_pkcs7(buf: &mut [u8], len: i32) -> Result<i32, ()> {
        let len = len as usize;
        if len == 0 { return Err(()); }
        let pad_len = buf[len - 1] as usize;
        if pad_len == 0 || pad_len > AES_BLOCKSIZE || pad_len > len { return Err(()); }
        if !buf[len - pad_len .. len].iter().all(|b| *b as usize == pad_len) { return Err(()); }
        Ok((len - pad_len) as i32)
    }
}

#[cfg(test)]
mod cbc_decrypt_validation {
    use super::*;

    /// A single‑byte corruption in the PKCS‑7 padding must cause
    /// `cbc_decrypt` to fail (return `0`) and *not* expose plaintext.
    #[traced_test]
    fn invalid_padding_is_rejected() {
        let mut rng = thread_rng();

        let mut key = [0u8; AES256_KEYSIZE];
        let mut iv  = [0u8; AES_BLOCKSIZE];
        rng.fill(&mut key);
        rng.fill(&mut iv);

        // two complete blocks of random data
        let plain_in = [rng.gen(); 32];

        let mut cipher  = [0u8; 48]; // +one block for padding
        let mut corrupt = [0u8; 48];
        let mut plain_out = [0u8; 48];

        let enc = AES256Encrypt::from(key);
        let mut dec = AES256Decrypt::default();
        dec.init(key);

        let written = cbc_encrypt(
            &enc,
            iv,
            plain_in.as_ptr(),
            plain_in.len() as i32,
            true,
            cipher.as_mut_ptr(),
        );

        // flip one byte in the PKCS‑7 padding
        corrupt[..written as usize].copy_from_slice(&cipher[..written as usize]);
        corrupt[written as usize - 1] ^= 0xFF;

        let ok = cbc_decrypt(
            &dec,
            iv,
            corrupt.as_ptr(),
            written,
            true,
            plain_out.as_mut_ptr(),
        );

        info!(target: "test", written, ok, "CBC padding corruption result");
        assert_eq!(ok, 0, "corrupted padding was *not* rejected");
    }

    #[traced_test]
    fn copy_block_roundtrip() {
        let mut bytes = [0u8; AES_BLOCKSIZE];
        rand::thread_rng().fill(&mut bytes);
        let p = bytes.as_ptr();
        let copy = crate::cbc_decrypt::impl_::copy_ct_block(p);
        assert_eq!(bytes, copy);
    }

    #[traced_test]
    fn xor_helper() {
        let mut a = [1u8; AES_BLOCKSIZE];
        let b     = [2u8; AES_BLOCKSIZE];
        crate::cbc_decrypt::impl_::xor_in_place(&mut a, &b);
        assert_eq!(a, [3u8; AES_BLOCKSIZE]);
    }

    #[traced_test]
    fn strip_pkcs7_ok() {
        let mut buf = [0u8; 32];
        buf[..19].copy_from_slice(b"hello, padded world");

        for b in &mut buf[17..32] { *b = 15 };
        let n = crate::cbc_decrypt::impl_::strip_pkcs7(&mut buf, 32).unwrap();
        assert_eq!(n, 17);
    }

    #[traced_test]
    fn strip_pkcs7_err() {
        let mut buf = [0u8; 16];
        buf[15] = 17; // impossible pad byte
        assert!(crate::cbc_decrypt::impl_::strip_pkcs7(&mut buf, 16).is_err());
    }

    #[traced_test]
    fn decrypt_encrypt_roundtrip() {
        let key  = [0x11u8; 32];
        let iv   = [0x22u8; 16];
        let data = b"All your base are belong to us!!";          // 30 B
                                                                 // encrypt
        let mut cipher = vec![0u8; 48]; // 3 blocks incl. padding
        let mut enc = aes256cbc_encrypt::AES256CBCEncrypt::new(key, iv, true);
        let written = enc.encrypt(
            data.as_ptr(),
            data.len() as i32,
            cipher.as_mut_ptr(),
        );
        let expected_len =
            ((data.len() + AES_BLOCKSIZE) / AES_BLOCKSIZE) * AES_BLOCKSIZE;
        assert_eq!(written as usize, expected_len,
            "ciphertext length with PKCS‑7 padding");

        // decrypt
        let mut plain = vec![0u8; 48];
        let mut dec = aes256cbc_decrypt::AES256CBCDecrypt::new(key, iv, true);
        let read   = dec.decrypt(
            cipher.as_ptr(),
            written,
            plain.as_mut_ptr(),
        );
        assert_eq!(read as usize, data.len(), "plaintext length");
        assert_eq!(&plain[..read as usize], data);
    }
}
