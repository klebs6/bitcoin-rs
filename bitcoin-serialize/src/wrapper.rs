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

/* -------- integration with Serialize / BtcUnserialize -------- */

impl<'a, F, T, S> BtcSerialize<S> for Wrapper<'a, F, T>
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

impl<'a, F, T, S> BtcUnserialize<S> for Wrapper<'a, F, T>
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

#[cfg(test)]
mod wrapper_tests {
    use super::*;
    use std::io::Cursor;

    #[traced_test]
    fn wrapper_roundtrip_with_varint_formatter() {
        let mut value  = 300u64;
        let mut buf    = Cursor::new(Vec::<u8>::new());

        /* ----------------‑‑ serialize ‑‑---------------- */
        {
            let wrapper = Wrapper::<
                VarIntFormatter<{ VarIntMode::Default }>,
                u64,
            >::new(&mut value);
            wrapper.serialize(&mut buf);
        }

        /* zap and read back */
        value = 0;
        buf.set_position(0);
        {
            let mut wrapper = Wrapper::<
                VarIntFormatter<{ VarIntMode::Default }>,
                u64,
            >::new(&mut value);
            wrapper.unserialize(&mut buf);
        }

        assert_eq!(value, 300);
    }

    // Higher‑ranked trait bound fixes   E0106 (“missing lifetime”) ----------------
    #[allow(dead_code)]
    fn _compile_time_traits()
    where
        for<'a> Wrapper<
            'a,
            VarIntFormatter<{ VarIntMode::Default }>,
            u64,
        >: crate::serialize::BtcSerialize<Cursor<Vec<u8>>>
         + crate::unserialize::BtcUnserialize<Cursor<Vec<u8>>>,
    {}
}
