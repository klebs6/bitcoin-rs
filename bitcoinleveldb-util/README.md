# bitcoinleveldb-util

Explicit destructor suppression for long-lived LevelDB support objects.

`NoDestructor<T>::new(value)` stores a fully initialized `T` in `MaybeUninit<T>`.
Dropping the wrapper does not run `T`'s destructor. This is useful for objects
whose resources are deliberately retained until process exit.

`get(&self)` returns a mutable raw pointer to the stored object. It does not pin
the wrapper, extend its storage lifetime, or enforce aliasing/synchronization
for dereferences. Keep the wrapper stationary and live while using such a
pointer, and do not use it to create conflicting mutable access.

Destructor suppression also suppresses ordinary resource cleanup; it should be
an intentional lifetime choice, not a replacement for normal ownership.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
