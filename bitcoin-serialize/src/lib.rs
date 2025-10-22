// ---------------- [ File: bitcoin-serialize/src/lib.rs ]
#![allow(incomplete_features)]
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

x!{action}
x!{compact_size}
x!{compact_size_formatter}
x!{constants}
x!{custom_uint_formatter}
x!{default_formatter}
x!{get_serialize_size}
x!{limited_string_formatter}
x!{macros}
x!{many}
x!{map_into_range}
x!{meta}
x!{read_write}
x!{read_write_data}
x!{serialize}
x!{size_computer}
x!{unserialize}
x!{var_int_formatter}
x!{var_int_mode}
x!{check_var_int_mode}
x!{vector_formatter}
x!{wrapper}
x!{write_with_size_computer}
x!{formatter}
x!{encoded_double_formatter}
x!{primitives}
