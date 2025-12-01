# bitcoin-random

A carefully ported subset of Bitcoin Core's entropy collection and pseudo‑random number generation, exposed as a reusable Rust crate.

## Overview

`bitcoin-random` provides a layered randomness subsystem modeled on Bitcoin Core's production code. It combines OS entropy, CPU hardware RNG instructions, high‑resolution timers, environment snapshots, and an internal SHA‑512–based DRBG to supply both fast and cryptographically strong random bytes.

You typically want this crate if:

- You are implementing or porting Bitcoin Core subsystems that expect the same RNG semantics (e.g. consensus‑relevant logic, wallet key generation, or P2P behavior).
- You need a battle‑tested entropy aggregator with defensive mixing, strengthening, and hardware feature detection.
- You want deterministic test RNGs with the same distributional properties as Bitcoin Core’s `FastRandomContext`.

The crate exposes:

- **High‑level APIs** for fast and strong randomness:
  - `get_rand_bytes`, `get_strong_rand_bytes`
  - `get_rand`, `get_rand_int`, `get_rand_hash`
  - `get_random_duration`
- **An internal DRBG and state machine** (`RNGState`, `RNGInnerState`, `RNGStateEvents`) that maintains 256 bits of entropy and continuously mixes in new sources.
- **FastRandomContext**: a deterministic, ChaCha20‑based PRNG implementing `rand_core::RngCore` and a custom `RandRange` trait.
- **Environment entropy collectors** (`rand_add_dynamic_env`, `rand_add_static_env`, and several OS/CPU‑specific helpers) closely matching Bitcoin Core’s C++ implementation.
- **Hardware RNG integration and reporting** for Intel/AMD `RDRAND` and `RDSEED`.
- **Low‑level OS entropy backends** for Linux (`getrandom`/`/dev/urandom`), BSD (`getentropy`, `sysctl KERN_ARND`), macOS, Windows (CryptoAPI), and a `/dev/urandom` fallback.

The design emphasizes:

- **Cryptographic robustness**: entropy is aggregated into a SHA‑512 DRBG which maintains at least 256 bits of internal state; additional entropy is mixed in on each call to strong RNG interfaces.
- **Side‑channel resilience**: sensitive buffers are cleared via `memory_cleanse` after use where appropriate.
- **Deterministic testing**: `FastRandomContext` can be seeded deterministically; a global flag `G_MOCK_DETERMINISTIC_TESTS` exists to route `get_rand` into deterministic mode.
- **Close fidelity to Bitcoin Core**: structure, sequencing, and environment gathering paths are designed to be isomorphic to the upstream C++ implementation, preserving security properties and behavior.

## Crate status and scope

This crate is derived from the `bitcoin-rs` project and is intended primarily to support that ecosystem. It is not a general‑purpose, standards‑audited cryptographic library; however, it is based on code that has undergone extensive real‑world scrutiny in the context of Bitcoin Core.

Use for key generation and consensus‑relevant randomness should follow Bitcoin Core’s threat model and assumptions. If you need FIPS‑validated primitives or formal proofs, you may prefer a dedicated cryptographic RNG crate.

## RNG layers and conceptual model

### 1. OS entropy (`get_os_rand` and friends)

The lowest layer is direct OS entropy:

- **Linux/Android**: `getrandom(2)` when available; falls back to reading `/dev/urandom`.
- **OpenBSD**: `getentropy`.
- **macOS**: `getentropy`.
- **FreeBSD/NetBSD and similar**: `sysctl` with `KERN_ARND`.
- **Windows**: CryptoAPI `CryptAcquireContextW` + `CryptGenRandom`.
- **Portable fallback**: `/dev/urandom` where no specialized support is compiled in.

These backends always try to fill a 32‑byte buffer (`NUM_OS_RANDOM_BYTES`) and **fail hard** via `rand_failure()` if the OS cannot supply randomness.

### 2. Internal DRBG (`RNGState` and `RNGInnerState`)

`RNGInnerState` maintains:

- `state: [u8; 32]` — 256 bits of internal entropy.
- `counter: u64` — a monotonically increasing counter mixed into each extraction.
- `strongly_seeded: bool` — indicates whether a strong (OS‑backed) seed has been mixed in at least once.

`RNGState` wraps this inner state with mutexes and an event hasher (`Sha256`) used to mix in timing and application events.

Entropy extraction and mixing is performed by:

```rust
pub fn mix_extract(
    &mut self,
    out: &mut [u8],
    num: usize,
    hasher: Sha512,
    strong_seed: bool,
) -> bool
```

Key properties:

- The current `state` and `counter` are appended to a `Sha512` hasher plus any external entropy provided by the caller.
- `finalize()` of the hasher yields 64 bytes:
  - The **last 32 bytes** become the new internal `state`.
  - The **first 32 bytes** (optionally truncated to `num ≤ 32`) are returned as output.
- If `strong_seed` is `true`, `strongly_seeded` is updated; the function returns the current `strongly_seeded` flag so callers (e.g. `proc_rand`) can determine whether strong seeding has ever happened.

This structure matches Bitcoin Core’s DRBG: the new state is a one‑way function of the old state, the counter, and new entropy. Compromise of a single output does not reveal prior state.

### 3. Seeding strategies

Several seeding functions, each with distinct cost and entropy assumptions, feed into the DRBG:

- **`seed_fast`**: minimal latency seed for high‑frequency use.
  - Uses stack pointer, optional hardware RNG (`seed_hardware_fast`), and a high‑resolution timestamp (`seed_timestamp`).
- **`seed_slow`**: stronger reseed path.
  - Calls `seed_fast`.
  - Adds 32 bytes of OS randomness (`get_os_rand`).
  - Incorporates event hash via `rng.seed_events`.
  - Adds another high‑precision timestamp.
- **`seed_hardware_slow`**: high‑quality hardware RNG (if available).
  - Prefers `RDSEED` and collects 256 bits directly.
  - Falls back to repeatedly XORed `RDRAND` results to obtain 256 bits with inter‑call reseeding.
- **`seed_periodic`**: expensive reseed path intended to run periodically (e.g. once a minute).
  - Includes `seed_fast` + `seed_timestamp` + `rng.seed_events`.
  - Mixes a wide set of dynamic environment data via `rand_add_dynamic_env` (timers, `/proc` snapshots, resource usage, etc.).
  - Strengthens the resulting seed with a `strengthen` loop over ~10 ms.
- **`seed_startup`**: heavy startup seeding for first use.
  - Uses `seed_hardware_slow`.
  - Uses `seed_slow`.
  - Mixes dynamic and static environment data (`rand_add_dynamic_env`, `rand_add_static_env`).
  - Applies `strengthen` for ~100 ms.

The `strengthen` function repeatedly hashes an initial 32‑byte seed through `Sha512` for a configurable number of microseconds, periodically folding in high‑resolution performance counters. This is conceptually a time‑hardening step: it converts any partial entropy from noisy sources into pseudorandom bits while amortizing over CPU time.

### 4. Hardware RNG integration (x86/x86_64)

When compiled with `have_getcpuid` and on supported targets:

- `init_hardware_rand` uses CPUID leaves to query `RDRAND` and `RDSEED` support, setting `G_RDRAND_SUPPORTED` and `G_RDSEED_SUPPORTED`.
- `report_hardware_rand` logs which features are in use.
- `get_rd_rand` and `get_rd_seed` use inline assembly to issue the corresponding instructions, including the recommended retry loops to handle transient failure.

The seeding functions (`seed_hardware_fast` / `seed_hardware_slow`) only use these instructions when the corresponding hardware support flags are set.

### 5. Event and environment entropy

To defend against state compromise and to continuously add entropy, the RNG mixes in information from:

- **Event timing**: `rand_add_event(event_info: u32)` writes the event identifier and low 32 bits of a high‑frequency performance counter into a `Sha256` hasher in `RNGStateEvents`. This is suitable for network packet arrivals, user input, or other external events.
- **Dynamic environment** (`rand_add_dynamic_env`):
  - System clocks: `CLOCK_REALTIME`, `CLOCK_MONOTONIC`, `CLOCK_BOOTTIME` (Linux), `gettimeofday`, C++‑style triple of `system_clock`, `steady_clock`, `high_resolution_clock` equivalents.
  - Resource usage: `getrusage(RUSAGE_SELF)`.
  - `/proc` snapshots: several small files on Linux (`diskstats`, `vmstat`, `schedstat`, `zoneinfo`, `meminfo`, `softirqs`, `stat`, `self/schedstat`, `self/status`).
  - Heap/stack pointers: allocations via `malloc` and raw addresses.
  - Network interfaces: `getifaddrs` enumeration, interface names, flags, and socket addresses.
  - Hostname, uname fields, and key filesystem paths.
- **Static environment** (`rand_add_static_env`):
  - Sizes of fundamental types and signedness of `char`.
  - Linux `getauxval` data (`AT_HWCAP`, `AT_HWCAP2`, `AT_RANDOM`, `AT_PLATFORM`, `AT_EXECFN`).
  - CPU feature set via CPUID enumeration (`add_allcpuid`).
  - Addresses of code and data symbols (e.g. function pointers, `malloc`, `errno`, `environ`).
  - Full environment variables.
  - Process and thread identifiers (`pid`, `ppid`, `sid`, `pgid`, `uid/gid`, `euid/egid`, and `pthread_self`).

These sources are not relied upon as primary entropy; instead, they hedge against partial compromise and OS RNG failures by adding unpredictability from system‑specific and timing‑dependent state.

## Public API

### Top‑level RNG functions

These are the main entry points most consumers should use.

#### `get_rand_bytes`

```rust
pub fn get_rand_bytes(buf: &mut [u8], num: i32)
```

- Fills `buf[..num]` with random bytes generated via the internal PRNG.
- Uses `RNGLevel::FAST` seeding via `proc_rand`, so it is tuned for throughput rather than maximum entropy.
- Thread‑safe.

Usage:

```rust
use bitcoin_random::get_rand_bytes;

let mut key = [0u8; 32];
get_rand_bytes(&mut key, key.len() as i32);
```

#### `get_strong_rand_bytes`

```rust
pub fn get_strong_rand_bytes(buf: &mut [u8], num: i32)
```

- Fills `buf[..num]` with random bytes using the **strong** seeding path (`RNGLevel::SLOW`).
- This path always incorporates OS randomness and event information and will abort if the OS RNG fails.
- Intended for security‑critical operations such as key generation.

Usage:

```rust
use bitcoin_random::get_strong_rand_bytes;

let mut secret = [0u8; 32];
get_strong_rand_bytes(&mut secret, secret.len() as i32);
```

#### `get_rand`

```rust
pub fn get_rand(n_max: u64) -> u64
```

- Returns a uniform random integer in `[0, n_max)` using `FastRandomContext::randrange`.
- Preconditions: `n_max > 0`.
- Uses the global `G_MOCK_DETERMINISTIC_TESTS` flag to optionally switch into deterministic mode for tests.

Example:

```rust
use bitcoin_random::get_rand;

let x = get_rand(1_000_000);
assert!(x < 1_000_000);
```

#### `get_rand_int`

```rust
pub fn get_rand_int(n_max: i32) -> i32
```

- Thin wrapper around `get_rand`, returning `i32` instead of `u64`.
- Preconditions: `n_max > 0`.

#### `get_rand_hash`

```rust
pub fn get_rand_hash() -> u256
```

- Returns a random 256‑bit value as a `u256` type (from the surrounding Bitcoin numeric crate).
- Backed by `get_rand_bytes` over the raw bytes of the `u256` structure.

This is the primitive used by `FastRandomContext::random_seed` to construct ChaCha20 keys.

#### `get_random_duration`

```rust
pub fn get_random_duration(max: Duration) -> Duration
```

- Returns a random `Duration` uniformly distributed in `[0, max)`.
- Preconditions: `max.as_seconds_f64() > 0`.
- Used to derive randomized timers or retry backoffs.

### RNG lifecycle helpers

#### `random_init`

```rust
pub fn random_init()
```

- Optional explicit initialization step.
- Internally calls `proc_rand(&mut [], 0, RNGLevel::FAST)` to trigger `RNGState` setup and hardware RNG detection.
- Then calls `report_hardware_rand` to log discovered CPU RNG features.

You may call this early in process startup to ensure RNG initialization happens on your own schedule, instead of lazily on first use.

#### `random_sanity_check`

```rust
pub fn random_sanity_check() -> bool
```

- Performs a self‑test of the OS randomness backend and the performance counter:
  - Repeatedly calls `get_os_rand` on a zeroed buffer and checks that each byte is overwritten at least once within a bounded number of attempts.
  - Verifies that `get_performance_counter` increases across a call to `get_os_rand` + 1 ms sleep.
  - Feeds the performance counter values back into the global `RNGState` as additional entropy.
- Returns `false` if the OS RNG appears to misbehave or if the performance counter does not advance.

This is primarily a diagnostic / defensive primitive.

#### `rand_add_periodic`

```rust
pub fn rand_add_periodic()
```

- Invokes `proc_rand(&mut [], 0, RNGLevel::PERIODIC)`.
- Intended to be called periodically (e.g. via a timer) to add slow, expensive environmental entropy (perfmon, `/proc`, network interfaces, etc.) and strengthen the state.

#### `rand_add_event`

```rust
pub fn rand_add_event(event_info: u32)
```

- Adds an event descriptor plus timing information into the RNG’s event hasher.
- Thread‑safe.

You can use this to feed in application‑specific events that may provide extra entropy, such as network message arrivals, user actions, or I/O completions.

### `FastRandomContext`

`FastRandomContext` is a fast, non‑cryptographic PRNG seeded from strong randomness (unless explicitly configured as deterministic). It is conceptually similar to a C++ `UniformRandomBitGenerator` and implements both the custom `RandRange` trait and `rand_core::RngCore`.

Key properties:

- Underlying core: `ChaCha20` stream cipher.
- Seed: 256‑bit `u256` key from `get_rand_hash()` when `requires_seed` is true.
- Deterministic option: when constructed with `deterministic = true`, the context is seeded with a fixed all‑zero `u256` key.
- Not thread‑safe; each instance is intended for single‑threaded use.

#### Construction

```rust
impl FastRandomContext {
    pub fn new(deterministic: bool) -> Self { /* ... */ }
}

impl Default for FastRandomContext { /* deterministic = false */ }

impl From<&u256> for FastRandomContext { /* seed from explicit u256 */ }
impl From<&mut FastRandomContext> for FastRandomContext { /* move semantics */ }
```

Typical usage:

```rust
use bitcoin_random::FastRandomContext;

// Non‑deterministic context, seeded from global RNG
let mut rng = FastRandomContext::default();
let x = rng.rand64();

// Deterministic context for tests or simulations
let mut det_rng = FastRandomContext::new(true);
let a = det_rng.rand32();
let b = det_rng.rand32();
```

If a `FastRandomContext` is moved using `From<&mut FastRandomContext>`, the original instance becomes logically invalid until it reseeds itself on next use (`requires_seed` is set back to `true`).

#### Basic methods

```rust
impl FastRandomContext {
    pub fn rand64(&mut self) -> u64;
    pub fn rand32(&mut self) -> u32;
    pub fn randbits(&mut self, bits: i32) -> u64; // 0 ≤ bits ≤ 64
    pub fn randbool(&mut self) -> bool;
    pub fn rand256(&mut self) -> u256;
    pub fn randbytes(&mut self, len: usize) -> Vec<u8>;

    pub fn min() -> u64 { 0 }
    pub fn max() -> u64 { u64::MAX }
    #[inline]
    pub fn invoke(&mut self) -> u64 { self.rand64() }
}
```

`randbits` uses an internal bit buffer (`bitbuf`) to collect and reuse entropy at bit‑level granularity, avoiding waste when many small random values are needed.

`randbytes` uses the underlying ChaCha20 keystream directly and is efficient for filling large buffers.

#### Integration with `rand_core`

`FastRandomContext` implements `rand_core::RngCore`:

```rust
impl rand_core::RngCore for FastRandomContext {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
    fn fill_bytes(&mut self, dest: &mut [u8]);
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error>;
}
```

This allows you to plug it into consumers that expect a `RngCore` implementation.

#### `RandRange` trait

The crate defines a minimal trait for generating uniform integers in a given range:

```rust
pub trait RandRange {
    fn randrange(&mut self, range: u64) -> u64;
}

impl RandRange for FastRandomContext { /* rejection sampling */ }
```

`randrange` implements rejection sampling:

- Let `range > 0`; compute `bits = count_bits(range - 1)`.
- Draw `bits` random bits via `randbits(bits)`.
- If the sample is ≤ `range - 1`, accept; otherwise, retry.

This ensures uniform distribution in `[0, range)` for any range not necessarily a power of two.

### Shuffling utilities

The crate offers a highly specialized shuffle function optimized to minimize entropy waste and avoid specific issues with `std::shuffle` in some C++ standard libraries. The Rust port uses a generic iterator/index model constrained by several traits.

```rust
pub fn shuffle<'a, I: 'a, R>(mut first: I, last: I, mut rng: R)
where
    I: PartialEq + Copy + Sub<Output = I> + AddAssign<u64> + Add<Output = u64>,
    I: Deref<Target = &'a mut I> + DerefMut<Target = &'a mut I>,
    I: Add<u64>,
    R: RandRange,
    <I as Add<u64>>::Output: Deref<Target = &'a mut I>,
    <I as Add<u64>>::Output: DerefMut<Target = &'a mut I>,
    u64: From<I>,
```

This is a direct structural translation of the C++ `shuffle` used in Bitcoin Core's container utilities, which assumes a pointer‑like index type. It is not intended as a general drop‑in replacement for typical Rust iterators and may require custom wrapper types; in many Rust codebases you will prefer `rand::seq::SliceRandom` instead.

`shuffle_all` is present but currently `todo!()`, mirroring an unimplemented C++ pattern.

## Platform behavior and performance counters

### `get_performance_counter`

```rust
pub fn get_performance_counter() -> i64
```

- On x86/x86_64 Windows: uses `RDTSC` via `_rdtsc()` intrinsics.
- On non‑Windows x86/x86_64: uses inline assembly `rdtsc`.
- On other platforms: falls back to `quanta::Instant::now().as_u64()`.

The counter is used for:

- Event timing in `rand_add_event`.
- Strengthening loops (`strengthen`).
- Periodic sanity checks.

It is treated as an entropy source only in combination with other inputs; alone it is not assumed to be unpredictable.

## Example: generating keys and nonces

```rust
use bitcoin_random::{
    random_init,
    random_sanity_check,
    get_strong_rand_bytes,
    get_rand_bytes,
    FastRandomContext,
};

fn main() {
    // Optional: ensure RNG is initialized early and hardware RNG reported.
    random_init();

    // Optional: run a one‑time self‑test.
    assert!(random_sanity_check(), "OS RNG failed sanity check");

    // Generate a 32‑byte secret (e.g. for an EC private key).
    let mut sk = [0u8; 32];
    get_strong_rand_bytes(&mut sk, sk.len() as i32);

    // Generate a non‑cryptographic nonce using the fast path.
    let mut nonce = [0u8; 16];
    get_rand_bytes(&mut nonce, nonce.len() as i32);

    // Use a dedicated, fast context for many draws.
    let mut ctx = FastRandomContext::default();
    let session_id = ctx.rand64();

    println!("secret key: {:02x?}", sk);
    println!("nonce: {:02x?}", nonce);
    println!("session id: {}", session_id);
}
```

## Safety and correctness notes

- Many functions marked `unsafe` in the implementation are strictly internal; the public API is safe, but relies heavily on FFI and OS behavior.
- Misconfigured `cfg` flags can change which backend is used for `get_os_rand`; link errors or runtime aborts typically indicate such misconfiguration.
- `G_MOCK_DETERMINISTIC_TESTS` is a global `AtomicBool` that affects `get_rand`; it is intended for tests and should not be enabled in production.
- Functions like `rand_add_dynamic_env` and `rand_add_static_env` may read a substantial amount of system state. They are not cheap and should be used via the seeding interfaces (`seed_startup`, `seed_periodic`) rather than directly in hot paths.

## Integration with the rest of `bitcoin-rs`

This crate is designed to be used by other crates in the `bitcoin-rs` repository, sharing:

- The `u256` type for 256‑bit values.
- Common logging macros (`log_print!`, `log_printf!`).
- Shared cryptographic primitives (`Sha256`, `Sha512`, `ChaCha20`, `SecureAllocator`).

When using `bitcoin-random` in isolation, be aware that some items (numeric types, logging, memory allocators) come from sibling crates in the same repository.

## License

This crate is licensed under the **MIT License**, consistent with the rest of the `bitcoin-rs` repository.

## Repository

The source for this crate lives in:

- GitHub: <https://github.com/klebs6/bitcoin-rs>

The RNG implementation here is intended to be kept structurally close to its C++ counterpart. When in doubt about behavior or security properties, consult the corresponding Bitcoin Core source files and documentation.
