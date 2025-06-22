// ---------------- [ File: bitcoin-asmap/src/bits.rs ]
/*!
This function, `decode_bits`, decodes a value from
a provided iterator of boolean values using the
given bit sizes. The iterator `bitpos` represents
the current position in the ASMAP data, and
`endpos` represents the end position of the
data. The `minval` parameter is the minimum value
of the decoded result, and the `bit_sizes` vector
contains the list of bit sizes to be used for
decoding. The purpose of this function is to
decode a value from the compact representation in
the ASMAP data.

Here's a brief explanation of the C++ code:

1. It initializes the `val` variable with the
   provided `minval`.

2. It iterates through the `bit_sizes` vector,
   using a nested loop to decode the value based
   on the bits in the ASMAP data.

3. If the iterator reaches the end position
   (`endpos`) during decoding, it returns
   `INVALID`. This means that the decoding has
   reached the end of the file or an unexpected
   position, and the resulting value is considered
   invalid.

4. If the decoding completes successfully, it
   returns the decoded value `val`.

In Rust, the `decode_bits` function will have
a similar structure. It will decode a value from
the given iterator, end position, minimum value,
and bit sizes vector. The `todo!();` macro is
a placeholder that should be replaced with the
Rust implementation that follows the same logic as
the C++ code provided.
*/


crate::ix!();

/// Core bit‑stream decoder, mirroring the C++ implementation in behaviour.
pub fn decode_bits(
    asmap:     &[bool],
    pos:       &mut usize,
    minval:    u8,
    bit_sizes: &[u8],
) -> u32 {
    trace!(start_pos = *pos, "decode_bits: enter");

    let mut val: u32 = minval as u32;

    for (i, bitsize) in bit_sizes.iter().enumerate() {
        let last_size = i + 1 == bit_sizes.len();

        // Read exponent bit unless this is the sentinel (last) size.
        let bit = if !last_size {
            if *pos >= asmap.len() {
                trace!("decode_bits: hit EOF in exponent");
                return INVALID;
            }
            let b = asmap[*pos];
            *pos += 1;
            b
        } else {
            false
        };

        if bit {
            // We set the exponent and continue parsing larger bit‑sizes.
            val += 1u32 << *bitsize;
        } else {
            // Mantissa path: consume `bitsize` bits and finish.
            for b in 0..*bitsize {
                if *pos >= asmap.len() {
                    trace!("decode_bits: hit EOF in mantissa");
                    return INVALID;
                }
                let bit_val = asmap[*pos] as u32;
                *pos += 1;
                val += bit_val << (*bitsize - 1 - b);
            }
            trace!(end_pos = *pos, result = val, "decode_bits: ok");
            return val;
        }
    }

    trace!("decode_bits: reached EOF in exponent without terminator");
    INVALID // Reached EOF in exponent
}
