# bitcoin-argsman

Robust argument, configuration, and network-parameter management for Bitcoin-style Rust applications.

This crate is an idiomatic Rust port of Bitcoin Core's `ArgsManager` subsystem. It provides a high-fidelity, network-aware configuration layer for CLIs and daemons that follow Bitcoin's conventions:

- `-foo`, `-foo=bar`, `-nofoo`, and double-negatives (`-nofoo=0`) with Bitcoin-compatible semantics
- Layered configuration resolution: forced settings, command line, network-scoped config, default section
- Chain selection and base parameters (`main`, `testnet`, `signet`, `regtest`)
- Network-specific datadirs and blocks directories
- Config-file parsing with sections, `includeconf`, and error reporting
- Automatic help message generation grouped by option category

The design goal is behavioral equivalence with Bitcoin Core, enabling Rust-based tooling to interoperate with existing `bitcoin.conf` files and operational practices.

---

## Features

- **Typed argument descriptors** via `ArgDescriptor` and `ArgsManagerFlags`
- **Global argument manager** (`G_ARGS`) mirroring Bitcoin Core's singleton
- **Config-file support** with sections, `includeconf`, and detection of unrecognized sections
- **Per-network datadirs** and chain parameters via `BaseChainParams`
- **Sensitive argument redaction** in logs (`SENSITIVE` flag)
- **Command-style arguments** and subcommands (`COMMAND` flag, `add_command`/`get_command`)
- **Help text formatting** with hard-wrapped paragraphs and grouped sections
- **Settings file** (dynamic rw-settings) with atomic write / rename semantics

If you already know Bitcoin Core's `ArgsManager`, this crate is intentionally familiar: type names, flag behavior, and configuration precedence are kept aligned so that operational reasoning transfers directly.

---

## Installation

In your `Cargo.toml`:

```toml
[dependencies]
bitcoin-argsman = "0.1.22"
```

Requires Rust **1.56+** (edition 2021).

---

## Core Types and Concepts

### `OptionsCategory`

Options are grouped into semantic categories, used to structure help output:

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum OptionsCategory {
    OPTIONS,
    CONNECTION,
    WALLET,
    WALLET_DEBUG_TEST,
    ZMQ,
    DEBUG_TEST,
    CHAINPARAMS,
    NODE_RELAY,
    BLOCK_CREATION,
    RPC,
    GUI,
    COMMANDS,
    REGISTER_COMMANDS,
    HIDDEN,
}
```

Hidden options are parsed but not printed in normal help output.

### `ArgsManagerFlags`

Flags describe admissible value types, security properties, and special behavior:

```rust
bitflags::bitflags! {
    pub struct ArgsManagerFlags: u32 {
        const ALLOW_BOOL   = 0x01;
        const ALLOW_INT    = 0x02;
        const ALLOW_STRING = 0x04;
        const ALLOW_ANY    = Self::ALLOW_BOOL.bits
                            | Self::ALLOW_INT.bits
                            | Self::ALLOW_STRING.bits;

        const DEBUG_ONLY   = 0x100;
        const NETWORK_ONLY = 0x200; // values must not leak across networks
        const SENSITIVE    = 0x400; // masked in logs
        const COMMAND      = 0x800; // command-style (non -foo) arguments
    }
}
```

Key behavioral guarantees:

- `ALLOW_BOOL` governs legality of `-nofoo` negation.
- `NETWORK_ONLY` marks options whose values are scoped to a specific chain (`main`, `testnet`, etc.) and must not be accidentally shared via an unqualified config section.
- `SENSITIVE` options are printed as `****` in logs.

### `ArgDescriptor`

`ArgDescriptor` is the declarative definition of an option:

```rust
pub struct ArgDescriptor {
    pub name:     &'static str,   // e.g. "-rpcuser=<user>"
    pub help:     String,         // human-readable text
    pub flags:    ArgsManagerFlags,
    pub category: OptionsCategory,
}
```

You register descriptors on `ArgsManager` (or `ArgsManagerInner`) to make options available.

### `ArgsManager` and `G_ARGS`

`ArgsManager` is the main façade; internally it wraps `ArgsManagerInner` with `Arc<Mutex<_>>` and exposes a synchronous API appropriate for startup configuration parsing:

```rust
#[derive(Default)]
pub struct ArgsManager {
    cs_args: Arc<Mutex<ArgsManagerInner>>, // configuration state
}

lazy_static::lazy_static! {
    pub static ref G_ARGS: Mutex<ArgsManager> = Mutex::new(ArgsManager::default());
}
```

Both `ArgsManager` and `ArgsManagerInner` provide methods for:

- Defining available arguments (`add_arg`, `add_command`, `add_hidden_args`, `setup_cli_args`, `setup_chain_params_base_options`)
- Parsing command line and config (`parse_parameters`, `read_config_files`, `read_config_stream`)
- Querying values (`get_arg`, `get_int_arg`, `get_bool_arg`, `get_args`, `get_setting`, `get_settings_list`)
- Accessing derived paths (`get_data_dir_base`, `get_data_dir_net`, `get_blocks_dir_path`, `get_settings_path`)
- Producing diagnostics (`get_help_message`, `log_args`, `get_unrecognized_sections`, `get_unsuitable_section_only_args`)

All public methods on `ArgsManager` are thin, inlined lock-and-delegate wrappers around the inner type.

### Network and Chain Parameters

`BaseChainParams` models base properties of the selected Bitcoin network:

```rust
#[derive(Default)]
pub struct BaseChainParams {
    rpc_port:                  u16,
    onion_service_target_port: u16,
    str_data_dir:              String, // relative chain datadir ("", "testnet3", ...)
}
```

Key functions and statics:

- `create_base_chain_params(chain: &str) -> Result<Box<BaseChainParams>, StdException>`
- `select_base_params(chain: &str)` – updates the global `BaseChainParams` and network selection in `G_ARGS`.
- Statics for the canonical networks:
  - `DEFAULT_BASE_PARAMS` (main)
  - `TESTNET_BASE_PARAMS`
  - `SIGNET_BASE_PARAMS`
  - `REGTEST_BASE_PARAMS`

`ArgsManagerInner::get_chain_name` inspects `-regtest`, `-signet`, `-testnet`, and `-chain` flags to determine the active chain, enforcing mutual exclusivity.

### Datadirs and Path Semantics

The crate reproduces Bitcoin Core's datadir rules across platforms:

- Windows: `C:\Users\<Username>\AppData\Roaming\Bitcoin`
- macOS: `~/Library/Application Support/Bitcoin`
- Unix-like: `~/.bitcoin`

Functions:

- `get_default_data_dir() -> PathBuf`
- `get_home_dir() -> PathBuf`
- `strip_redundant_last_elements_of_path(path: &mut PathBuf)` – normalizes trailing `.` components; ensures idempotence via `same-file` checks.
- `abs_path_for_config_val(path: &Path, net_specific: Option<bool>) -> PathBuf` – resolves relative paths against network-specific or base datadir.
- `ArgsManagerInner::get_data_dir_base` / `get_data_dir_net` / `get_data_dir(net_specific: bool)` – create the datadir and `wallets/` subdir on first use, cache the result, and respect a user-specified `-datadir`.
- `ArgsManagerInner::get_blocks_dir_path()` – detects optional `-blocksdir`, falls back to `datadir/<chain>/blocks`, ensures the directory exists, and caches the result.

### Settings Resolution Model

The crate implements a deterministic, layered settings model analogous to a constrained lattice:

1. **Command line options** – most recent occurrence wins (`self.settings.command_line_options()`), including `force_set_arg` injections.
2. **Forced settings** – via `force_set_arg`, held in `settings.forced_settings()`. They also reflect into the command-line layer to unify resolution semantics.
3. **Config file (ro_config)** – potentially sectioned by network and by default section.
4. **Dynamic settings file (rw_settings)** – read and written by `init_settings`, subject to argument recognition.
5. **Defaults** – provided by callers of `get_*_arg` or by the absence of a setting.

Utility methods:

- `get_setting(&self, arg: &str) -> SettingsValue` – returns a `UniValue`-backed wrapper which may be null, bool, string, or numeric, following the Bitcoin Core precedence rules.
- `get_settings_list(&self, arg: &str) -> Vec<SettingsValue>` – returns all values (e.g. for options that may appear multiple times).
- `is_arg_set`, `is_arg_negated` – semantic presence and negation checks.
- `use_default_section(&self, arg: &str) -> bool` – determines whether default-section config applies, depending on network and `NETWORK_ONLY` marking.

This model ensures that configuration from `bitcoin.conf`, dynamic settings, and CLI overrides are combined safely and predictably.

### Boolean Semantics and Parsing Logic

The crate mirrors Bitcoin Core's nuanced handling of boolean and numeric options:

- `interpret_bool(str_value: &str) -> bool`:
  - Empty string => `true`
  - Non-empty string => parsed via `locale_independent_atoi::<i32>`; value `!= 0` means `true`.
  - Non-numeric strings produce `0` and therefore `false`.
- `interpret_option(section: &mut String, key: &mut String, value: &String) -> SettingsValue`:
  - Recognizes `no` prefix: `-nofoo=1` becomes `foo=false`; `-nofoo=0` becomes `foo=true` and emits a warning.
  - Parses dotted keys like `regtest.foo` into `section="regtest"`, `key="foo"`.

This behavior is critical for backward compatibility with existing deployments that rely on Core's somewhat intricate legacy semantics.

---

## Help and Documentation Generation

### Paragraph Formatting

`format_paragraph` implements a simple word-wrapping algorithm with explicit newlines treated as hard breaks:

```rust
pub fn format_paragraph(
    in_: &str,
    width: Option<usize>,
    indent: Option<usize>,
) -> Result<String, FormatParagraphError> { /* ... */ }
```

Behavior:

- Default `width` is 79 columns.
- Default `indent` is 0 spaces for wrapped lines.
- Each line is broken at the last space or tab before the width, or at `width` if no such break exists.
- Explicit `\n` in the input reset the line-length budget and are preserved as hard breaks.

Errors are currently represented by `FormatParagraphError`, but for the provided implementation, all fallible paths are internal IO-like operations (e.g. via intermediate writers) and typical usage should just observe `Ok`.

### Option Help Formatting

`help_message_opt` formats an option line and its description for human-readable CLI output:

```rust
pub fn help_message_opt(option: &str, message: &str) -> String {
    let paragraph = format_paragraph(
        message,
        Some(SCREEN_WIDTH - MSG_INDENT),
        Some(MSG_INDENT),
    );

    match paragraph {
        Ok(paragraph) => {
            format!(
                "{}{}\n{}{}\n\n",
                " ".repeat(OPT_INDENT),
                option,
                " ".repeat(MSG_INDENT),
                paragraph,
            )
        }
        Err(e) => panic!("format_paragraph failed with error: {:?}", e),
    }
}
```

`ArgsManagerInner::get_help_message` iterates all `available_args`, grouped by `OptionsCategory`, and produces the full help string. `DEBUG_ONLY` arguments only appear when `-help-debug` is set.

This allows you to build self-documenting CLIs with a single source of truth (your `ArgDescriptor` definitions).

---

## Config Files and Sections

### Parsing `bitcoin.conf`-style Files

Config parsing is handled by `get_config_options` and `ArgsManagerInner::read_config_stream` / `read_config_files`.

Supported syntax:

- `name=value` key-value pairs
- Section headers: `[section]`, where `section` typically matches a chain (e.g. `regtest`, `signet`, `testnet`, `main`)
- `#` comments (with special treatment if they interact with `rpcpassword`)
- Whitespace trimming around names and values

Key behaviors:

- Options **must not** be prefixed with `-` in the config file; that is an error (`-foo=bar` is invalid; use `foo=bar`).
- When a section is seen, it is pushed to `config_sections` and used as a prefix for subsequent options (`[regtest] foo=1` becomes key `regtest.foo`).
- Unknown options can be ignored or treated as errors depending on `ignore_invalid_keys`.
- `includeconf` is supported in config files but **forbidden** at the command line except as `-noincludeconf`. Recursive `includeconf` from included files is warned about and ignored.

### Recognized Sections

`AVAILABLE_SECTIONS` defines valid chain sections:

```rust
lazy_static::lazy_static! {
    static ref AVAILABLE_SECTIONS: std::collections::HashSet<String> = hashset! {
        base_chain_params::REGTEST.to_string(),
        base_chain_params::SIGNET.to_string(),
        base_chain_params::TESTNET.to_string(),
        base_chain_params::MAIN.to_string(),
    };
}
```

`ArgsManagerInner::get_unrecognized_sections` returns the list of `SectionInfo` (name, file, line) that do *not* appear in `AVAILABLE_SECTIONS`. This is valuable for catching mis-typed chain names or environment-specific configuration mistakes.

### Network-Only Arguments

Some options, if shared accidentally between mainnet and test/test-like networks, can cause operational hazards. `ArgsManagerFlags::NETWORK_ONLY` is used to mark such options, and `ArgsManager::get_unsuitable_section_only_args` computes which of them are only configured in the default section for a non-main network.

This is a static analysis step at startup that allows you to warn operators when a `NETWORK_ONLY` option should be moved into a specific `[regtest]`/`[test]`/`[signet]` section.

---

## Commands and Bitcoin-style Pseudo-Commands

There are two conceptually separate command styles supported:

1. **Registered commands** via `add_command(cmd, help)` with `ArgsManagerFlags::COMMAND`.
2. **Pseudo-commands** used for `bitcoin-tx`-like tools (e.g. `delin=N`, `outaddr=VALUE:ADDRESS`), which are registered via `add_arg` but placed in the `COMMANDS` or `REGISTER_COMMANDS` categories.

`ArgsManagerInner::parse_parameters` enforces the following invariant:

- Once a non-dash argument (not starting with `-`) is seen:
  - If `accept_any_command` is `false`, that argument must correspond to a known command (have `COMMAND` flag) or parsing fails.
  - That first non-dash argument becomes the `command` part of `ArgsManagerCommand`, with all remaining arguments collected into `args`.

`get_command()` exposes this structured view:

```rust
pub struct ArgsManagerCommand {
    pub command: Option<String>,
    pub args:    Vec<String>,
}
```

If no command is present, `get_command()` returns `None`.

`setup_bitcoin_tx_args` shows a canonical usage pattern for a `bitcoin-tx`-analog in Rust: it registers a rich set of transaction-editing pseudo-commands and necessary chain-parameter options.

---

## Settings File (Dynamic RW Settings)

Beyond statically-parsed config files and CLI arguments, the crate supports a *dynamic settings file* whose path is derived from `-settings` and the network datadir. This mechanism is useful for GUIs or long-running processes that need to persist configuration changes without rewriting the entire `bitcoin.conf`.

Key methods:

- `get_settings_path(&self, filepath: Option<&mut Box<Path>>, temp: Option<bool>) -> bool`
  - Honors `-nosettings` (by checking if `-settings` is negated).
  - Derives the final path under `get_data_dir_net()` and resolves both the canonical and temporary (`.tmp`) names.
- `init_settings(&mut self, error: &mut String) -> Result<(), String>`
  - Reads the settings file via `read_settings_file`.
  - Then writes it back via `write_settings_file`, gathering and surfacing errors.
- `read_settings_file(&mut self, errors: Option<&mut Vec<String>>) -> bool`
  - Clears existing `rw_settings`.
  - Reads into a map and validates that keys correspond to known arguments; unknown keys are logged.
- `write_settings_file(&self, errors: Option<&mut Vec<String>>) -> Result<bool, StdException>`
  - Writes to a temporary file and atomically renames it into place using `rename_over`.

This is designed to be safe under concurrent readers and robust against intermediate failures.

---

## Usage Examples

### 1. Basic CLI Setup and Parsing

```rust
use bitcoin_argsman::{ArgsManager, ArgsManagerFlags, OptionsCategory, ArgDescriptor};

fn build_args() -> ArgsManager {
    let mut am = ArgsManager::default();

    // Standard help options (-?, -h, -help, -help-debug)
    am.setup_help_options();

    // Add a custom boolean option: -verbose / -noverbose
    let verbose_desc = ArgDescriptor {
        name:     "-verbose",
        help:     "Enable verbose logging".to_string(),
        flags:    ArgsManagerFlags::ALLOW_BOOL,
        category: OptionsCategory::OPTIONS,
    };
    am.add_arg(&verbose_desc);

    am
}

fn main() {
    let mut argsman = build_args();

    let argv: Vec<String> = std::env::args().collect();
    let mut error = String::new();

    if !argsman.parse_parameters(&argv, &mut error) {
        eprintln!("error: {}", error);
        std::process::exit(1);
    }

    if argsman.help_requested() {
        println!("{}", argsman.get_help_message());
        return;
    }

    let verbose = argsman.get_bool_arg("-verbose", false);
    println!("verbose = {}", verbose);
}
```

### 2. Chain Selection and Network-Specific Paths

```rust
use bitcoin_argsman::{select_base_params, G_ARGS};

fn main() {
    // Example: assume CLI parsing has already populated G_ARGS.
    {
        // Determine network from arguments and configure global base params
        let mut guard = G_ARGS.lock().unwrap();
        let mut error = String::new();
        let argv: Vec<String> = std::env::args().collect();
        guard.parse_parameters(&argv, &mut error);
        let chain_name = guard.get_chain_name().expect("invalid chain");
        select_base_params(&chain_name);
    }

    // Later, obtain a network-specific datadir
    let datadir = G_ARGS.lock().unwrap().get_data_dir_net();
    println!("network datadir: {}", datadir.display());
}
```

### 3. Logging Effective Configuration

```rust
use bitcoin_argsman::G_ARGS;

fn log_effective_config() {
    let guard = G_ARGS.lock().unwrap();
    guard.log_args(); // will mask SENSITIVE args
}
```

### 4. Parsing a Custom Config Stream

```rust
use std::io::Cursor;
use std::io::BufReader;
use bitcoin_argsman::ArgsManager;

fn load_inline_conf(am: &mut ArgsManager) {
    let config_text = r#"
[regtest]
rpcport=18443

[testnet]
rpcport=18332
"#;

    let cursor = Cursor::new(config_text.as_bytes());
    let mut reader = BufReader::new(cursor);
    let mut error = String::new();

    if !am.read_config_stream(&mut reader, "<inline>", &mut error, Some(false)) {
        panic!("config parse error: {}", error);
    }
}
```

---

## Error Handling

`format_paragraph` uses a dedicated error type:

```rust
#[derive(thiserror::Error, Debug)]
pub enum FormatParagraphError {
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    IntoInnerError(std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>),
}
```

Most other operations in this crate either:

- Return `bool` with an out-parameter `&mut String` for error messages (e.g. parsing functions), or
- Return `Result<_, StdException>` for failures modeled on C++ exceptions, or
- Panic in invariants expected never to fail (e.g. internal IO in help formatting).

In a production integration you are expected to:

- Treat a `false` return with a non-empty error string as fatal at startup.
- Use `get_unsuitable_section_only_args` and `get_unrecognized_sections` for soft warnings.

---

## Mathematical / Algorithmic Notes

The crate is primarily concerned with configuration logic rather than heavy numerical computation. Still, several minor algorithmic properties are worth noting:

- **Word-wrapping**: `format_paragraph` performs a greedy line-breaking algorithm. Its time complexity is linear in the input length, with each character visited at most a constant number of times.
- **Configuration section validation**: `get_unrecognized_sections` filters section appearances against a hash set of known names; the complexity is linear in the number of seen sections, with constant-time expected lookups.
- **Settings precedence**: `get_setting` and `get_settings_list` are O(1) expected time due to direct hash-map lookups.
- **Datadir normalization**: `strip_redundant_last_elements_of_path` iteratively removes trailing `.` segments and validates the path equality via `same-file`; this is linear in the number of path segments, with I/O-bound checks.

These properties make the crate suitable for early-initialization usage even under large and complex configurations.

---

## Repository, License, and Authorship

- **Repository**: <https://github.com/klebs6/bitcoin-rs>
- **Crate**: `bitcoin-argsman`
- **Edition**: Rust 2021
- **License**: MIT
- **Authors**: `klebs <none>`

The crate is part of a broader effort to provide a Rust reimplementation of Bitcoin Core components. Behavioral compatibility with Bitcoin Core is a guiding constraint; when in doubt, cross-check against the upstream C++ `ArgsManager`.

---

## Caveats

- The public interface may expose types (`Settings`, `SettingsSpan`, `UniValue`, `StdException`, etc.) whose definitions are in sibling modules or crates within `bitcoin-rs`. Consult the repository for their details.
- I/O paths, error messages, and some internal invariants are designed to match Core's behavior and tests; altering them may break cross-compatibility.
- This README is AI-generated and may lag behind the latest code; when precision is critical, inspect the actual source implementation.
