#[macro_use] mod imports; use imports::*;

x!{sync_impl}
x!{threadinterrupt}
x!{thread}
x!{scoped_raw_mutex}

pub struct RemovePointer<T>   {p: std::marker::PhantomData<T>}
pub struct RemoveReference<T> {p: std::marker::PhantomData<T>}
