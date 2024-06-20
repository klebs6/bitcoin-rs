# bitcoin-bufferedfile

`bitcoin-bufferedfile` is a Rust crate providing a non-refcounted RAII wrapper around a `FILE*` that implements a ring buffer for deserialization. It ensures the ability to rewind a given number of bytes and guarantees proper file closure when the object goes out of scope.

## Overview

This crate defines a single primary type:

- `BufferedFile`: An RAII wrapper around a `FILE*` that implements a ring buffer to deserialize from.

### `BufferedFile`

The `BufferedFile` structure manages a `FILE*` pointer with a ring buffer, ensuring proper cleanup when the object is destroyed. It provides methods for reading, rewinding, and managing the read limit.

#### Example Usage

```rust
use bitcoin_bufferedfile::BufferedFile;
use std::ptr;

let file_ptr: *mut libc::FILE = unsafe { libc::fopen(b"example.dat\0".as_ptr() as *const libc::c_char, b"rb\0".as_ptr() as *const libc::c_char) };
if file_ptr.is_null() {
    panic!("Failed to open file");
}

let mut buffered_file = BufferedFile::new(file_ptr, 4096, 128, 0, 0);
let mut buffer = [0u8; 1024];
buffered_file.read(buffer.as_mut_ptr(), buffer.len());
buffered_file.set_pos(0);
buffered_file.set_limit(Some(2048));
buffered_file.find_byte(0x0A);

let file_ptr = buffered_file.release();  // Transfer ownership
unsafe {
    libc::fclose(file_ptr);
}
```

## Features

- RAII wrapper for FILE* with automatic file closure.

- Ring buffer implementation for efficient deserialization.

- Methods to rewind, set read limits, and find specific bytes.

- Ensures the ability to rewind a specified number of bytes.
