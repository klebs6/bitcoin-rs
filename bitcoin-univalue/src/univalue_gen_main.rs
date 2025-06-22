// ---------------- [ File: bitcoin-univalue/src/gen.rs ]
/*!
  | To re-create univalue_escapes.h:
  | $ g++ -o gen gen.cpp
  | $ ./gen > univalue_escapes.h
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/gen/gen.cpp]

pub fn univalue_gen_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
        initJsonEscape();
        outputEscape();
        return 0;
        */
}
