// ---------------- [ File: bitcoinleveldb-arena/src/arena_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/arena_test.cc]

/// A simple pseudo-random generator to mirror the C++ `Random` usage.
/// This is not cryptographically secure.
#[derive(Debug, Getters, Setters, Builder)]
struct LcgRandom {
    #[builder(default = "301_u64")]
    seed: u64,
}

impl LcgRandom {
    /// Create a new pseudo-random generator with the given seed.
    pub fn new(seed: u64) -> Self {
        info!("Creating LcgRandom with seed={}", seed);
        Self { seed }
    }

    /// Returns a random number in [0, range).
    /// Uses a basic linear congruential generator approach.
    pub fn uniform(&mut self, range: u64) -> u64 {
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        let val = (self.seed >> 32) % range;
        trace!("LcgRandom::uniform -> {}", val);
        val
    }

    /// Returns true with probability 1/n.
    pub fn one_in(&mut self, n: u64) -> bool {
        let r = self.uniform(n);
        let decision = r == 0;
        trace!("LcgRandom::one_in({}) -> {}", n, decision);
        decision
    }
}

/// A trivial struct to align with the original `struct ArenaTest {}` from C++.
/// (No internal fields needed, just a placeholder.)
#[derive(Debug)]
pub struct ArenaTest { }

impl Default for ArenaTest {
    fn default() -> Self {
        info!("Creating an ArenaTest placeholder");
        Self {}
    }
}

#[traced_test]
fn test_arena_empty() {
    info!("Running test_arena_empty");
    let arena = Arena::default();
    assert_eq!(
        arena.memory_usage(),
        0,
        "Expected memory usage to be 0 in an empty Arena"
    );
    info!("test_arena_empty passed.");
}

#[traced_test]
fn test_arena_simple() {
    info!("Running test_arena_simple");

    // We track each allocation in a vector of (size, pointer).
    // Then we verify the allocated bytes and also check memory usage thresholds.
    let mut allocated: Vec<(usize, *mut u8)> = Vec::new();
    let mut arena = Arena::default();

    // Match the original test structure
    let n = 100_000;
    let mut total_bytes: usize = 0;
    let mut rnd = LcgRandom::new(301);

    for i in 0..n {
        // Determine how many bytes to allocate.
        let mut s = if i % (n / 10) == 0 {
            i as usize
        } else if rnd.one_in(4000) {
            rnd.uniform(6000) as usize
        } else if rnd.one_in(10) {
            rnd.uniform(100) as usize
        } else {
            rnd.uniform(20) as usize
        };

        if s == 0 {
            // Our arena disallows size 0 allocations, so use 1.
            s = 1;
        }

        // Occasionally allocate aligned
        let ptr = if rnd.one_in(10) {
            arena.allocate_aligned(s)
        } else {
            arena.allocate(s)
        };

        // Fill the "i-th" allocation with (i % 256).
        unsafe {
            for b in 0..s {
                *ptr.add(b) = (i % 256) as u8;
            }
        }

        total_bytes += s;
        allocated.push((s, ptr));

        // Check that memory usage is never less than we've allocated.
        let mem_usage = arena.memory_usage();
        assert!(
            mem_usage >= total_bytes,
            "Arena memory usage should be >= total allocated bytes ({} >= {})",
            mem_usage,
            total_bytes
        );

        // After the first 1/10 of allocations, usage should not exceed 110%.
        if i > n / 10 {
            let ratio = (mem_usage as f64) / (total_bytes as f64);
            assert!(
                ratio <= 1.10,
                "Arena memory usage should stay within 110% of total bytes. Ratio was {:.4}",
                ratio
            );
        }
    }

    // Now verify that the data was written correctly for each allocation.
    for (i, (num_bytes, raw_ptr)) in allocated.iter().enumerate() {
        let expected_val = (i % 256) as u8;
        unsafe {
            for b in 0..*num_bytes {
                let actual_val = *raw_ptr.add(b);
                assert_eq!(
                    actual_val, expected_val,
                    "Data mismatch at allocation {}, byte {}, got {}, expected {}",
                    i, b, actual_val, expected_val
                );
            }
        }
    }

    info!("test_arena_simple passed.");
}
