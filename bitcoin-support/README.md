# bitcoin-support

Low-level allocation, memory-cleansing, event stand-in and utility helpers
for `bitcoin-rs`; see [src/lib.rs](src/lib.rs) and [Cargo.toml](Cargo.toml).
The crate requires nightly allocator features.

`SecureAllocator` delegates allocation to Rust's global allocator and attempts
`mlock` on Unix. A failed lock is logged while the allocation is returned.
Deallocation cleanses the allocated block and attempts `munlock` before
release. `ZeroAfterFreeAllocator` cleanses its block before deallocation.
These behaviors do not promise that all copies of sensitive data are erased
or that an allocation is successfully locked against swapping.

`SecureString` is a `Box<String, SecureAllocator>`: the allocator applies to
the boxed `String` object, while the String's separate character buffer uses
its ordinary allocator. The alias does not itself lock or cleanse that
character buffer. `SerializeData` is a byte vector using
`ZeroAfterFreeAllocator` for its backing allocation.

`memory_cleanse` writes zeros with volatile stores and a compiler fence.
For a non-null pointer and nonzero length, callers must provide a live,
writable region of that length and uphold exclusive access during the call.

The event constructors in [src/events.rs](src/events.rs) create Rust stand-ins;
they do not install callbacks or drive a libevent loop. Their Rust structures
are not interchangeable with native libevent objects merely because a
callback type uses `extern "C"`.

Alignment and unique-path helpers have their own source contracts.
`get_unique_path` constructs a candidate pathname; callers must create it
exclusively and handle collisions. It neither reserves a path nor supplies
cryptographic randomness.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [License](LICENSE) · [Upstream notices](NOTICE)
