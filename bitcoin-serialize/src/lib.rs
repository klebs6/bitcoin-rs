#![feature(adt_const_params)]
#![feature(allocator_api)]
#![feature(const_generics_defaults)]
#![feature(generic_const_exprs)]
#![feature(test)]

extern crate test;

#[macro_use] extern crate static_assertions;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitcoin_derive;
#[macro_use] extern crate maplit;

macro_rules! ternary {
    ($condition:expr,$if_true:expr,$if_false:expr) => {
        match $condition {
            true => $if_true,
            false => $if_false,
        }
    }
}

#[macro_use] mod imports; use imports::*;

x!{serialize}
