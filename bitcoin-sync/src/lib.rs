// ---------------- [ File: bitcoin-sync/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

pub struct RemovePointer<T>   {p: std::marker::PhantomData<T>}
pub struct RemoveReference<T> {p: std::marker::PhantomData<T>}

x!{annotated_mixin}
x!{reverse_lock}
x!{debug_lockorder}
x!{macros}
x!{scoped_raw_mutex}
x!{semaphore}
x!{semaphore_grant}
x!{thread}
x!{threadinterrupt}
x!{types}
x!{unique_lock}
x!{lock_api}
