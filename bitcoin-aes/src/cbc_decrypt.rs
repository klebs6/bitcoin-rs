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

    // --------‑ basic argument validation ---------------------------------
    if data.is_null() || out.is_null() || size == 0 {
        debug!(target: "bitcoin_aes::cbc_decrypt",
               size, "null pointer or zero size — aborting");
        return 0;
    }
    if size % (AES_BLOCKSIZE as i32) != 0 {
        debug!(target: "bitcoin_aes::cbc_decrypt",
               size, "ciphertext not block‑aligned — aborting");
        return 0;
    }

    // Running XOR buffer (“mixed”), initialised with the IV
    let mut mixed = iv;
    let mut read: i32 = 0;

    // --------‑ full blocks ------------------------------------------------
    while read < size {
        // Copy the next ciphertext block out of *data
        let mut cipher_block = [0u8; AES_BLOCKSIZE];
        unsafe {
            core::ptr::copy_nonoverlapping(
                data,
                cipher_block.as_mut_ptr(),
                AES_BLOCKSIZE,
            );
            data = data.add(AES_BLOCKSIZE);
        }

        // Decrypt the cipher block
        let mut plain_block = [0u8; AES_BLOCKSIZE];
        dec.decrypt(plain_block, cipher_block);

        // XOR with the running IV to obtain the plaintext and write it out
        for i in 0..AES_BLOCKSIZE {
            unsafe {
                *out.add(read as usize + i) = plain_block[i] ^ mixed[i];
            }
        }

        trace!(target: "bitcoin_aes::cbc_decrypt", block = read / 16 + 1);

        // Current ciphertext becomes the IV for the next round
        mixed = cipher_block;
        read  += AES_BLOCKSIZE as i32;
    }

    // --------‑ optional PKCS‑7 padding removal ---------------------------
    if pad {
        // The last byte of the stream encodes the padding length (1‥16)
        if read == 0 {
            return 0; // should be impossible after earlier checks
        }
        let pad_len = unsafe { *out.add((read - 1) as usize) };
        if pad_len == 0 || (pad_len as usize) > AES_BLOCKSIZE {
            debug!(target: "bitcoin_aes::cbc_decrypt",
                   pad_len, "invalid pad length byte — aborting");
            return 0;
        }

        // Every padding byte must equal `pad_len`
        for i in 0..pad_len as usize {
            let idx = (read as usize) - 1 - i;
            let b   = unsafe { *out.add(idx) };
            if b != pad_len {
                debug!(target: "bitcoin_aes::cbc_decrypt",
                       idx, expect = pad_len, got = b,
                       "non‑uniform padding byte — aborting");
                return 0;
            }
        }

        let unpadded = read - pad_len as i32;
        debug!(target: "bitcoin_aes::cbc_decrypt",
               read, pad_len, unpadded, "padding verified/removed");
        return unpadded;
    }

    // No padding: return the full number of bytes read
    read
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
}
