# bitcoin-tokenpipe

Low-level, single-byte token pipes for coordinating concurrent work in `bitcoin-rs` and related systems.

This crate exposes a very small, explicit API around OS pipes / file descriptors, specialized for transmitting *tokens* (one-byte messages) between threads or processes. It is designed to be:

- **Minimal**: no heap allocation, no framing, no buffering beyond the kernel pipe buffer.
- **Deterministic**: explicit ownership transfer of file descriptors, predictable close semantics.
- **Interop-friendly**: uses raw `libc` FDs under the hood, so it integrates directly with other C/Rust I/O primitives and event loops.

---

## Motivation

`bitcoin-tokenpipe` provides a primitive synchronization and wakeup mechanism built on top of OS pipes:

- Send a single-byte *token* to signal an event (e.g., new work, shutdown, timer expiration).
- Read a single-byte *token* from another thread or process to detect and react to that event.
- Integrate the read-end FD into `poll`, `epoll`, `kqueue`, or `select` without additional layers.

This pattern is frequently used in high-performance networking and systems code to:

- Wake an event loop from another thread.
- Implement cancellation channels, shutdown notifiers, or work-queue nudges.
- Bridge between non-Rust components using plain file descriptors.

The crate deliberately operates on **raw `i32` file descriptors** (or `_pipe` handles on Windows), avoiding higher-level abstractions when direct control is required.

---

## Platform behavior

### Non-Windows (POSIX)

On Unix-like systems:

- `TokenPipe::make()` uses `libc::pipe` to create a blocking, unidirectional pipe (`fds[0]` = read end, `fds[1]` = write end).
- `TokenPipeEnd::token_write` and `TokenPipeEnd::token_read` call `sys_write`/`sys_read` wrappers around `write(2)` and `read(2)`, retrying on `EINTR`.
- Errors are reported as integral status codes via `TokenPipeEndStatus`.

### Windows

On Windows (MSVCRT environment):

- `TokenPipe::make()` uses `libc::_pipe` with a fixed buffer size and `_O_BINARY`.
- Only the pipe creation and close semantics are exposed at this crate level. The token read/write API is currently defined only for non-Windows via `#[cfg(not(windows))]`.

---

## Core types

### `TokenPipe`

```rust
pub struct TokenPipe {
    fds: [i32; 2],
}
```

- Represents both ends of a pipe as raw file descriptors.
- Manages lifetime: `Drop` automatically closes any still-open endpoints.
- Designed for move semantics and explicit transfer of ownership between components.

Key methods:

- `TokenPipe::make() -> Option<Self>`
  - Creates a new OS pipe.
  - Returns `Some(pipe)` on success, `None` on failure (errors logged via `tracing`).

- `TokenPipe::new(fds: [i32; 2]) -> Self`
  - Wraps an existing pair of FDs.
  - Caller must ensure `fds` are valid pipe ends in the expected order (read, write).

- `take_read_end(&mut self) -> TokenPipeEnd`
  - Moves out the read end (index 0), replacing it with `-1` internally.
  - Can be called at most once for a valid read end.

- `take_write_end(&mut self) -> TokenPipeEnd`
  - Moves out the write end (index 1), replacing it with `-1` internally.

- `close(&mut self)`
  - Explicitly closes any still-open FDs.
  - Idempotent: repeated `close` calls are safe.

- `new_from_other(other: Self) -> Self`
  - Move-constructs a new `TokenPipe` by taking ownership of all FDs from `other` and invalidating `other`.

- `assign_from(&mut self, other: Self) -> &mut Self`
  - Closes current FDs, then takes ownership from `other`.

`Drop` automatically calls `close`, so you normally do not need to call `close` manually unless you want to eagerly release resources.

### `TokenPipeEnd`

```rust
pub struct TokenPipeEnd {
    fd: i32,
}
```

- Models one endpoint of the pipe: either the read or write side.
- Owns exactly one FD and closes it on `Drop`.

On non-Windows platforms (`#[cfg(not(windows))]`), the following methods are provided:

- `TokenPipeEnd::new(fd: Option<i32>) -> Self`
  - Constructs an endpoint from an optional FD.
  - `None` or `Some(-1)` creates a closed endpoint.

- `token_write(&mut self, token: u8) -> i32`
  - Writes a single byte to the pipe.
  - Return values:
    - `0` on success.
    - `< 0` on failure (see `TokenPipeEndStatus`).

- `token_read(&mut self) -> i32`
  - Reads a single byte from the pipe.
  - Return values:
    - `>= 0`: the token value (0–255) cast to `i32`.
    - `< 0`: status code (see `TokenPipeEndStatus`).

- `close(&mut self)`
  - Explicitly closes the FD if open, sets it to `-1`.

- `is_open(&mut self) -> bool`
  - Returns `true` if `fd != -1`.

- `new_from_other(other: TokenPipeEnd) -> Self`
  - Move-constructs from `other`, invalidating `other`.

- `assign_from(&mut self, other: TokenPipeEnd) -> &mut Self`
  - Closes the current FD, then takes ownership from `other`.

`Drop` semantics mirror `TokenPipe`: dropping a `TokenPipeEnd` closes the underlying FD if still open.

### `TokenPipeEndStatus` (non-Windows)

```rust
#[repr(i32)]
pub enum TokenPipeEndStatus {
    TS_ERR,
    TS_EOS,
}
```

C-style integer codes used by `token_read` and `token_write`:

- `TS_ERR` (typically `0` before casting, but use the variant, not the integer): indicates an I/O error, closed FD usage, or non-recoverable error from `read(2)`/`write(2)`.
- `TS_EOS`: end-of-stream. The peer closed the pipe (EOF).

Both `token_read` and `token_write` return plain `i32`, not a Rust `Result`. Callers must interpret negative values as `TokenPipeEndStatus` codes.

---

## Usage examples

### Basic token signalling between threads (Unix-like systems)

```rust
use bitcoin_tokenpipe::{TokenPipe, TokenPipeEndStatus};
use std::thread;

fn main() {
    // Create the pipe
    let mut pipe = TokenPipe::make().expect("pipe creation failed");

    // Split into read / write ends
    let mut reader = pipe.take_read_end();
    let mut writer = pipe.take_write_end();

    let handle = thread::spawn(move || {
        loop {
            let rc = reader.token_read();
            if rc < 0 {
                match rc {
                    x if x == TokenPipeEndStatus::TS_EOS as i32 => {
                        // peer closed: normal shutdown
                        break;
                    }
                    _ => {
                        // error: log / abort as appropriate
                        break;
                    }
                }
            } else {
                let token = rc as u8;
                // handle token
                println!("received token: {}", token);
            }
        }
    });

    // Send a few tokens
    for t in 0u8..3 {
        let rc = writer.token_write(t);
        assert_eq!(rc, 0);
    }

    // Close writer to signal EOS
    writer.close();

    handle.join().unwrap();
}
```

### Integration with a poll-based event loop (Unix-like systems)

The read-end FD can be registered with `epoll`, `kqueue`, or `poll`. When a token is written from another thread, the event loop wakes and consumes the token.

```rust
use bitcoin_tokenpipe::TokenPipe;
use std::os::fd::AsRawFd; // or `std::os::unix::io::RawFd` depending on Rust version

fn register_with_epoll(epoll_fd: i32, read_end: &bitcoin_tokenpipe::TokenPipeEnd) {
    use libc::{epoll_ctl, epoll_event, EPOLL_CTL_ADD, EPOLLIN};

    let fd = read_end.get_fd(); // via `getset::Getters` (crate-internal visibility may differ)

    let mut ev = epoll_event {
        events: EPOLLIN as u32,
        u64: fd as u64,
    };

    unsafe {
        let rc = epoll_ctl(epoll_fd, EPOLL_CTL_ADD, fd, &mut ev as *mut _);
        if rc != 0 {
            // handle error
        }
    }
}
```

Note: The actual accessor visibility for `fd` is `pub(crate)` in the current definition, so external consumers may wrap or expose the FD via higher-level components provided by the parent project (`bitcoin-rs`). The example illustrates the intended use pattern at the systems boundary.

---

## Error handling and semantics

`token_read` and `token_write` encapsulate some low-level error behavior:

- `EINTR` is transparently retried.
- A return of `0` bytes from the OS is mapped to `TS_EOS` (end-of-stream).
- Any other negative return from `read(2)`/`write(2)` becomes `TS_ERR`.

Given the current API shape, callers typically treat non-negative values as data and negative values as control/status codes.

Example pattern:

```rust
use bitcoin_tokenpipe::TokenPipeEndStatus;

fn handle_read(rc: i32) {
    if rc >= 0 {
        let token = rc as u8;
        // process token
    } else if rc == TokenPipeEndStatus::TS_EOS as i32 {
        // graceful shutdown
    } else {
        // error path
    }
}
```

---

## Ownership and lifetime

`TokenPipe` and `TokenPipeEnd` are intentionally move-only by design (no `Clone` implementation):

- Each FD is conceptually owned by exactly one Rust value at a time.
- The `*_from_other` and `assign_from` methods provide explicit, logged transfer of FDs between owning structures.
- Internally, FDs are invalidated using `-1` after transfer or close.

This design prevents double-close and makes it easier to reason about descriptor lifetimes when piping through layered components.

---

## Logging and diagnostics

The crate uses the `tracing` ecosystem for diagnostic logging:

- `trace!` for fine-grained lifecycle events (creation, drop, close, read/write start, success, and failure).
- `debug!`, `info!`, `warn!`, `error!` for increasingly severe conditions.

Integrate with `tracing-subscriber` or your preferred backend to capture and analyze runtime behavior.

---

## Relationship to `bitcoin-rs`

This crate resides in the `https://github.com/klebs6/bitcoin-rs` repository and is primarily intended as an internal infrastructure component:

- It provides a minimal, testable, and well-logged primitive for inter-thread/inter-process signaling.
- It can be used by higher-level concurrency and networking code within the broader `bitcoin-rs` project without depending on heavyweight async frameworks.

While it is reusable in other projects, developers should treat it as a low-level building block rather than a complete concurrency abstraction.

---

## License

Licensed under the MIT license.

See the `LICENSE` file in the `bitcoin-rs` repository for more details.

---

## Safety considerations

- All system calls are wrapped in `unsafe` blocks, but the public API is safe.
- Correctness depends on respecting the intended ownership model: do not duplicate FDs externally without ensuring they are not double-closed.
- The pipe is blocking by default; if integrating into non-blocking or time-bounded systems, configure the underlying FDs accordingly (e.g., via `fcntl` in external code) and handle `EAGAIN`/`EWOULDBLOCK` as appropriate.
