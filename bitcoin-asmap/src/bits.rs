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

pub fn decode_bits<'a, I>(
    bitpos:    &mut I,
    endpos:    &I,
    minval:    u8,
    bit_sizes: &Vec<u8>) -> u32 
where I: Iterator<Item = &'a bool> 
{
    todo!();
        /*
            uint32_t val = minval;
        bool bit;
        for (std::vector<uint8_t>::const_iterator bit_sizes_it = bit_sizes.begin();
            bit_sizes_it != bit_sizes.end(); ++bit_sizes_it) {
            if (bit_sizes_it + 1 != bit_sizes.end()) {
                if (bitpos == endpos) break;
                bit = *bitpos;
                bitpos++;
            } else {
                bit = 0;
            }
            if (bit) {
                val += (1 << *bit_sizes_it);
            } else {
                for (int b = 0; b < *bit_sizes_it; b++) {
                    if (bitpos == endpos) return INVALID; // Reached EOF in mantissa
                    bit = *bitpos;
                    bitpos++;
                    val += bit << (*bit_sizes_it - 1 - b);
                }
                return val;
            }
        }
        return INVALID; // Reached EOF in exponent
        */
}

