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
    /// Parse *raw[0 … size)* as JSON, storing the result in `self`.
    /// The logic is a direct, line‑for‑line translation of Bitcoin‑Core’s
    /// `UniValue::read`, including depth‑checking and expectation masks.
    #[instrument(level = "trace", skip(self))]
    pub fn read(&mut self, raw: *const u8, size: usize) -> bool {
        use crate::ExpectBits::{EXP_ARR_VALUE, EXP_COLON, EXP_NOT_VALUE, EXP_OBJ_NAME, EXP_VALUE};

        unsafe {

            self.clear();

            let mut expect_mask: u32 = 0;
            let mut stack: Vec<*mut UniValue> = Vec::new();

            let mut token_val = String::new();
            let mut consumed:  u32 = 0;
            let mut tok        = JTokenType::JTOK_NONE;
            let mut last_tok   = JTokenType::JTOK_NONE;

            let mut p   = raw;
            let end_ptr = raw.add(size);

            while {
                last_tok = tok;
                tok = get_json_token(&mut token_val, &mut consumed, p, end_ptr);
                if matches!(tok, JTokenType::JTOK_NONE | JTokenType::JTOK_ERR) {
                    return false;
                }

                /* advance past the token itself */
                p = p.add(consumed as usize);

                /* NEW: consume any subsequent whitespace or NUL padding       *
                 *      (Bitcoin‑Core’s C++ version silently tolerates both).   */
                while p < end_ptr && ((*p == 0) || json_isspace(*p as i32)) {
                    p = p.add(1);
                }

                /* ---------- expectation bookkeeping ---------- */
                let is_value_open = json_token_is_value(tok)
                    || tok == JTokenType::JTOK_OBJ_OPEN
                    || tok == JTokenType::JTOK_ARR_OPEN;


                if expect!(expect_mask, EXP_VALUE) {
                    if !is_value_open {
                        return false;
                    }
                    clear_expect!(expect_mask, EXP_VALUE);
                } else if expect!(expect_mask, EXP_ARR_VALUE) {
                    let is_arr_val = is_value_open || tok == JTokenType::JTOK_ARR_CLOSE;
                    if !is_arr_val {
                        return false;
                    }
                    clear_expect!(expect_mask, EXP_ARR_VALUE);
                } else if expect!(expect_mask, EXP_OBJ_NAME) {
                    let ok = tok == JTokenType::JTOK_OBJ_CLOSE || tok == JTokenType::JTOK_STRING;
                    if !ok {
                        return false;
                    }
                } else if expect!(expect_mask, EXP_COLON) {
                    if tok != JTokenType::JTOK_COLON {
                        return false;
                    }
                    clear_expect!(expect_mask, EXP_COLON);
                } else if !expect!(expect_mask, EXP_COLON) && tok == JTokenType::JTOK_COLON {
                    return false;
                }

                if expect!(expect_mask, EXP_NOT_VALUE) {
                    if is_value_open {
                        return false;
                    }
                    clear_expect!(expect_mask, EXP_NOT_VALUE);
                }

                /* ---------- state machine ---------- */
                match tok {
                    /* ----- structure openers ----- */
                    JTokenType::JTOK_OBJ_OPEN | JTokenType::JTOK_ARR_OPEN => {
                        let utyp = if tok == JTokenType::JTOK_OBJ_OPEN {
                            uni_value::VType::VOBJ
                        } else {
                            uni_value::VType::VARR
                        };

                        if stack.is_empty() {
                            if utyp == uni_value::VType::VOBJ {
                                self.set_object();
                            } else {
                                self.set_array();
                            }
                            stack.push(self as *mut UniValue);
                        } else {
                            let mut tmp = UniValue::new(utyp, None);
                            let top = *stack.last().unwrap();
                            (*top).values_mut().push(tmp);
                            let new_top = (*top).values_mut().last_mut().unwrap() as *mut UniValue;
                            stack.push(new_top);
                        }

                        if stack.len() > MAX_JSON_DEPTH {
                            return false;
                        }

                        if utyp == uni_value::VType::VOBJ {
                            set_expect!(expect_mask, EXP_OBJ_NAME);
                        } else {
                            set_expect!(expect_mask, EXP_ARR_VALUE);
                        }
                    }

                    /* ----- structure closers ----- */
                    JTokenType::JTOK_OBJ_CLOSE | JTokenType::JTOK_ARR_CLOSE => {
                        if stack.is_empty() || last_tok == JTokenType::JTOK_COMMA {
                            return false;
                        }

                        let utyp = if tok == JTokenType::JTOK_OBJ_CLOSE {
                            uni_value::VType::VOBJ
                        } else {
                            uni_value::VType::VARR
                        };

                        let top = *stack.last().unwrap();
                        if (*top).get_type() != utyp {
                            return false;
                        }
                        stack.pop();
                        clear_expect!(expect_mask, EXP_OBJ_NAME);
                        set_expect!(expect_mask, EXP_NOT_VALUE);
                    }

                    /* ----- colon / comma ----- */
                    JTokenType::JTOK_COLON => {
                        if stack.is_empty() {
                            return false;
                        }
                        let top = *stack.last().unwrap();
                        if (*top).get_type() != uni_value::VType::VOBJ {
                            return false;
                        }
                        set_expect!(expect_mask, EXP_VALUE);
                    }

                    JTokenType::JTOK_COMMA => {
                        if stack.is_empty()
                            || last_tok == JTokenType::JTOK_COMMA
                            || last_tok == JTokenType::JTOK_ARR_OPEN
                        {
                            return false;
                        }
                        let top = *stack.last().unwrap();
                        if (*top).get_type() == uni_value::VType::VOBJ {
                            set_expect!(expect_mask, EXP_OBJ_NAME);
                        } else {
                            set_expect!(expect_mask, EXP_ARR_VALUE);
                        }
                    }

                    /* ----- literals ----- */
                    JTokenType::JTOK_KW_NULL | JTokenType::JTOK_KW_TRUE | JTokenType::JTOK_KW_FALSE => {
                        let mut tmp = UniValue::default();
                        if tok == JTokenType::JTOK_KW_TRUE {
                            tmp.set_bool(true);
                        } else if tok == JTokenType::JTOK_KW_FALSE {
                            tmp.set_bool(false);
                        }

                        if stack.is_empty() {
                            *self = tmp;
                        } else {
                            let top = *stack.last().unwrap();
                            (*top).values_mut().push(tmp);
                            set_expect!(expect_mask, EXP_NOT_VALUE);
                        }
                    }

                    /* ----- numbers ----- */
                    JTokenType::JTOK_NUMBER => {
                        let mut tmp = UniValue::new(uni_value::VType::VNUM, Some(&token_val));
                        if stack.is_empty() {
                            *self = tmp;
                        } else {
                            let top = *stack.last().unwrap();
                            (*top).values_mut().push(tmp);
                            set_expect!(expect_mask, EXP_NOT_VALUE);
                        }
                    }

                    /* ----- strings ----- */
                    JTokenType::JTOK_STRING => {
                        if expect!(expect_mask, EXP_OBJ_NAME) {
                            let top = *stack.last().unwrap();
                            (*top).keys_mut().push(token_val.clone());
                            clear_expect!(expect_mask, EXP_OBJ_NAME);
                            set_expect!(expect_mask, EXP_COLON);
                        } else {
                            let mut tmp = UniValue::new(uni_value::VType::VSTR, Some(&token_val));
                            if stack.is_empty() {
                                *self = tmp;
                            } else {
                                let top = *stack.last().unwrap();
                                (*top).values_mut().push(tmp);
                                set_expect!(expect_mask, EXP_NOT_VALUE);
                            }
                        }
                    }

                    /* ----- anything else ----- */
                    _ => return false,
                };

                !stack.is_empty()
            } {}

            /* ---------- ensure no trailing junk ---------- */
            let mut dummy = String::new();
            let mut n     = 0u32;
            if get_json_token(&mut dummy, &mut n, p, end_ptr) != JTokenType::JTOK_NONE {
                return false;
            }
            true
        }
    }
}

#[cfg(test)]
mod read_spec {
    use super::*;

    fn parse(src: &str) -> bool {
        let mut uv = UniValue::default();
        uv.read(src.as_ptr(), src.len())
    }

    #[traced_test]
    fn parses_scalars() {
        for s in ["null", "true", "false", "123", r#""hi""#] {
            assert!(parse(s));
        }
    }

    #[traced_test]
    fn parses_structures() {
        assert!(parse("[1,2,3]"));
        assert!(parse(r#"{"a":1,"b":[true,false]}"#));
    }

    #[traced_test]
    fn depth_limit_enforced() {
        let deep = "[".repeat(513) + &"]".repeat(513);
        assert!(!parse(&deep));
    }

    #[traced_test]
    fn detects_errors() {
        for bad in ["[1,2,]", r#"{"a" 1}"#, "{]"] {
            assert!(!parse(bad));
        }
    }
}
