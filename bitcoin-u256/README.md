# bitcoin-u256

`bitcoin-u256` is a Rust crate providing a 256-bit opaque blob type and a 256-bit unsigned big integer type for working with Bitcoin data structures.

## Overview

This crate defines two primary types:

- `u256`: An opaque 256-bit blob.
- `ArithU256`: A 256-bit unsigned big integer with arithmetic operations.

### `u256`

The `u256` type represents a 256-bit blob with no integer operations. It is primarily used for scenarios where a fixed-size 256-bit container is needed without arithmetic capabilities.

```rust
#[derive(Default,Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct u256 {
    pub blob: BaseBlob<256>,
}

impl u256 {
    pub fn is_null(&self) -> bool;
    pub fn set_null(&mut self);
    pub fn to_string(&self) -> String;
}
```

### `ArithU256`

The `ArithU256` type represents a 256-bit unsigned big integer that supports arithmetic operations.

```rust
#[derive(Default,Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct ArithU256 {
    base: BaseUInt<256>,
}

impl ArithU256 {
    pub fn set_compact(&mut self, n_compact: u32, pf_negative: *mut bool, pf_overflow: *mut bool) -> &mut ArithU256;
    pub fn get_compact(&self, negative: Option<bool>) -> u32;
}
```

### Example Usage

```rust
use bitcoin_u256::{u256, ArithU256};

// Working with u256
let mut value = u256::default();
value.set_null();
assert!(value.is_null());

// Working with ArithU256
let mut value = ArithU256::default();
value.set_compact(0x1d00ffff, std::ptr::null_mut(), std::ptr::null_mut());
let compact = value.get_compact(None);
```


## Features

- u256 type for opaque 256-bit blobs.

- ArithU256 type for 256-bit unsigned big integers with arithmetic operations.

- Conversion functions between u256 and ArithU256.

## License

This project is licensed under the MIT License.
