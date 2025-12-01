# bitcoin-sock

Low-level, cross-platform socket primitives extracted from the Bitcoin Core networking layer, written in Rust.

`bitcoin-sock` provides a thin, well-instrumented RAII wrapper over OS sockets (`CSocket`) together with carefully-behaved higher-level operations such as:

- total-send with transient error handling and timeouts
- incremental receive until a terminator byte with zero over-read
- connectivity probing without consuming data (via `MSG_PEEK`)
- portable timeout/wait logic via `select(2)` or `poll(2)`
- error classification (`io_error_is_permanent`) and OS-neutral error strings

The crate is designed for consumers that need explicit control over system calls and error semantics (e.g. Bitcoin node implementations, protocol daemons, or custom P2P stacks) while still benefiting from RAII and structured logging.

---

## Design overview

### CSocket and platform abstraction

The core primitive is the platform-dependent alias `CSocket`:

```rust
#[cfg(target_os = "windows")]
pub type CSocket = usize;      // SOCKET

#[cfg(not(target_os = "windows"))]
pub type CSocket = libc::c_int; // POSIX fd
``

All operations in this crate are expressed in terms of `CSocket`, with conditional compilation to choose between Winsock (`send`, `recv`, `connect`, `closesocket`, error codes) and POSIX (`send`, `recv`, `connect`, `close`, `errno`).

This makes the crate appropriate for cross-platform network stacks where you want the exact C API semantics, but in Rust.

### Sock: RAII wrapper around CSocket

`Sock` is the central type:

```rust
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Sock {
    socket: CSocket,
}
```

Key properties:

- **RAII lifetime management**: `Drop` closes the socket (if not empty), mirroring `std::unique_ptr<SOCKET>` semantics from C++.
- **Ownership stealing**: `assign_from` and `From<CSocket>` implement move-style transfer of ownership between `Sock` instances.
- **Empty state**: The sentinel `INVALID_SOCKET` denotes an inert `Sock` that does nothing on drop.

### Low-level system call wrappers

`Sock` exposes exact-thin wrappers around the core socket syscalls:

- `send(&self, data: *const c_void, len: usize, flags: i32) -> isize`
- `recv(&self, buf: *mut c_void, len: usize, flags: i32) -> isize`
- `connect(&self, addr: *const SocketAddr, addr_len: libc::socklen_t) -> i32`
- `get_sock_opt(&self, level, opt_name, opt_val, opt_len) -> i32`

These mirror the C interfaces verbatim, including flags like `MSG_NOSIGNAL` and `MSG_PEEK` where applicable. The functions are deliberately unsafe in spirit: they accept raw pointers and sizes and return raw OS return codes, converted to Rust integer types.

### Higher-level behaviour and algorithms

On top of the primitive calls, `Sock` adds value in several ways:

#### 1. Error classification: `io_error_is_permanent`

Network errors are not all equal. Temporary conditions such as `EAGAIN`, `EINTR`, or `EWOULDBLOCK` should be retried, while others should be treated as fatal.

```rust
#[cfg(unix)]
#[inline]
pub fn io_error_is_permanent(err: i32) -> bool {
    use libc::{EAGAIN, EINTR, EWOULDBLOCK, EINPROGRESS};
    !(err == EAGAIN || err == EINTR || err == EWOULDBLOCK || err == EINPROGRESS)
}

#[cfg(windows)]
#[inline]
pub fn io_error_is_permanent(err: i32) -> bool {
    use winapi::um::winsock2::{WSAEAGAIN, WSAEINTR, WSAEWOULDBLOCK, WSAEINPROGRESS};
    !(err == WSAEAGAIN || err == WSAEINTR || err == WSAEWOULDBLOCK || err == WSAEINPROGRESS)
}
```

This function is used centrally in send/receive loops to distinguish between retryable and terminal errors.

#### 2. Wait and readiness: `wait` and `is_selectable_socket`

`Sock::wait` blocks until the socket becomes ready for the requested `SockEvent`s (read and/or write), bounded by a timeout:

- On Linux with `feature = "use_poll"`: uses `poll(2)`.
- Otherwise: falls back to `select(2)` with an `FD_SETSIZE` check via `is_selectable_socket` to avoid UB on large descriptors.

This design is intended to faithfully reproduce the behaviour of the original Bitcoin networking code while staying portable.

#### 3. Total send with timeout and interruption: `send_complete`

```rust
impl Sock {
    pub fn send_complete(
        &self,
        data: &String,
        timeout: chrono::Duration,
        interrupt: &mut ThreadInterrupt,
    ) {
        // ...
    }
}
```

Algorithmically:

1. Convert the user-specified logical timeout (`chrono::Duration`) into a hard deadline (`Instant`).
2. Loop until all bytes are written.
3. For each iteration:
   - Invoke `send` on the remaining byte slice.
   - On success, advance `sent` and continue.
   - On failure (`ret <= 0`):
     - Fetch `last_socket_error` and decide permanent vs transient via `io_error_is_permanent`.
     - If permanent, panic with an error message from `network_error_string`.
     - Check `deadline` and `interrupt`; if either fires, panic with an informative message about partial progress.
     - Compute a bounded wait interval with `compute_bounded_wait(deadline)` and call `wait` for `SOCK_SEND` before retry.

The logic is carefully structured to avoid busy-waiting while still honouring both timeout and interrupt conditions.

Mathematically, you can model the loop as a bounded retry algorithm on a non-deterministic channel with the following invariants:

- **Progress**: `sent` is monotonically non-decreasing and bounded by `data.len()`.
- **Safety**: All error paths either classify the error as transient (retry) or terminate the function with a precise error message.
- **Liveness**: Assuming the OS eventually makes the socket writable before `deadline` and `interrupt` stays false, the loop must terminate with `sent == len`.

#### 4. Efficient receive-until-terminator: `recv_until_terminator`

```rust
impl Sock {
    pub fn recv_until_terminator(
        &self,
        terminator: u8,
        timeout: chrono::Duration,
        interrupt: &mut ThreadInterrupt,
        max_data: usize,
    ) -> String {
        // ...
    }
}
```

Core idea: we want to read until a sentinel byte without consuming any data beyond that point from the socket. Reading byte-by-byte would preserve the invariant but be asymptotically inefficient (roughly `O(n)` syscalls for `n` bytes).

Instead, the implementation uses a repeated pattern of:

1. `MSG_PEEK`-style `recv` into a small buffer (e.g. 512 bytes).
2. Search for the terminator.
3. If found at index `pos`, issue a non-peeking read of exactly `pos + 1` bytes and append them to `data`.
4. If not found, read the full peeked slice.
5. Maintain `data.len() <= max_data`, panicking if exceeded.
6. Honour timeout and `interrupt`, with the same `compute_bounded_wait` pattern as in `send_complete`.

This yields a roughly 50× improvement over byte-wise reading for realistic workloads (as noted by the inline comments), while preserving a strong invariant:

> After `recv_until_terminator` returns, the underlying socket has consumed exactly the bytes up to and including the first occurrence of `terminator`, and no further.

The returned `String` strips the terminator before returning, and asserts UTF‑8 validity.

#### 5. Connectivity probe without consumption: `is_connected`

```rust
impl Sock {
    pub fn is_connected(&self, errmsg: &mut String) -> bool {
        // ...
    }
}
```

`is_connected` implements the canonical Bitcoin approach:

1. If the socket is `INVALID_SOCKET`, set `errmsg` to `"not connected"` and return `false`.
2. Issue a `recv` of one byte with `MSG_PEEK`.
3. Interpret results as:
   - `ret == -1` and error is transient → treat as still connected.
   - `ret == -1` and error is permanent → set `errmsg` to the OS error string and return `false`.
   - `ret == 0` → remote has closed the connection; set `errmsg` to `"closed"` and return `false`.
   - `ret > 0` → data available; socket considered connected.

This function allows higher-level code to cheaply check connectivity status without altering the read cursor.

### Error reporting helpers

The crate also provides utilities for system-level error introspection:

- `last_socket_error() -> i32`: returns `errno` on Unix, `WSAGetLastError()` on Windows.
- `network_error_string(err: i32) -> String`: returns a descriptive message including the error code, using `strerror_r` on Unix or `FormatMessageW` on Windows.

These are engineered to behave consistently across libc variants (GNU vs POSIX strerror_r) and to produce strings of the form:

```text
"<system message> (<code>)"
```

which mirrors the logic used in C++ Bitcoin Core.

### Socket selectability and descriptor constraints

On Unix platforms that use `select(2)`, there is a hard ceiling (`FD_SETSIZE`) on descriptors that can safely be used. `is_selectable_socket` encodes this constraint:

```rust
pub fn is_selectable_socket(s: &CSocket) -> bool {
    #[cfg(any(target_os = "windows", feature = "use_poll"))]
    { true }

    #[cfg(not(any(target_os = "windows", feature = "use_poll")))]
    { (*s as usize) < libc::FD_SETSIZE as usize }
}
``

Higher-level code can use this to decide whether a given socket can participate in a global `select`-based event loop or must instead be handled differently (e.g., via `poll` or epoll in an external layer).

### Socket pair utility (Unix)

For local IPC and test harnesses, the crate includes:

```rust
#[cfg(unix)]
pub fn make_socket_pair() -> (libc::c_int, libc::c_int) {
    // ... AF_UNIX, SOCK_STREAM pair
}
```

This constructs a bidirectional connected pair of Unix domain sockets, asserting on failure; useful for deterministic test constructions and local pipelines.

---

## Traits and abstraction points

The crate defines several traits that abstract over socket-like backends:

```rust
pub trait SockInterface {}

pub trait SockGet {
    fn get(&self) -> CSocket;
}

pub trait SockRelease {
    fn release(&mut self) -> CSocket;
}

pub trait Reset {
    fn reset(&mut self);
}

pub trait SockSend {
    fn send(&self, data: *const c_void, len: usize, flags: i32) -> isize;
}

pub trait SockRecv {
    fn recv(&self, buf: *mut c_void, len: usize, flags: i32) -> isize;
}

pub trait SockConnect {
    fn connect(&self, addr: *const SocketAddr, addr_len: libc::socklen_t) -> i32;
}

pub trait SockGetSockOpt {
    fn get_sock_opt(
        &self,
        level: i32,
        opt_name: i32,
        opt_val: *mut c_void,
        opt_len: *mut libc::socklen_t,
    ) -> i32;
}

pub trait SockWait {
    fn wait(&self, timeout: Instant, requested: SockEvent, occurred: *mut SockEvent) -> bool;
}

pub trait SockSendComplete {
    fn send_complete(
        &self,
        data: &String,
        timeout: Instant,
        interrupt: &mut ThreadInterrupt,
    );
}

pub trait SockRecvUntilTerminator {
    fn recv_until_terminator(
        &self,
        terminator: u8,
        timeout: Instant,
        interrupt: &mut ThreadInterrupt,
        max_data: usize,
    ) -> String;
}

pub trait SockIsConnected {
    fn is_connected(&self, errmsg: &mut String) -> bool;
}
```

The concrete `Sock` type effectively implements the semantics of these traits. In your own code, you can:

- Implement these traits for alternative backends (e.g., mock sockets for tests, different transport layers, wrappers around async runtimes), while still reusing shared logic.
- Depend on trait objects or generics bounded by these traits to write code that is backend-agnostic, e.g. a protocol parser that only needs `SockRecvUntilTerminator` and `SockSendComplete`.

This allows you to decouple *protocol logic* from *transport implementation* while retaining the precise semantics that the Bitcoin networking stack expects.

---

## Usage examples

### Basic RAII management

```rust
use bitcoin_sock::{CSocket, Sock};

fn adopt_raw_socket(raw: CSocket) {
    // Transfer ownership from a raw descriptor into RAII-managed Sock.
    let mut sock = Sock::from(raw);

    // Query the underlying handle without transferring ownership.
    let fd = sock.get();
    println!("underlying fd = {}", fd);

    // Release ownership back to the caller; Sock becomes inert.
    let raw_again = sock.release();
    assert_eq!(raw_again, fd);
}
```

### Complete send with timeout and interrupt

```rust
use bitcoin_sock::Sock;
use chrono::Duration;

fn send_message(sock: &Sock, payload: &str, interrupt: &mut ThreadInterrupt) {
    let timeout = Duration::seconds(30);
    sock.send_complete(&payload.to_string(), timeout, interrupt);
}
```

The function will either:

- send all bytes, or
- panic on a permanent error, timeout, or interrupt, describing how many bytes were sent and why the operation aborted.

You can wrap it and convert panics into error values if you need a non-panicking API at a higher layer.

### Receive a line (or other delimited frame)

```rust
use bitcoin_sock::Sock;
use chrono::Duration;

fn recv_line(sock: &Sock, interrupt: &mut ThreadInterrupt) -> String {
    // Read until '\n', up to 64 KiB, with a 60s timeout.
    let timeout = Duration::seconds(60);
    sock.recv_until_terminator(b'\n', timeout, interrupt, 64 * 1024)
}
```

The returned `String` will exclude the newline terminator and be valid UTF‑8.

### Connectivity probing

```rust
use bitcoin_sock::Sock;

fn ensure_connected(sock: &Sock) -> Result<(), String> {
    let mut errmsg = String::new();
    if sock.is_connected(&mut errmsg) {
        Ok(())
    } else {
        Err(errmsg)
    }
}
```

This is particularly useful for long-lived P2P connections where you want to periodically verify liveness without mutating the receive stream.

### Waiting for readiness

```rust
use bitcoin_sock::{Sock, SockEvent};
use time::Duration;

const SOCK_RECV: SockEvent = 0x01;
const SOCK_SEND: SockEvent = 0x02;

fn wait_for_read(sock: &Sock, max_wait_ms: i64) -> bool {
    let requested = SOCK_RECV;
    let mut occurred: SockEvent = 0;

    let timeout = Duration::milliseconds(max_wait_ms);
    if sock.wait(timeout, requested, &mut occurred as *mut SockEvent) {
        occurred & SOCK_RECV != 0
    } else {
        false
    }
}
```

This pattern allows you to build sophisticated event loops or integrate `bitcoin-sock` into existing synchronous multiplexing infrastructure.

---

## Integration notes

- **Logging / tracing**: The implementation is instrumented with `trace!`, `debug!`, and `warn!` macros. You should configure your logging/tracing backend (for example via `tracing` or `log`) to obtain detailed diagnostics about socket behaviour.
- **Panics vs errors**: Several higher-level operations (`send_complete`, `recv_until_terminator`) panic on unrecoverable conditions, as they are modeled after C++ exceptions. If your application requires error-returning APIs, wrap these functions and translate panic payloads into typed error values.
- **ThreadInterrupt**: The crate expects a `ThreadInterrupt` type exposing an `as_bool()` method to indicate whether a cooperative interrupt has been requested. Integrate this with your own cancellation infrastructure.
- **Select vs poll**: On Linux, enabling the `use_poll` feature switches the readiness mechanism from `select` to `poll`. This is useful when dealing with many file descriptors or when you need to avoid the `FD_SETSIZE` limit.

---

## Repository, license, and provenance

- Crate name: `bitcoin-sock`
- Version: `0.1.19`
- Repository: <https://github.com/klebs6/bitcoin-rs>
- License: MIT
- Edition: Rust 2021
- Author: `klebs <none>`

This crate appears to be a Rust reimplementation/port of socket handling from Bitcoin Core, with the goal of preserving semantics and behaviour while leveraging Rust's ownership model and RAII.

> Reminder: This README was generated automatically by an AI model. It may omit details or slightly misrepresent edge semantics. Consult the source code in the repository for definitive behaviour.
