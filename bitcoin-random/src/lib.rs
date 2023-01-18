/*!
  | Overall design of the RNG and entropy sources.
  |
  | We maintain a single global 256-bit RNG state
  | for all high-quality randomness.
  |
  | The following (classes of) functions interact
  | with that state by mixing in new entropy, and
  | optionally extracting random output from it:
  |
  | - The GetRand*() class of functions, as well as
  |   construction of FastRandomContext objects,
  |   perform 'fast' seeding, consisting of mixing
  |   in:
  |
  |   - A stack pointer (indirectly committing to
  |   calling thread and call stack)
  |
  |   - A high-precision timestamp (rdtsc when
  |   available, c++ high_resolution_clock
  |   otherwise)
  |
  |   - 64 bits from the hardware RNG (rdrand) when
  |   available.
  |
  |   These entropy sources are very fast, and only
  |   designed to protect against situations where
  |   a VM state restore/copy results in multiple
  |   systems with the same randomness.
  |
  |   FastRandomContext on the other hand does not
  |   protect against this once created, but is
  |   even faster (and acceptable to use inside
  |   tight loops).
  |
  | - The GetStrongRand*() class of function
  |   perform 'slow' seeding, including everything
  |   that fast seeding includes, but additionally:
  |
  |   - OS entropy (/dev/urandom, getrandom(),
  |     ...). The application will terminate if
  |     this entropy source fails.
  |
  |   - Another high-precision timestamp
  |     (indirectly committing to a benchmark of
  |     all the previous sources).
  |
  |   These entropy sources are slower, but
  |   designed to make sure the RNG state contains
  |   fresh data that is unpredictable to
  |   attackers.
  |
  | - RandAddPeriodic() seeds everything that fast
  |   seeding includes, but additionally:
  |
  |   - A high-precision timestamp
  |
  |   - Dynamic environment data (performance
  |   monitoring, ...)
  |
  |   - Strengthen the entropy for 10 ms using
  |   repeated SHA512.
  |
  |   This is run once every minute.
  |
  | On first use of the RNG (regardless of what
  | function is called first), all entropy sources
  | used in the 'slow' seeder are included, but
  | also:
  |
  | - 256 bits from the hardware RNG 
  |   (rdseed or rdrand) when available.
  |
  | - Dynamic environment data (performance
  |   monitoring, ...)
  |
  | - Static environment data
  |
  | - Strengthen the entropy for 100 ms using
  |   repeated SHA512.
  |
  | When mixing in new entropy, H = SHA512(entropy
  | || old_rng_state) is computed, and (up to)
  | the first 32 bytes of H are produced as output,
  | while the last 32 bytes become the new RNG
  | state.
  */

#![feature(const_fn_trait_bound)]
#![feature(core_intrinsics)]
#![feature(allocator_api)]

#[macro_use] mod imports; use imports::*;

x!{dev_urandom}
x!{fast_random_context}
x!{get_os_rand}
x!{get_rand}
x!{hardware_rand}
x!{init}
x!{performance_counter}
x!{proc_rand}
x!{random}
x!{randomenv}
x!{rd_rand}
x!{rd_seed}
x!{sanity}
x!{seed}
x!{seed_hardware}
x!{shuffle}
x!{state}
x!{strengthen}
