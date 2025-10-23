// ---------------- [ File: bitcoin-random/src/random.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/random.h]

/**
  | Generate a uniform random duration
  | in the range [0..max). Precondition:
  | max.count() > 0
  |
  */
pub fn get_random_duration(max: Duration) -> Duration 
{
    let ticks = max.as_seconds_f64();
    let rand  = get_rand(ticks as u64) as f64;

    Duration::seconds_f64(rand)
}

lazy_static!{
    pub static ref GET_RAND_MICROS: Duration = get_random_duration(Duration::microseconds(i64::MAX));
    pub static ref GET_RAND_MILLIS: Duration = get_random_duration(Duration::milliseconds(i64::MAX));
}

/**
  | Number of random bytes returned by GetOSRand.
  | 
  | When changing this constant make sure
  | to change all call sites, and make sure
  | that the underlying OS APIs for all platforms
  | support the number. (many cap out at
  | 256 bytes).
  |
  */
pub const NUM_OS_RANDOM_BYTES: i32 = 32;

//-------------------------------------------[.cpp/bitcoin/src/random.cpp]

pub fn rand_failure()  {
    log_printf!("Failed to read randomness, aborting\n");
    std::intrinsics::abort();
}

#[cfg(have_getcpuid)]
lazy_static!{
    pub static ref G_RDRAND_SUPPORTED: AtomicBool = AtomicBool::new(false);
    pub static ref G_RDSEED_SUPPORTED: AtomicBool = AtomicBool::new(false);
}

#[cfg(have_getcpuid)]
pub const CPUID_F1_ECX_RDRAND: u32 = 0x40000000;

#[cfg(have_getcpuid)]
pub const CPUID_F7_EBX_RDSEED: u32 = 0x00040000;

#[cfg(have_getcpuid)]
#[cfg(bit_RDRND)]
const_assert!{
    CPUID_F1_ECX_RDRAND == bit_RDRND, 
    "Unexpected value for bit_RDRND"
}

#[cfg(have_getcpuid)]
#[cfg(bit_RDSEED)]
const_assert!{
    CPUID_F7_EBX_RDSEED == bit_RDSEED, 
    "Unexpected value for bit_RDSEED"
}

pub enum RNGLevel {

    /**
      | Automatically called by GetRandBytes
      |
      */
    FAST, 

    /**
      | Automatically called by GetStrongRandBytes
      |
      */
    SLOW, 

    /**
      | Called by RandAddPeriodic()
      |
      */
    PERIODIC, 
}

lazy_static!{
    pub static ref G_MOCK_DETERMINISTIC_TESTS: AtomicBool = AtomicBool::new(false);
}

#[cfg(test)]
mod random_spec {
    use super::*;

    #[traced_test]
    fn get_random_duration_is_within_max() {
        let max = Duration::seconds(10_000); // reasonably large
        let d = get_random_duration(max);
        assert!(d >= Duration::ZERO);
        assert!(d < max);
    }

    #[traced_test]
    fn globals_can_be_constructed() {
        // Accessing these ensures they are constructed.
        let _ = &*GET_RAND_MICROS;
        let _ = &*GET_RAND_MILLIS;
    }
}
