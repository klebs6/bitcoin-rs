// ---------------- [ File: bitcoin-asmap/src/ty.rs ]
/*!
This function, `decode_type`, decodes the
instruction type from the given iterator
positions. It takes a mutable reference to an
iterator (`bitpos`) and a reference to an iterator
(`endpos`). The iterator item type is `&bool`,
which is a reference to a boolean value. The
function returns an `Instruction` enum value
representing the decoded instruction type.

The function utilizes the `decode_bits` function,
providing it with the `bitpos`, `endpos`, the
`minval` set to `0`, and a reference to the
`TYPE_BIT_SIZES` vector. The `decode_bits`
function will return a `u32` value, which is then
passed to the `Instruction::new` function to
create the appropriate `Instruction` enum value.

This Rust implementation is a direct translation
of the corresponding C++ code provided, and it
should work correctly to decode the instruction
type from the given iterator positions.
*/

crate::ix!();

lazy_static! {
    static ref TYPE_BIT_SIZES: Vec<u8> = vec![0, 0, 1];
}

/// Decode an `Instruction` opcode located at `*pos`.
pub fn decode_type(asmap: &[bool], pos: &mut usize) -> Instruction {
    trace!(pos = *pos, "decode_type");
    Instruction::new(decode_bits(asmap, pos, 0, &TYPE_BIT_SIZES))
}
