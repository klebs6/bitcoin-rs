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

