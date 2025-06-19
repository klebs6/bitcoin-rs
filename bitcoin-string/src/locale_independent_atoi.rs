crate::ix!();

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
