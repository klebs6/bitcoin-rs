# bitcoinleveldb-rand

A small, deterministic pseudo-random number generator (PRNG) that reproduces the behavior of LevelDB's historical `Random` implementation. This crate exists to make the LevelDB-compatible RNG available as an independent, well-documented Rust component.

## Motivation

LevelDB (and derivatives) rely on a trivial but deterministic PRNG for tests and some internal randomized decisions. When re-implementing LevelDB in Rust—such as in the `bitcoin-rs` project—it is often desirable to:

- Preserve bit-for-bit compatibility with the original C++ implementation.
- Keep the PRNG extremely small, cheap, and reproducible across platforms.
- Avoid pulling in a heavyweight RNG stack for internal, non-cryptographic randomness.

`bitcoinleveldb-rand` provides that: a single-struct PRNG with LevelDB semantics.

This crate is **not** intended for cryptography or security-sensitive randomness. It is a simple linear congruential generator designed for reproducible test data and randomized algorithm steps where statistical quality is secondary to determinism and compatibility.

## Core Idea: Park–Miller "minimal standard" LCG

The generator is a classical **Park–Miller LCG**:

- Modulus: \( M = 2^{31} - 1 = 2147483647 \)
- State: integer `seed` in the range \([1, M-1]\)
- Recurrence: \( \text{seed}_{k+1} = A \cdot \text{seed}_k \bmod M \)

where `A` is a fixed multiplier (matching LevelDB's choice). This recurrence produces a full-period sequence over \([1, M-1]\) if `seed` is not 0 or `M` and `A` is chosen correctly.

The implementation uses the classic Park–Miller trick to avoid overflow and to compute the modulus efficiently, mirroring LevelDB's `util/random.h` exactly.

## Crate Features

- **Deterministic**: given the same initial seed, you get the same sequence on all platforms.
- **LevelDB-compatible**: the sequence is designed to match LevelDB's historical RNG.
- **Minimal surface area**: a single struct, a handful of methods.
- **Non-allocating & `no_std`-friendly** (aside from logging macros, which can be compiled out or adapted depending on your feature set).
- **Logging hooks**: uses `debug!`, `trace!`, and `error!`/`warn!` calls around edge cases for easier debugging in larger systems.

Again, this PRNG is **not cryptographically secure** and must not be used for secrets, key generation, or adversarial settings.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
bitcoinleveldb-rand = "0.1.19"
```

### Basic example

```rust
use bitcoinleveldb_rand::Random;

fn main() {
    // Seed with an arbitrary 32-bit value. The implementation normalizes it
    // into the internal 31-bit domain, avoiding degenerate seeds.
    let mut rng = Random::new(0x1234_5678);

    // Raw 31-bit-ish pseudo-random values
    let a = rng.next();
    let b = rng.next();

    println!("a = {a}, b = {b}");
}
```

### Uniform integers in [0, n)

```rust
use bitcoinleveldb_rand::Random;

fn main() {
    let mut rng = Random::new(1);

    // Uniform in [0, 99]
    let sample = rng.uniform(100);
    assert!(sample < 100);
}
```

**Important contract:**

- `uniform(n)` requires `n > 0`. If `n <= 0`, it logs an error and returns `0`.

### Bernoulli trial: `one_in(n)`

```rust
use bitcoinleveldb_rand::Random;

fn main() {
    let mut rng = Random::new(42);

    // ~1/10 chance of true each call
    if rng.one_in(10) {
        println!("Hit");
    } else {
        println!("Miss");
    }
}
```

Semantics:

- `one_in(n)` logically returns `true` with probability approximately `1/n` and `false` otherwise.
- It requires `n > 0`. If `n <= 0`, it logs an error and returns `false`.

Internally it computes `next() % n == 0`.

### Exponentially biased distribution: `skewed(max_log)`

```rust
use bitcoinleveldb_rand::Random;

fn main() {
    let mut rng = Random::new(7);

    // Exponentially biased towards small integers, but bounded above by 2^max_log - 1.
    let v = rng.skewed(10);
    assert!(v < (1u32 << 10));
}
```

Semantics in detail:

1. Choose `base` uniformly from `[0, effective_max_log]` where `effective_max_log = min(max_log, 30)`.
2. Compute `range = 1 << base`.
3. Draw uniformly in `[0, range-1]` by `uniform(range as i32)`.

Thus the distribution over \(v\) is piecewise-uniform with an **exponential bias** toward smaller values. Most draws are small; large draws become exponentially rare. This mirrors LevelDB's method for choosing skewed random sizes, e.g., for randomized tests.

Contracts and edge handling:

- Requires `max_log >= 0`. If `max_log < 0`, it logs an error and returns `0`.
- For `max_log >= 31`, it logs a warning and caps the effective exponent at 30 to avoid overflow and keep `1u32 << base` well-defined on all platforms.

### Seeding rules and degeneracy avoidance

You construct a `Random` as:

```rust
pub fn new(s: u32) -> Random
```

- The raw seed `s` is first masked with the internal modulus `M = 0x7fffffff` (31 bits).
- If the normalized `seed` is `0` or `M`, the constructor overwrites it with `1` and logs a warning. These two pathological seeds would otherwise collapse the sequence to a fixed point (all subsequent values would be `0` or `M`).

This treatment preserves the full-period property of the Park–Miller LCG.

### Determinism and reproducibility

Because this is a pure integer-state LCG with a fixed modulus and multiplier, the sequence is entirely deterministic once the seed is set. For any given seed:

- `Random::new(seed).next()` will always yield the same first value.
- The same applies to `uniform`, `one_in`, and `skewed`: their behavior depends only on the internal state transitions from `next()`.

This is especially useful when you want to:

- Reproduce failures in randomized tests.
- Keep protocol or data structure behavior reproducible across platforms.
- Stay compatible with LevelDB-based test vectors.

## API Summary

```rust
pub struct Random {
    seed: u32,
}

impl Random {
    /// Create a new RNG seeded from `s`.
    ///
    /// The seed is normalized to 31 bits and coerced away from the
    /// degenerate 0 and M states.
    pub fn new(s: u32) -> Self;

    /// Advance the internal state and return the next pseudo-random `u32`.
    ///
    /// The returned value is effectively a 31-bit number in (0, M],
    /// matching LevelDB's intent.
    pub fn next(&mut self) -> u32;

    /// Return an integer uniformly distributed in `[0, n-1]`.
    ///
    /// Requires `n > 0`. On invalid input, logs and returns `0`.
    pub fn uniform(&mut self, n: i32) -> u32;

    /// Return `true` with probability approximately `1/n`.
    ///
    /// Requires `n > 0`. On invalid input, logs and returns `false`.
    pub fn one_in(&mut self, n: i32) -> bool;

    /// Return an exponentially biased random value in
    /// `[0, 2^max_log - 1]`, heavily skewed toward smaller values.
    ///
    /// Requires `max_log >= 0`. On invalid input, logs and returns `0`.
    /// For `max_log >= 31`, caps the effective exponent at 30.
    pub fn skewed(&mut self, max_log: i32) -> u32;
}
```

## Integration Notes

- **Logging**: The implementation uses `log`-style macros (`debug!`, `trace!`, `warn!`, `error!`). To see diagnostic output, configure a logger in your binary or test harness (e.g., `env_logger`, `tracing`, or a custom adapter).
- **Std vs no_std**: The core logic itself is integer arithmetic and is amenable to `no_std` environments if logging is either disabled or redirected. Check the crate features and your target environment; if you are compiling for a constrained platform, you may want to compile with logging turned off.
- **Not for cryptography**: This generator is trivial to predict and invert. Do not use it for authentication tokens, keys, or anything requiring unpredictability.

## Relationship to `bitcoin-rs`

This crate lives in the [`bitcoin-rs` repository](https://github.com/klebs6/bitcoin-rs) and primarily serves the LevelDB re-implementation embedded there. Extracting this RNG into its own crate allows:

- Independent versioning and reuse.
- Focused documentation and tests around the RNG behavior.
- Reuse by other LevelDB-style projects that need the same deterministic generator.

## License

This crate is distributed under the MIT license.

See the repository for full license text and contribution guidelines.
