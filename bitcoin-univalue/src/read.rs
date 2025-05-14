// ---------------- [ File: bitcoin-univalue/src/read.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue_read.cpp]

/**
  | According to stackexchange, the original
  | json test suite wanted to limit depth
  | to 22. Widely-deployed PHP bails at
  | depth 512, so we will follow PHP's lead,
  | which should be more than sufficient
  | (further stackexchange comments indicate
  | depth > 32 rarely occurs).
  |
  */
pub const MAX_JSON_DEPTH: usize = 512;

pub fn json_isdigit(ch: i32) -> bool {

    let ch: u8 = ch.try_into().unwrap();

    let ch = char::from(ch);
    
    (ch >= '0') && (ch <= '9')
}

/**
  | convert hexadecimal string to unsigned
  | integer
  |
  */
pub fn hatoui(
        first: *const u8,
        last:  *const u8,
        out:   &mut u32) -> *const u8 {
    
    let mut result: u32 = 0;

    todo!();
        /*

        for (; first != last; ++first)
        {
            int digit;
            if (json_isdigit(*first))
                digit = *first - '0';

            else if (*first >= 'a' && *first <= 'f')
                digit = *first - 'a' + 10;

            else if (*first >= 'A' && *first <= 'F')
                digit = *first - 'A' + 10;

            else
                break;

            result = 16 * result + digit;
        }
        out = result;

        */

    first
}

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

#[repr(u32)]
pub enum expect_bits {
    EXP_OBJ_NAME  = 1 << 0,
    EXP_COLON     = 1 << 1,
    EXP_ARR_VALUE = 1 << 2,
    EXP_VALUE     = 1 << 3,
    EXP_NOT_VALUE = 1 << 4,
}

macro_rules! expect {
    ($bit:ident) => {
        /*
                (expectMask & (EXP_##bit))
        */
    }
}

macro_rules! set_expect {
    ($bit:ident) => {
        /*
                (expectMask |= EXP_##bit)
        */
    }
}

macro_rules! clear_expect {
    ($bit:ident) => {
        /*
                (expectMask &= ~EXP_##bit)
        */
    }
}

impl UniValue {
    
    pub fn read(&mut self, 
        raw:  *const u8,
        size: usize) -> bool {
        
        todo!();
        /*
        clear();

        uint32_t expectMask = 0;
        std::vector<UniValue*> stack;

        std::string tokenVal;
        unsigned int consumed;
        enum jtokentype tok = JTOK_NONE;
        enum jtokentype last_tok = JTOK_NONE;
        const char* end = raw + size;
        do {
            last_tok = tok;

            tok = getJsonToken(tokenVal, consumed, raw, end);
            if (tok == JTOK_NONE || tok == JTOK_ERR)
                return false;
            raw += consumed;

            bool isValueOpen = jsonTokenIsValue(tok) ||
                tok == JTOK_OBJ_OPEN || tok == JTOK_ARR_OPEN;

            if (expect(VALUE)) {
                if (!isValueOpen)
                    return false;
                clearExpect(VALUE);

            } else if (expect(ARR_VALUE)) {
                bool isArrValue = isValueOpen || (tok == JTOK_ARR_CLOSE);
                if (!isArrValue)
                    return false;

                clearExpect(ARR_VALUE);

            } else if (expect(OBJ_NAME)) {
                bool isObjName = (tok == JTOK_OBJ_CLOSE || tok == JTOK_STRING);
                if (!isObjName)
                    return false;

            } else if (expect(COLON)) {
                if (tok != JTOK_COLON)
                    return false;
                clearExpect(COLON);

            } else if (!expect(COLON) && (tok == JTOK_COLON)) {
                return false;
            }

            if (expect(NOT_VALUE)) {
                if (isValueOpen)
                    return false;
                clearExpect(NOT_VALUE);
            }

            switch (tok) {

            case JTOK_OBJ_OPEN:
            case JTOK_ARR_OPEN: {
                VType utyp = (tok == JTOK_OBJ_OPEN ? VOBJ : VARR);
                if (!stack.size()) {
                    if (utyp == VOBJ)
                        setObject();
                    else
                        setArray();
                    stack.push_back(this);
                } else {
                    UniValue tmpVal(utyp);
                    UniValue *top = stack.back();
                    top->values.push_back(tmpVal);

                    UniValue *newTop = &(top->values.back());
                    stack.push_back(newTop);
                }

                if (stack.size() > MAX_JSON_DEPTH)
                    return false;

                if (utyp == VOBJ)
                    setExpect(OBJ_NAME);
                else
                    setExpect(ARR_VALUE);
                break;
                }

            case JTOK_OBJ_CLOSE:
            case JTOK_ARR_CLOSE: {
                if (!stack.size() || (last_tok == JTOK_COMMA))
                    return false;

                VType utyp = (tok == JTOK_OBJ_CLOSE ? VOBJ : VARR);
                UniValue *top = stack.back();
                if (utyp != top->getType())
                    return false;

                stack.pop_back();
                clearExpect(OBJ_NAME);
                setExpect(NOT_VALUE);
                break;
                }

            case JTOK_COLON: {
                if (!stack.size())
                    return false;

                UniValue *top = stack.back();
                if (top->getType() != VOBJ)
                    return false;

                setExpect(VALUE);
                break;
                }

            case JTOK_COMMA: {
                if (!stack.size() ||
                    (last_tok == JTOK_COMMA) || (last_tok == JTOK_ARR_OPEN))
                    return false;

                UniValue *top = stack.back();
                if (top->getType() == VOBJ)
                    setExpect(OBJ_NAME);
                else
                    setExpect(ARR_VALUE);
                break;
                }

            case JTOK_KW_NULL:
            case JTOK_KW_TRUE:
            case JTOK_KW_FALSE: {
                UniValue tmpVal;
                switch (tok) {
                case JTOK_KW_NULL:
                    // do nothing more
                    break;
                case JTOK_KW_TRUE:
                    tmpVal.setBool(true);
                    break;
                case JTOK_KW_FALSE:
                    tmpVal.setBool(false);
                    break;
                default: /* impossible */ break;
                }

                if (!stack.size()) {
                    *this = tmpVal;
                    break;
                }

                UniValue *top = stack.back();
                top->values.push_back(tmpVal);

                setExpect(NOT_VALUE);
                break;
                }

            case JTOK_NUMBER: {
                UniValue tmpVal(VNUM, tokenVal);
                if (!stack.size()) {
                    *this = tmpVal;
                    break;
                }

                UniValue *top = stack.back();
                top->values.push_back(tmpVal);

                setExpect(NOT_VALUE);
                break;
                }

            case JTOK_STRING: {
                if (expect(OBJ_NAME)) {
                    UniValue *top = stack.back();
                    top->keys.push_back(tokenVal);
                    clearExpect(OBJ_NAME);
                    setExpect(COLON);
                } else {
                    UniValue tmpVal(VSTR, tokenVal);
                    if (!stack.size()) {
                        *this = tmpVal;
                        break;
                    }
                    UniValue *top = stack.back();
                    top->values.push_back(tmpVal);
                }

                setExpect(NOT_VALUE);
                break;
                }

            default:
                return false;
            }
        } while (!stack.empty ());

        /* Check that nothing follows the initial construct (parsed above).  */
        tok = getJsonToken(tokenVal, consumed, raw, end);
        if (tok != JTOK_NONE)
            return false;

        return true;
        */
    }
}
