// ---------------- [ File: bitcoin-aes/src/aes128.rs ]
crate::ix!();

#[inline(always)]
pub fn aes128_init(ctx: *mut AES128_ctx, key16: *const u8) {
    tracing::info!(target: "aes", "aes128_init – ctx {:p}", ctx);
    unsafe { aes_setup((*ctx).rk.as_mut_ptr(), key16, 4, 10) };
}

#[inline(always)]
pub fn aes128_encrypt(
    ctx: *const AES128_ctx,
    mut blocks: usize,
    mut cipher16: *mut u8,
    mut plain16: *const u8,
) {
    tracing::info!(target: "aes", "aes128_encrypt – blocks = {}", blocks);

    unsafe {
        while blocks != 0 {
            aes_encrypt((*ctx).rk.as_ptr(), 10, cipher16, plain16);
            cipher16 = cipher16.add(16);
            plain16 = plain16.add(16);
            blocks -= 1;
        }
    }
}

#[inline(always)]
pub fn aes128_decrypt(
    ctx: *const AES128_ctx,
    mut blocks: usize,
    mut plain16: *mut u8,
    mut cipher16: *const u8,
) {
    tracing::info!(target: "aes", "aes128_decrypt – blocks = {}", blocks);

    unsafe {
        while blocks != 0 {
            aes_decrypt((*ctx).rk.as_ptr(), 10, plain16, cipher16);
            cipher16 = cipher16.add(16);
            plain16 = plain16.add(16);
            blocks -= 1;
        }
    }
}

#[cfg(test)]
mod aes128_roundtrip_validation {
    use super::*;

    /// Randomised encryption → decryption round‑trip for AES‑128.
    #[traced_test]
    fn random_roundtrip_identity() {
        let mut rng = thread_rng();

        for _ in 0..5_000 {
            // Random key & plaintext
            let mut key = [0u8; 16];
            let mut plain_in = [0u8; AES_BLOCKSIZE];
            rng.fill(&mut key);
            rng.fill(&mut plain_in);

            let mut cipher    = [0u8; AES_BLOCKSIZE];
            let mut plain_out = [0u8; AES_BLOCKSIZE];

            // Initialise key schedule
            let mut ctx = AES128_ctx::default();
            unsafe { aes128_init(&mut ctx as *mut _, key.as_ptr()) };

            // One‑block encrypt & decrypt
            unsafe {
                aes128_encrypt(&ctx as *const _, 1, cipher.as_mut_ptr(), plain_in.as_ptr());
                aes128_decrypt(&ctx as *const _, 1, plain_out.as_mut_ptr(), cipher.as_ptr());
            }

            info!(target: "test", ?key, ?plain_in, ?cipher, ?plain_out, "AES‑128 round‑trip");
            assert_eq!(plain_out, plain_in, "AES‑128 round‑trip mismatch");
        }
    }
}
