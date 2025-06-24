// ---------------- [ File: bitcoin-univalue/src/test_no_nul.rs ]
use bitcoin_univalue::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/univalue/test/no_nul.cpp]

pub fn univalue_test_no_nul_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    let buf: &[u8] = b"___[1,2,3]___";

    let mut val = UniValue::default();

    match unsafe { val.read(buf.as_ptr().add(3), 7) } {
        true   => 0,
        false  => 1
    }
}
