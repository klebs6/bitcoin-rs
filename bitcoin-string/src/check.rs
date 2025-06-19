crate::ix!();

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
