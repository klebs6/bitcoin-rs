// ---------------- [ File: bitcoin-serialize/src/formatter.rs ]
crate::ix!();

/// **Common interface implemented by every formatter.**
///
/// A formatter is responsible for writing a value of type `T` to a
/// byte‑stream (`ser`) and reading it back (`unser`).  This is the glue
/// that lets the `Wrapper` and `VectorFormatter` compose formatters in a
/// type‑safe way.
pub trait ValueFormatter<T>: Default {
    fn ser<S: Write>(&mut self, s: &mut S, value: &T);
    fn unser<S: Read>(&mut self, s: &mut S, value: &mut T);
}
