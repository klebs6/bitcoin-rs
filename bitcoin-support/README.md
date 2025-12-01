# bitcoin-support

Low-level support primitives for a Bitcoin node implementation, focusing on:

- **Secure memory allocation** for secrets and serialization buffers
- **Libevent-compatible opaque handles** with lifetime tracking and zeroization
- **Utility functions** for alignment, path uniqueness, and memory cleansing

This crate is a small, security-oriented substrate used by `bitcoin-rs`. It exposes allocators, type aliases, and helper functions rather than a full protocol implementation.

---

## Design goals

- **Security first**: Minimize secret retention in memory by zeroizing on free, and by using `mlock(2)` where available to keep pages resident in RAM.
- **Deterministic lifetimes**: Opaque handle types track live instances via `AtomicUsize` counters and overwrite their own representation on drop.
- **Interop**: Preserve C ABI signatures where interoperation with `libevent` or C shims is required.
- **Portability**: Avoid binding directly to platform-specific networking types; use simple Rust aliases (`EvutilSocket = i32`) and dummy event constructors for higher-level testing and integration.

---

## Features at a glance

### Secure memory allocators

Two primary allocator types are provided:

- `SecureAllocator`
  - Intended for **long‑lived secrets** (keys, passwords, private material).
  - Uses `std::alloc::Global` underneath, but on Unix:
    - Calls `mlock(2)` to prevent the mapped pages from being swapped out.
    - Calls `munlock(2)` on deallocation.
  - Calls `memory_cleanse` before returning memory to the global allocator.

- `ZeroAfterFreeAllocator`
  - Lightweight allocator for general byte buffers.
  - Uses the global allocator for allocation, but **securely wipes** contents (via `memory_cleanse`) before free.
  - Intended for structs like `SerializeData` that may transiently contain sensitive or protocol‑critical bytes.

These allocators implement `std::alloc::Allocator` and can therefore be used with any allocator‑aware standard container.

#### Provided type aliases

```rust
/// String stored on a `SecureAllocator` heap.
pub type SecureString = Box<String, SecureAllocator>;

/// Canonical serialization buffer that zeroizes on drop.
pub type SerializeData = Vec<u8, ZeroAfterFreeAllocator>;
```

- `SecureString` is functionally analogous to `std::string` with a custom allocator in C++ Bitcoin Core.
- `SerializeData` is the canonical buffer type for the bit‑stream / serialization layer. When dropped, its underlying allocation is wiped with an optimizer‑resistant zeroing routine.

### Optimizer‑resistant memory cleansing

```rust
#[instrument(level = "trace", skip(ptr))]
pub fn memory_cleanse(ptr: *mut c_void, len: usize)
```

- Performs a byte‑wise `write_volatile` loop followed by a `compiler_fence` with `SeqCst` ordering.
- Goal: prevent the compiler from eliding apparent dead stores or reordering them such that the buffer is not fully cleared.
- This is relevant when handling material such as private keys, HD seeds, or authentication tokens.

From a memory‑model perspective, the combination of `volatile` writes and a fence provides a strong guarantee to the optimizer that the stores are observable side effects and cannot be removed by dead‑store elimination.

### Libevent‑style opaque handles

The crate defines a macro, `declare_event_type!`, which generates opaque, RAII‑style event types with:

- A unique `id: usize` per instance.
- Global live‑instance counters (`AtomicUsize`).
- Logging on creation and drop using the `tracing` crate.
- `Drop` implementations that:
  - Log the drop event.
  - Call `memory_cleanse` on the struct's memory representation.
  - Decrement the live counter.

Generated types include:

```rust
pub struct EventBase { /* opaque */ }
pub struct LibEvent { /* opaque */ }
pub struct EvHttp { /* opaque */ }
pub struct EvHttpRequest { /* opaque */ }
pub struct EvHttpConnection { /* opaque */ }
```

Associated helper functions create boxed instances with tracing instrumentation:

```rust
pub fn obtain_event_base() -> Box<EventBase>;

pub fn obtain_event(
    base:   *mut EventBase,
    s:      EvutilSocket,
    events: i16,
    cb:     Option<EventCallback>,
    arg:    *mut c_void,
) -> Box<LibEvent>;

pub fn obtain_evhttp(base: *mut EventBase) -> Box<EvHttp>;

pub fn obtain_evhttp_request(
    cb:  Option<EvHttpRequestCallback>,
    arg: *mut c_void,
) -> Box<EvHttpRequest>;

pub fn obtain_evhttp_connection_base(
    base: *mut EventBase,
    host: &str,
    port: u16,
) -> Box<EvHttpConnection>;
```

At present these constructors largely behave as pure Rust stand‑ins (they don't yet drive a real event loop), but they preserve the shape and semantics necessary to plug into or emulate `libevent`‑based networking stacks.

### Libevent compatibility aliases

```rust
/// Platform‑independent alias for a socket descriptor in libevent.
pub type EvutilSocket = i32;

/// Event callback with C ABI parity.
pub type EventCallback = unsafe extern "C" fn(*mut LibEvent, *mut c_void);

pub type EvHttpRequestCallback = unsafe extern "C" fn(*mut EvHttpRequest, *mut c_void);
```

These retain exact ABI compatibility with C‑side callbacks, which is crucial when linking into an existing `libevent`‑driven network reactor or for building FFI shims.

### Utility functions

#### Alignment

```rust
#[inline]
pub fn align_up(x: usize, align: usize) -> usize
```

- Asserts that `align` is a power of two.
- Returns the smallest multiple of `align` greater than or equal to `x`:
  \[
  \text{align\_up}(x, a) = (x + a - 1) \& \neg(a - 1)
  \]
- Useful for arena allocators, custom memory pools, and any low‑level component that must align blocks to cache line or page boundaries.

#### Unique paths

```rust
pub fn get_unique_path(base: &Path) -> PathBuf
```

- Produces a path of the form `base/<8-hex-chars>`.
- Derives the hex component by mixing:
  - `SystemTime::now().duration_since(UNIX_EPOCH).as_nanos()`
  - The function's own address `get_unique_path as usize` (cast to `u128`)

This is not intended as cryptographically strong randomness, but it provides extremely low collision probability for ephemeral filesystem paths (e.g., temporary directories, per‑test scratch areas) without invoking a full PRNG.

---

## Example usage

### Secure strings and serialization buffers

```rust
use bitcoin_support::{SecureString, SerializeData};

fn handle_secret_input(passphrase: &str) {
    // Store secret in RAM backed by SecureAllocator (mlocked on Unix).
    let mut s: SecureString = SecureString::default();
    s.push_str(passphrase);

    // Use SerializeData for transient protocol buffers.
    let mut buf: SerializeData = SerializeData::new_in(
        bitcoin_support::ZeroAfterFreeAllocator::default(),
    );
    buf.extend_from_slice(b"some serialized data");

    // When `s` and `buf` drop, their memory gets wiped and (on Unix) unlocked.
}
```

### Event handle lifecycle

```rust
use bitcoin_support::{
    obtain_event_base,
    obtain_event,
    EvutilSocket,
    EventCallback,
};
use std::os::raw::c_void;

unsafe extern "C" fn on_event(_ev: *mut bitcoin_support::LibEvent, _arg: *mut c_void) {
    // handle I/O readiness here
}

fn setup_dummy_event(sock: EvutilSocket) {
    let mut base = obtain_event_base();

    let _event = obtain_event(
        &mut *base,
        sock,
        /* events: e.g., EV_READ | EV_PERSIST */ 0,
        Some(on_event as EventCallback),
        std::ptr::null_mut(),
    );

    // When `_event` and `base` go out of scope, they are logged and scrubbed.
}
```

### Unique temporary paths

```rust
use bitcoin_support::get_unique_path;
use std::path::Path;

fn create_temp_dir() {
    let base = Path::new("/tmp/bitcoin-rs-tests");
    let unique = get_unique_path(base);
    std::fs::create_dir_all(&unique).expect("failed to create temp dir");

    // ... perform I/O in `unique` ...
}
```

---

## Tracing and diagnostics

The crate integrates with the `tracing` ecosystem:

- Constructors and destructors for event types are instrumented with `#[instrument]` and log via targets such as `"events"` and `"secure_alloc"`.
- `memory_cleanse` and allocator methods emit trace‑level diagnostics.

To leverage this, install a subscriber, for example:

```rust
use tracing_subscriber::FmtSubscriber;

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter("bitcoin_support=trace")
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}
```

This provides deep visibility into allocation, deallocation, and event handle lifetimes, which is valuable when hunting down resource leaks or verifying that secret data is not being retained unexpectedly.

---

## Safety and threat model

- **Zeroization**: `memory_cleanse` uses volatile stores and fences to mitigate compiler optimizations. It does *not* attempt to defeat all forms of microarchitectural data remanence.
- **Locking**: `SecureAllocator` uses `mlock/munlock` on Unix, but failure to lock does not abort allocations; instead it logs a warning, preserving API infallibility while signaling a potential degradation of protection.
- **Callbacks**: `EventCallback` and `EvHttpRequestCallback` are `unsafe extern "C"` and must obey C ABI rules. Callers are responsible for ensuring pointer validity and correct lifetime discipline.

This crate should be considered a **building block**: it reduces certain classes of accidental data retention and mis‑lifetime, but does not by itself guarantee end‑to‑end privacy or resistance against powerful physical adversaries.

---

## License

This crate is distributed under the **MIT** license, as part of the [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs) project.
