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
