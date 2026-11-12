# bitcoinleveldbt-autocompaction

Read-driven compaction regression tests and deterministic progress observations.

The `AutoCompactTest` fixture checks reading all or part of a populated range and observes compaction barriers and topology changes. These tests create temporary databases; their helpers are test infrastructure, not an application database API.

Sources: [bitcoinleveldbt_autocompaction](src/bitcoinleveldbt_autocompaction.rs), [read_progress_observation](src/read_progress_observation.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-autocompaction`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
