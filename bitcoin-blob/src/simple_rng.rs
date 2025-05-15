// ---------------- [ File: bitcoin-blob/src/simple_rng.rs ]
crate::ix!();

/// A simple pseudo-random generator to test comparisons, etc.
/// We can keep it predictable for reproducibility.
pub struct SimpleRng(u64);

impl SimpleRng {

    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        // linear congruential generator
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        self.0
    }

    /// Fill a mutable buffer with pseudo-random bytes.
    pub fn fill_bytes(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let rnd = self.next_u64().to_le_bytes();
            let n = chunk.len();
            chunk.copy_from_slice(&rnd[..n]);
        }
    }
}
