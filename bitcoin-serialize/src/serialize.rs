// ---------------- [ File: bitcoin-serialize/src/serialize.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/serialize.h]

/*
  | Templates for serializing to anything
  | that looks like a stream, i.e. anything
  | that supports .read(char*, size_t)
  | and .write(char*, size_t)
  |
  */
pub trait Serialize<Stream> {
    fn serialize(&self, s: &mut Stream);
}

impl<Stream> Serialize<Stream> for i8
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize i8");
        ser_writedata8(s, *self as u8);
    }
}

impl<Stream> Serialize<Stream> for u8
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize u8");
        ser_writedata8(s, *self);
    }
}

impl<Stream> Serialize<Stream> for i16
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize i16");
        ser_writedata16(s, *self as u16);
    }
}

impl<Stream> Serialize<Stream> for u16
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize u16");
        ser_writedata16(s, *self);
    }
}


impl<Stream> Serialize<Stream> for i32
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize i32");
        ser_writedata32(s, *self as u32);
    }
}

impl<Stream> Serialize<Stream> for u32
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize u32");
        ser_writedata32(s, *self);
    }
}

impl<Stream> Serialize<Stream> for i64
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize i64");
        ser_writedata64(s, *self as u64);
    }
}

impl<Stream> Serialize<Stream> for u64
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize u64");
        ser_writedata64(s, *self);
    }
}

impl<Stream, const N: usize> Serialize<Stream> for [u8; N]
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(len = N, "serialize [u8; N]");
        s.write_all(self)
            .expect("I/O error while writing fixed‑length byte array");
    }
}

impl<'a, Stream> Serialize<Stream> for &'a [u8]
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(len = self.len(), "serialize &[u8]");
        s.write_all(*self)
            .expect("I/O error while writing byte slice");
    }
}

impl<Stream> Serialize<Stream> for bool
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize bool");
        ser_writedata8(s, if *self { 1 } else { 0 });
    }
}

/// Construct a `Wrapper` that forces (de)serialisation of `t` to go
/// through the given `Formatter`.
///
/// This mirrors Bitcoin Core’s `Using<Formatter>(obj)` helper.
///
/// Cause serialization/deserialization of an object to be done using a specified formatter class.
/// 
/// To use this, you need a class Formatter that has public functions Ser(stream, const object&)
/// for serialization, and Unser(stream, object&) for deserialization. Serialization routines
/// (inside
/// 
/// READWRITE, or directly with << and >> operators), can then use Using<Formatter>(object).
/// 
/// This works by constructing a Wrapper<Formatter, T>-wrapped version of object, where T is const
/// during serialization, and non-const during deserialization, which maintains const correctness.
///
/// Helper identical in spirit to Bitcoin Core’s `Using<Formatter>(obj)`.
///
/// ```ignore
/// let mut value = 42u64;
/// stream << using::<VarIntFormatter<VarIntMode::Default>, _>(&mut value);
/// ```
#[inline]
pub fn using<'a, F, T>(t: &'a mut T) -> Wrapper<'a, F, T> {
    Wrapper::new(t)
}


impl<Stream> Serialize<Stream> for String
where
    Stream: Write,
{
    fn serialize(&self, os: &mut Stream) {
        write_compact_size(os, self.len() as u64);
        if !self.is_empty() {
            os.write_all(self.as_bytes())
                .expect("I/O error while writing String");
        }
        trace!(len = self.len(), "serialize String");
    }
}

impl<Stream, T, const N: usize> Serialize<Stream> for PreVector<T, N>
where
    Stream: Write,
    T: Default + crate::serialize::Serialize<Stream>,
{
    /// Serialise a `PreVector` by
    /// 1. writing its length as a CompactSize,
    /// 2. serialising every element with the
    ///    element’s own `Serialize` impl.
    ///
    /// The implementation matches the DoS‑safe
    /// element‑by‑element strategy used for
    /// `Vec<T>` above while preserving the
    /// original fast‑path for fixed buffers.
    #[inline]
    fn serialize(&self, os: &mut Stream) {
        write_compact_size(os, self.len() as u64);
        trace!(
            len       = self.len(),
            capacity  = N,
            "serialize PreVector"
        );

        for elem in self.iter() {
            crate::serialize::Serialize::<Stream>::serialize(elem, os);
        }
    }
}

/* -------- Vec<T, A> -------- */
impl<Stream, T, A> Serialize<Stream> for Vec<T, A>
where
    Stream: Write,
    T: crate::serialize::Serialize<Stream>,
    A: std::alloc::Allocator + Clone,
{
    fn serialize(&self, os: &mut Stream) {
        write_compact_size(os, self.len() as u64);
        trace!(len = self.len(), "serialize Vec");

        for elem in self {
            crate::serialize::Serialize::<Stream>::serialize(elem, os);
        }
    }
}

/* -------- (K, V) tuple -------- */
impl<Stream, K, V> Serialize<Stream> for (K, V)
where
    Stream: Write,
    K: crate::serialize::Serialize<Stream>,
    V: crate::serialize::Serialize<Stream>,
{
    fn serialize(&self, os: &mut Stream) {
        self.0.serialize(os);
        self.1.serialize(os);
        trace!("serialize (K,V) tuple");
    }
}

/* -------- HashMap -------- */
impl<Stream, K, V, S> Serialize<Stream> for HashMap<K, V, S>
where
    Stream: Write,
    K: crate::serialize::Serialize<Stream>,
    V: crate::serialize::Serialize<Stream>,
    S: std::hash::BuildHasher,
{
    fn serialize(&self, os: &mut Stream) {
        write_compact_size(os, self.len() as u64);
        trace!(len = self.len(), "serialize HashMap");
        for (k, v) in self {
            k.serialize(os);
            v.serialize(os);
        }
    }
}

/* -------- HashSet -------- */
impl<Stream, K, S> Serialize<Stream> for HashSet<K, S>
where
    Stream: Write,
    K: crate::serialize::Serialize<Stream>,
    S: std::hash::BuildHasher,
{
    fn serialize(&self, os: &mut Stream) {
        write_compact_size(os, self.len() as u64);
        trace!(len = self.len(), "serialize HashSet");
        for key in self {
            key.serialize(os);
        }
    }
}

/* -------- Box<T> -------- */
impl<Stream, T> Serialize<Stream> for Box<T>
where
    Stream: Write,
    T: crate::serialize::Serialize<Stream> + ?Sized,
{
    fn serialize(&self, os: &mut Stream) {
        (**self).serialize(os);
        trace!("serialize Box<T>");
    }
}

/* -------- Arc<T> -------- */
impl<Stream, T> Serialize<Stream> for Arc<T>
where
    Stream: Write,
    T: crate::serialize::Serialize<Stream> + ?Sized,
{
    fn serialize(&self, os: &mut Stream) {
        (**self).serialize(os);
        trace!("serialize Arc<T>");
    }
}
