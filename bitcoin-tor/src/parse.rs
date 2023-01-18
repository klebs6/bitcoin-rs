crate::ix!();

/****** General parsing utilities ********/

/**
  | Split reply line in the form 'AUTH METHODS=...'
  | into a type 'AUTH' and arguments 'METHODS=...'.
  | 
  | Grammar is implicitly defined in https://spec.torproject.org/control-spec
  | by the server reply formats for PROTOCOLINFO
  | (S3.21) and AUTHCHALLENGE (S3.24).
  |
  */
pub fn split_tor_reply_line(s: &String) -> (String,String) {
    
    todo!();
        /*
            size_t ptr=0;
        std::string type;
        while (ptr < s.size() && s[ptr] != ' ') {
            type.push_back(s[ptr]);
            ++ptr;
        }
        if (ptr < s.size())
            ++ptr; // skip ' '
        return make_pair(type, s.substr(ptr));
        */
}

/**
  | Parse reply arguments in the form 'METHODS=COOKIE,SAFECOOKIE
  | COOKIEFILE=".../control_auth_cookie"'.
  | 
  | Returns a map of keys to values, or an
  | empty map if there was an error.
  | 
  | Grammar is implicitly defined in https://spec.torproject.org/control-spec
  | by the server reply formats for PROTOCOLINFO
  | (S3.21), AUTHCHALLENGE (S3.24), and
  | ADD_ONION (S3.27). See also sections
  | 2.1 and 2.3.
  |
  */
pub fn parse_tor_reply_mapping(s: &String) -> HashMap<String,String> {
    
    todo!();
        /*
            std::map<std::string,std::string> mapping;
        size_t ptr=0;
        while (ptr < s.size()) {
            std::string key, value;
            while (ptr < s.size() && s[ptr] != '=' && s[ptr] != ' ') {
                key.push_back(s[ptr]);
                ++ptr;
            }
            if (ptr == s.size()) // unexpected end of line
                return std::map<std::string,std::string>();
            if (s[ptr] == ' ') // The remaining string is an OptArguments
                break;
            ++ptr; // skip '='
            if (ptr < s.size() && s[ptr] == '"') { // Quoted string
                ++ptr; // skip opening '"'
                bool escape_next = false;
                while (ptr < s.size() && (escape_next || s[ptr] != '"')) {
                    // Repeated backslashes must be interpreted as pairs
                    escape_next = (s[ptr] == '\\' && !escape_next);
                    value.push_back(s[ptr]);
                    ++ptr;
                }
                if (ptr == s.size()) // unexpected end of line
                    return std::map<std::string,std::string>();
                ++ptr; // skip closing '"'
                /**
                 * Unescape value. Per https://spec.torproject.org/control-spec section 2.1.1:
                 *
                 *   For future-proofing, controller implementors MAY use the following
                 *   rules to be compatible with buggy Tor implementations and with
                 *   future ones that implement the spec as intended:
                 *
                 *     Read \n \t \r and \0 ... \377 as C escapes.
                 *     Treat a backslash followed by any other character as that character.
                 */
                std::string escaped_value;
                for (size_t i = 0; i < value.size(); ++i) {
                    if (value[i] == '\\') {
                        // This will always be valid, because if the QuotedString
                        // ended in an odd number of backslashes, then the parser
                        // would already have returned above, due to a missing
                        // terminating double-quote.
                        ++i;
                        if (value[i] == 'n') {
                            escaped_value.push_back('\n');
                        } else if (value[i] == 't') {
                            escaped_value.push_back('\t');
                        } else if (value[i] == 'r') {
                            escaped_value.push_back('\r');
                        } else if ('0' <= value[i] && value[i] <= '7') {
                            size_t j;
                            // Octal escape sequences have a limit of three octal digits,
                            // but terminate at the first character that is not a valid
                            // octal digit if encountered sooner.
                            for (j = 1; j < 3 && (i+j) < value.size() && '0' <= value[i+j] && value[i+j] <= '7'; ++j) {}
                            // Tor restricts first digit to 0-3 for three-digit octals.
                            // A leading digit of 4-7 would therefore be interpreted as
                            // a two-digit octal.
                            if (j == 3 && value[i] > '3') {
                                j--;
                            }
                            escaped_value.push_back(strtol(value.substr(i, j).c_str(), nullptr, 8));
                            // Account for automatic incrementing at loop end
                            i += j - 1;
                        } else {
                            escaped_value.push_back(value[i]);
                        }
                    } else {
                        escaped_value.push_back(value[i]);
                    }
                }
                value = escaped_value;
            } else { // Unquoted value. Note that values can contain '=' at will, just no spaces
                while (ptr < s.size() && s[ptr] != ' ') {
                    value.push_back(s[ptr]);
                    ++ptr;
                }
            }
            if (ptr < s.size() && s[ptr] == ' ')
                ++ptr; // skip ' ' after key=value
            mapping[key] = value;
        }
        return mapping;
        */
}

