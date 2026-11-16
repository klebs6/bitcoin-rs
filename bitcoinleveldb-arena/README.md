# bitcoinleveldb-arena

Arena allocation for the `bitcoin-rs` LevelDB port. `Arena` hands out raw byte
storage from retained blocks and releases its backing storage when dropped.

`Arena::default()` creates an empty arena; `allocate` and `allocate_aligned`
return `*mut u8`, and `memory_usage` reports the implementation's allocation
accounting. There is no individual-allocation free operation. Allocated pointers
borrow the arena's storage and must not be used after the arena is dropped.
Callers are responsible for initialized values, valid access, and appropriate
alignment; this is a low-level component, not a general-purpose safe allocator.

See [the allocator implementation](src/arena.rs) and [its tests](src/arena_test.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
