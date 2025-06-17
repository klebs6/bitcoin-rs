// ---------------- [ File: bitcoin-serialize/src/wrapper.rs ]
crate::ix!();

/// Adapter that applies a user‑supplied `Formatter` when (de)serialising
/// an existing value.
///
/// The wrapper holds a **mutable** reference so the same value can be
/// deserialised into after it has been serialised out, mirroring the C++
/// behaviour of Bitcoin Core’s `Using<Formatter>(obj)` helper.
pub struct Wrapper<'a, F, T> {
    object: T,
    _marker: PhantomData<&'a mut F>,
}

impl<'a, F, T> Wrapper<'a, F, T> {
    /// Construct a new wrapper around `object`.
    #[inline]
    pub const fn new(object: T) -> Self {
        Self {
            object,
            _marker: PhantomData,
        }
    }
}

/* ---------- `Serialize` / `Unserialize` glue ---------- */

impl<'a, F, T, Stream> crate::serialize::Serialize<Stream> for Wrapper<'a, F, T>
where
    Stream: Write,
    F: Default,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!("Wrapper::serialize using {}", std::any::type_name::<F>());
        let mut fmt = F::default();
        // SAFETY: `self.object` is an immutable reference when serialising.
        fmt.ser(s, &self.object);
    }
}

impl<'a, F, T, Stream> crate::unserialize::Unserialize<Stream> for Wrapper<'a, F, T>
where
    Stream: Read,
    F: Default,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        trace!("Wrapper::unserialize using {}", std::any::type_name::<F>());
        let mut fmt = F::default();
        // SAFETY: `self.object` is a mutable reference when deserialising.
        fmt.unser(s, &mut self.object);
    }
}

