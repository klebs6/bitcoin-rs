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
