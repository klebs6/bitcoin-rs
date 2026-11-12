# bitcoinleveldb-rand

Deterministic pseudo-random generator used by LevelDB tests and randomized
data structures. `Random` implements the LevelDB-style Park–Miller sequence
with a 31-bit state and multiplier 16807.

`Random::new(seed)` normalizes the seed; `next` advances it. `uniform(n)` samples
by reducing the next value modulo a positive bound, `one_in(n)` tests the same
remainder for zero, and `skewed(max_log)` biases values toward smaller ranges.
Use a fixed seed for repeatable tests. Modulo reduction is not an exact unbiased
sampler for every bound, and the generator is not cryptographically secure.

See [the implementation and input handling](src/random.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
