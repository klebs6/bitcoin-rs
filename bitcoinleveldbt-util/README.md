# bitcoinleveldbt-util

Shared test helpers for the LevelDB regression crates: deterministic generators,
seeds, temporary database paths, comparator probes, environment adapters, delays,
and a small test harness.

`random_key_bytes` returns the requested number of literal bytes from the
boundary-key alphabet, including NUL and `fd/fe/ff`. `dbtest_random_key_bytes`
selects its length using the original conditional PRNG draw order. The existing
`random_key` and `dbtest_random_key` remain legacy text generators: each selected
byte becomes a Unicode scalar, so high bytes occupy two UTF-8 bytes. Callers must
opt into the byte API; existing randomized text tapes are unchanged.

`random_string` generates printable ASCII. `compressible_string` repeats an
ASCII pattern and returns its requested-length prefix. Supply practical finite
sizes and fractions within the test runner's resource limits. Both return an
empty Slice without changing output or advancing the PRNG for a null PRNG or
destination. Key generators return empty for a null PRNG; fixed-length generators
also return empty without draws for nonpositive lengths.

These helpers are test support, not cryptographic randomness or an application
runtime. Source: [modules](src/lib.rs) · [generator contracts](tests/generator_byte_contracts.rs).
For bounded repository verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
