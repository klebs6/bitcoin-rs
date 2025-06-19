crate::ix!();

pub const util_hexdigit: [i8; 256] = [ 
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    0,1,2,3,4,5,6,7,8,9,-1,-1,-1,-1,-1,-1,
    -1,0xa,0xb,0xc,0xd,0xe,0xf,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,0xa,0xb,0xc,0xd,0xe,0xf,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 
];

pub fn hex_digit(c: u8) -> i8 {
    
    todo!();
        /*
            return p_util_hexdigit[(unsigned char)c];
        */
}

/**
  | Returns true if each character in str
  | is a hex character, and has an even number
  | of hex digits.
  |
  */
pub fn is_hex(str_: &str) -> bool {
    
    todo!();
        /*
            for(std::string::const_iterator it(str.begin()); it != str.end(); ++it)
        {
            if (HexDigit(*it) < 0)
                return false;
        }
        return (str.size() > 0) && (str.size()%2 == 0);
        */
}

/**
  | Return true if the string is a hex number,
  | optionally prefixed with "0x"
  |
  */
pub fn is_hex_number(str_: &str) -> bool {
    
    todo!();
        /*
            size_t starting_location = 0;
        if (str.size() > 2 && *str.begin() == '0' && *(str.begin()+1) == 'x') {
            starting_location = 2;
        }
        for (const char c : str.substr(starting_location)) {
            if (HexDigit(c) < 0) return false;
        }
        // Return false for empty string or "0x".
        return (str.size() > starting_location);
        */
}

pub fn parse_hex(psz: &str) -> Vec<u8> {
    
    todo!();
        /*
            // convert hex dump to vector
        std::vector<unsigned char> vch;
        while (true)
        {
            while (IsSpace(*psz))
                psz++;
            signed char c = HexDigit(*psz++);
            if (c == (signed char)-1)
                break;
            unsigned char n = (c << 4);
            c = HexDigit(*psz++);
            if (c == (signed char)-1)
                break;
            n |= c;
            vch.push_back(n);
        }
        return vch;
        */
}

/**
  | Convert a span of bytes to a lower-case
  | hexadecimal string.
  |
  */
pub fn hex_str(s: &[u8]) -> String {
    
    todo!();
        /*
            std::string rv(s.size() * 2, '\0');
        static constexpr char hexmap[16] = { '0', '1', '2', '3', '4', '5', '6', '7',
                                             '8', '9', 'a', 'b', 'c', 'd', 'e', 'f' };
        auto it = rv.begin();
        for (uint8_t v : s) {
            *it++ = hexmap[v >> 4];
            *it++ = hexmap[v & 15];
        }
        assert(it == rv.end());
        return rv;
        */
}
