/**
  | This section implements a checksum algorithm for
  | descriptors with the following properties:
  |
  | - Mistakes in a descriptor string are measured in
  |   "symbol errors". The higher the number of symbol
  |   errors, the harder it is to detect:
  |
  |   - An error substituting a character from
  |     0123456789()[],'/\*abcdefgh@:$%{} for another
  |     in that set always counts as 1 symbol error.
  |
  |     - Note that hex encoded keys are covered by
  |       these characters. Xprvs and xpubs use other
  |       characters too, but already have their own
  |       checksum mechanism.
  |
  |     - Function names like "multi()" use other
  |       characters, but mistakes in these would
  |       generally result in an unparsable
  |       descriptor.
  |
  |   - A case error always counts as 1 symbol error.
  |   
  |   - Any other 1 character substitution error
  |     counts as 1 or 2 symbol errors.
  |   
  | - Any 1 symbol error is always detected.
  |
  | - Any 2 or 3 symbol error in a descriptor of up to
  |   49154 characters is always detected.
  |
  | - Any 4 symbol error in a descriptor of up to 507
  |   characters is always detected.
  |
  | - Any 5 symbol error in a descriptor of up to 77
  |   characters is always detected.
  |
  | - Is optimized to minimize the chance a 5 symbol
  |   error in a descriptor up to 387 characters is
  |   undetected
  |
  | - Random errors have a chance of 1 in 2**40 of
  |   being undetected.
  |
  | These properties are achieved by expanding every
  | group of 3 (non checksum) characters into
  | 4 GF(32) symbols, over which a cyclic code is
  | defined.
  */

crate::ix!();

pub fn descriptor_checksum(span: &[u8]) -> String {
    
    todo!();
        /*
            /* 
         | A character set designed such that:
         |
         |  - The most common 'unprotected' descriptor
         |  characters (hex, keypaths) are in the
         |  first group of 32.
         |
         |  - Case errors cause an offset that's
         |  a multiple of 32.
         |
         |  - As many alphabetic characters are in the
         |  same group (while following the above
         |  restrictions).
         |
         |
         | If p(x) gives the position of a character
         | c in this character set, every group of
         | 3 characters (a,b,c) is encoded as the
         | 4 symbols (p(a) & 31, p(b) & 31, p(c) & 31,
         | (p(a) / 32) + 3 * (p(b) / 32) + 9 * (p(c)
         | / 32).
         |
         | This means that changes that only affect
         | the lower 5 bits of the position, or only
         | the higher 2 bits, will just affect
         | a single symbol.
         |
         | As a result, within-group-of-32 errors
         | count as 1 symbol, as do cross-group errors
         | that don't affect the position within the
         | groups.
         */
        static std::string INPUT_CHARSET =
            "0123456789()[],'/\*abcdefgh@:$%{}"
            "IJKLMNOPQRSTUVWXYZ&+-.;<=>?!^_|~"
            "ijklmnopqrstuvwxyzABCDEFGH`#\"\\ ";

        /*
          | The character set for the checksum itself
          | (same as bech32).
          |
          */
        static std::string CHECKSUM_CHARSET = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

        uint64_t c = 1;
        int cls = 0;
        int clscount = 0;
        for (auto ch : span) {
            auto pos = INPUT_CHARSET.find(ch);
            if (pos == std::string::npos) return "";
            c = PolyMod(c, pos & 31); // Emit a symbol for the position inside the group, for every character.
            cls = cls * 3 + (pos >> 5); // Accumulate the group numbers
            if (++clscount == 3) {
                // Emit an extra symbol representing the group numbers, for every 3 characters.
                c = PolyMod(c, cls);
                cls = 0;
                clscount = 0;
            }
        }
        if (clscount > 0) c = PolyMod(c, cls);
        for (int j = 0; j < 8; ++j) c = PolyMod(c, 0); // Shift further to determine the checksum.
        c ^= 1; // Prevent appending zeroes from not affecting the checksum.

        std::string ret(8, ' ');
        for (int j = 0; j < 8; ++j) ret[j] = CHECKSUM_CHARSET[(c >> (5 * (7 - j))) & 31];
        return ret;
        */
}

pub fn add_checksum(str_: &String) -> String {
    
    todo!();
        /*
            return str + "#" + DescriptorChecksum(str);
        */
}

/**
  | Check a descriptor checksum, and update
  | desc to be the checksum-less part.
  |
  */
pub fn check_checksum(
        sp:               &mut [u8],
        require_checksum: bool,
        error:            &mut String,
        out_checksum:     Option<*mut String>) -> bool {

    todo!();
        /*
            using namespace spanparsing;

        auto check_split = Split(sp, '#');
        if (check_split.size() > 2) {
            error = "Multiple '#' symbols";
            return false;
        }
        if (check_split.size() == 1 && require_checksum){
            error = "Missing checksum";
            return false;
        }
        if (check_split.size() == 2) {
            if (check_split[1].size() != 8) {
                error = strprintf("Expected 8 character checksum, not %u characters", check_split[1].size());
                return false;
            }
        }
        auto checksum = DescriptorChecksum(check_split[0]);
        if (checksum.empty()) {
            error = "Invalid characters in payload";
            return false;
        }
        if (check_split.size() == 2) {
            if (!std::equal(checksum.begin(), checksum.end(), check_split[1].begin())) {
                error = strprintf("Provided checksum '%s' does not match computed checksum '%s'", std::string(check_split[1].begin(), check_split[1].end()), checksum);
                return false;
            }
        }
        if (out_checksum) *out_checksum = std::move(checksum);
        sp = check_split[0];
        return true;
        */
}

/**
  | Get the checksum for a `descriptor`.
  | 
  | - If it already has one, and it is correct,
  | return the checksum in the input.
  | 
  | - If it already has one that is wrong,
  | return "".
  | 
  | - If it does not already have one, return
  | the checksum that would need to be added.
  |
  */
pub fn get_descriptor_checksum(descriptor: &String) -> String {
    
    todo!();
        /*
            std::string ret;
        std::string error;
        Span<const char> sp{descriptor};
        if (!CheckChecksum(sp, false, error, &ret)) return "";
        return ret;
        */
}

