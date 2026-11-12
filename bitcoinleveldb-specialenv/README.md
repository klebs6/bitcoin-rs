# bitcoinleveldb-specialenv

Controllable environment wrapper for LevelDB tests. `SpecialEnv` wraps a
shared `Env` and creates file wrappers that inject selected I/O behavior.

Its controls can delay data sync, fail data sync, simulate unwritable/no-space
conditions, fail manifest writes or syncs, and count random-access reads.
`AtomicCounter` provides the test counter. The wrapper is used by database
regression tests to drive error-handling and scheduling paths; it is not a
replacement production filesystem or a general fault-injection service.

Construct it with `SpecialEnv::new(Rc<RefCell<dyn Env>>)` and coordinate control
changes with the tested operation. See [the environment](src/specialenv.rs),
[data-file faults](src/data_file.rs), [manifest faults](src/manifest_file.rs),
and [read counting](src/counting_file.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
