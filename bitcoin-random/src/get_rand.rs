// ---------------- [ File: bitcoin-random/src/get_rand.rs ]
crate::ix!();

/**
  | Generate random data via the internal
  | PRNG.
  | 
  | These functions are designed to be fast
  | (sub microsecond), but do not necessarily
  | meaningfully add entropy to the PRNG
  | state.
  | 
  | Thread-safe.
  |
  */
pub fn get_rand_bytes(
        buf: &mut [u8],
        num: i32)  {
    proc_rand(buf, num, RNGLevel::FAST);
}

/**
  | Gather entropy from various sources,
  | feed it into the internal PRNG, and generate
  | random data using it.
  | 
  | This function will cause failure whenever
  | the OS RNG fails.
  | 
  | Thread-safe.
  |
  */
pub fn get_strong_rand_bytes(
        buf: &mut [u8],
        num: i32)  {
    proc_rand(buf, num, RNGLevel::SLOW);
}

/**
  | Generate a uniform random integer in
  | the range [0..range). Precondition:
  | range > 0
  |
  */
pub fn get_rand(n_max: u64) -> u64 {
    
    let deterministic = G_MOCK_DETERMINISTIC_TESTS.load(atomic::Ordering::Relaxed);
    let mut ctx = FastRandomContext::new(deterministic);
    ctx.randrange(n_max)
}

pub fn get_rand_int(n_max: i32) -> i32 {

    let n_max: u64 = n_max.try_into().unwrap();

    let result: u64 = get_rand(n_max);

    result.try_into().unwrap()
}

pub fn get_rand_hash() -> u256 {
    
    let mut hash: u256 = u256::default();

    get_rand_bytes(
        unsafe { std::slice::from_raw_parts_mut(&mut hash as *mut _ as *mut u8, hash.byte_len()) }, 
        size_of_val(&hash).try_into().unwrap()
    );

    hash
}

#[cfg(test)]
mod get_rand_spec {
    use super::*;

    #[traced_test]
    fn get_rand_bytes_and_strong_bytes_fill_requested_length() {
        let mut buf_fast = [0u8; 32];
        get_rand_bytes(&mut buf_fast, 32);
        assert!(buf_fast.iter().any(|&x| x != 0));

        let mut buf_strong = [0u8; 32];
        get_strong_rand_bytes(&mut buf_strong, 32);
        assert!(buf_strong.iter().any(|&x| x != 0));

        // strong bytes need not differ from fast in content, but likely will.
    }

    #[traced_test]
    fn get_rand_and_get_rand_int_are_in_range() {
        for r in [1u64, 2, 10, 1_000] {
            let v = get_rand(r);
            assert!(v < r);
        }
        for r in [1i32, 2, 10, 123456] {
            let v = get_rand_int(r);
            assert!(v < r);
        }
    }

    #[traced_test]
    fn deterministic_mock_flag_makes_get_rand_repeatable() {
        G_MOCK_DETERMINISTIC_TESTS.store(true, atomic::Ordering::Relaxed);
        let a = get_rand(1_000_000);
        let b = get_rand(1_000_000);
        // With deterministic contexts (new(true) per call), both calls start from same seed and sequence.
        // The first sample (same range) should be identical.
        assert_eq!(a, b);
        G_MOCK_DETERMINISTIC_TESTS.store(false, atomic::Ordering::Relaxed);
    }

    #[traced_test]
    fn get_rand_hash_returns_u256_bytes() {
        let h1 = get_rand_hash();
        let h2 = get_rand_hash();
        // Extremely unlikely to be equal:
        assert_ne!(h1.blob(), h2.blob());
    }
}
