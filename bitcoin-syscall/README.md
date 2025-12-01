# bitcoin-syscall

A focused Rust port of Bitcoin Core's seccomp-based syscall sandbox. This crate exposes the same high‑level policy buckets and enforcement semantics as the upstream C++ implementation, enabling you to harden long‑running Bitcoin‑adjacent services (or any Linux x86‑64 daemon) against unexpected syscall usage.

---

## Features at a Glance

- **Policy‑driven syscall sandboxing** via `SyscallSandboxPolicy` – maps closely to Bitcoin Core thread roles.
- **Allow‑list enforcement** using Linux **seccomp‑BPF** filters.
- **Configurable violation behavior** via `SyscallSandboxAction`:
  - Kill offending process.
  - Trigger a SIGSYS debug handler that logs the violating syscall, then aborts.
- **Deterministic, monotone restriction model**: once enabled, capabilities can *only* be reduced.
- **Thread‑scoped enforcement**: each call to `set_syscall_sandbox_policy` installs a filter for the current thread and its descendants.
- **Portable API**: on non‑Linux‑x86_64, the crate compiles but behaves as a no‑op shim.

This crate is especially appropriate when you want to adopt Bitcoin Core's syscall hardening strategy in Rust code, or when embedding Rust components into a hardened `bitcoind`-like environment.

---

## Platform and Safety Model

- **Supported for enforcement**: `target_os = "linux"`, `target_arch = "x86_64"`, built with the `USE_SYSCALL_SANDBOX` cfg.
- **All other targets**: the same public API exists, but functions are effectively no‑ops and simply return conservative defaults. This allows unconditional dependency in cross‑platform crates.
- **Privilege monotonicity**: once `setup_syscall_sandbox` succeeds and `set_syscall_sandbox_policy` installs a filter, the process cannot regain previously dropped syscall capabilities. This is enforced by `prctl(PR_SET_NO_NEW_PRIVS)`.

Internally, each policy maps to a set of allowed syscall numbers, which are compiled into a seccomp BPF program. At runtime the kernel evaluates `seccomp_data.nr` against that allow‑list, permitting or terminating the calling task according to the configured default action.

---

## Core Concepts

### SyscallSandboxPolicy

`SyscallSandboxPolicy` categorizes workloads into coarse "buckets" that are tuned for specific operational phases of a Bitcoin‑style node:

```rust
pub enum SyscallSandboxPolicy {
    Initialization,
    InitializationDnsSeed,
    InitializationLoadBlocks,
    InitializationMapPort,
    MESSAGE_HANDLER,
    NET,
    NET_ADD_CONNECTION,
    NET_HTTP_SERVER,
    NET_HTTP_SERVER_WORKER,
    NET_OPEN_CONNECTION,
    SCHEDULER,
    TOR_CONTROL,
    TX_INDEX,
    VALIDATION_SCRIPT_CHECK,
    SHUTOFF,
}
```

Semantically (mirroring the C++ code):

- **Initialization** / **InitializationDnsSeed** / **InitializationMapPort**:
  - Most permissive profiles used during early startup.
  - Allow network and filesystem operations.
- **InitializationLoadBlocks**:
  - Filesystem access only (e.g., block files, state, index data), no additional networking beyond the default set.
- **MESSAGE_HANDLER**, **SCHEDULER**, **TX_INDEX**:
  - Steady‑state workers that require filesystem I/O but no direct network initiation.
- **NET**, **NET_ADD_CONNECTION**, **NET_HTTP_SERVER**, **NET_HTTP_SERVER_WORKER**, **NET_OPEN_CONNECTION**, **TOR_CONTROL**:
  - Network‑intensive components.
  - Allow relevant socket operations & filesystem access.
- **VALIDATION_SCRIPT_CHECK**:
  - Extremely constrained execution; relies on the minimal default syscall set defined in `SeccompPolicyBuilder::default`.
- **SHUTOFF**:
  - Late shutdown profile permitting basic filesystem operations to flush and close resources.

Policies are **monotone** in power: `Initialization` is constructed as a strict superset of all other policies, and later policy applications cannot restore previously removed syscalls.

### SyscallSandboxAction

Controls the **default reaction** when an unlisted syscall is attempted after a filter is active:

```rust
pub enum SyscallSandboxAction {
    KILL_PROCESS,
    INVOKE_SIGNAL_HANDLER,
}
```

- `KILL_PROCESS` – the kernel uses `SECCOMP_RET_KILL_PROCESS` to terminate the process immediately.
- `INVOKE_SIGNAL_HANDLER` – the kernel uses `SECCOMP_RET_TRAP`, delivering `SIGSYS` to the offending thread. The crate registers `syscall_sandbox_debug_signal_handler` to:
  - Read the offending syscall number from the `ucontext_t` (RAX).
  - Resolve the symbolic name via `get_linux_syscall_name`.
  - Determine the logical thread name via `thread_get_internal_name`.
  - Log an explicit error and abort.

This makes it straightforward to identify missing allow‑list entries during development or test by intentionally causing violations.

---

## High‑Level Usage

### 1. Enable the sandbox

Call `setup_syscall_sandbox` once early in process initialization (typically on the main thread, before spawning worker threads):

```rust
use bitcoin_syscall::setup_syscall_sandbox;

fn main() {
    // If true, use SIGSYS reporting before termination.
    let log_before_terminate = true;

    if !setup_syscall_sandbox(log_before_terminate) {
        // On supported platforms, a `false` result indicates a failure to
        // install the debug handler. The sandbox itself will *not* be active.
        eprintln!("Failed to set up syscall sandbox debug handler");
    }

    // From this point onwards, you can apply specific policies per thread.
}
```

Semantics:

- Ensures `SetupSyscallSandbox` is only executed once (further calls panic).
- Sets the global mode `G_SYSCALL_SANDBOX_ENABLED`.
- Configures whether violations trigger the debug signal handler.
- Immediately installs the **Initialization** policy as the most permissive baseline.

On unsupported platforms this returns `false` and performs no enforcement.

### 2. Apply policies per thread

Each worker thread should adopt an appropriate policy as soon as its functional role is known.

```rust
use bitcoin_syscall::{
    set_syscall_sandbox_policy,
    SyscallSandboxPolicy,
};

fn spawn_network_worker() {
    std::thread::spawn(|| {
        // Mark this thread as a network thread before performing network I/O.
        set_syscall_sandbox_policy(SyscallSandboxPolicy::NET);

        // ... network event loop, connection management, etc.
    });
}

fn spawn_validation_worker() {
    std::thread::spawn(|| {
        set_syscall_sandbox_policy(SyscallSandboxPolicy::VALIDATION_SCRIPT_CHECK);

        // Execute script validation tasks with an aggressive sandbox.
    });
}
```

Key points:

- Calling `set_syscall_sandbox_policy` when the global sandbox is not enabled is a no‑op (compatible with the C++ logic).
- When enabled, it builds and installs a seccomp BPF filter for the **current thread** using the relevant methods on `SeccompPolicyBuilder`.
- You can tighten restrictions over time (e.g., early initialization → steady state) but cannot re‑expand capabilities.

### 3. Testing your policies

`test_disallowed_sandbox_call` triggers a syscall (`getgroups`) assumed *not* to be on any allow‑list. This is useful in unit tests to assert that your environment correctly enforces seccomp policies.

```rust
use bitcoin_syscall::{
    setup_syscall_sandbox,
    set_syscall_sandbox_policy,
    SyscallSandboxPolicy,
    test_disallowed_sandbox_call,
};

#[test]
fn sandbox_violations_are_handled() {
    if !setup_syscall_sandbox(true) {
        // On unsupported platforms, skip.
        return;
    }

    set_syscall_sandbox_policy(SyscallSandboxPolicy::MESSAGE_HANDLER);

    // This should terminate the process or raise SIGSYS
    // depending on configuration, so typically you run it in
    // a subprocess / integration test harness.
    test_disallowed_sandbox_call();
}
```

---

## Low‑Level API Surface

### `SeccompPolicyBuilder`

`SeccompPolicyBuilder` is a thin builder around a `HashSet<u32>` of syscall numbers. It exists primarily as an internal mechanism to mirror the upstream C++ definitions, but it is public and can be used directly if you need a custom profile.

```rust
pub struct SeccompPolicyBuilder {
    allowed_syscalls: HashSet<u32>,
}
```

The `Default` implementation pre‑allows a base set of syscalls required for typical user‑space operation:

- Address space operations: `mmap`, `mprotect`, `madvise`, `brk`, etc.
- Epoll: `epoll_create1`, `epoll_ctl`, `epoll_wait`, `epoll_pwait`.
- Futexes: `futex`, `set_robust_list`.
- Eventfd, pipes, `prctl`, sleep, signal handling, process lifecycle, etc.
- Minimal introspection (`getrlimit`, `getrusage`, `sysinfo`, `uname`).

Additional capability sets can be enabled incrementally via methods such as:

- `allow_file_system()` – directory and file metadata operations (`open`, `fstat`, `getdents{,64}`, `rename`, `statx`, etc.).
- `allow_network()` – socket creation and connection (`socket`, `bind`, `accept`, `connect`, `listen`, `setsockopt`, `socketpair`).
- `allow_network_socket_information()` – `getsockname`, `getpeername`, `getsockopt`.
- `allow_general_io()` – read/write, vectored I/O, polling, and socket send/recv.
- `allow_get_random()`, `allow_get_time()`, `allow_global_system_status()`, etc.

The key constructor is:

```rust
impl SeccompPolicyBuilder {
    pub fn build_filter(&mut self, default_action: SyscallSandboxAction) -> Vec<libc::sock_filter> { /* ... */ }
}
```

`build_filter` assembles the BPF instruction sequence equivalent to the macros in `seccomp-bpf.h`:

1. Load architecture from `seccomp_data.arch` and compare with `AUDIT_ARCH_X86_64`. If not equal, kill the process.
2. Load the syscall number from `seccomp_data.nr`.
3. For each allowed syscall, emit a conditional jump and an `ALLOW` return.
4. Emit a final `RET` with either `SECCOMP_RET_KILL_PROCESS` or `SECCOMP_RET_TRAP` depending on `default_action`.

The resulting program can be handed directly to `prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, ...)`.

### Debug and Introspection Helpers

- `get_linux_syscall_name(syscall_number: u32) -> String`
  - Looks up a numeric syscall in the `LINUX_SYSCALLS` table.
  - Returns `"*unknown*"` if not present.
- `thread_get_internal_name() -> String`
  - Returns the current thread's logical name.
  - Uses `std::thread::current().name()` first, falling back to `pthread_getname_np` for unnamed threads.

These helpers are mostly used within the debug signal handler to provide high‑quality diagnostics when a disallowed syscall is encountered.

### `syscall_sandbox_debug_signal_handler`

A Rust port of Bitcoin Core's `SyscallSandboxDebugSignalHandler`:

```rust
#[no_mangle]
pub fn syscall_sandbox_debug_signal_handler(
    _signal: i32,
    signal_info: *mut siginfo_t,
    void_signal_context: *mut c_void,
) { /* ... */ }
```

- Validates that `signal_info` and `ucontext_t` are non‑null and that the signal originated from seccomp (`si_code == SYS_SECCOMP_SI_CODE`).
- Extracts RAX from the general registers, casts to `u32` as the syscall number, performs name lookup, and formats an error message that includes the thread name.
- Logs to stderr and through the crate's logging target (`compat::syscall_sandbox`), then aborts.

Because it is marked `#[no_mangle]`, it can be referred to by C or C++ code if required for mixed‑language integration.

---

## Behavior on Unsupported Platforms

For targets other than Linux x86‑64, the crate exposes a `stubs` module with the same re‑exported API shape:

- `setup_syscall_sandbox` and `setup_syscall_sandbox_debug_handler` return `false`.
- `set_syscall_sandbox_policy` and `test_disallowed_sandbox_call` are no‑ops.
- `SeccompPolicyBuilder::build_filter` returns an empty vector.
- `get_linux_syscall_name` always returns `"*unknown*"`.

This design allows downstream code to unconditionally call the API while **feature‑gating enforcement behavior** on `cfg(all(target_os = "linux", target_arch = "x86_64"))` only when necessary.

---

## Integration Notes

- **Linkage & libc**: The crate relies on `libc`, seccomp constants, and `prctl`. When cross‑compiling, ensure that your toolchain provides the appropriate headers and sysroot.
- **Thread model**: Filters are installed per‑thread. Typically you:
  1. Call `setup_syscall_sandbox` on the main thread.
  2. Spawn threads.
  3. Immediately call `set_syscall_sandbox_policy` in each thread before performing I/O.
- **Logging**: The crate uses Rust logging macros. Configure your logging subsystem to observe the `compat::syscall_sandbox` target for detailed diagnostics.
- **Upstream alignment**: Field names, enum variants, and control flow are intentionally close to the upstream C++ in Bitcoin Core for ease of auditing and future porting.

---

## License

This crate is distributed under the **MIT** license.

---

## Disclaimer

This README was programmatically generated by an AI model based on supplied interface and commentary. It is intended to be accurate and precise, but minor discrepancies with the actual implementation may exist. Always validate security‑relevant behavior against the source code and your target environment.
