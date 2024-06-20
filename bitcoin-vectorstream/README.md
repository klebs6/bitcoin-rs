# bitcoin-vectorstream

`bitcoin-vectorstream` is a Rust crate providing a minimal stream for reading from an existing vector by reference. It is designed for scenarios where efficient deserialization from a byte vector is required.

## Overview

This crate defines a single primary type:

- `VectorReader`: A minimal stream for reading from an existing vector by reference.

### `VectorReader`

The `VectorReader` structure allows reading data from a `Vec<u8>` starting from a specified position. It supports deserialization and provides methods to query the current read position and the remaining size.

#### Example Usage

```rust
use bitcoin_vectorstream::VectorReader;
use std::sync::Arc;

let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
let data_arc = Arc::new(data);

let mut reader = VectorReader::new(0, 0, &data_arc, 0);
let mut buffer = [0u8; 5];
reader.read(buffer.as_mut_ptr(), buffer.len());

assert_eq!(&buffer, &[1, 2, 3, 4, 5]);

```

## Features

- Minimal stream for reading from an existing Vec<u8> by reference.

- Supports deserialization.

- Methods to query the serialization type and version.

- Methods to query the current read position and the remaining size.

## License

This project is licensed under the MIT License.
