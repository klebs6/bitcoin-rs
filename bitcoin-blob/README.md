# bitcoin-blob

`bitcoin-blob` is a Rust crate providing a template base class for fixed-sized opaque blobs. It is designed for scenarios where a fixed-size container is needed without arithmetic capabilities.

## Overview

This crate defines a generic type:

- `BaseBlob`: A template base class for fixed-sized opaque blobs of a specified bit size.

### `BaseBlob`

The `BaseBlob` type represents a fixed-sized opaque blob defined by the number of bits. It provides basic functionalities like comparison, serialization, and conversion.

```rust
pub const fn base_blob_width<const BITS: usize>() -> usize {
    BITS / 8
}

/**
  | Template base class for fixed-sized
  | opaque blobs.
  |
  */
#[derive(Clone,Debug,Hash)]
pub struct BaseBlob<const BITS: usize> 
where [u8; base_blob_width::<BITS>()]: {
    pub data: [u8; base_blob_width::<BITS>()],
}

```

# Example Usage

```rust
use bitcoin_blob::BaseBlob;

// Creating a BaseBlob instance with 256 bits
let blob_256 = BaseBlob::<256>::default();

// Check if the blob is null
assert!(blob_256.is_null());

// Set the blob to null
blob_256.set_null();
```

# Features

- Generic BaseBlob type for opaque blobs of any bit size.

- Methods for checking nullity, setting null, and comparison.

- Serialization and deserialization support.

- Conversion from byte vectors and individual bytes.

# License

This project is licensed under the MIT License.
