# bitcoin-version

Versioning and BIP14-compliant sub-version formatting utilities for Bitcoin node and wallet implementations in Rust.

---

## Overview

`bitcoin-version` provides small, focused primitives for constructing version strings used in Bitcoin software, closely mirroring the behavior and conventions of Bitcoin Core:

- Canonical **semantic-like version formatting** from a packed integer (`i32`) representation.
- **Full version string assembly** that incorporates git tags, commit hashes, and release/dev status at compile time.
- **BIP14-compliant sub-version strings** for the `User-Agent` field in the Bitcoin P2P protocol.
- A compile-time **copyright macro** driven by environment variables.

The crate is intended for applications that:

- Implement a Bitcoin node, wallet, indexer, or related service.
- Need to report accurate, reproducible version information over the network (e.g., P2P `version` messages or HTTP APIs).
- Want build metadata (release vs. dev, git commit, tag) wired into the binary in a deterministic way.

The design follows the conventions of upstream Bitcoin Core, which uses an integer-encoded client version and BIP14-defined sub-version format (`/Name:major.minor.revision(optional-comments)/`).

---

## Encoding of Versions

### `format_version(n_version: i32) -> String`

`format_version` interprets `n_version` as a packed integer and renders it as `MAJOR.MINOR.REVISION`:

```text
major    = n_version / 10000
minor    = (n_version / 100) % 100
revision = n_version % 100
``

For example:

- `format_version(10000)` → `"1.0.0"`
- `format_version(10100)` → `"1.1.0"`
- `format_version(10102)` → `"1.1.2"`

This encoding is historically used in Bitcoin Core so that

```text
n_version = 10000 * major + 100 * minor + revision
```

which preserves lexicographic and numeric ordering across common version progressions.

### Example

```rust
use bitcoin_version::format_version;

let n_version = 10102; // 1.1.2
assert_eq!(format_version(n_version), "1.1.2");
```

---

## Full Version Strings

### `format_full_version() -> String`

`format_full_version` constructs a complete version string, potentially including git metadata, depending on compile-time configuration.

The function is gated behind `#[cfg(not(WINDRES_PREPROC))]` and uses a combination of constants or environment-driven values such as:

- `PACKAGE_VERSION` – the crate/package version (e.g., `0.1.19`).
- `BUILD_GIT_TAG` – indicates a tagged build.
- `BUILD_GIT_COMMIT` – an abbreviated git commit hash.
- `GIT_COMMIT_ID` – a commit identifier from `git archive`.
- `CLIENT_VERSION_IS_RELEASE` – discriminates release vs. dev builds.

The logic (simplified):

- **Tagged builds** (`BUILD_GIT_TAG`) typically yield something like:
  - `v0.1.19`
  - or `v0.1.19-rc1` depending on configuration.
- **Untagged dev builds with commit hash**:
  - `v0.1.19-<commit>`
- **Untagged dev builds with archive commit ID**:
  - `v0.1.19-g<id>`

Example (conceptual):

```rust
#[cfg(not(WINDRES_PREPROC))]
use bitcoin_version::format_full_version;

#[cfg(not(WINDRES_PREPROC))]
fn print_version_banner() {
    let full = format_full_version();
    println!("bitcoin-rs node {}", full);
}
```

Because this function relies on `lazy_static!` and build-time configuration symbols, you typically consume it as-is from your application without needing to manage those details directly (they are usually wired in by your build system or a higher-level crate in the `bitcoin-rs` workspace).

---

## BIP14 Sub-Version Strings

### Background: BIP14

[BIP14](https://github.com/bitcoin/bips/blob/master/bip-0014.mediawiki) standardizes the user agent string ("sub-version") included in the Bitcoin P2P `version` message. A compliant sub-version string has the form:

```text
/Name:major.minor.revision(optional;comments;here)/
```

Examples:

- `/Satoshi:0.21.0/`
- `/MyNode:1.3.2(beta;custom-patch)/`

This string is used in network-level analytics, compatibility logic, and sometimes feature-gating behavior in other nodes.

### `format_sub_version(name: &String, n_client_version: i32, comments: &Vec<String>) -> String`

`format_sub_version` builds a BIP14-compliant sub-version string:

1. Prefix `/`.
2. Append the client `name`.
3. Append `:`.
4. Append the formatted version from `format_version(n_client_version)`.
5. Optionally append `(` `comments.join("; ")` `)` if any comments exist.
6. Suffix `/`.

The `comments` field is designed for optional qualifiers like `"beta"`, `"pruned"`, `"experimental"`, or deployment-specific tags.

#### Example

```rust
#[cfg(not(WINDRES_PREPROC))]
use bitcoin_version::format_sub_version;

#[cfg(not(WINDRES_PREPROC))]
fn user_agent() -> String {
    let name = "bitcoin-rs-node".to_string();
    let n_client_version = 10102; // -> 1.1.2
    let comments = vec![
        "beta".to_string(),
        "x86_64-linux".to_string(),
    ];

    format_sub_version(&name, n_client_version, &comments)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(not(WINDRES_PREPROC))]
    fn sub_version_is_bip14_like() {
        let ua = user_agent();
        assert!(ua.starts_with('/'));
        assert!(ua.ends_with('/'));
        assert!(ua.contains(":1.1.2"));
        assert!(ua.contains("(beta; x86_64-linux)"));
    }
}
```

The resulting string will be:

```text
/bitcoin-rs-node:1.1.2(beta; x86_64-linux)/
```

---

## Copyright Macro

### `copyright_str!()`

The crate exposes a macro that expands at compile time to a copyright string driven by environment variables:

```rust
let c = bitcoin_version::copyright_str!();
println!("{}", c);
```

The expansion is effectively:

```text
"2009-<COPYRIGHT_YEAR> <COPYRIGHT_HOLDERS_FINAL>"
```

You must provide the following environment variables at compile time (e.g., via `build.rs` or `RUSTFLAGS`):

- `COPYRIGHT_YEAR` – last year in the copyright range, e.g. `2024`.
- `COPYRIGHT_HOLDERS_FINAL` – holder name, e.g. `bitcoin-rs developers`.

This is useful for producing headers or banners that closely match Bitcoin Core’s legal attributions.

---

## Logging

Several functions use structured logging macros like `trace!` and `debug!`. In most setups these originate from the `tracing` crate or a similar logging framework.

To benefit from these logs, ensure that your binary has appropriate logging initialization, for example:

```rust
fn main() {
    // Example with `tracing-subscriber`
    tracing_subscriber::fmt::init();

    // Use bitcoin-version functionality
    #[cfg(not(WINDRES_PREPROC))]
    {
        let full = bitcoin_version::format_full_version();
        tracing::info!(%full, "starting node");
    }
}
```

If you do not initialize logging, the functions still behave correctly; you just lose the diagnostic output.

---

## Build Configuration and Conditional Compilation

Several pieces of functionality are gated by `cfg` flags that mirror the C++/Autotools configuration of Bitcoin Core:

- `BUILD_GIT_TAG`
- `BUILD_GIT_COMMIT`
- `GIT_COMMIT_ID`
- `CLIENT_VERSION_IS_RELEASE`
- `WINDRES_PREPROC`

In a simple standalone usage, you can ignore these and rely on the defaults provided by the crate or workspace. In a more advanced integration, you can define them via `build.rs` and `cargo:rustc-cfg` outputs or via custom build systems to precisely control how version strings are constructed.

Conceptually:

- **Release build**: define `CLIENT_VERSION_IS_RELEASE`, possibly `BUILD_GIT_TAG`, and omit commit IDs.
- **Dev build**: do *not* define `CLIENT_VERSION_IS_RELEASE`, but provide `BUILD_GIT_COMMIT` or `GIT_COMMIT_ID`.

The `WINDRES_PREPROC` flag is used to avoid emitting Rust-only constructs into Windows resource preprocessing; when set, the corresponding functions are simply not compiled.

---

## Integration Pattern

A typical integration into a Bitcoin-related Rust binary might look like:

```rust
use bitcoin_version::{format_version, copyright_str};

#[cfg(not(WINDRES_PREPROC))]
use bitcoin_version::{format_full_version, format_sub_version};

fn main() {
    #[cfg(not(WINDRES_PREPROC))]
    {
        let full_version = format_full_version();
        let name = "bitcoin-rs-node".to_string();
        let n_client_version = 10102;
        let comments = vec!["release".to_string()];

        let user_agent = format_sub_version(&name, n_client_version, &comments);

        println!("version: {}", full_version);
        println!("user agent: {}", user_agent);
    }

    let c = copyright_str!();
    println!("copyright: {}", c);
}
```

---

## Repository and License

This crate lives in the [`bitcoin-rs` repository](https://github.com/klebs6/bitcoin-rs) and is licensed under the MIT license.

---

## Caveats

- The crate design and behavior are closely tied to the surrounding `bitcoin-rs` workspace and build system. When used outside of that context, ensure your build configuration provides the necessary environment variables and cfg flags, or accept the default behavior.
