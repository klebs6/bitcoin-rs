# bitcoin-cfg

Bitcoin-cfg is a Rust crate that provides
functionality for aliasing multiple `cfg` gates
into a single alias. This can be useful in
scenarios where multiple `cfg` gates need to be
repeated across multiple code blocks or projects.

The crate includes a `cfg_alias!` macro that
allows developers to create a new `cfg` alias by
combining existing `cfg` gates. The macro takes
two arguments: the name of the new alias, and
a list of `cfg` gates to combine.

For example, suppose a developer needs to specify
that a particular code block should only be
compiled on Linux x86_64 systems with the "foo"
feature enabled. They could write:

```rust
#[cfg(all(target_os = "linux", target_arch = "x86_64", feature = "foo"))]
fn my_function() { /* ... */ }
```

However, if this `cfg` gate needs to be repeated
across multiple code blocks or projects, it can
become cumbersome to write it out each time. With
bitcoin-cfg, the developer can create a new `cfg`
alias using the `cfg_alias!` macro:

```rust
bitcoin_cfg::cfg_alias! {
    linux_x86_64_foo,
    all(target_os = "linux", target_arch = "x86_64", feature = "foo")
}
```

Then, in their code blocks, the developer can use
this new alias instead of the original `cfg` gate:

```rust
#[cfg(linux_x86_64_foo)]
fn my_function() { /* ... */ }
```

This makes the code more concise and easier to
read, and reduces the likelihood of errors when
copying and pasting `cfg` gates across code
blocks.

Overall, bitcoin-cfg is a useful crate for
simplifying the use of `cfg` gates in Rust code by
allowing developers to create aliases for commonly
used combinations of `cfg` gates.

4> exactly.  there are several of these which are
useful in the bitcoin rust ecosystem, and they are
held in this crate

# bitcoin-cfg

Bitcoin-cfg is a Rust crate that provides
a collection of aliases for commonly used `cfg`
gates in the Bitcoin Rust ecosystem. It allows
developers to create concise and readable `cfg`
blocks by combining multiple `cfg` gates into
a single alias.

The crate includes several predefined aliases for
commonly used combinations of `cfg` gates. These
aliases can be used directly in `cfg` blocks
without having to define them manually.

For example, `bitcoin_cfg::target_os_linux` can be
used to specify that a code block should only be
compiled on Linux systems, while
`bitcoin_cfg::feature_serde` can be used to
specify that a code block requires the "serde"
feature to be enabled.

In addition to the predefined aliases, bitcoin-cfg
allows developers to create their own aliases
using the `cfg_alias!` macro. This can be useful
in scenarios where multiple `cfg` gates need to be
repeated across multiple code blocks or projects.

Overall, bitcoin-cfg is a useful crate for
simplifying the use of `cfg` gates in Rust code in
the Bitcoin ecosystem by providing a collection of
aliases for commonly used combinations of `cfg`
gates.
