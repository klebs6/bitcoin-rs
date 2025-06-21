// ---------------- [ File: bitcoin-string/src/capitalize.rs ]
crate::ix!();

/// Capitalize the first character of the provided ASCII string.
///
/// Only bytes in the standard 7‑bit ASCII range are affected; the function is
/// locale‑independent.
pub fn capitalize(str_: &str) -> String {
    trace!("capitalize: input = {}", str_);
    if str_.is_empty() {
        return str_.to_owned();
    }

    let mut bytes = str_.as_bytes().to_vec();
    if let Some(first) = bytes.get_mut(0) {
        if (*first >= b'a') && (*first <= b'z') {
            *first = *first - b'a' + b'A';
        }
    }

    // SAFETY: the original input is valid UTF‑8 and we only mutate byte 0 in
    // the ASCII range, so the result is still valid UTF‑8.
    String::from_utf8(bytes).expect("ASCII is valid UTF‑8")
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
        trace!("to_lower(String): input = {}", self);
        let mut out = String::with_capacity(self.len());
        for &byte in self.as_bytes() {
            let lowered = if (byte >= b'A') && (byte <= b'Z') {
                byte - b'A' + b'a'
            } else {
                byte
            };
            out.push(lowered as char);
        }
        out
    }
}

impl ToUpper for String {
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
    fn to_upper(&self) -> String {
        trace!("to_upper(String): input = {}", self);
        let mut out = String::with_capacity(self.len());
        for &byte in self.as_bytes() {
            let uppered = if (byte >= b'a') && (byte <= b'z') {
                byte - b'a' + b'A'
            } else {
                byte
            };
            out.push(uppered as char);
        }
        out
    }
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
    #[inline]
    fn to_lower(&self) -> u8 {
        let c = *self;
        if (c >= b'A') && (c <= b'Z') {
            c - b'A' + b'a'
        } else {
            c
        }
    }
}

impl ToUpper for u8 {

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
    #[inline]
    fn to_upper(&self) -> u8 {
        let c = *self;
        if (c >= b'a') && (c <= b'z') {
            c - b'a' + b'A'
        } else {
            c
        }
    }
}

#[cfg(test)]
mod tests_case_conversion {
    use super::*;

    #[traced_test]
    fn capitalize_basic_ascii() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[traced_test]
    fn capitalize_non_alpha_initial() {
        assert_eq!(capitalize("1world"), "1world");
    }

    #[traced_test]
    fn string_to_lower() {
        assert_eq!("HeLLo123".to_string().to_lower(), "hello123");
    }

    #[traced_test]
    fn string_to_upper() {
        assert_eq!("HeLLo123".to_string().to_upper(), "HELLO123");
    }

    #[traced_test]
    fn byte_case_roundtrip() {
        let upper_a: u8 = b'A';
        assert_eq!(upper_a.to_lower().to_upper(), upper_a);
    }
}
