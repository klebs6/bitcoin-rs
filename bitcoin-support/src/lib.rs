#![feature(allocator_api)]

#[macro_use] mod imports; use imports::*;

x!{allocators_secure}
x!{allocators_zeroafterfree}
x!{cleanse}
x!{events}
x!{lockedpool}
x!{getuniquepath}
x!{system}

pub struct Signal<T> { p: std::marker::PhantomData<T> }

pub trait GetName {

    fn get_name(&self) -> &'static str;
}
