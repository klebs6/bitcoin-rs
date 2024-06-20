# bitcoin-u160

`bitcoin-u160` is a Rust crate providing a 160-bit opaque blob type for working with Bitcoin data structures.

## Overview

This crate defines a single primary type:

- `u160`: An opaque 160-bit blob.

### `u160`

The `u160` type represents a 160-bit blob with no integer operations. It is primarily used for scenarios where a fixed-size 160-bit container is needed without arithmetic capabilities.

```rust
#[derive(Clone,Default,PartialEq,Eq,Hash)]
pub struct u160 {
    pub blob: BaseBlob<160>,
}

impl From<&Vec<u8>> for u160 {
    fn from(vch: &Vec<u8>) -> Self {
        todo!();
    }
}
```

# Example Usage

```rust
use bitcoin_u160::u160;

// Creating a u160 instance from a vector of bytes
let bytes: Vec<u8> = vec![0; 20];
let value = u160::from(&bytes);
```

# Features

- u160 type for opaque 160-bit blobs.

- Conversion from a vector of bytes to u160.

# License

This project is licensed under the MIT License.
