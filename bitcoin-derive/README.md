# bitcoin-derive

Procedural attributes used while translating Bitcoin Core and LevelDB source.
The exported names are defined in [src/lib.rs](src/lib.rs).

Thread-annotation attributes such as `GUARDED_BY`, `LOCKABLE` and
`EXCLUSIVE_LOCK_FUNCTION` preserve descriptive metadata in generated Rust
documentation and tracing. Their [implementation](src/annotate_thread_safety_attr.rs)
does not enforce lock ownership, ordering or Clang thread-safety analysis.

Other attributes, including `no_copy`, `no_move`, the `Q_*` helpers, `fuzz`
and `fuzz_test`, currently return their input item unchanged. They do not
implement a signal/slot runtime, prevent Rust moves/copies, or install a fuzz
runner. Apply Rust's actual type and synchronization rules independently.

Use the [manifest](Cargo.toml) and source for the available attribute names;
this README does not advertise a registry version or additional guarantees.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [License](LICENSE) · [Upstream notices](NOTICE)

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
