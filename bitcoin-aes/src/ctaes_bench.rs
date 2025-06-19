// ---------------- [ File: bitcoin-aes/src/ctaes_bench.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ctaes/bench.c]

/// Benchmark: AES‑128 key‑schedule initialisation.
#[inline(never)]
pub fn bench_aes128_init(data: *mut c_void) {
    info!(target: "bench", "bench_aes128_init – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes128_init – early‑exit (null ptr)");
        return;
    }

    let ctx = data as *mut AES128_ctx;
    const ITERATIONS: usize = 50_000;

    unsafe {
        for _ in 0..ITERATIONS {
            // Use the context memory itself as a 16‑byte zero key (faithful to C code).
            aes128_init(ctx, ctx as *const u8);
        }
    }

    info!(target: "bench", "bench_aes128_init – exit");
}

/// One‑time setup for AES‑128 encryption benchmarks (all‑zero key).
#[inline(never)]
pub fn bench_aes128_encrypt_setup(data: *mut c_void) {
    info!(target: "bench", "bench_aes128_encrypt_setup – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes128_encrypt_setup – early‑exit (null ptr)");
        return;
    }

    const KEY: [u8; 16] = [0u8; 16];
    let ctx = data as *mut AES128_ctx;

    unsafe { aes128_init(ctx, KEY.as_ptr()); }

    info!(target: "bench", "bench_aes128_encrypt_setup – exit");
}

/// Benchmark: AES‑128 encryption of 4 000 000 bytes (250 000 blocks).
#[inline(never)]
pub fn bench_aes128_encrypt(data: *mut c_void) {
    info!(target: "bench", "bench_aes128_encrypt – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes128_encrypt – early‑exit (null ptr)");
        return;
    }

    let ctx = data as *const AES128_ctx;
    let mut scratch = [0u8; 16];
    const ITERATIONS: usize = 4_000_000 / 16; // 250 000

    unsafe {
        for _ in 0..ITERATIONS {
            aes128_encrypt(ctx, 1, scratch.as_mut_ptr(), scratch.as_ptr());
        }
    }

    info!(target: "bench", "bench_aes128_encrypt – exit");
}

/// Benchmark: AES‑128 decryption of 4 000 000 bytes (250 000 blocks).
#[inline(never)]
pub fn bench_aes128_decrypt(data: *mut c_void) {
    info!(target: "bench", "bench_aes128_decrypt – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes128_decrypt – early‑exit (null ptr)");
        return;
    }

    let ctx = data as *const AES128_ctx;
    let mut scratch = [0u8; 16];
    const ITERATIONS: usize = 4_000_000 / 16; // 250 000

    unsafe {
        for _ in 0..ITERATIONS {
            aes128_decrypt(ctx, 1, scratch.as_mut_ptr(), scratch.as_ptr());
        }
    }

    info!(target: "bench", "bench_aes128_decrypt – exit");
}

/// Benchmark: AES‑192 key‑schedule initialisation.
#[inline(never)]
pub fn bench_aes192_init(data: *mut c_void) {
    info!(target: "bench", "bench_aes192_init – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes192_init – early‑exit (null ptr)");
        return;
    }

    let ctx = data as *mut AES192_ctx;
    const ITERATIONS: usize = 50_000;

    unsafe {
        for _ in 0..ITERATIONS {
            aes192_init(ctx, ctx as *const u8);
        }
    }

    info!(target: "bench", "bench_aes192_init – exit");
}

/// One‑time setup for AES‑192 encryption benchmarks (all‑zero 24‑byte key).
#[inline(never)]
pub fn bench_aes192_encrypt_setup(data: *mut c_void) {
    info!(target: "bench", "bench_aes192_encrypt_setup – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes192_encrypt_setup – early‑exit (null ptr)");
        return;
    }

    const KEY: [u8; 24] = [0u8; 24];
    let ctx = data as *mut AES192_ctx;

    unsafe { aes192_init(ctx, KEY.as_ptr()); }

    info!(target: "bench", "bench_aes192_encrypt_setup – exit");
}

/// Benchmark: AES‑192 encryption of 4 000 000 bytes (250 000 blocks).
#[inline(never)]
pub fn bench_aes192_encrypt(data: *mut c_void) {
    info!(target: "bench", "bench_aes192_encrypt – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes192_encrypt – early‑exit (null ptr)");
        return;
    }

    let ctx = data as *const AES192_ctx;
    let mut scratch = [0u8; 16];
    const ITERATIONS: usize = 4_000_000 / 16;

    unsafe {
        for _ in 0..ITERATIONS {
            aes192_encrypt(ctx, 1, scratch.as_mut_ptr(), scratch.as_ptr());
        }
    }

    info!(target: "bench", "bench_aes192_encrypt – exit");
}

/// Benchmark: AES‑192 decryption of 4 000 000 bytes (250 000 blocks).
#[inline(never)]
pub fn bench_aes192_decrypt(data: *mut c_void) {
    info!(target: "bench", "bench_aes192_decrypt – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes192_decrypt – early‑exit (null ptr)");
        return;
    }

    let ctx = data as *const AES192_ctx;
    let mut scratch = [0u8; 16];
    const ITERATIONS: usize = 4_000_000 / 16;

    unsafe {
        for _ in 0..ITERATIONS {
            aes192_decrypt(ctx, 1, scratch.as_mut_ptr(), scratch.as_ptr());
        }
    }

    info!(target: "bench", "bench_aes192_decrypt – exit");
}

/// Benchmark: AES‑256 key‑schedule initialisation.
#[inline(never)]
pub fn bench_aes256_init(data: *mut c_void) {
    info!(target: "bench", "bench_aes256_init – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes256_init – early‑exit (null ptr)");
        return;
    }

    let ctx = data as *mut AES256_ctx;
    const ITERATIONS: usize = 50_000;

    unsafe {
        for _ in 0..ITERATIONS {
            aes256_init(ctx, ctx as *const u8);
        }
    }

    info!(target: "bench", "bench_aes256_init – exit");
}

/// One‑time setup for AES‑256 encryption benchmarks (all‑zero 32‑byte key).
#[inline(never)]
pub fn bench_aes256_encrypt_setup(data: *mut c_void) {
    info!(target: "bench", "bench_aes256_encrypt_setup – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes256_encrypt_setup – early‑exit (null ptr)");
        return;
    }

    const KEY: [u8; 32] = [0u8; 32];
    let ctx = data as *mut AES256_ctx;

    unsafe { aes256_init(ctx, KEY.as_ptr()); }

    info!(target: "bench", "bench_aes256_encrypt_setup – exit");
}

/// Benchmark: AES‑256 encryption of 4 000 000 bytes (250 000 blocks).
#[inline(never)]
pub fn bench_aes256_encrypt(data: *mut c_void) {
    info!(target: "bench", "bench_aes256_encrypt – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes256_encrypt – early‑exit (null ptr)");
        return;
    }

    let ctx = data as *const AES256_ctx;
    let mut scratch = [0u8; 16];
    const ITERATIONS: usize = 4_000_000 / 16;

    unsafe {
        for _ in 0..ITERATIONS {
            aes256_encrypt(ctx, 1, scratch.as_mut_ptr(), scratch.as_ptr());
        }
    }

    info!(target: "bench", "bench_aes256_encrypt – exit");
}

/// Benchmark: AES‑256 decryption of 4 000 000 bytes (250 000 blocks).
#[inline(never)]
pub fn bench_aes256_decrypt(data: *mut c_void) {
    info!(target: "bench", "bench_aes256_decrypt – entry {:p}", data);
    if data.is_null() {
        trace!(target: "bench", "bench_aes256_decrypt – early‑exit (null ptr)");
        return;
    }

    let ctx = data as *const AES256_ctx;
    let mut scratch = [0u8; 16];
    const ITERATIONS: usize = 4_000_000 / 16;

    unsafe {
        for _ in 0..ITERATIONS {
            aes256_decrypt(ctx, 1, scratch.as_mut_ptr(), scratch.as_ptr());
        }
    }

    info!(target: "bench", "bench_aes256_decrypt – exit");
}

//#[bench] 
pub fn crypto_ctaes_bench(b: &mut Bencher) -> i32 {
    
    let mut ctx128 = AES128_ctx::default();
    let mut ctx192 = AES192_ctx::default();
    let mut ctx256 = AES256_ctx::default();

    run_benchmark("aes128_init",         Some(bench_aes128_init),     None,                             None, mut_cvoid![ctx128], 20, 50000);
    run_benchmark("aes128_encrypt_byte", Some(bench_aes128_encrypt),  Some(bench_aes128_encrypt_setup), None, mut_cvoid![ctx128], 20, 4000000);
    run_benchmark("aes128_decrypt_byte", Some(bench_aes128_decrypt),  Some(bench_aes128_encrypt_setup), None, mut_cvoid![ctx128], 20, 4000000);
    run_benchmark("aes192_init",         Some(bench_aes192_init),     None,                             None, mut_cvoid![ctx192], 20, 50000);
    run_benchmark("aes192_encrypt_byte", Some(bench_aes192_encrypt),  Some(bench_aes192_encrypt_setup), None, mut_cvoid![ctx192], 20, 4000000);
    run_benchmark("aes192_decrypt_byte", Some(bench_aes192_decrypt),  Some(bench_aes192_encrypt_setup), None, mut_cvoid![ctx192], 20, 4000000);
    run_benchmark("aes256_init",         Some(bench_aes256_init),     None,                             None, mut_cvoid![ctx256], 20, 50000);
    run_benchmark("aes256_encrypt_byte", Some(bench_aes256_encrypt),  Some(bench_aes256_encrypt_setup), None, mut_cvoid![ctx256], 20, 4000000);
    run_benchmark("aes256_decrypt_byte", Some(bench_aes256_decrypt),  Some(bench_aes256_encrypt_setup), None, mut_cvoid![ctx256], 20, 4000000);

    0
}
