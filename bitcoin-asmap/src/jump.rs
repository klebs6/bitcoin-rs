// ---------------- [ File: bitcoin-asmap/src/jump.rs ]
/*!
This function, `decode_jump`, decodes a jump value
from a provided iterator of boolean values. The
iterator `bitpos` represents the current position
in the ASMAP data, and `endpos` represents the end
position of the data. The purpose of this function
is to decode a jump value from the compact
representation in the ASMAP data.

The function uses a static vector
`JUMP_BIT_SIZES`, which contains a list of
possible jump bit sizes.

The C++ code provided for this function calls the
`DecodeBits` function with `bitpos`, `endpos`,
a minimum bit size of 17, and the `JUMP_BIT_SIZES`
vector.

In Rust, the `decode_jump` function will have
a similar structure. It will call a Rust
implementation of the `DecodeBits` function with
the given iterator, end position, a minimum bit
size of 17, and the `JUMP_BIT_SIZES` vector. The
`todo!();` macro is a placeholder that should be
replaced with the Rust implementation that follows
the same logic as the C++ code provided.
*/

crate::ix!();

lazy_static!{
    static ref JUMP_BIT_SIZES: Vec<u8> 
    = vec!{
         5,  6,  7,  8, 
         9, 10, 11, 12, 
        13, 14, 15, 16, 
        17, 18, 19, 20, 
        21, 22, 23, 24, 
        25, 26, 27, 28, 
        29, 30
    };
}

/// Decode a jump offset from `asmap`.
pub fn decode_jump(asmap: &[bool], pos: &mut usize) -> u32 {
    trace!(pos = *pos, "decode_jump");
    decode_bits(asmap, pos, 17, &JUMP_BIT_SIZES)
}
