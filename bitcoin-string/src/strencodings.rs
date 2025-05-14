// ---------------- [ File: bitcoin-string/src/strencodings.rs ]
/*!
  | Utilities for converting data from/to
  | strings.
  |
  */

crate::ix!();

pub trait ToUpper {
    fn to_upper(&self) -> Self;
}

pub trait ToLower {
    fn to_lower(&self) -> Self;
}

//-------------------------------------------[.cpp/bitcoin/src/util/strencodings.h]

/**
  | Used by SanitizeString()
  |
  */
pub enum SafeChars
{
    /**
      | The full set of allowed chars
      |
      */
    SAFE_CHARS_DEFAULT, 

    /**
      | BIP-0014 subset
      |
      */
    SAFE_CHARS_UA_COMMENT, 

    /**
      | Chars allowed in filenames
      |
      */
    SAFE_CHARS_FILENAME, 

    /**
      | Chars allowed in URIs (RFC 3986)
      |
      */
    SAFE_CHARS_URI, 
}

/**
  | LocaleIndependentAtoi is provided for backwards
  | compatibility reasons.
  |
  | New code should use ToIntegral or the ParseInt*
  | functions which provide parse error feedback.
  |
  | The goal of LocaleIndependentAtoi is to
  | replicate the exact defined behaviour of atoi
  | and atoi64 as they behave under the "C" locale.
  */
pub fn locale_independent_atoi<T>(str_: &str) -> T {

    todo!();
        /*
            const_assert(std::is_integral<T>::value);
        T result;
        // Emulate atoi(...) handling of white space and leading +/-.
        std::string s = TrimString(str);
        if (!s.empty() && s[0] == '+') {
            if (s.length() >= 2 && s[1] == '-') {
                return 0;
            }
            s = s.substr(1);
        }
        auto [_, error_condition] = std::from_chars(s.data(), s.data() + s.size(), result);
        if (error_condition != std::errc{}) {
            return 0;
        }
        return result;
        */
}

/**
  | Tests if the given character is a decimal
  | digit.
  | 
  | -----------
  | @param[in] c
  | 
  | character to test
  | 
  | -----------
  | @return
  | 
  | true if the argument is a decimal digit;
  | otherwise false.
  |
  */
pub fn is_digit(c: u8) -> bool {
    
    todo!();
        /*
            return c >= '0' && c <= '9';
        */
}

/**
  | Tests if the given character is a whitespace
  | character. The whitespace characters
  | are: 
  |
  | space, 
  | form-feed ('\f'), 
  | newline ('\n'), 
  | carriage return ('\r'), 
  | horizontal tab ('\t'), 
  | and vertical tab ('\v').
  | 
  | This function is locale independent.
  | Under the C locale this function gives
  | the same result as std::isspace.
  | 
  | -----------
  | @param[in] c
  | 
  | character to test
  | 
  | -----------
  | @return
  | 
  | true if the argument is a whitespace
  | character; otherwise false
  |
  */
#[inline] pub fn is_space(c: u8) -> bool {
    
    todo!();
        /*
            return c == ' ' || c == '\f' || c == '\n' || c == '\r' || c == '\t' || c == '\v';
        */
}

/**
  | Convert string to integral type T. Leading
  | whitespace, a leading +, or any trailing
  | character fail the parsing.
  | 
  | The required format expressed as regex
  | is `-?[0-9]+`. The minus sign is only
  | permitted for signed integer types.
  | 
  | -----------
  | @return
  | 
  | std::nullopt if the entire string could
  | not be parsed, or if the parsed value
  | is not in the range representable by
  | the type T.
  |
  */
pub fn to_integral<T>(str_: &str) -> Option<T> {

    todo!();
        /*
            const_assert(std::is_integral<T>::value);
        T result;
        const auto [first_nonmatching, error_condition] = std::from_chars(str.data(), str.data() + str.size(), result);
        if (first_nonmatching != str.data() + str.size() || error_condition != std::errc{}) {
            return std::nullopt;
        }
        return result;
        */
}

/**
  | Timing-attack-resistant comparison.
  | 
  | Takes time proportional to length of
  | first argument.
  |
  */
pub fn timing_resistant_equal<T>(a: &T, b: &T) -> bool {

    todo!();
        /*
            if (b.size() == 0) return a.size() == 0;
        size_t accumulator = a.size() ^ b.size();
        for (size_t i = 0; i < a.size(); i++)
            accumulator |= a[i] ^ b[i%b.size()];
        return accumulator == 0;
        */
}

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

impl ToLower for u8 {

    /**
      | Converts the given character to its
      | lowercase equivalent.
      | 
      | This function is locale independent.
      | It only converts uppercase characters
      | in the standard 7-bit ASCII range.
      | 
      | This is a feature, not a limitation.
      | 
      | -----------
      | @param[in] c
      | 
      | the character to convert to lowercase.
      | 
      | -----------
      | @return
      | 
      | the lowercase equivalent of c; or the
      | argument if no conversion is possible.
      |
      */
    fn to_lower(&self) -> u8 {

        todo!();
        /*
                return (c >= 'A' && c <= 'Z' ? (c - 'A') + 'a' : c);
            */
    }
}

/**
  | Converts the given character to its
  | uppercase equivalent.
  | 
  | This function is locale independent.
  | It only converts lowercase characters
  | in the standard 7-bit ASCII range.
  | 
  | This is a feature, not a limitation.
  | 
  | -----------
  | @param[in] c
  | 
  | the character to convert to uppercase.
  | 
  | -----------
  | @return
  | 
  | the uppercase equivalent of c; or the
  | argument if no conversion is possible.
  |
  */
impl ToUpper for u8 {

    fn to_upper(&self) -> u8 {
        
        todo!();
            /*
                return (c >= 'a' && c <= 'z' ? (c - 'a') + 'A' : c);
            */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/util/strencodings.cpp]

pub const CHARS_ALPHA_NUM: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

lazy_static!{
    static ref SAFE_CHARS: Box<[String]> = 
        Box::new([
            CHARS_ALPHA_NUM.to_owned() + " .,;-_/:?@()",            // SAFE_CHARS_DEFAULT
            CHARS_ALPHA_NUM.to_owned() + " .,;-_?@",                // SAFE_CHARS_UA_COMMENT
            CHARS_ALPHA_NUM.to_owned() + ".-_",                     // SAFE_CHARS_FILENAME
            CHARS_ALPHA_NUM.to_owned() + "!*'();:@&=+$,/?#[]-_.~%", // SAFE_CHARS_URI
        ]);
}

/**
  | Remove unsafe chars. Safe chars chosen
  | to allow simple messages/URLs/email
  | addresses, but avoid anything even
  | possibly remotely dangerous like &
  | or >
  | 
  | -----------
  | @param[in] str
  | 
  | The string to sanitize
  | ----------
  | @param[in] rule
  | 
  | The set of safe chars to choose (default:
  | least restrictive)
  | 
  | -----------
  | @return
  | 
  | A new string without unsafe chars
  |
  */
pub fn sanitize_string(
        str_: &str,
        rule: Option<i32>) -> String {

    let rule: i32 = rule.unwrap_or(SafeChars::SAFE_CHARS_DEFAULT as i32);
    
    todo!();
        /*
            std::string strResult;
        for (std::string::size_type i = 0; i < str.size(); i++)
        {
            if (SAFE_CHARS[rule].find(str[i]) != std::string::npos)
                strResult.push_back(str[i]);
        }
        return strResult;
        */
}

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

pub fn encode_base64_bytes(input: &[u8]) -> String {
    
    todo!();
        /*
            static const char *pbase64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        std::string str;
        str.reserve(((input.size() + 2) / 3) * 4);
        ConvertBits<8, 6, true>([&](int v) { str += pbase64[v]; }, input.begin(), input.end());
        while (str.size() % 4) str += '=';
        return str;
        */
}

pub fn encode_base64(str_: &String) -> String {
    
    todo!();
        /*
            return EncodeBase64(MakeUCharSpan(str));
        */
}

pub fn decode_base64_bytes(
        p:          *const u8,
        pf_invalid: Option<*mut bool>) -> Vec<u8> {
    
    todo!();
        /*
            static const int decode64_table[256] =
        {
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, 62, -1, -1, -1, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1,
            -1, -1, -1, -1, -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
            15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, 26, 27, 28,
            29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
            49, 50, 51, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
        };

        const char* e = p;
        std::vector<uint8_t> val;
        val.reserve(strlen(p));
        while (*p != 0) {
            int x = decode64_table[(unsigned char)*p];
            if (x == -1) break;
            val.push_back(x);
            ++p;
        }

        std::vector<unsigned char> ret;
        ret.reserve((val.size() * 3) / 4);
        bool valid = ConvertBits<6, 8, false>([&](unsigned char c) { ret.push_back(c); }, val.begin(), val.end());

        const char* q = p;
        while (valid && *p != 0) {
            if (*p != '=') {
                valid = false;
                break;
            }
            ++p;
        }
        valid = valid && (p - e) % 4 == 0 && p - q < 4;
        if (pf_invalid) *pf_invalid = !valid;

        return ret;
        */
}

pub fn decode_base64(
        str_:       &str,
        pf_invalid: Option<&mut bool>) -> String {
    
    todo!();
        /*
            if (!ValidAsCString(str)) {
            if (pf_invalid) {
                *pf_invalid = true;
            }
            return {};
        }
        std::vector<unsigned char> vchRet = DecodeBase64(str.c_str(), pf_invalid);
        return std::string((const char*)vchRet.data(), vchRet.size());
        */
}

/**
  | Base32 encode.
  | 
  | If `pad` is true, then the output will
  | be padded with '=' so that its length
  | is a multiple of 8.
  |
  */
pub fn encode_base32_bytes(
        input: &[u8],
        pad:   Option<bool>) -> String {
    let pad: bool = pad.unwrap_or(true);
    
    todo!();
        /*
            static const char *pbase32 = "abcdefghijklmnopqrstuvwxyz234567";

        std::string str;
        str.reserve(((input.size() + 4) / 5) * 8);
        ConvertBits<8, 5, true>([&](int v) { str += pbase32[v]; }, input.begin(), input.end());
        if (pad) {
            while (str.size() % 8) {
                str += '=';
            }
        }
        return str;
        */
}

/**
  | Base32 encode.
  | 
  | If `pad` is true, then the output will
  | be padded with '=' so that its length
  | is a multiple of 8.
  |
  */
pub fn encode_base32(
        str_: &[u8],
        pad:  Option<bool>) -> String {
    let pad: bool = pad.unwrap_or(true);
    
    todo!();
        /*
            return EncodeBase32(MakeUCharSpan(str), pad);
        */
}

pub fn decode_base32_bytes(
        p:          *const u8,
        pf_invalid: Option<*mut bool>) -> Vec<u8> {
    
    todo!();
        /*
            static const int decode32_table[256] =
        {
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1,
            -1, -1, -1, -1, -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
            15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1,  0,  1,  2,
             3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
        };

        const char* e = p;
        std::vector<uint8_t> val;
        val.reserve(strlen(p));
        while (*p != 0) {
            int x = decode32_table[(unsigned char)*p];
            if (x == -1) break;
            val.push_back(x);
            ++p;
        }

        std::vector<unsigned char> ret;
        ret.reserve((val.size() * 5) / 8);
        bool valid = ConvertBits<5, 8, false>([&](unsigned char c) { ret.push_back(c); }, val.begin(), val.end());

        const char* q = p;
        while (valid && *p != 0) {
            if (*p != '=') {
                valid = false;
                break;
            }
            ++p;
        }
        valid = valid && (p - e) % 8 == 0 && p - q < 8;
        if (pf_invalid) *pf_invalid = !valid;

        return ret;
        */
}

pub fn decode_base32(
        str_:       &str,
        pf_invalid: Option<*mut bool>) -> String {
    
    todo!();
        /*
            if (!ValidAsCString(str)) {
            if (pf_invalid) {
                *pf_invalid = true;
            }
            return {};
        }
        std::vector<unsigned char> vchRet = DecodeBase32(str.c_str(), pf_invalid);
        return std::string((const char*)vchRet.data(), vchRet.size());
        */
}

pub fn parse_integral<T>(
        str_: &String,
        out:  *mut T) -> bool {

    todo!();
        /*
            const_assert(std::is_integral<T>::value);
        // Replicate the exact behavior of strtol/strtoll/strtoul/strtoull when
        // handling leading +/- for backwards compatibility.
        if (str.length() >= 2 && str[0] == '+' && str[1] == '-') {
            return false;
        }
        const std::optional<T> opt_int = ToIntegral<T>((!str.empty() && str[0] == '+') ? str.substr(1) : str);
        if (!opt_int) {
            return false;
        }
        if (out != nullptr) {
            *out = *opt_int;
        }
        return true;
        */
}

/**
  | Convert string to signed 32-bit integer
  | with strict parse error feedback.
  | 
  | 
  | -----------
  | @return
  | 
  | true if the entire string could be parsed
  | as valid integer, false if not the entire
  | string could be parsed or when overflow
  | or underflow occurred.
  |
  */
pub fn parse_int32(
        str_: &String,
        out:  *mut i32) -> bool {
    
    todo!();
        /*
            return ParseIntegral<int32_t>(str, out);
        */
}

/**
  | Convert string to signed 64-bit integer
  | with strict parse error feedback.
  | 
  | -----------
  | @return
  | 
  | true if the entire string could be parsed
  | as valid integer, false if not the entire
  | string could be parsed or when overflow
  | or underflow occurred.
  |
  */
pub fn parse_int64(
        str_: &String,
        out:  *mut i64) -> bool {
    
    todo!();
        /*
            return ParseIntegral<int64_t>(str, out);
        */
}

/**
  | Convert decimal string to unsigned
  | 8-bit integer with strict parse error
  | feedback.
  | 
  | -----------
  | @return
  | 
  | true if the entire string could be parsed
  | as valid integer, false if not the entire
  | string could be parsed or when overflow
  | or underflow occurred.
  |
  */
pub fn parse_uint8(
        str_: &str,
        out:  *mut u8) -> bool {
    
    todo!();
        /*
            return ParseIntegral<uint8_t>(str, out);
        */
}

/**
  | Convert decimal string to unsigned
  | 16-bit integer with strict parse error
  | feedback.
  | 
  | -----------
  | @return
  | 
  | true if the entire string could be parsed
  | as valid integer, false if the entire
  | string could not be parsed or if overflow
  | or underflow occurred.
  |
  */
pub fn parse_uint16(
        str_: &str,
        out:  *mut u16) -> bool {
    
    todo!();
        /*
            return ParseIntegral<uint16_t>(str, out);
        */
}

/**
  | Convert decimal string to unsigned
  | 32-bit integer with strict parse error
  | feedback.
  | 
  | -----------
  | @return
  | 
  | true if the entire string could be parsed
  | as valid integer, false if not the entire
  | string could be parsed or when overflow
  | or underflow occurred.
  |
  */
pub fn parse_uint32(
        str_: &str,
        out:  *mut u32) -> bool {
    
    todo!();
        /*
            return ParseIntegral<uint32_t>(str, out);
        */
}

/**
  | Convert decimal string to unsigned
  | 64-bit integer with strict parse error
  | feedback.
  | 
  | 
  | -----------
  | @return
  | 
  | true if the entire string could be parsed
  | as valid integer, false if not the entire
  | string could be parsed or when overflow
  | or underflow occurred.
  |
  */
pub fn parse_uint64(
        str_: &str,
        out:  *mut u64) -> bool {
    
    todo!();
        /*
            return ParseIntegral<uint64_t>(str, out);
        */
}

/** 
 | Upper bound for mantissa.
 |
 | 10^18-1 is the largest arbitrary decimal that
 | will fit in a signed 64-bit integer.
 |
 | Larger integers cannot consist of arbitrary
 | combinations of 0-9:
 |
 |   999999999999999999  1^18-1
 |  9223372036854775807  (1<<63)-1  (max int64_t)
 |  9999999999999999999  1^19-1     (would overflow)
 */
pub const UPPER_BOUND: i64 = 1000000000000000000 - 1;

/**
  | Helper function for ParseFixedPoint
  |
  */
#[inline] pub fn process_mantissa_digit(
        ch:              u8,
        mantissa:        &mut i64,
        mantissa_tzeros: &mut i32) -> bool {
    
    todo!();
        /*
            if(ch == '0')
            ++mantissa_tzeros;
        else {
            for (int i=0; i<=mantissa_tzeros; ++i) {
                if (mantissa > (UPPER_BOUND / 10LL))
                    return false; /* overflow */
                mantissa *= 10;
            }
            mantissa += ch - '0';
            mantissa_tzeros = 0;
        }
        return true;
        */
}

/**
  | Parse number as fixed point according
  | to JSON number syntax.
  | 
  | See https://json.org/number.gif
  | 
  | -----------
  | @note
  | 
  | The result must be in the range (-10^18,10^18),
  | otherwise an overflow error will trigger.
  | 
  | -----------
  | @return
  | 
  | true on success, false on error.
  |
  */
pub fn parse_fixed_point(
        val:        &str,
        decimals:   i32,
        amount_out: *mut i64) -> bool {
    
    todo!();
        /*
            int64_t mantissa = 0;
        int64_t exponent = 0;
        int mantissa_tzeros = 0;
        bool mantissa_sign = false;
        bool exponent_sign = false;
        int ptr = 0;
        int end = val.size();
        int point_ofs = 0;

        if (ptr < end && val[ptr] == '-') {
            mantissa_sign = true;
            ++ptr;
        }
        if (ptr < end)
        {
            if (val[ptr] == '0') {
                /* pass single 0 */
                ++ptr;
            } else if (val[ptr] >= '1' && val[ptr] <= '9') {
                while (ptr < end && IsDigit(val[ptr])) {
                    if (!ProcessMantissaDigit(val[ptr], mantissa, mantissa_tzeros))
                        return false; /* overflow */
                    ++ptr;
                }
            } else return false; /* missing expected digit */
        } else return false; /* empty string or loose '-' */
        if (ptr < end && val[ptr] == '.')
        {
            ++ptr;
            if (ptr < end && IsDigit(val[ptr]))
            {
                while (ptr < end && IsDigit(val[ptr])) {
                    if (!ProcessMantissaDigit(val[ptr], mantissa, mantissa_tzeros))
                        return false; /* overflow */
                    ++ptr;
                    ++point_ofs;
                }
            } else return false; /* missing expected digit */
        }
        if (ptr < end && (val[ptr] == 'e' || val[ptr] == 'E'))
        {
            ++ptr;
            if (ptr < end && val[ptr] == '+')
                ++ptr;
            else if (ptr < end && val[ptr] == '-') {
                exponent_sign = true;
                ++ptr;
            }
            if (ptr < end && IsDigit(val[ptr])) {
                while (ptr < end && IsDigit(val[ptr])) {
                    if (exponent > (UPPER_BOUND / 10LL))
                        return false; /* overflow */
                    exponent = exponent * 10 + val[ptr] - '0';
                    ++ptr;
                }
            } else return false; /* missing expected digit */
        }
        if (ptr != end)
            return false; /* trailing garbage */

        /* finalize exponent */
        if (exponent_sign)
            exponent = -exponent;
        exponent = exponent - point_ofs + mantissa_tzeros;

        /* finalize mantissa */
        if (mantissa_sign)
            mantissa = -mantissa;

        /* convert to one 64-bit fixed-point value */
        exponent += decimals;
        if (exponent < 0)
            return false; /* cannot represent values smaller than 10^-decimals */
        if (exponent >= 18)
            return false; /* cannot represent values larger than or equal to 10^(18-decimals) */

        for (int i=0; i < exponent; ++i) {
            if (mantissa > (UPPER_BOUND / 10LL) || mantissa < -(UPPER_BOUND / 10LL))
                return false; /* overflow */
            mantissa *= 10;
        }
        if (mantissa > UPPER_BOUND || mantissa < -UPPER_BOUND)
            return false; /* overflow */

        if (amount_out)
            *amount_out = mantissa;

        return true;
        */
}

impl ToLower for String {

    /**
      | Returns the lowercase equivalent of
      | the given string.
      | 
      | This function is locale independent.
      | It only converts uppercase characters
      | in the standard 7-bit ASCII range.
      | 
      | This is a feature, not a limitation.
      | 
      | -----------
      | @param[in] str
      | 
      | the string to convert to lowercase.
      | 
      | -----------
      | @return
      | 
      | lowercased equivalent of str
      |
      */
    fn to_lower(&self) -> String {

        todo!();
            /*
                std::string r;
            for (auto ch : str) r += ToLower((unsigned char)ch);
            return r;
            */
    }
}

/**
  | Returns the uppercase equivalent of
  | the given string.
  | 
  | This function is locale independent.
  | It only converts lowercase characters
  | in the standard 7-bit ASCII range.
  | 
  | This is a feature, not a limitation.
  | 
  | -----------
  | @param[in] str
  | 
  | the string to convert to uppercase.
  | 
  | -----------
  | @return
  | 
  | UPPERCASED EQUIVALENT OF str
  |
  */
impl ToUpper for String {
    fn to_upper(&self) -> String {
        
        todo!();
            /*
                std::string r;
            for (auto ch : str) r += ToUpper((unsigned char)ch);
            return r;
            */
    }
}

/**
  | Capitalizes the first character of
  | the given string.
  | 
  | This function is locale independent.
  | It only converts lowercase characters
  | in the standard 7-bit ASCII range.
  | 
  | This is a feature, not a limitation.
  | 
  | -----------
  | @param[in] str
  | 
  | the string to capitalize.
  | 
  | -----------
  | @return
  | 
  | string with the first letter capitalized.
  |
  */
pub fn capitalize(str_: &str) -> String {
    
    todo!();
        /*
            if (str.empty()) return str;
        str[0] = ToUpper(str.front());
        return str;
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
