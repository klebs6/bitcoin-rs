// ---------------- [ File: bitcoin-univalue/src/univalue_gen_main.rs ]
/*!
  | To re-create univalue_escapes.h:
  | $ g++ -o gen gen.cpp
  | $ ./gen > univalue_escapes.h
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/gen/gen.cpp]

/// Stand‑alone helper mirroring the original `gen.cpp`.  
/// It initialises the escape lookup table and immediately
/// writes it to *stdout*.
///
/// The arguments are ignored – they exist solely to keep the
/// C++ signature intact.
#[instrument(level = "trace", skip_all)]
pub fn univalue_gen_main(_argc: i32, _argv: &[*mut u8]) -> i32 {
    init_json_escape();
    output_escape();
    0
}

#[cfg(test)]
mod univalue_gen_main_spec {
    use super::*;

    #[traced_test]
    fn returns_zero() {
        // We do **not** capture stdout here – that is covered by
        // `output_escape`’s own test‑suite.
        assert_eq!(univalue_gen_main(0, &[]), 0);
    }
}
