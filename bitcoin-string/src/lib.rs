// ---------------- [ File: bitcoin-string/src/lib.rs ]
#![feature(test)]
extern crate test;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate static_assertions;

#[macro_use] mod imports;
use imports::*;

x!{bilingual}
x!{capitalize}
x!{check}
x!{convert_bits}
x!{decode}
x!{decode64}
x!{encode}
x!{format_money}
x!{hex}
x!{locale_independent_atoi}
x!{parse_fixed_point}
x!{parse_integral}
x!{parse_money}
x!{process_mantissa_digit}
x!{safe_chars}
x!{sanitize}
x!{trim}
x!{timing_resistant_equal}
x!{to_integral}
x!{traits}

//-------------------------------------------[.cpp/bitcoin/src/util/strencodings.h]
//-------------------------------------------[.cpp/bitcoin/src/util/strencodings.cpp]
