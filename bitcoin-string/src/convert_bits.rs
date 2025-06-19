crate::ix!();

/**
  | Convert from one power-of-2 number
  | base to another.
  |
  */
pub fn convert_bits<O, I, const frombits: i32, const tobits: i32, const pad: bool>(
        outfn: &O,
        it:    I,
        end:   I) -> bool {

    todo!();
        /*
            size_t acc = 0;
        size_t bits = 0;
        constexpr size_t maxv = (1 << tobits) - 1;
        constexpr size_t max_acc = (1 << (frombits + tobits - 1)) - 1;
        while (it != end) {
            acc = ((acc << frombits) | *it) & max_acc;
            bits += frombits;
            while (bits >= tobits) {
                bits -= tobits;
                outfn((acc >> bits) & maxv);
            }
            ++it;
        }
        if (pad) {
            if (bits) outfn((acc << (tobits - bits)) & maxv);
        } else if (bits >= frombits || ((acc << (tobits - bits)) & maxv)) {
            return false;
        }
        return true;
        */
}
