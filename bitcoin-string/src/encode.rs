crate::ix!();

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
