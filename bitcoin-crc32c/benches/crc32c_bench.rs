// ---------------- [ File: bitcoin-crc32c/src/benchmark.rs ]

use bitcoin_crc32c::*;

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_benchmark.cc]

use criterion::{black_box, criterion_group, criterion_main,
                BenchmarkId, Criterion, Throughput};

const BLOCK_SIZES: &[usize] = &[
    256,            // 2 ⁸
    4_096,          // 2 ¹²
    65_536,         // 2 ¹⁶
    1_048_576,      // 2 ²⁰
    16_777_216,     // 2 ²⁴
];

/* ------------------------------------------------------------------------- */
/* “Public” – front‑door dispatcher (= crc32c_extend)                        */
/* ------------------------------------------------------------------------- */
fn bench_public(c: &mut Criterion) {
    let mut group = c.benchmark_group("Public (auto‑dispatch)");
    for &size in BLOCK_SIZES {
        // allocate once per size; identical to `std::string(block_size_, 'x')`
        let block = vec![b'x'; size];
        let ptr   = block.as_ptr();

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &s| {
                unsafe {
                    b.iter(|| {
                        // local variable matches original `uint32_t crc = 0`
                        let mut crc = 0u32;
                        crc = crc32c_extend(crc, ptr, s);
                        black_box(crc);
                    });
                }
            },
        );
    }
    group.finish();
}

/* ------------------------------------------------------------------------- */
/* Portable back‑end only                                                   */
/* ------------------------------------------------------------------------- */
fn bench_portable(c: &mut Criterion) {
    let mut group = c.benchmark_group("Portable");
    for &size in BLOCK_SIZES {
        let block = vec![b'x'; size];
        let ptr   = block.as_ptr();

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &s| {
                unsafe {
                    b.iter(|| {
                        let mut crc = 0u32;
                        crc = crc32c_extend_portable(crc, ptr, s);
                        black_box(crc);
                    });
                }
            },
        );
    }
    group.finish();
}

/* ------------------------------------------------------------------------- */
/* AArch64 hardware CRC/PMULL back‑end                                      */
/* ------------------------------------------------------------------------- */
#[cfg(target_arch = "aarch64")]
fn bench_arm_crc32c(c: &mut Criterion) {
    if !can_use_arm64_crc32() {
        eprintln!("arm64 CRC instructions not available – skipping benchmark");
        return;
    }
    let mut group = c.benchmark_group("Arm CRC32C (hw)");
    for &size in BLOCK_SIZES {
        let block = vec![b'x'; size];
        let ptr   = block.as_ptr();

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &s| {
                unsafe {
                    b.iter(|| {
                        let mut crc = 0u32;
                        crc = crc32c_extend_arm64(crc, ptr, s);
                        black_box(crc);
                    });
                }
            },
        );
    }
    group.finish();
}

/* ------------------------------------------------------------------------- */
/* x86‑64 SSE4.2 back‑end                                                   */
/* ------------------------------------------------------------------------- */
#[cfg(target_arch = "x86_64")]
fn bench_sse42(c: &mut Criterion) {
    if !can_use_sse42() {
        eprintln!("SSE4.2 not available – skipping benchmark");
        return;
    }
    let mut group = c.benchmark_group("SSE4.2 (hw)");
    for &size in BLOCK_SIZES {
        let block = vec![b'x'; size];
        let ptr   = block.as_ptr();

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &s| {
                unsafe {
                    b.iter(|| {
                        let mut crc = 0u32;
                        crc = crc32c_extend_sse42(crc, ptr, s);
                        black_box(crc);
                    });
                }
            },
        );
    }
    group.finish();
}

/* ------------------------------------------------------------------------- */
/* Criterion boiler‑plate                                                   */
/* ------------------------------------------------------------------------- */
criterion_group!(
    benches,
    bench_public,
    bench_portable,
    /* only compile where relevant */
    #[cfg(target_arch = "aarch64")] bench_arm_crc32c,
    #[cfg(target_arch = "x86_64")]  bench_sse42
);
criterion_main!(benches);
