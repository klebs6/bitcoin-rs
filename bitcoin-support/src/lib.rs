#![feature(allocator_api)]
#![feature(slice_ptr_get)]

// ---------------- [ File: bitcoin-support/src/lib.rs ]

#[macro_use] mod imports; use imports::*;

x!{allocators_secure}
x!{allocators_zeroafterfree}
x!{cleanse}
x!{events}
x!{printchunk}
x!{align_up}
x!{getuniquepath}

pub struct Signal<T> { p: std::marker::PhantomData<T> }

pub trait GetName {

    fn get_name(&self) -> &'static str;
}
