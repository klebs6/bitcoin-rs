/*!
This function, `decode_match`, decodes a match value from
a provided iterator of boolean values. The iterator `bitpos`
represents the current position in the ASMAP data, and
`endpos` represents the end position of the data. The
purpose of this function is to decode a match value from the
compact representation in the ASMAP data.

The function uses a static vector `MATCH_BIT_SIZES`, which
contains a list of possible match bit sizes.

The C++ code provided for this function calls the
`DecodeBits` function with `bitpos`, `endpos`, a minimum bit
size of 2, and the `MATCH_BIT_SIZES` vector.

In Rust, the `decode_match` function will have a similar
structure. It will call a Rust implementation of the
`DecodeBits` function with the given iterator, end position,
a minimum bit size of 2, and the `MATCH_BIT_SIZES`
vector. The `todo!();` macro is a placeholder that should be
replaced with the Rust implementation that follows the same
logic as the C++ code provided.
*/

crate::ix!();

lazy_static!{
    static ref MATCH_BIT_SIZES: Vec<u8> = vec!{1, 2, 3, 4, 5, 6, 7, 8};
}

pub fn decode_match<'a, I>(
        bitpos: &mut I,
        endpos: &I) -> u32 
where I: Iterator<Item = &'a bool> {
    
    todo!();
        /*
            return DecodeBits(bitpos, endpos, 2, MATCH_BIT_SIZES);
        */
}

