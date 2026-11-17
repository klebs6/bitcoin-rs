# bitcoinleveldb-cfg

Compile-time constants used by the LevelDB implementation.

| Constant | Value | Purpose |
| --- | ---: | --- |
| `NUM_LEVELS` | 7 | Number of table levels |
| `L0_COMPACTION_TRIGGER` | 4 | Level-0 file count that starts compaction |
| `L0_SLOWDOWN_WRITES_TRIGGER` | 8 | Level-0 write slowdown threshold |
| `L0_STOP_WRITES_TRIGGER` | 12 | Level-0 write stop threshold |
| `MAX_MEM_COMPACT_LEVEL` | 2 | Highest level targeted by a compacted memtable |
| `READ_BYTES_PERIOD` | 1048576 | Approximate iterator read-sampling interval, in bytes |

These are constants, not a configuration loader. This crate does not define
database roles, resolve filesystem paths, or construct `Options`.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
