// ---------------- [ File: bitcoinleveldb-rand/src/random.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/random.h]

/**
  | A very simple random number generator.  Not
  | especially good at generating truly random
  | bits, but good enough for our needs in this
  | package.
  */
pub struct Random {
    seed: u32,
}

impl Random {

    const MODULUS: u32 = 2_147_483_647; // 2^31-1

    const MULTIPLIER: u64 = 16_807;     // bits 14, 8, 7, 5, 2, 1, 0

    pub fn new(s: u32) -> Self {
        debug!("Random::new invoked with raw seed {}", s);

        // Mask to 31 bits, matching LevelDB's `s & 0x7fffffffu` behavior.
        let mut seed = s & Self::MODULUS;

        // Avoid degenerate seeds that would destroy randomness.
        if seed == 0 || seed == Self::MODULUS {
            warn!(
                "Random::new received pathological normalized seed {}; \
                 forcing seed to 1 to avoid a degenerate sequence",
                seed
            );
            seed = 1;
        }

        debug!("Random::new using internal seed {}", seed);
        Random { seed }
    }
    
    pub fn next(&mut self) -> u32 {

        // We are computing
        //       seed_ = (seed_ * A) % M,    where M = 2^31-1
        //
        // seed_ must not be zero or M, or else all subsequent computed values
        // will be zero or M respectively.  For all other values, seed_ will end
        // up cycling through every number in [1,M-1]
        let previous_seed = self.seed;
        trace!("Random::next called; previous_seed={}", previous_seed);

        // Compute seed_ = (seed_ * A) % M using the classic Park-Miller trick
        // implemented exactly as in LevelDB's util/random.h.
        let product = (self.seed as u64) * Self::MULTIPLIER;

        // Compute (product % M) using the fact that ((x << 31) % M) == x.
        let mut new_seed =
            ((product >> 31) + (product & (Self::MODULUS as u64))) as u32;

        if new_seed > Self::MODULUS {
            new_seed -= Self::MODULUS;
        }

        self.seed = new_seed;

        trace!(
            "Random::next produced new_seed={} from previous_seed={}",
            new_seed,
            previous_seed
        );

        new_seed
    }

    /// Returns a uniformly distributed value in the range [0..n-1].
    ///
    /// REQUIRES: n > 0 (invalid input is logged and returns 0).
    pub fn uniform(&mut self, n: i32) -> u32 {
        trace!("Random::uniform called with n={}", n);

        if n <= 0 {
            error!(
                "Random::uniform requires n > 0; received {}. Returning 0.",
                n
            );
            return 0;
        }

        let bound = n as u32;
        let value = self.next() % bound;

        trace!(
            "Random::uniform returning value {} in range [0, {})",
            value,
            bound
        );

        value
    }

    /// Randomly returns true ~"1/n" of the time, and false otherwise.
    ///
    /// REQUIRES: n > 0 (invalid input is logged and returns false).
    pub fn one_in(&mut self, n: i32) -> bool {
        trace!("Random::one_in called with n={}", n);

        if n <= 0 {
            error!(
                "Random::one_in requires n > 0; received {}. Returning false.",
                n
            );
            return false;
        }

        let bound = n as u32;
        let is_hit = (self.next() % bound) == 0;

        trace!(
            "Random::one_in with n={} returning {}",
            n,
            is_hit
        );

        is_hit
    }

    /// Skewed: pick "base" uniformly from range [0,max_log] and then
    /// return "base" random bits. The effect is to pick a number in the
    /// range [0,2^max_log-1] with exponential bias towards smaller numbers.
    ///
    /// REQUIRES: max_log >= 0.
    /// For very large max_log values, the effective max_log is capped at 30
    /// to avoid overflow and remain well-defined on all platforms.
    pub fn skewed(&mut self, max_log: i32) -> u32 {
        trace!("Random::skewed called with max_log={}", max_log);

        if max_log < 0 {
            error!(
                "Random::skewed requires max_log >= 0; received {}. Returning 0.",
                max_log
            );
            return 0;
        }

        if max_log >= 31 {
            warn!(
                "Random::skewed received large max_log={}; \
                 capping effective max_log at 30 to avoid overflow.",
                max_log
            );
        }

        let effective_max_log = if max_log >= 31 { 30 } else { max_log };

        // `base` is uniformly distributed in [0, effective_max_log].
        let base = self.uniform(effective_max_log + 1);
        let range = 1u32 << base;

        let value = self.uniform(range as i32);

        trace!(
            "Random::skewed using effective_max_log={}, selected base={}, range={}, value={}",
            effective_max_log,
            base,
            range,
            value
        );

        value
    }
}
