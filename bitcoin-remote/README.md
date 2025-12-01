# bitcoin-remote

Low-level, zero-magic utilities for speaking Bitcoin Core's JSON‑RPC over HTTP, faithfully ported from upstream C++ with strongly typed Rust interfaces.

This crate does **not** try to be a full Bitcoin wallet or a high-level client library. Instead, it focuses on being a **precise, observable, and testable** substrate for:

- constructing and serializing JSON‑RPC requests and replies,
- handling Bitcoin Core's RPC error model (codes and HTTP mappings),
- managing `rpcauth`/cookie-based authentication,
- resolving Bitcoin-style configuration paths,
- and handling parameter-conversion metadata reproduced from Bitcoin Core's own RPC tables.

If you want exact behavioral parity with `bitcoind`'s RPC interface—down to error codes, HTTP status behavior, and batch semantics—this crate is intended for you.

---

## Features at a Glance

- **HTTP / JSON‑RPC fidelity**
  - `HTTPStatusCode` enum for mapping internal failures to HTTP status codes.
  - JSON‑RPC 1.0 compatible wire format with selective adoption of 1.1/2.0 semantics for error structure.
  - Batch‑reply processing that mirrors upstream C++ behavior, including panic conditions on malformed replies.

- **Bitcoin Core RPC error codes**
  - `RPCErrorCode` is a `bitflags!` set of all Bitcoin Core RPC error codes, including wallet, P2P, chain, and general application errors.
  - Aliases for backward compatibility (e.g. `RPC_TRANSACTION_ERROR`) closely track upstream.

- **RPC parameter conversion metadata**
  - `RPCConvertParam` and `RPCConvertTable` encode which RPC parameters must be treated as *JSON* even when passed as textual CLI arguments.
  - Lookup by `(method, index)` or `(method, name)` gives you a boolean indicating whether a parameter must be JSON‑parsed before being passed to Core.

- **Authentication cookie helpers**
  - `generate_auth_cookie`, `get_auth_cookie`, and `delete_auth_cookie` implement the same cookie‑file strategy as Bitcoin Core:
    - random password generation
    - temporary file + atomic rename
    - respect for `-rpccookiefile` from global args

- **JSON‑RPC request and response composition**
  - `JSONRPCRequest` encapsulates method, params, id, and metadata like `uri`, `auth_user`, and `peer_addr`.
  - `jsonrpc_request_obj`, `jsonrpc_reply_obj`, `jsonrpc_reply`, and `jsonrpc_error` are low-level helpers over `UniValue`.

- **Instrumentation**
  - Strategic, structured logging via the `tracing` crate at trace/debug/info levels, intended for production observability without polluting the hot path.

The crate assumes you are comfortable working close to the protocol: you will handle HTTP transport concerns, lifetimes, and concurrency at your own abstraction layer.

---

## Design Philosophy

The design mirrors Bitcoin Core's internals:

- **Minimal abstraction leak**: the Rust types are thin wrappers around the behavior of upstream C++ constructs, including panic conditions and error code usage.
- **Deterministic edge behavior**: malformed batch responses, invalid request objects, and type errors produce panics consistent with the original exception‑throwing C++ implementation.
- **Explicit data flow**: all JSON objects are composed using `UniValue` (from `uni_value`), not `serde_json`, to follow Core's semantics exactly.

The goal is to allow you to *reconstruct, inspect, and control* your interaction with a Bitcoin node with maximal transparency.

---

## Core Types and Modules

### HTTPStatusCode

```rust
#[repr(i32)]
pub enum HTTPStatusCode {
    HTTP_OK,
    HTTP_BAD_REQUEST,
    HTTP_UNAUTHORIZED,
    HTTP_FORBIDDEN,
    HTTP_NOT_FOUND,
    HTTP_BAD_METHOD,
    HTTP_INTERNAL_SERVER_ERROR,
    HTTP_SERVICE_UNAVAILABLE,
}
```

`HTTPStatusCode` encodes the subset of HTTP codes relevant to JSON‑RPC error mapping. Upstream Core maps certain RPC errors to specific HTTP codes (e.g. `RPC_INVALID_REQUEST` → `HTTP_BAD_REQUEST`). Use this when designing your HTTP layer.

### JSONRPCRequestMode

```rust
pub enum JSONRPCRequestMode {
    EXECUTE,
    GET_HELP,
    GET_ARGS,
}
```

`JSONRPCRequestMode` allows you to model whether an incoming request is meant to be executed or used for help/argument introspection. This is primarily useful for CLI frontends or meta‑RPC layers.

### RPCErrorCode (bitflags)

`RPCErrorCode` is defined using `bitflags!` over `i32`. It enumerates the entire Bitcoin Core RPC error space:

- **Standard JSON‑RPC 2.0 errors**: `RPC_INVALID_REQUEST`, `RPC_METHOD_NOT_FOUND`, `RPC_INVALID_PARAMS`, `RPC_INTERNAL_ERROR`, `RPC_PARSE_ERROR`.
- **General application errors**: `RPC_MISC_ERROR`, `RPC_TYPE_ERROR`, `RPC_INVALID_PARAMETER`, etc.
- **P2P client errors**: `RPC_CLIENT_NOT_CONNECTED`, `RPC_CLIENT_IN_INITIAL_DOWNLOAD`, `RPC_CLIENT_NODE_CAPACITY_REACHED`, etc.
- **Chain and mempool errors**.
- **Wallet errors**: `RPC_WALLET_ERROR`, `RPC_WALLET_INSUFFICIENT_FUNDS`, `RPC_WALLET_UNLOCK_NEEDED`, `RPC_WALLET_ALREADY_LOADED`, etc.
- **Legacy aliases and reserved codes**, retained for backwards compatibility.

Each error is a bitflag with a specific negative code matching Bitcoin Core's public RPC interface. Example:

```rust
use bitcoin_remote::RPCErrorCode;

fn classify(err_code: i32) {
    if err_code == RPCErrorCode::RPC_WALLET_INSUFFICIENT_FUNDS.bits() {
        eprintln!("insufficient funds: {err_code}");
    }
}
```

Using a bitflag type instead of raw `i32` clarifies intent and allows consistent code completion.

### RPCConvertParam & RPCConvertTable

Bitcoin Core distinguishes between positional and named RPC parameters that must be treated as *JSON* instead of simple strings. For example, `sendrawtransaction` expects a hex string, whereas `createrawtransaction` may expect complex JSON objects.

This crate mirrors Core's table‑driven approach using:

```rust
#[derive(Debug, Getters, MutGetters, Setters, Default, Builder)]
pub struct RPCConvertParam {
    method_name: &'static str,
    param_idx:   i32,
    param_name:  &'static str,
}

#[derive(Debug, Getters, MutGetters, Setters, Default, Builder)]
pub struct RPCConvertTable {
    members:         HashSet<(String, i32)>,
    members_by_name: HashSet<(String, String)>,
}
```

and the global instance:

```rust
lazy_static! {
    pub static ref RPC_CVT_TABLE: std::sync::Mutex<RPCConvertTable> =
        std::sync::Mutex::new(RPCConvertTable::new());
}
```

`RPCConvertTable::new()` populates `members` and `members_by_name` from a static `vRPCConvertParams` list emitted from upstream definitions. You can query it as follows:

```rust
use bitcoin_remote::RPC_CVT_TABLE;

fn param_requires_json(method: &str, index: i32) -> bool {
    let mut tbl = RPC_CVT_TABLE.lock().expect("RPC_CVT_TABLE poisoned");
    tbl.convert_with_method_and_idx(method, index)
}

fn param_requires_json_by_name(method: &str, name: &str) -> bool {
    let mut tbl = RPC_CVT_TABLE.lock().expect("RPC_CVT_TABLE poisoned");
    tbl.convert_with_method_and_name(method, name)
}
```

These APIs are intentionally simple: they answer *"must this parameter be JSON‑parsed?"* without hiding the table's structure.

### JSONRPCRequest

```rust
pub struct JSONRPCRequest {
    id:         UniValue,
    str_method: String,
    params:     UniValue,
    mode:       JSONRPCRequestMode,
    uri:        String,
    auth_user:  String,
    peer_addr:  String,
    context:    Box<dyn Any>,
}
```

The `parse` method implements JSON‑RPC request parsing consistent with Bitcoin Core:

- Validates that the top-level value is an object.
- Extracts `id` early so subsequent error reports can include it.
- Checks `method` is present and a string.
- Accepts `params` as either an array or object; `null` becomes an empty array; all other types panic as invalid.

Example:

```rust
use bitcoin_remote::{JSONRPCRequest, jsonrpc_error, RPCErrorCode};
use uni_value::{UniValue, VType};

fn parse_single_request(val_request: &UniValue) -> JSONRPCRequest {
    let mut req = JSONRPCRequest {
        // reasonable defaults; context populated by caller
        id: UniValue::null(),
        str_method: String::new(),
        params: UniValue::empty_array(),
        mode: bitcoin_remote::JSONRPCRequestMode::EXECUTE,
        uri: String::new(),
        auth_user: String::new(),
        peer_addr: String::new(),
        context: Box::new(()),
    };

    // May panic with a JSON‑encoded error, per Core behavior
    req.parse(val_request);
    req
}
```

Because the implementation intentionally panics on structural violations (mirroring C++ exceptions), you should either sanitize input at your HTTP boundary or wrap parsing in higher‑level error handling if you need a non‑panic path.

---

## JSON‑RPC Utilities

### Building requests

```rust
pub fn jsonrpc_request_obj(str_method: &str, params: &UniValue, id: &UniValue) -> UniValue
```

Creates a minimal JSON‑RPC request object:

```json
{ "method": <str_method>, "params": <params>, "id": <id> }
```

Example:

```rust
use bitcoin_remote::jsonrpc_request_obj;
use uni_value::{UniValue, VType};

let mut params = UniValue::new(VType::VARR, None);
params.push_back("getblockchaininfo");

let id = UniValue::from(1_i64);
let req = jsonrpc_request_obj("getblockchaininfo", &params, &id);

let wire = req.write(None, None);
```

### Building replies and errors

```rust
pub fn jsonrpc_reply_obj(result: &UniValue, error: &UniValue, id: &UniValue) -> UniValue
pub fn jsonrpc_reply(result: &UniValue, error: &UniValue, id: &UniValue) -> String
pub fn jsonrpc_error(code: i32, message: &str) -> UniValue
```

Semantics:

- If `error` is not `null`, the reply's `"result"` field is forced to `null`.
- `jsonrpc_reply` serializes the reply to a compact string and appends a newline (matching Core's `write() + "\n"`).

Example:

```rust
use bitcoin_remote::{jsonrpc_reply, jsonrpc_error, RPCErrorCode};
use uni_value::UniValue;

// success reply
let result = UniValue::from("ok");
let error = UniValue::null();
let id = UniValue::from(1_i64);
let success_wire = jsonrpc_reply(&result, &error, &id);

// error reply
let err_obj = jsonrpc_error(
    RPCErrorCode::RPC_INVALID_PARAMS.bits(),
    "invalid parameters supplied",
);
let error_wire = jsonrpc_reply(&UniValue::null(), &err_obj, &id);
```

### Batch reply processing

```rust
pub fn jsonrpc_process_batch_reply(in_: &UniValue) -> Vec<UniValue>
```

Parses a batch reply into a `Vec<UniValue>` indexed by numeric `"id"`:

- `in_` must be an array; otherwise the function panics with *"Batch must be an array"*.
- Each member must be an object; non‑objects panic with *"Batch member must be an object"*.
- Each member must have an integer `"id"` between `0` and `batch_size - 1`; otherwise, a panic with *"Batch member id is larger than batch size"* occurs.

This deterministic behavior makes it straightforward to implement batch clients that rely strictly on index‑based mapping.

Example:

```rust
use bitcoin_remote::jsonrpc_process_batch_reply;
use uni_value::{UniValue, VType};

fn handle_batch(batch_uv: &UniValue) {
    let replies = jsonrpc_process_batch_reply(batch_uv);
    for (idx, reply) in replies.iter().enumerate() {
        println!("reply {idx}: {reply:?}");
    }
}
```

---

## Authentication Cookie Helpers

Bitcoin Core supports a cookie‑file based authentication mechanism. This crate provides low-level helpers that implement the same behavior, including path resolution and atomic writes.

### get_auth_cookie_file

```rust
pub fn get_auth_cookie_file(temp: Option<bool>) -> Box<Path>
```

- Resolves the absolute path for the RPC auth cookie.
- If `temp == Some(true)`, appends `.tmp` and returns the path for the temporary file location.
- Uses `-rpccookiefile=<path>` from global args if present; otherwise it falls back to the default (typically something like `$DATADIR/.cookie`).

### generate_auth_cookie

```rust
pub fn generate_auth_cookie(cookie_out: &mut String) -> bool
```

Steps:

1. Generate 32 random bytes and hex‑encode them as the password portion.
2. Construct `"<COOKIEAUTH_USER>:<hex_password>"`.
3. Write to the temporary cookie file (`.tmp`), creating parent directories as needed.
4. Atomically rename the temporary file to the final auth cookie path.

On success, `cookie_out` is populated and `true` is returned. Errors are logged via `tracing::error!` and `false` is returned; best‑effort clean‑up is attempted.

Example:

```rust
use bitcoin_remote::generate_auth_cookie;

let mut cookie = String::new();
if generate_auth_cookie(&mut cookie) {
    println!("Generated cookie: {cookie}");
}
```

### get_auth_cookie

```rust
pub fn get_auth_cookie(cookie_out: &mut String) -> bool
```

- Reads the cookie from disk into `cookie_out`.
- Trims trailing `\n` and optional `\r` to match C++ `getline` behavior.
- Returns `false` if the file cannot be opened or read, logging at `debug` or `warn` levels.

### delete_auth_cookie

```rust
pub fn delete_auth_cookie()
```

Attempts to delete the cookie file. Any I/O error other than *file not found* is logged but otherwise ignored.

Example integration pattern:

```rust
use bitcoin_remote::{generate_auth_cookie, get_auth_cookie, delete_auth_cookie};

fn rotate_cookie() {
    let mut new_cookie = String::new();
    if !generate_auth_cookie(&mut new_cookie) {
        eprintln!("failed to generate new cookie");
        return;
    }

    let mut read_back = String::new();
    if get_auth_cookie(&mut read_back) {
        assert_eq!(new_cookie, read_back);
    }

    delete_auth_cookie();
}
```

---

## Error Semantics and Stability

- **Panics vs. errors**: many functions (`JSONRPCRequest::parse`, `jsonrpc_process_batch_reply`) intentionally use `panic!` on structural violations, to mimic Bitcoin Core's use of exceptions for programmer / protocol errors rather than expected runtime failures.
- **Logging**: `trace!`, `debug!`, `info!`, `warn!`, and `error!` invocations align with logical phases—construction, parsing, and I/O. In production, you can adjust your `tracing` subscriber level to tune verbosity.
- **Error codes**: `RPCErrorCode` values are kept numerically identical to upstream. Where upstream deprecates or repurposes codes, the crate favors backward compatibility.

If you are building public APIs or user‑facing tooling on top of this crate, you may want to wrap these primitives into a non‑panicking facade that translates panics into structured error values.

---

## Example: End‑to‑End Single Call

Below is a sketch of how you might integrate `bitcoin-remote` into a higher-level HTTP client. This example omits error‑handling and TLS for brevity.

```rust
use bitcoin_remote::{
    jsonrpc_request_obj,
    jsonrpc_reply,
    jsonrpc_process_batch_reply,
};
use uni_value::{UniValue, VType};

fn build_getblockchaininfo_request() -> String {
    let params = UniValue::empty_array();
    let id = UniValue::from(0_i64);
    let req_obj = jsonrpc_request_obj("getblockchaininfo", &params, &id);
    let mut s = req_obj.write(None, None);
    s.push('\n');
    s
}

fn main() {
    let body = build_getblockchaininfo_request();

    // send `body` over HTTP POST to bitcoind's RPC endpoint using your
    // HTTP client of choice (e.g., reqwest). Parse the response into
    // `UniValue` using the `uni_value` crate, then handle it.
}
```

---

## Crate Metadata

- **Crate name**: `bitcoin-remote`
- **Version**: `0.1.22`
- **Edition**: Rust 2021
- **License**: MIT
- **Repository**: <https://github.com/klebs6/bitcoin-rs>
- **Primary author**: `klebs <none>`

---

## Caveats

- This README was produced by an AI model and may diverge slightly from the exact current API surface, especially around auxiliary functions and constants not shown in the provided interface.
- Always rely on the actual Rustdoc and source code in the repository for authoritative reference.
