# bitcoin-autofile

`bitcoin-autofile` is a Rust crate providing a non-refcounted RAII wrapper for `FILE*`. It is designed to manage the lifecycle of file pointers, ensuring that files are automatically closed when they go out of scope.

## Overview

This crate defines a structure:

- `AutoFile`: An RAII wrapper for `FILE*` that automatically closes the file when it goes out of scope.

### `AutoFile`

The `AutoFile` structure manages a `FILE*` pointer, ensuring proper cleanup when the object is destroyed. It provides methods for file operations and ownership management.

## Example Usage

```rust
use bitcoin_autofile::AutoFile;
use std::ptr;

let file_ptr: *mut libc::FILE = unsafe { libc::fopen(b"example.txt\0".as_ptr() as *const libc::c_char, b"w+\0".as_ptr() as *const libc::c_char) };
if file_ptr.is_null() {
    panic!("Failed to open file");
}

let mut autofile = AutoFile::new(file_ptr, 0, 0);
autofile.write(b"Hello, World!\0".as_ptr(), 13);

let file_ptr = autofile.release();  // Transfer ownership
unsafe {
    libc::fclose(file_ptr);
}

```

## Features

- RAII wrapper for FILE* to ensure automatic file closure.

- Methods to release ownership or get the raw FILE* without releasing ownership.

- Read, write, and ignore methods for file operations.
