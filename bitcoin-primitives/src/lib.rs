#![feature(generic_const_exprs)]

#![feature(test)]
extern crate test;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate static_assertions;

#[macro_use] mod imports; use imports::*;

//-------------------------------------------[.cpp/bitcoin/src/uint256.h]
//-------------------------------------------[.cpp/bitcoin/src/uint256.cpp]

x!{arith}
x!{blob}
x!{streams}
x!{common}
x!{checkpoint}
