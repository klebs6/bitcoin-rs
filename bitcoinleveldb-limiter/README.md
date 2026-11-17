# bitcoinleveldb-limiter

A non-blocking resource counter used by LevelDB's POSIX file implementations.

`Limiter::new(max_acquires)` sets the capacity. `acquire` returns `true` when a
slot was reserved and `false` when none was available. Call `release` exactly
once for each successful acquisition; a failed acquisition needs no release.

The counter uses relaxed atomics because it tracks capacity independently of
the resource's own state. It does not allocate, close, or synchronize access to
the resource, and it does not return an RAII reservation guard. The limiter
must outlive consumers that retain a pointer to it.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
