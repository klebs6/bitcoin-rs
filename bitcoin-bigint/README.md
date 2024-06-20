# bitcoin-bigint

`bitcoin-bigint` is a Rust crate providing a template base class for unsigned big integers. It is designed for scenarios where arbitrary-size unsigned integers are needed with full arithmetic capabilities.

## Overview

This crate defines a generic type:

- `BaseUInt`: A template base class for unsigned big integers of a specified bit size.

### `BaseUInt`

The `BaseUInt` type represents an unsigned big integer defined by the number of bits. It provides full arithmetic operations, bitwise operations, and other utility functions.

```rust
/**
  | Template base class for unsigned big
  | integers.
  |
  */
#[derive(Clone,Debug)]
pub struct BaseUInt<const BITS: usize> 
where [(); BITS / 32 ]: 
{
    pn: [u32; BITS / 32],
}

```

## Example Usage

```rust
use bitcoin_bigint::BaseUInt;

// Creating a BaseUInt instance with 256 bits
let mut bigint_256 = BaseUInt::<256>::default();

// Performing arithmetic operations
bigint_256 += 42u64;
bigint_256 *= 2u32;
```

## Features

- Generic BaseUInt type for unsigned big integers of any bit size.

- Full support for arithmetic operations: addition, subtraction, multiplication, and division.

- Bitwise operations: AND, OR, XOR, NOT.

- Shift operations: left shift, right shift.

- Utility functions: comparison, bit length, hex conversion, etc.

## License

This project is licensed under the MIT License.
