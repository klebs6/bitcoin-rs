// ---------------- [ File: bitcoin-bloom/src/common.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/common/bloom.h]
//-------------------------------------------[.cpp/bitcoin/src/common/bloom.cpp]

/**
  | 20,000 items with fp rate < 0.1% or 10,000
  | items and <0.0001%
  |
  */
pub const MAX_BLOOM_FILTER_SIZE: usize = 36000; // bytes
pub const MAX_HASH_FUNCS:        u32 = 50;

pub const LN2SQUARED: f64 = 0.4804530139182014246671025263266649717305529515945455;
pub const LN2:        f64 = 0.6931471805599453094172321214581765680755001343602552;

/**
  | Similar to BloomFilter::Hash
  |
  */
#[inline] pub fn rolling_bloom_hash(
        n_hash_num:   u32,
        n_tweak:      u32,
        data_to_hash: &[u8]) -> u32 {
    
    murmur_hash3(n_hash_num * 0xFBA4C795 + n_tweak, data_to_hash)
}

/**
  | A replacement for x % n. This assumes that
  | x and n are 32bit integers, and x is
  | a uniformly random distributed 32bit value
  | which should be the case for a good hash.  See
  | https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
  */
#[inline] pub fn fast_mod(x: u32, n: usize) -> u32 {
    (((x as u64) * (n as u64)) >> 32).try_into().unwrap()
}
