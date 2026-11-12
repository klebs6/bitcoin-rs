# bitcoinleveldb-options

Database, read, and write configuration for the `bitcoin-rs` LevelDB port.
`Options` selects the comparator, environment, cache, filter policy, file/block
sizes, compression, and open/recovery behavior. `ReadOptions` controls read-side
checksum/cache/snapshot choices; `WriteOptions` controls write synchronization.

`Options::default()` uses bytewise ordering, the thread's shared `PosixEnv`, a
4 MiB write buffer, 4 KiB blocks, and Snappy compression. It does not create a
missing database unless `create_if_missing` is enabled. Comparator and filter
policy owners use `Arc`; an explicit environment is an `Rc<RefCell<dyn Env>>`.
Changing the comparator's ordering or persistent name for an existing database
is invalid even if the Rust type is unchanged.

`snapshot_for_table` intentionally omits the environment owner while retaining
table-read settings. Raw logger/cache/snapshot pointers still carry caller or
database lifetime obligations; cloning an options value does not deep-copy or
transfer ownership of those pointed-to resources.

See [database defaults](src/options.rs), [read options](src/read_options.rs),
[write options](src/write_options.rs), and [option sanitization](src/sanitize.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
