#![feature(test)]
extern crate test;

macro_rules! mx { 
    ($x:ident) => { #[macro_use] mod $x; pub use $x::*; } 
}

#[macro_use] extern crate static_assertions;

#[macro_use] mod imports; use imports::*;

//----------------------------------------

x!{assumptions}
x!{gen_context}
x!{preallocated}
x!{secp256k1}
x!{selftest}
x!{testrand}
x!{tests}
x!{tests_exhaustive}
x!{valgrind_ctime_test}
