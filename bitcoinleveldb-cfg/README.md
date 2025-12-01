# bitcoinleveldb-cfg

Configuration primitives for running LevelDB-backed Bitcoin data stores in Rust.

This crate provides a small, opinionated configuration layer that standardizes how LevelDB databases are named, located, and parameterized within the broader `bitcoin-rs` workspace. It focuses exclusively on the *configuration surface* (paths, logical database roles, and tunable parameters), keeping I/O and schema logic in adjacent crates.

---

## Scope and goals

`bitcoinleveldb-cfg` aims to:

- Provide a **typed configuration model** for LevelDB instances used by Bitcoin-indexing components.
- Encode **stable naming conventions** for different logical databases (e.g., headers, UTXO, blocks, indexes).
- Support **environment-dependent layouts** (e.g., mainnet / testnet / regtest / custom data directories).
- Centralize **LevelDB tuning parameters** (cache sizes, write buffers, compression flags) behind a single interface so downstream crates can depend on a consistent configuration contract.

It does **not** implement a database driver. Instead it is designed to be consumed by a LevelDB wrapper (e.g., `bitcoinleveldb` or a raw `leveldb-sys` binding) to open and manage the actual database instances.

---

## Design overview

The crate typically centers around a small number of core types (exact names may differ slightly in the actual code):

- `Network` / `Chain` enum — identifies which Bitcoin network the configuration targets (e.g., `Mainnet`, `Testnet`, `Regtest`, possibly `Signet` or custom chains).
- `DbRole` / `LogicalDb` enum — enumerates logical database roles such as headers, blocks, UTXO set, and additional indexes. These roles determine the on-disk subdirectory or file names.
- `LevelDbTuning` / `LevelDbOptionsCfg` — collects tunable parameters such as cache size, block size, write buffer size, and bloom filter options.
- `BitcoinLevelDbCfg` / `DbCfg` — the primary configuration object that binds network, base data directory, logical database role, and tuning parameters into a ready-to-use configuration that a LevelDB wrapper can consume.

In a typical architecture, the configuration object is pure data: it does not open files or sockets; it simply describes *what* should be opened and *with which parameters*.

---

## Example usage

Below is a representative example for how a consumer crate might use `bitcoinleveldb-cfg`. Names are indicative, not guaranteed; consult the actual API in this crate for precise signatures.

```rust
use bitcoinleveldb_cfg::{
    BitcoinLevelDbCfg,    // primary configuration type
    Network,              // network/chain enumeration
    DbRole,               // logical database role
    LevelDbTuning,        // low-level tuning parameters
};

fn main() -> anyhow::Result<()> {
    // Decide which chain and data directory to use.
    let network = Network::Mainnet;
    let datadir = "/var/lib/bitcoin-rs";

    // Configure base tuning parameters; these will be translated into
    // concrete LevelDB options by your LevelDB wrapper crate.
    let tuning = LevelDbTuning {
        block_cache_bytes: 512 * 1024 * 1024, // 512 MiB
        write_buffer_bytes: 128 * 1024 * 1024,
        max_open_files: 2048,
        use_compression: true,
        ..LevelDbTuning::default()
    };

    // Describe a specific logical database: e.g., the block index.
    let cfg = BitcoinLevelDbCfg::new(datadir, network, DbRole::BlockIndex, tuning)?;

    // Turn this into actual LevelDB options in your driver crate.
    // (The below call is conceptual; adapt to your driver.)
    let options = cfg.to_leveldb_options();
    let path = cfg.path();

    // Example integration with a hypothetical `bitcoinleveldb` crate:
    // let db = bitcoinleveldb::open(path, &options)?;

    Ok(())
}
```

A crate that integrates with `bitcoinleveldb-cfg` usually:

1. Selects a `Network` and base data directory.
2. Chooses one or more `DbRole` values per logical database.
3. Constructs a `BitcoinLevelDbCfg` for each role.
4. Derives concrete LevelDB options and paths using helper methods.
5. Opens the LevelDB instances via an external database crate.

---

## Configuration patterns

### Network and layout

The crate typically enforces a consistent directory structure such as:

```text
<datadir>/
    mainnet/
        blocks/
        chainstate/
        index/
        ...
    testnet/
        ...
    regtest/
        ...
```

The exact layout is encoded in the config types, so all collaborating components agree on the set of LevelDB instances and their on-disk locations. This is particularly important for:

- Indexers and explorers built within the `bitcoin-rs` workspace.
- Node components that share data directories.
- Tools that must interact with the same LevelDBs without conflicting naming conventions.

### Tuning and performance

Bitcoin-scale data volumes make LevelDB tuning nontrivial. `bitcoinleveldb-cfg` centralizes the parameters that control:

- **Read amplification** — via block size, bloom filters, and cache shaping.
- **Write amplification** — via write buffer sizes and compaction parameters.
- **Memory footprint** — via block cache and table cache limits.

By encoding these parameters in a structured configuration type, you can:

- Dynamically adjust tuning based on deployment profile (e.g., laptop vs dedicated server).
- Maintain **deterministic configurations** for reproducible benchmarking.
- Share tuning defaults across multiple binaries without copy-paste duplication.

---

## Integration within `bitcoin-rs`

Within the `https://github.com/klebs6/bitcoin-rs` repository, this crate is expected to be consumed by:

- Other internal workspace crates that implement Bitcoin indexing, chainstate management, or analytics.
- Binary targets that expose CLI tools or services relying on LevelDB persistence.

For consumers outside the workspace, the crate can be used as a *configuration boundary* so you can:

- Align your LevelDB layout with `bitcoin-rs` tools.
- Safely interoperate on the same on-disk databases.

---

## Feature flags

If this crate defines Cargo features, they are typically used to:

- Toggle support for specific networks (e.g., include or exclude `Regtest` or experimental networks).
- Adjust default tuning profiles (e.g., `heavy-load`, `low-mem`).

Check `Cargo.toml` in the repository for the precise set of available features.

---

## Error handling

Configuration constructors usually validate:

- Path legality (e.g., non-empty, well-formed components).
- Value ranges for tuning parameters (non-negative sizes, minimums, and maximums).

These validations typically return structured errors (e.g., an `enum` implementing `std::error::Error`) suitable for integration with error stacks (e.g., `anyhow`, `eyre`) in consumer crates.

---

## When to use this crate

Use `bitcoinleveldb-cfg` if you:

- Operate multiple LevelDB instances representing different aspects of a Bitcoin dataset and want **consistent and reproducible configuration**.
- Build tools that should **coexist with or extend** the `bitcoin-rs` workspace.
- Want a **single source of truth** for LevelDB paths and tuning rather than ad-hoc hard-coded values distributed across binaries.

If you only need a raw LevelDB binding with no Bitcoin-specific semantics, a general LevelDB crate may be sufficient.

---

## License

This crate is distributed under the MIT license.

---

## Repository

Source code and issue tracking are hosted in the `bitcoin-rs` repository:

- Repository: <https://github.com/klebs6/bitcoin-rs>

Contributions, bug reports, and improvement suggestions are welcome through that repository.
