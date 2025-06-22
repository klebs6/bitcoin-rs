// ---------------- [ File: bitcoin-univalue/src/jtoken_type.rs ]
crate::ix!();

pub enum JTokenType {
    JTOK_ERR        = -1,
    JTOK_NONE       = 0,                           // eof
    JTOK_OBJ_OPEN,
    JTOK_OBJ_CLOSE,
    JTOK_ARR_OPEN,
    JTOK_ARR_CLOSE,
    JTOK_COLON,
    JTOK_COMMA,
    JTOK_KW_NULL,
    JTOK_KW_TRUE,
    JTOK_KW_FALSE,
    JTOK_NUMBER,
    JTOK_STRING,
}

lazy_static!{
    /*
    extern enum jtokentype getJsonToken(std::string& tokenVal,
                                        unsigned int& consumed, const char *raw, const char *end);
    */
}

lazy_static!{
    /*
    extern const char *uvTypeName(UniValue::VType t);
    */
}
