// ---------------- [ File: bitcoin-get-json-token/src/json_isdigit.rs ]
crate::ix!();

pub fn json_isdigit(ch: i32) -> bool {

    let ch: u8 = ch.try_into().unwrap();

    let ch = char::from(ch);
    
    (ch >= '0') && (ch <= '9')
}
