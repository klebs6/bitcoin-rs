# bitcoin-settings

A faithful, low-level Rust port of Bitcoin Core's configuration and runtime settings machinery. This crate models the full precedence stack (forced values, command-line arguments, read–write settings file, and read-only config file sections) and reproduces Bitcoin Core's historical quirks around negation, precedence, and array flattening.

---

## Overview

`bitcoin-settings` provides a small but precise abstraction over Bitcoin-style settings resolution. It is intentionally close to the original C++ implementation to enable:

- Bit‑for‑bit compatible behavior with Bitcoin Core's `GetSetting` / `GetSettingsList` / `MergeSettings` logic.
- Deterministic, pretty-printed JSON persistence of runtime settings.
- Fine-grained introspection of where an effective value "came from" in the precedence chain.

The crate assumes a JSON‑like value type (`UniValue`) and wraps it in a `SettingsValue` newtype that exposes the subset of behavior needed for the Bitcoin settings algorithm.

This is not a general configuration framework; it is an emulation of Bitcoin Core's settings semantics suitable for interoperable tooling, alternative node implementations, and integration tests that need to match Core behavior.

---

## Core Concepts and Data Model

### Sources and Precedence

Settings are aggregated from multiple sources with the following **strict precedence order** (highest to lowest):

1. `FORCED` – hard‑wired overrides not exposed to the user.
2. `COMMAND_LINE` – `-foo` and `-nofoo` style arguments.
3. `RW_SETTINGS` – runtime read–write settings file (e.g. GUI preferences).
4. `CONFIG_FILE_NETWORK_SECTION` – `bitcoin.conf` entries inside a named `[section]` (e.g. `[main]`, `[test]`).
5. `CONFIG_FILE_DEFAULT_SECTION` – `bitcoin.conf` entries before any explicit section.

These are represented by the enum:

```rust
pub enum Source {
    FORCED,
    COMMAND_LINE,
    RW_SETTINGS,
    CONFIG_FILE_NETWORK_SECTION,
    CONFIG_FILE_DEFAULT_SECTION,
}
```

### Settings Storage

All sources are combined into a single `Settings` struct:

```rust
pub struct Settings {
    forced_settings:      HashMap<String, SettingsValue>,
    command_line_options: HashMap<String, Vec<SettingsValue>>,
    rw_settings:          HashMap<String, SettingsValue>,
    ro_config:            HashMap<String, HashMap<String, Vec<SettingsValue>>>,
}
```

- `forced_settings` – single value per key.
- `command_line_options` – potentially multiple occurrences per key (`-foo=1 -foo=2`).
- `rw_settings` – single value per key, persisted via `write_settings`.
- `ro_config` – hierarchical: `section -> name -> values`, where `section == ""` encodes the default section.

The crate uses `derive_builder` and `getset` to provide convenient builders and accessors.

### Value Representation: `SettingsValue`

```rust
#[derive(Debug, Clone)]
pub struct SettingsValue(pub UniValue);
```

`SettingsValue` is a thin newtype around `UniValue`, and is intended to mirror Bitcoin Core's use of `UniValue` for settings. It supports:

- Booleans, integers, strings, null, and arrays (arrays are flattened in some APIs).
- Display as JSON via `fmt::Display` (delegates to `UniValue::write`).
- Semantic equality via stringified JSON (`PartialEq`, `Eq`).
- Fast negation detection via `is_false()` (true iff the inner value is a JSON boolean `false`).

### Spans and Negation Semantics: `SettingsSpan`

```rust
pub struct SettingsSpan {
    data: *const SettingsValue,
    size: usize,
}
```

`SettingsSpan` is a low-level, pointer-based view over a contiguous range of `SettingsValue`s. Its role is to express Bitcoin Core's **negation rules** without allocation or copying:

- A sequence is considered **negated** if its last value is `false`.
- All values at and before the last boolean `false` are treated as *negated* and therefore excluded from effective iteration.
- `begin()` returns a pointer to the first *non‑negated* value.
- `end()` returns the standard one‑past‑the‑end pointer.
- `empty()` is true if the span is logically empty after taking negation into account.

This is crucial for modeling user behavior like:

```text
-foo=1 -foo=2 -foo=0   # final `false` negates earlier settings
```

The final `false` does not become a value; it cancels prior ones.

> ⚠ Safety: `SettingsSpan` is fundamentally unsafe: it stores raw pointers and assumes the backing allocations remain alive for the duration of use. The crate's public functions (`get_setting`, `get_settings_list`, etc.) are written to respect these invariants.

### Tuple Formatting: `SettingsTuple`

```rust
pub struct SettingsTuple((String, SettingsValue));
```

`SettingsTuple` provides a `Display` impl that renders as a single‑entry JSON object, escaping the key and delegating to `SettingsValue` for the value. This directly mirrors C++ code that prints tuples as JSON objects for logging or RPC responses.

---

## Reading and Writing Settings Files

The crate supports JSON‑encoded settings files for runtime persistence of `rw_settings`.

### Writing: `write_settings`

```rust
pub fn write_settings(
    path:   &std::path::Path,
    values: &HashMap<String, SettingsValue>,
    errors: &mut Vec<String>,
) -> bool
```

Behavior:

- Constructs a pretty‑printed JSON object with 4‑space indentation.
- Sorts keys lexicographically for deterministic output (aids debugging and tests).
- Serializes each `SettingsValue` via its `Display` implementation.
- On any I/O error, logs and pushes a descriptive message to `errors` and returns `false`.
- On success, flushes the file descriptor, returns `true`.

Example:

```rust
use bitcoin_settings::{SettingsValue, write_settings};
use std::{collections::HashMap, path::PathBuf};

let mut map = HashMap::new();
map.insert("maxconnections".into(), SettingsValue::from(64_i64));
map.insert("server".into(), SettingsValue::from(true));

let mut errors = Vec::new();
let path = PathBuf::from("settings.json");

if !write_settings(&path, &map, &mut errors) {
    eprintln!("failed to write settings: {errors:?}");
}
```

### Reading: `read_settings`

```rust
pub fn read_settings(
    path:   &std::path::Path,
    values: &mut HashMap<String, SettingsValue>,
    errors: &mut Vec<String>,
) -> bool
```

Behavior:

- Clears `values` and `errors` at the beginning.
- Silently succeeds if `path` does not exist.
- Reads the entire file into memory and parses it as a `UniValue`.
- Requires the top‑level value to be a JSON object; otherwise, records an error.
- Populates `values` with `key -> SettingsValue(value)` pairs.
- If a key appears multiple times in the file, insertion triggers a warning and an error entry (the last value wins in the map).
- Returns `true` iff `errors` remains empty.

Example:

```rust
use bitcoin_settings::{read_settings, SettingsValue};
use std::{collections::HashMap, path::Path};

let mut map: HashMap<String, SettingsValue> = HashMap::new();
let mut errors = Vec::new();

if read_settings(Path::new("settings.json"), &mut map, &mut errors) {
    if let Some(v) = map.get("maxconnections") {
        println!("maxconnections = {v}");
    }
} else {
    eprintln!("error(s) reading settings.json: {errors:?}");
}
```

---

## Merging and Querying Settings

The heart of this crate is the replication of Bitcoin's settings merge algorithm, including historical corner cases.

### Low-Level Merge Driver: `merge_settings`

```rust
pub fn merge_settings<F>(
    settings: &Settings,
    section:  &String,
    name:     &String,
    mut fn_:  F,
) where
    F: FnMut(SettingsSpan, Source),
```

`merge_settings` traverses all sources (in canonical precedence order) and calls your callback `fn_` with a `SettingsSpan` and the corresponding `Source` tag whenever that source has data for `name`.

You can implement arbitrary merge policies by interpreting the span:

- `span.empty()` – is there any effective value?
- `span.last_negated()` – does the last value represent a negation?
- `span.negated()` – how many values from the end are negated?
- `span.begin() .. span.end()` – slice of *effective* values (skips negated ones).

This function is the foundation for `get_setting`, `get_settings_list`, and `only_has_default_section_setting`.

### Single Effective Value: `get_setting`

```rust
pub fn get_setting(
    settings:                      &Settings,
    section:                       &str,
    name:                          &str,
    ignore_default_section_config: bool,
    get_chain_name:                bool,
) -> SettingsValue
```

This function computes the **effective single value** for a given option after taking all sources, precedence, negation, and certain legacy quirks into account.

Key behaviors:

- Forced values win outright.
- For non‑config‑file sources, **later** values override earlier ones.
- For config file values (both network‑specific and default sections), precedence is **reversed** — the first assignment wins — except when `get_chain_name == true` (special case for chain selection semantics).
- `ignore_default_section_config == true` suppresses values coming from the default config file section, *except* when the last value is an explicit negation, which is always honored.
- When `get_chain_name == true`, negated command‑line settings are silently skipped (mirroring Core's idiosyncratic treatment of `-testnet`/`-regtest` negations).
- If there is no effective value at all, the function returns JSON `null`.

Example (simple, non‑chain setting):

```rust
use bitcoin_settings::{Settings, SettingsValue, get_setting};

let mut settings = Settings::default();

// Suppose command line had: -maxconnections=20 -maxconnections=40
settings.command_line_options_mut()
    .insert("maxconnections".into(), vec![
        SettingsValue::from(20_i64),
        SettingsValue::from(40_i64),
    ]);

let v = get_setting(&settings, "", "maxconnections", false, false);
println!("effective maxconnections = {v}"); // 40
```

### All Effective Values: `get_settings_list`

```rust
pub fn get_settings_list(
    settings:                      &Settings,
    section:                       &String,
    name:                          &String,
    ignore_default_section_config: bool,
) -> Vec<SettingsValue>
```

This function returns **all effective values** in their final merge order, not just the last one. It also implements several compatibility quirks:

- Negated values (boolean `false`) terminate effective values at their position and mark lower‑priority sources as ignored.
- However, config file values can temporarily resurrect as **zombie values** if a later command‑line negation is followed by a non‑negated value; in that particular case, config file values become visible again but earlier command‑line values remain ignored.
- Any `SettingsValue` containing a JSON array is **flattened** into multiple scalar values in the result.

This is primarily useful for options that are conceptually lists (e.g. addnodes, whitelists).

Example:

```rust
use bitcoin_settings::{Settings, SettingsValue, get_settings_list, sv_json};

let mut settings = Settings::default();

// Config file: addnode=["1.2.3.4", "5.6.7.8"]
settings.ro_config_mut()
    .entry("".into())               // default section
    .or_default()
    .insert("addnode".into(), vec![sv_json("[\"1.2.3.4\", \"5.6.7.8\"]")]);

let values = get_settings_list(&settings, &"".to_string(), &"addnode".to_string(), false);
for v in values {
    println!("addnode => {v}");
}
// prints each IP as separate SettingsValue
```

### Detecting Default-Section-Only Values

```rust
pub fn only_has_default_section_setting(
    settings: &Settings,
    section:  &str,
    name:     &str,
) -> bool
```

This helper answers the question:

> "Is this option set in the default config file section and *not* overridden by any higher‑priority setting (command line, RW settings, or network section)?"

It is used to produce warnings about values that are present but effectively ignored, so that users understand why their `bitcoin.conf` changes had no effect.

Example:

```rust
use bitcoin_settings::{Settings, SettingsValue, only_has_default_section_setting};

let mut settings = Settings::default();

// Default section: txindex=1
settings.ro_config_mut()
    .entry("".into())
    .or_default()
    .insert("txindex".into(), vec![SettingsValue::from(true)]);

let ignored = only_has_default_section_setting(&settings, "main", "txindex");
println!("txindex default-only? {ignored}"); // true if not overridden
```

---

## Utility: JSON Literal Construction

### `sv_json`

```rust
pub fn sv_json(j: &str) -> SettingsValue
```

Parses a JSON literal into a `SettingsValue` using `UniValue::read`. This is primarily intended for tests and concise construction of complex values (arrays, objects) without hand‑assembling `UniValue` trees.

- Panics if `j` is not valid JSON from `UniValue`'s perspective.

Example:

```rust
use bitcoin_settings::sv_json;

let v = sv_json("[1, 2, 3]");
println!("v = {v}"); // prints the JSON array
```

---

## Safety and Correctness Considerations

- `SettingsSpan` internally uses raw pointers. All public APIs consuming `SettingsSpan` are carefully written to keep referents alive for the duration of the call. Do not store `SettingsSpan` beyond the immediate callback in `merge_settings`.
- Comparison (`PartialEq`) for `SettingsValue` is performed by serializing both sides and comparing the resulting strings. This matches Bitcoin Core's behavior but is more expensive than structural comparison. Prefer identity or pointer comparison if you maintain external caches.
- JSON serialization and deserialization rely on `UniValue`. For strict compatibility with Bitcoin Core, ensure you build `UniValue` with matching parser and writer options.

---

## Intended Use Cases

This crate is suitable for:

- Alternative Bitcoin nodes that want **drop‑in‑compatible settings resolution**.
- External tooling that interprets or manipulates Core's JSON settings files.
- Integration tests verifying that new logic preserves or only intentionally diverges from Core's historical behavior.
- Research or refactoring efforts that seek to decouple user‑facing configuration semantics from a particular implementation language.

It is intentionally **not** a general-purpose configuration framework; its design is constrained by compatibility with an existing, widely deployed implementation.

---

## License and Repository

- License: MIT
- Repository: <https://github.com/klebs6/bitcoin-rs>

Within that repository, this crate lives under the `bitcoin-settings` package. Consult the repository for examples, tests, and integration context.
