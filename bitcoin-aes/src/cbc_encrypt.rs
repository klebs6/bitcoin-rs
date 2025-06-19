// ---------------- [ File: bitcoin-aes/src/cbc_encrypt.rs ]
crate::ix!();

pub fn cbc_encrypt<T: Encrypt>(
    enc:      &T,
    iv:       [u8; AES_BLOCKSIZE],
    mut data: *const u8,
    size:     i32,
    pad:      bool,
    out:      *mut u8,
) -> i32 {
    if data.is_null() || out.is_null() || size == 0 {
        return 0;
    }

    let mut written: i32 = 0;
    let padsize: i32     = size % (AES_BLOCKSIZE as i32);

    if !pad && padsize != 0 {
        return 0;
    }

    // running XOR buffer (“mixed”)
    let mut mixed = iv;

    // --------------- full blocks ---------------------------------------
    while written + AES_BLOCKSIZE as i32 <= size {
        for i in 0..AES_BLOCKSIZE {
            unsafe {
                mixed[i] ^= *data;
                data = data.add(1);
            }
        }

        unsafe {
            enc.encrypt(
                core::slice::from_raw_parts_mut(out.add(written as usize), AES_BLOCKSIZE)
                    .try_into()
                    .unwrap(),
                mixed,
            );
            // ciphertext becomes next IV
            mixed.copy_from_slice(core::slice::from_raw_parts(
                out.add(written as usize),
                AES_BLOCKSIZE,
            ));
        }

        written += AES_BLOCKSIZE as i32;
    }

    // --------------- final padded block --------------------------------
    if pad {
        for i in 0..padsize as usize {
            unsafe {
                mixed[i] ^= *data;
                data = data.add(1);
            }
        }
        for i in padsize as usize..AES_BLOCKSIZE {
            mixed[i] ^= (AES_BLOCKSIZE as i32 - padsize) as u8;
        }

        unsafe {
            enc.encrypt(
                core::slice::from_raw_parts_mut(out.add(written as usize), AES_BLOCKSIZE)
                    .try_into()
                    .unwrap(),
                mixed,
            );
        }

        written += AES_BLOCKSIZE as i32;
    }

    written
}

#[cfg(test)]
mod cbc_encrypt_validation {
    use super::*;

    /// PKCS‑7 padded CBC: encrypt‑then‑decrypt must return the
    /// *exact* input bytes and length for a wide range of sizes.
    #[traced_test]
    fn padded_encrypt_then_decrypt_roundtrip() {
        let mut rng = thread_rng();

        for _ in 0..2_000 {
            let mut key = [0u8; AES256_KEYSIZE];
            let mut iv  = [0u8; AES_BLOCKSIZE];
            rng.fill(&mut key);
            rng.fill(&mut iv);

            // random length (1‥256 bytes)
            let pt_len: usize = rng.gen_range(1..=256);
            let mut plain_in  = vec![0u8; pt_len];
            rng.fill(&mut plain_in[..]);

            // output buffers (cipher may grow by ≤1 block)
            let mut cipher    = vec![0u8; pt_len + AES_BLOCKSIZE];
            let mut plain_out = vec![0u8; cipher.len()];

            // single‑block primitives
            let enc = AES256Encrypt::from(key);
            let mut dec = AES256Decrypt::default();
            dec.init(key);

            // ---------- encryption ----------
            let written = cbc_encrypt(
                &enc,
                iv,
                plain_in.as_ptr(),
                pt_len as i32,
                true,                    // PKCS‑7 padding
                cipher.as_mut_ptr(),
            );
            assert!(
                written > 0 && written % AES_BLOCKSIZE as i32 == 0,
                "ciphertext length"
            );

            // ---------- decryption ----------
            let read = cbc_decrypt(
                &dec,
                iv,
                cipher.as_ptr(),
                written,
                true,                    // PKCS‑7 padding
                plain_out.as_mut_ptr(),
            );

            info!(target: "test", pt_len, written, read, "CBC round‑trip");

            assert_eq!(read as usize, pt_len, "reported plaintext length");
            assert_eq!(
                &plain_out[..pt_len],
                &plain_in[..],
                "cipher‑→‑plain data mismatch"
            );
        }
    }

    /// With `pad == false`, the API must reject non‑block‑aligned
    /// input by returning `0`.
    #[traced_test]
    fn refuses_unpadded_non_block_multiple() {
        let mut rng = thread_rng();

        let mut key = [0u8; AES256_KEYSIZE];
        let mut iv  = [0u8; AES_BLOCKSIZE];
        rng.fill(&mut key);
        rng.fill(&mut iv);

        const PT_LEN: usize = 31; // deliberately not a multiple of 16
        let mut plain  = [0u8; PT_LEN];
        rng.fill(&mut plain[..]);

        let mut cipher = [0u8; PT_LEN + AES_BLOCKSIZE];

        let enc = AES256Encrypt::from(key);

        let written = cbc_encrypt(
            &enc,
            iv,
            plain.as_ptr(),
            PT_LEN as i32,
            false,                 // *no* padding
            cipher.as_mut_ptr(),
        );

        debug!(target: "test", PT_LEN, written, "CBC unpadded rejection");
        assert_eq!(written, 0, "unenforced block multiple when pad == false");
    }
}
