use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use sha3::{Digest as _, Sha3_256 as RefSha3_256};

// ---- Import your implementation ----
// If you re-exported in lib.rs:
use bitcoin_sha3::SHA3_256;
// If you did NOT re-export, use the module path instead:
// use bitcoin_sha3::sha3::SHA3_256;

use std::time::Duration;

// Deterministic, dependency-free PRNG for payloads.
struct XorShift64(u64);
impl XorShift64 {
    fn new(seed: u64) -> Self { Self(seed) }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn fill_bytes(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let v = self.next_u64().to_le_bytes();
            let n = chunk.len();
            chunk.copy_from_slice(&v[..n]);
        }
    }
}

// ----- Helpers to run the hash in various modes -----

fn ours_oneshot(data: &[u8]) -> [u8; 32] {
    let mut h = SHA3_256::default();
    h.write(data);
    let mut out = [0u8; 32];
    h.finalize(&mut out);
    out
}

fn ours_chunked(data: &[u8], chunk: usize) -> [u8; 32] {
    let mut h = SHA3_256::default();
    let mut i = 0;
    while i < data.len() {
        let end = (i + chunk).min(data.len());
        h.write(&data[i..end]);
        i = end;
    }
    let mut out = [0u8; 32];
    h.finalize(&mut out);
    out
}

fn ref_oneshot(data: &[u8]) -> [u8; 32] {
    let mut r = RefSha3_256::new();
    r.update(data);
    let res = r.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&res);
    out
}

// ----- Bench 1: throughput across sizes (one-shot + chunked) -----

fn bench_sha3_throughput(c: &mut Criterion) {
    // Sizes target common boundaries: <8, ==8, near 136 (rate), multiples, and larger buffers.
    const SIZES: &[usize] = &[
        0, 3, 7, 8, 15, 31, 32, 63, 64, 127, 135, 136, 137, 1024, 8192, 65536,
    ];

    let mut rng = XorShift64::new(0xC0DEC0DE_1234_5678);

    let mut group = c.benchmark_group("sha3-256/throughput");
    // A little more time improves stability for the larger buffers.
    group.sample_size(80);
    group.warm_up_time(Duration::from_millis(400));
    group.measurement_time(Duration::from_secs(6));

    for &size in SIZES {
        let mut data = vec![0u8; size];
        rng.fill_bytes(&mut data);

        group.throughput(Throughput::Bytes(size as u64));

        // Our implementation: one-shot
        group.bench_function(BenchmarkId::new("ours/oneshot", size), |b| {
            b.iter(|| {
                let out = ours_oneshot(black_box(&data));
                black_box(out);
            });
        });

        // Our implementation: chunked in 1, 7, 64 bytes
        for &chunk in &[1usize, 7, 64] {
            group.bench_function(BenchmarkId::new(format!("ours/chunked/{}", chunk), size), |b| {
                b.iter(|| {
                    let out = ours_chunked(black_box(&data), chunk);
                    black_box(out);
                });
            });
        }

        // Reference (RustCrypto) one-shot
        group.bench_function(BenchmarkId::new("ref/oneshot", size), |b| {
            b.iter(|| {
                let out = ref_oneshot(black_box(&data));
                black_box(out);
            });
        });
    }

    group.finish();
}

// ----- Bench 2: stress rate-boundary chunkings (control-flow hot paths) -----

fn bench_rate_boundaries(c: &mut Criterion) {
    const RATE: usize = 136; // 17 lanes * 8 bytes
    let patterns: &[&[usize]] = &[
        &[RATE],                 // exactly one rate block
        &[RATE + 1],             // just over the rate
        &[RATE - 1, 1, 1],       // straddle boundary with tiny tail
        &[1, RATE - 1, 1],       // absorb small, then nearly a block
        &[7; 20],                // many partial lanes
        &[8; 17],                // lane-aligned full block
        &[8; 34],                // two full blocks
        &[1, 135, 1, 135],       // repeat cross-boundary
        &[3, 5, 8, 13, 21, 34, 55, 89], // Fibonacci-ish
    ];

    // Build a buffer long enough for the largest pattern.
    let max_total = patterns.iter().map(|p| p.iter().sum::<usize>()).max().unwrap();
    let mut rng = XorShift64::new(0xFEEDFACE_FEED_FACE);
    let mut msg = vec![0u8; max_total];
    rng.fill_bytes(&mut msg);

    let mut group = c.benchmark_group("sha3-256/rate-boundaries");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(5));

    for (pi, pat) in patterns.iter().enumerate() {
        let total: usize = pat.iter().sum();
        group.throughput(Throughput::Bytes(total as u64));

        // Our impl: apply the chunk sizes as given.
        group.bench_function(BenchmarkId::new("ours/chunked", format!("pat-{}", pi)), |b| {
            b.iter(|| {
                let mut h = SHA3_256::default();
                let mut idx = 0usize;
                for &sz in *pat {
                    let end = idx + sz;
                    h.write(black_box(&msg[idx..end]));
                    idx = end;
                }
                let mut out = [0u8; 32];
                h.finalize(&mut out);
                black_box(out);
            });
        });

        // Reference: single shot over the same total bytes
        group.bench_function(BenchmarkId::new("ref/oneshot", format!("pat-{}", pi)), |b| {
            b.iter(|| {
                let mut r = RefSha3_256::new();
                r.update(black_box(&msg[..total]));
                let out = r.finalize();
                black_box(out);
            });
        });
    }

    group.finish();
}

// ----- Bench 3 (optional micro): keccakf permutation only -----
// Uncomment keccakf import in lib.rs for this micro-bench.
// use bitcoin_sha3::keccakf;

#[allow(dead_code)]
fn _bench_keccakf(c: &mut Criterion) {
    let mut group = c.benchmark_group("keccakf-only");
    group.sample_size(300);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(4));

    group.bench_function("keccakf/zero_state", |b| {
        let mut st = [0u64; 25];
        b.iter(|| {
            // black_box prevents the optimizer from discarding work.
            let s = black_box(&mut st);
            // keccakf uses in-place mutation
            bitcoin_sha3::keccakf(s);
        });
    });

    // Random-ish state
    group.bench_function("keccakf/random_state", |b| {
        let mut rng = XorShift64::new(0xDEADBEEF_BADC0FFE);
        let mut st = [0u64; 25];
        for s in st.iter_mut() { *s = rng.next_u64(); }
        b.iter(|| {
            let s = black_box(&mut st);
            bitcoin_sha3::keccakf(s);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_sha3_throughput,
    bench_rate_boundaries,
    // _bench_keccakf, // enable if keccakf is re-exported
);
criterion_main!(benches);
