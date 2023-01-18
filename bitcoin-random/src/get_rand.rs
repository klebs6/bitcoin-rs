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
