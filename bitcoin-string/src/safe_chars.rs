crate::ix!();

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
