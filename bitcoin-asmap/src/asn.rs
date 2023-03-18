/*!
This function, `decodeasn`, decodes an Autonomous System
Number (ASN) from a provided iterator of boolean values. The
iterator `bitpos` represents the current position in the
ASMAP data, and `endpos` represents the end position of the
data. The purpose of this function is to decode an ASN from
the compact representation in the ASMAP data.

The function uses a static vector `ASN_BIT_SIZES`, which
contains a list of possible ASN bit sizes.

The C++ code provided for this function calls the
`DecodeBits` function with `bitpos`, `endpos`, a minimum bit
size of 1, and the `ASN_BIT_SIZES` vector.

In Rust, the `decodeasn` function will have a similar
structure. It will call a Rust implementation of the
`DecodeBits` function with the given iterator, end position,
a minimum bit size of 1, and the `ASN_BIT_SIZES` vector. The
`todo!();` macro is a placeholder that should be replaced
with the Rust implementation that follows the same logic as
the C++ code provided.
*/

crate::ix!();

lazy_static!{
    static ref ASN_BIT_SIZES: Vec<u8> = vec!{15, 16, 17, 18, 19, 20, 21, 22, 23, 24};
}

pub fn decodeasn<'a, I>(
        bitpos: &mut I,
        endpos: &I) -> u32
where I: Iterator<Item = &'a bool> {
    
    todo!();
        /*
            return DecodeBits(bitpos, endpos, 1, ASN_BIT_SIZES);
        */
}
