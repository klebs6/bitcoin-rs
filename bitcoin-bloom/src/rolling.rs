// ---------------- [ File: bitcoin-bloom/src/rolling.rs ]
crate::ix!();

/**
  | RollingBloomFilter is a probabilistic "keep
  | track of most recently inserted" set.
  | Construct it with the number of items to keep
  | track of, and a false-positive rate. Unlike
  | CBloomFilter, by default nTweak is set to
  | a cryptographically secure random value for
  | you. Similarly rather than clear() the method
  | reset() is provided, which also changes nTweak
  | to decrease the impact of false-positives.
  |
  | contains(item) will always return true if item
  | was one of the last N to 1.5*N insert()'ed
  | ... but may also return true for items that
  | were not inserted.
  |
  | It needs around 1.8 bytes per element per
  | factor 0.1 of false positive rate.
  |
  | For example, if we want 1000 elements, we'd
  | need:
  |
  | - ~1800 bytes for a false positive rate of 0.1
  | - ~3600 bytes for a false positive rate of 0.01
  | - ~5400 bytes for a false positive rate of 0.001
  |
  | If we make these simplifying assumptions:
  |
  | - logFpRate / log(0.5) doesn't get rounded or
  |   clamped in the nHashFuncs calculation
  |
  | - nElements is even, so that
  |   nEntriesPerGeneration == nElements / 2
  |
  | Then we get a more accurate estimate for filter
  | bytes:
  |
  |     3/(log(256)*log(2)) * log(1/fpRate) * nElements
  */
pub struct RollingBloomFilter {
    n_entries_per_generation:  i32,
    n_entries_this_generation: i32,
    n_generation:              i32,
    data:                      Vec<u64>,
    n_tweak:                   u32,
    n_hash_funcs:              i32,
}

impl RollingBloomFilter {

    pub fn new(
        n_elements: u32,
        fp_rate:    f64) -> Self {

        let mut x: Self = unsafe { std::mem::zeroed() };

        let log_fp_rate: f64 = fp_rate.log10();

        /*
          | The optimal number of hash functions
          | is log(fpRate) / log(0.5), but restrict
          | it to the range 1-50.
          |
          */
        x.n_hash_funcs = {

            let h = 0.5_f64.log10();
            let r = (log_fp_rate / h).round();
            let m = min(r as i32, 50);

            max(1,m)
        };

        /*
          | In this rolling bloom filter, we'll
          | store between 2 and 3 generations of
          | nElements / 2 entries.
          |
          */
        x.n_entries_per_generation = ((n_elements + 1) / 2).try_into().unwrap();

        let n_max_elements: u32 
        = (x.n_entries_per_generation * 3)
            .try_into()
            .unwrap();

        /* 
         | The maximum fpRate = pow(1.0 - exp(-nHashFuncs * nMaxElements / nFilterBits), nHashFuncs)
         | =>          pow(fpRate, 1.0 / nHashFuncs) = 1.0 - exp(-nHashFuncs * nMaxElements / nFilterBits)
         | =>          1.0 - pow(fpRate, 1.0 / nHashFuncs) = exp(-nHashFuncs * nMaxElements / nFilterBits)
         | =>          log(1.0 - pow(fpRate, 1.0 / nHashFuncs)) = -nHashFuncs * nMaxElements / nFilterBits
         | =>          nFilterBits = -nHashFuncs * nMaxElements / log(1.0 - pow(fpRate, 1.0 / nHashFuncs))
         | =>          nFilterBits = -nHashFuncs * nMaxElements / log(1.0 - exp(logFpRate / nHashFuncs))
         */
        let n_filter_bits: u32 = {

            let num   = -1.0_f64 * (x.n_hash_funcs as f64) * (n_max_elements as f64);

            let denom = {

                let n = log_fp_rate;
                let d = x.n_hash_funcs;
                let e = (n / (d as f64)).exp();

                (1.0 - e).log10()
            };

            (num / denom).ceil() as u32
        };

        x.data.clear();

        /**
          | For each data element we need to store
          | 2 bits.
          | 
          | If both bits are 0, the bit is treated
          | as unset.
          | 
          | If the bits are (01), (10), or (11), the
          | bit is treated as set in generation 1,
          | 2, or 3 respectively.
          | 
          | These bits are stored in separate integers:
          | position P corresponds to bit (P & 63)
          | of the integers data[(P >> 6) * 2] and
          | data[(P >> 6) * 2 + 1].
          |
          */
        let new_size: usize = (((n_filter_bits + 63) / 64) << 1).try_into().unwrap();

        x.data.resize(new_size, Default::default());

        x.reset();

        x
    }
    
    pub fn insert_key(&mut self, key: &[u8])  {
        
        if self.n_entries_this_generation == self.n_entries_per_generation {

            self.n_entries_this_generation = 0;

            self.n_generation += 1;

            if self.n_generation == 4 {
                self.n_generation = 1;
            }

            let n_generation_mask1: u64 = 0 - (self.n_generation & 1) as u64;
            let n_generation_mask2: u64 = 0 - (self.n_generation >> 1) as u64;

            /*
              | Wipe old entries that used this generation
              | number.
              |
              */
            let mut p: usize = 0;

            while p < self.data.len() {

                let p1:   u64 = self.data[p];
                let p2:   u64 = self.data[p + 1];

                let mask: u64 = (p1 ^ n_generation_mask1) | (p2 ^ n_generation_mask2);

                self.data[p]     = p1 & mask;
                self.data[p + 1] = p2 & mask;

                p += 2
            }
        }

        self.n_entries_this_generation += 1;

        for n in 0..self.n_hash_funcs {

            let h: u32 = rolling_bloom_hash(
                n.try_into().unwrap(),
                self.n_tweak,
                key
            );

            let bit: i32 = (h & 0x3F).try_into().unwrap();

            /*
              | FastMod works with the upper bits of
              | h, so it is safe to ignore that the lower
              | bits of h are already used for bit.
              |
              */
            let pos: usize = fast_mod(h,self.data.len()).try_into().unwrap();

            /*
              | The lowest bit of pos is ignored, and
              | set to zero for the first bit, and to one
              | for the second.
              |
              */
            self.data[pos & !1] = (self.data[pos & !1] & !((1 as u64) << bit)) | ((self.n_generation & 1)  as u64) << bit;
            self.data[pos | 1]  = (self.data[pos | 1]  & !((1 as u64) << bit)) | ((self.n_generation >> 1) as u64) << bit;
        }
    }

    pub fn contains_key(&self, key: &[u8]) -> bool {
        
        for n in 0..self.n_hash_funcs {

            let h:   u32 = rolling_bloom_hash(
                n.try_into().unwrap(),
                self.n_tweak,
                key
            );

            let bit: i32 = (h & 0x3F).try_into().unwrap();

            let pos: usize = {

                let len = self.data.len();

                fast_mod(h, len).try_into().unwrap()
            };

            let j  = self.data[pos & !1] | self.data[pos | 1];
            let js = j >> bit;

            /*
              | If the relevant bit is not set in either
              | data[pos & ~1] or data[pos | 1], the filter
              | does not contain vKey
              |
              */
            if (js & 1) == 0 {
                return false;
            }
        }

        true
    }

    pub fn reset(&mut self)  {
        
        self.n_tweak                   = get_rand(u32::MAX.into()).try_into().unwrap();
        self.n_entries_this_generation = 0;
        self.n_generation              = 1;

        self.data.iter_mut().map(|x| *x = 0);
    }
}
