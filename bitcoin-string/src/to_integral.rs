crate::ix!();

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
