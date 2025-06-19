crate::ix!();

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
