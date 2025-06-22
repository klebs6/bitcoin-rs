// ---------------- [ File: bitcoin-univalue/src/get_json_token.rs ]
crate::ix!();

pub fn get_json_token(
    token_val: &mut String,
    consumed:  &mut u32,
    raw:       *const u8,
    end:       *const u8) -> JTokenType {

    todo!();
        /*
        tokenVal.clear();
        consumed = 0;

        const char *rawStart = raw;

        while (raw < end && (json_isspace(*raw)))          // skip whitespace
            raw++;

        if (raw >= end)
            return JTOK_NONE;

        switch (*raw) {

        case '{':
            raw++;
            consumed = (raw - rawStart);
            return JTOK_OBJ_OPEN;
        case '}':
            raw++;
            consumed = (raw - rawStart);
            return JTOK_OBJ_CLOSE;
        case '[':
            raw++;
            consumed = (raw - rawStart);
            return JTOK_ARR_OPEN;
        case ']':
            raw++;
            consumed = (raw - rawStart);
            return JTOK_ARR_CLOSE;

        case ':':
            raw++;
            consumed = (raw - rawStart);
            return JTOK_COLON;
        case ',':
            raw++;
            consumed = (raw - rawStart);
            return JTOK_COMMA;

        case 'n':
        case 't':
        case 'f':
            if (!strncmp(raw, "null", 4)) {
                raw += 4;
                consumed = (raw - rawStart);
                return JTOK_KW_NULL;
            } else if (!strncmp(raw, "true", 4)) {
                raw += 4;
                consumed = (raw - rawStart);
                return JTOK_KW_TRUE;
            } else if (!strncmp(raw, "false", 5)) {
                raw += 5;
                consumed = (raw - rawStart);
                return JTOK_KW_FALSE;
            } else
                return JTOK_ERR;

        case '-':
        case '0':
        case '1':
        case '2':
        case '3':
        case '4':
        case '5':
        case '6':
        case '7':
        case '8':
        case '9': {
            // part 1: int
            std::string numStr;

            const char *first = raw;

            const char *firstDigit = first;
            if (!json_isdigit(*firstDigit))
                firstDigit++;
            if ((*firstDigit == '0') && json_isdigit(firstDigit[1]))
                return JTOK_ERR;

            numStr += *raw;                       // copy first char
            raw++;

            if ((*first == '-') && (raw < end) && (!json_isdigit(*raw)))
                return JTOK_ERR;

            while (raw < end && json_isdigit(*raw)) {  // copy digits
                numStr += *raw;
                raw++;
            }

            // part 2: frac
            if (raw < end && *raw == '.') {
                numStr += *raw;                   // copy .
                raw++;

                if (raw >= end || !json_isdigit(*raw))
                    return JTOK_ERR;
                while (raw < end && json_isdigit(*raw)) { // copy digits
                    numStr += *raw;
                    raw++;
                }
            }

            // part 3: exp
            if (raw < end && (*raw == 'e' || *raw == 'E')) {
                numStr += *raw;                   // copy E
                raw++;

                if (raw < end && (*raw == '-' || *raw == '+')) { // copy +/-
                    numStr += *raw;
                    raw++;
                }

                if (raw >= end || !json_isdigit(*raw))
                    return JTOK_ERR;
                while (raw < end && json_isdigit(*raw)) { // copy digits
                    numStr += *raw;
                    raw++;
                }
            }

            tokenVal = numStr;
            consumed = (raw - rawStart);
            return JTOK_NUMBER;
            }

        case '"': {
            raw++;                                // skip "

            std::string valStr;
            JSONUTF8StringFilter writer(valStr);

            while (true) {
                if (raw >= end || (unsigned char)*raw < 0x20)
                    return JTOK_ERR;

                else if (*raw == '\\') {
                    raw++;                        // skip backslash

                    if (raw >= end)
                        return JTOK_ERR;

                    switch (*raw) {
                    case '"':  writer.push_back('\"'); break;
                    case '\\': writer.push_back('\\'); break;
                    case '/':  writer.push_back('/'); break;
                    case 'b':  writer.push_back('\b'); break;
                    case 'f':  writer.push_back('\f'); break;
                    case 'n':  writer.push_back('\n'); break;
                    case 'r':  writer.push_back('\r'); break;
                    case 't':  writer.push_back('\t'); break;

                    case 'u': {
                        unsigned int codepoint;
                        if (raw + 1 + 4 >= end ||
                            hatoui(raw + 1, raw + 1 + 4, codepoint) !=
                                   raw + 1 + 4)
                            return JTOK_ERR;
                        writer.push_back_u(codepoint);
                        raw += 4;
                        break;
                        }
                    default:
                        return JTOK_ERR;

                    }

                    raw++;                        // skip esc'd char
                }

                else if (*raw == '"') {
                    raw++;                        // skip "
                    break;                        // stop scanning
                }

                else {
                    writer.push_back(static_cast<unsigned char>(*raw));
                    raw++;
                }
            }

            if (!writer.finalize())
                return JTOK_ERR;
            tokenVal = valStr;
            consumed = (raw - rawStart);
            return JTOK_STRING;
            }

        default:
            return JTOK_ERR;
        }
        */
}
