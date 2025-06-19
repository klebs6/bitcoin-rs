crate::ix!();

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
