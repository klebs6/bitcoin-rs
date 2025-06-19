// ---------------- [ File: bitcoin-serialize/src/wrapper.rs ]
crate::ix!();

/// Wrapper that forces (de)serialisation of the enclosed object to go
/// through a user‑supplied `Formatter`.
///
/// Construct via [`using`](crate::serialize::using).
pub struct Wrapper<'a, F, T> {
    object: &'a mut T,
    _marker: PhantomData<F>,
}

impl<'a, F, T> Wrapper<'a, F, T> {
    #[inline]
    pub const fn new(object: &'a mut T) -> Self {
        Self {
            object,
            _marker: PhantomData,
        }
    }
}

/* -------- integration with Serialize / Unserialize -------- */

impl<'a, F, T, S> crate::serialize::Serialize<S> for Wrapper<'a, F, T>
where
    S: Write,
    F: ValueFormatter<T>,
{
    #[inline]
    fn serialize(&self, s: &mut S) {
        trace!(
            fmt = %std::any::type_name::<F>(),
            "Wrapper::serialize"
        );
        let mut fmt = F::default();
        fmt.ser(s, &*self.object);
    }
}

impl<'a, F, T, S> crate::unserialize::Unserialize<S> for Wrapper<'a, F, T>
where
    S: Read,
    F: ValueFormatter<T>,
{
    #[inline]
    fn unserialize(&mut self, s: &mut S) {
        trace!(
            fmt = %std::any::type_name::<F>(),
            "Wrapper::unserialize"
        );
        let mut fmt = F::default();
        fmt.unser(s, &mut *self.object);
    }
}
