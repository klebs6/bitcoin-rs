# bitcoin-bitstream

bitcoin-bitstream provides a set of utilities for reading and writing bitstreams. This crate includes BitStreamReader, BitStreamWriter, and additional utilities for handling bit-level data operations.

## Structures

`BitStreamReader`

The BitStreamReader structure is designed to read bits from an input stream.

```rust
pub struct BitStreamReader<IStream> {
    istream: Rc<RefCell<IStream>>,
    buffer: u8,   // Buffered byte read in from the input stream
    offset: i32,  // Number of high order bits in buffer already returned by previous Read() calls
}

impl<IStream> BitStreamReader<IStream> {
    pub fn new(istream: &mut IStream) -> Self {
        // Initialize a new BitStreamReader
    }

    pub fn read(&mut self, nbits: i32) -> u64 {
        // Read the specified number of bits from the stream
    }
}
```

`BitStreamWriter`

The BitStreamWriter structure is designed to write bits to an output stream.

```rust
pub struct BitStreamWriter<OStream> {
    ostream: Rc<RefCell<OStream>>,
    buffer: u8,   // Buffered byte waiting to be written to the output stream
    offset: i32,  // Number of high order bits in buffer already written by previous Write() calls
}

impl<OStream> Drop for BitStreamWriter<OStream> {
    fn drop(&mut self) {
        // Flush the buffer on drop
    }
}

impl<OStream> BitStreamWriter<OStream> {
    pub fn new(ostream: &mut OStream) -> Self {
        // Initialize a new BitStreamWriter
    }

    pub fn write(&mut self, data: u64, nbits: i32) {
        // Write the nbits least significant bits of data to the output stream
    }

    pub fn flush(&mut self) {
        // Flush any unwritten bits to the output stream
    }
}
```

## Utilities

`count_bits`

Returns the smallest number n such that (x >> n) == 0.

```rust
#[inline] pub fn count_bits(x: u64) -> u64 {
    // Implementation of count_bits
}
```

`DataStream`

A double-ended buffer combining vector and stream-like interfaces.

```rust
pub struct DataStream {
    vch: SerializeData,
    n_read_pos: u32,
    n_type: i32,
    n_version: i32,
}

// Various methods for DataStream...
```

## Endianness Handling

Functions to read and write integers in little-endian and big-endian formats.

```rust
#[inline] pub fn readle16(ptr: *const u8) -> u16 { /*...*/ }
#[inline] pub fn readle32(ptr: *const u8) -> u32 { /*...*/ }
#[inline] pub fn readle64(ptr: *const u8) -> u64 { /*...*/ }
#[inline] pub fn readbe16(ptr: *const u8) -> u16 { /*...*/ }
#[inline] pub fn readbe32(ptr: *const u8) -> u32 { /*...*/ }
#[inline] pub fn readbe64(ptr: *const u8) -> u64 { /*...*/ }

#[inline] pub fn writele16(ptr: *mut u8, x: u16) { /*...*/ }
#[inline] pub fn writele32(ptr: *mut u8, x: u32) { /*...*/ }
#[inline] pub fn writele64(ptr: *mut u8, x: u64) { /*...*/ }
#[inline] pub fn writebe32(ptr: *mut u8, x: u32) { /*...*/ }
#[inline] pub fn writebe64(ptr: *mut u8, x: u64) { /*...*/ }
```

## Traits

`GetType` and `GetVersion`

Traits to retrieve type and version information from a stream.

```rust
pub trait GetType {
    fn get_type(&self) -> i32;
}

pub trait GetVersion {
    fn get_version(&self) -> i32;
}
```

`StreamItems` and `StreamInto`

Traits for streaming items into and from a stream.

```rust
pub trait StreamItems {
    fn stream<Item>(&mut self, x: Item);
}

pub trait StreamInto {
    fn stream_into<Item>(&self, x: &mut Item);
}
```
