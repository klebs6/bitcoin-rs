crate::ix!();

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
