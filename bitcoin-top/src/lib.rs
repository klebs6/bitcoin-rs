#![feature(test)]

extern crate test;
extern crate atomic;
extern crate libc;

#[macro_use] extern crate modular_bitfield;

pub fn no_op() {}

#[macro_use] extern crate static_assertions;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitcoin_derive;
#[macro_use] extern crate maplit;

#[macro_use] extern crate bitcoin_leveldb   as leveldb;
#[macro_use] extern crate bitcoin_chain_consensus as consensus;

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }

#[macro_export] macro_rules! ix { 
    () => { 
        use crate::{ 
            imports::* , 
        };
        use crate::*;
    } 
}


macro_rules! ternary {
    ($condition:expr,$if_true:expr,$if_false:expr) => {
        match $condition {
            true => $if_true,
            false => $if_false,
        }
    }
}

//TODO: compile time error message?
macro_rules! static_assert {
    ($b:expr) => {  
        const_assert!(b);
    }
}

pub mod imports;

pub struct RemovePointer<T>   {p: std::marker::PhantomData<T>}
pub struct RemoveReference<T> {p: std::marker::PhantomData<T>}

