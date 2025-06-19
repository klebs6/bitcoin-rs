// ---------------- [ File: bitcoin-aes/src/aes256.rs ]
crate::ix!();

#[inline(always)]
pub fn aes256_init(ctx: *mut AES256_ctx, key32: *const u8) {
    tracing::info!(target: "aes", "aes256_init – ctx {:p}", ctx);
    unsafe { aes_setup((*ctx).rk.as_mut_ptr(), key32, 8, 14) };
}

#[inline(always)]
pub fn aes256_encrypt(
    ctx: *const AES256_ctx,
    mut blocks: usize,
    mut cipher16: *mut u8,
    mut plain16: *const u8,
) {
    tracing::info!(target: "aes", "aes256_encrypt – blocks = {}", blocks);

    unsafe {
        while blocks != 0 {
            aes_encrypt((*ctx).rk.as_ptr(), 14, cipher16, plain16);
            cipher16 = cipher16.add(16);
            plain16 = plain16.add(16);
            blocks -= 1;
        }
    }
}

#[inline(always)]
pub fn aes256_decrypt(
    ctx: *const AES256_ctx,
    mut blocks: usize,
    mut plain16: *mut u8,
    mut cipher16: *const u8,
) {
    tracing::info!(target: "aes", "aes256_decrypt – blocks = {}", blocks);

    unsafe {
        while blocks != 0 {
            aes_decrypt((*ctx).rk.as_ptr(), 14, plain16, cipher16);
            cipher16 = cipher16.add(16);
            plain16 = plain16.add(16);
            blocks -= 1;
        }
    }
}

#[cfg(test)]
mod aes256_roundtrip_validation {
    use super::*;

    /// Randomised encryption → decryption round‑trip for AES‑256.
    #[traced_test]
    fn random_roundtrip_identity() {
        let mut rng = thread_rng();

        for _ in 0..5_000 {
            let mut key = [0u8; 32];
            let mut plain_in = [0u8; AES_BLOCKSIZE];
            rng.fill(&mut key);
            rng.fill(&mut plain_in);

            let mut cipher    = [0u8; AES_BLOCKSIZE];
            let mut plain_out = [0u8; AES_BLOCKSIZE];

            let mut ctx = AES256_ctx::default();
            unsafe { aes256_init(&mut ctx as *mut _, key.as_ptr()) };

            unsafe {
                aes256_encrypt(&ctx as *const _, 1, cipher.as_mut_ptr(), plain_in.as_ptr());
                aes256_decrypt(&ctx as *const _, 1, plain_out.as_mut_ptr(), cipher.as_ptr());
            }

            info!(target: "test", ?key, ?plain_in, ?cipher, ?plain_out, "AES‑256 round‑trip");
            assert_eq!(plain_out, plain_in, "AES‑256 round‑trip mismatch");
        }
    }
}
