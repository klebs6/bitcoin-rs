# bitcoin-cfg

Build-script configuration shared by the `bitcoin-rs` workspace. The public
entry point is `setup()`, which emits Cargo configuration directives using
the private helpers in [src/lib.rs](src/lib.rs).

Call `bitcoin_cfg::setup()` from a consuming crate's `build.rs`. It installs
the currently declared platform aliases and the fixed secp256k1 configuration
selected by that source. It does not export a `cfg_alias!` macro or a runtime
platform-detection API. Inspect the actual alias predicates before depending
on them; this preparation does not change their spellings or semantics.

The [manifest](Cargo.toml) records its dependencies. Compatibility of
the complete LevelDB package closure is qualified separately from this small
build helper.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [License](LICENSE) · [Upstream notices](NOTICE)
