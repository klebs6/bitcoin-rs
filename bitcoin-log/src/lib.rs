// ---------------- [ File: bitcoin-log/src/lib.rs ]
#![feature(test)]
extern crate test;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate static_assertions;

extern crate libc;

#[macro_use] mod imports;
use imports::*;

x!{category}
x!{logger}
x!{timer}
x!{interface}
x!{trace}
