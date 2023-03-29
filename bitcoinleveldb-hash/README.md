## bitcoinleveldb-hash

This Rust crate is a direct translation of the
`hash.h` and `hash.cc` files from the Bitcoin Core
codebase. The code provides a simple hash function
that is used for internal data structures in
LevelDB, which is used as the database backend for
Bitcoin Core.

### Notice

This crate is part of a direct translation from
C++ to Rust of the Bitcoin Core. As such, some of
the function bodies may still be in the process of
translation. Please note that this system will
become testable once the translation process is
complete.

### Tokens

- `HASH`: A dummy struct used in a test function.

- `hash_signed_unsigned_issue()`: A test function
  that tests the `hash()` function with various
  inputs.

- `hash(data: *const u8, n: usize, seed: u32) ->
  u32`: A simple hash function that is used for
  internal data structures in LevelDB. The
  function takes a pointer to an array of bytes
  (`data`), the number of bytes to hash (`n`), and
  an initial seed value (`seed`) as inputs, and
  returns a 32-bit hash value. The hash function
  is similar to MurmurHash, with a fixed `m` and
  `r` value. The hash function picks up four bytes
  at a time until the last remaining bytes are
  less than four, and then switches to handle the
  remaining bytes using a switch-case statement.

### Usage

This crate is primarily intended to be used as
a dependency in other crates that require the hash
function for internal data structures in
LevelDB. To use the crate in your Rust project,
add the following line to your `Cargo.toml` file:

```toml
[dependencies]
bitcoinleveldb-hash = "0.1.0"
```

And then import the crate in your Rust code with:

```rust
extern crate bitcoinleveldb_hash;
```

Once the crate is imported, you can use the
`hash()` function as follows:

```rust
use bitcoinleveldb_hash::hash;

fn main() {
    let data: [u8; 3] = [0xe2, 0x99, 0xa5];
    let seed: u32 = 0xbc9f1d34;
    let n: usize = data.len();
    let h = hash(data.as_ptr(), n, seed);
    println!("Hash value: {}", h);
}
```
