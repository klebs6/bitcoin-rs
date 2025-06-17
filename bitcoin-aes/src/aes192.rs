crate::ix!();

#[inline(always)]
pub fn aes192_init(ctx: *mut AES192_ctx, key24: *const u8) {
    tracing::info!(target: "aes", "aes192_init – ctx {:p}", ctx);
    unsafe { aes_setup((*ctx).rk.as_mut_ptr(), key24, 6, 12) };
}

#[inline(always)]
pub fn aes192_encrypt(
    ctx: *const AES192_ctx,
    mut blocks: usize,
    mut cipher16: *mut u8,
    mut plain16: *const u8,
) {
    tracing::info!(target: "aes", "aes192_encrypt – blocks = {}", blocks);

    unsafe {
        while blocks != 0 {
            aes_encrypt((*ctx).rk.as_ptr(), 12, cipher16, plain16);
            cipher16 = cipher16.add(16);
            plain16 = plain16.add(16);
            blocks -= 1;
        }
    }
}

#[inline(always)]
pub fn aes192_decrypt(
    ctx: *const AES192_ctx,
    mut blocks: usize,
    mut plain16: *mut u8,
    mut cipher16: *const u8,
) {
    tracing::info!(target: "aes", "aes192_decrypt – blocks = {}", blocks);

    unsafe {
        while blocks != 0 {
            aes_decrypt((*ctx).rk.as_ptr(), 12, plain16, cipher16);
            cipher16 = cipher16.add(16);
            plain16 = plain16.add(16);
            blocks -= 1;
        }
    }
}
