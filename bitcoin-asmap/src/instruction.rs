// ---------------- [ File: bitcoin-asmap/src/instruction.rs ]
/*!
This code defines a constant `INVALID`, which represents an
invalid value in the ASMAP decoding process. The value is
set to `0xFFFFFFFF`, a common representation of an invalid
or error value.

Additionally, the code defines an enumeration `Instruction`
with four variants: `RETURN`, `JUMP`, `MATCH`, and
`DEFAULT`. These instruction types are used to represent
different actions during the ASMAP decoding process.

The `Instruction` enumeration also has an associated
implementation block containing a method `new` that takes
a `u32` parameter and returns an `Instruction` variant. The
method uses a match statement to map the provided `u32`
value to the corresponding `Instruction` variant. If the
value does not match any known variants, the
`unreachable!()` macro is called, which indicates that the
code should not reach that point.

This enumeration and its associated method are used to
facilitate the decoding process of ASMAP data by providing
a way to represent the different instructions that can be
encountered during decoding.
*/

crate::ix!();

pub const INVALID: u32 = 0xFFFFFFFF;

#[repr(u32)]
pub enum Instruction {
    RETURN  = 0,
    JUMP    = 1,
    MATCH   = 2,
    DEFAULT = 3,
}

impl Instruction {

    pub fn new(x: u32) -> Self {
        match x {
            0 => Instruction::RETURN,
            1 => Instruction::JUMP,
            2 => Instruction::MATCH,
            3 => Instruction::DEFAULT,
            _ => unreachable!(),
        }
    }
}
