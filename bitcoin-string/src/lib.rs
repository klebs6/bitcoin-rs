#![feature(test)]
extern crate test;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate static_assertions;

#[macro_use] mod imports;
use imports::*;

x!{string}
x!{translation}
x!{strencodings}
x!{moneystr}
