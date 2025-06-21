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
pub trait BtcSerialize<Stream> {
    fn serialize(&self, s: &mut Stream);
}

impl<Stream> BtcSerialize<Stream> for i8
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize i8");
        ser_writedata8(s, *self as u8);
    }
}

impl<Stream> BtcSerialize<Stream> for u8
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize u8");
        ser_writedata8(s, *self);
    }
}

impl<Stream> BtcSerialize<Stream> for i16
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize i16");
        ser_writedata16(s, *self as u16);
    }
}

impl<Stream> BtcSerialize<Stream> for u16
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize u16");
        ser_writedata16(s, *self);
    }
}


impl<Stream> BtcSerialize<Stream> for i32
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize i32");
        ser_writedata32(s, *self as u32);
    }
}

impl<Stream> BtcSerialize<Stream> for u32
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize u32");
        ser_writedata32(s, *self);
    }
}

impl<Stream> BtcSerialize<Stream> for i64
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize i64");
        ser_writedata64(s, *self as u64);
    }
}

impl<Stream> BtcSerialize<Stream> for u64
where
    Stream: Write,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        trace!(value = *self, "serialize u64");
        ser_writedata64(s, *self);
    }
}

impl<Stream, const N: usize> BtcSerialize<Stream> for [u8; N]
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

/*
impl<'a, Stream> BtcSerialize<Stream> for &'a [u8]
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
*/

impl<Stream> BtcSerialize<Stream> for bool
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


impl<Stream> BtcSerialize<Stream> for String
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

impl<Stream, T, const N: usize> BtcSerialize<Stream> for PreVector<T, N>
where
    Stream: Write,
    T: Default + BtcSerialize<Stream>,
{
    /// Serialise a `PreVector` by
    /// 1. writing its length as a CompactSize,
    /// 2. serialising every element with the
    ///    element’s own `BtcSerialize` impl.
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
            BtcSerialize::<Stream>::serialize(elem, os);
        }
    }
}

/* -------- Vec<T, A> -------- */
impl<Stream, T, A> BtcSerialize<Stream> for Vec<T, A>
where
    Stream: Write,
    T: BtcSerialize<Stream>,
    A: std::alloc::Allocator + Clone,
{
    fn serialize(&self, os: &mut Stream) {
        write_compact_size(os, self.len() as u64);
        trace!(len = self.len(), "serialize Vec");

        for elem in self {
            BtcSerialize::<Stream>::serialize(elem, os);
        }
    }
}

/* -------- (K, V) tuple -------- */
impl<Stream, K, V> BtcSerialize<Stream> for (K, V)
where
    Stream: Write,
    K: BtcSerialize<Stream>,
    V: BtcSerialize<Stream>,
{
    fn serialize(&self, os: &mut Stream) {
        self.0.serialize(os);
        self.1.serialize(os);
        trace!("serialize (K,V) tuple");
    }
}

/* -------- HashMap -------- */
impl<Stream, K, V, S> BtcSerialize<Stream> for HashMap<K, V, S>
where
    Stream: Write,
    K: BtcSerialize<Stream>,
    V: BtcSerialize<Stream>,
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
impl<Stream, K, S> BtcSerialize<Stream> for HashSet<K, S>
where
    Stream: Write,
    K: BtcSerialize<Stream>,
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
impl<Stream, T> BtcSerialize<Stream> for Box<T>
where
    Stream: Write,
    T: BtcSerialize<Stream> + ?Sized,
{
    fn serialize(&self, os: &mut Stream) {
        (**self).serialize(os);
        trace!("serialize Box<T>");
    }
}

/* -------- Arc<T> -------- */
impl<Stream, T> BtcSerialize<Stream> for Arc<T>
where
    Stream: Write,
    T: BtcSerialize<Stream> + ?Sized,
{
    fn serialize(&self, os: &mut Stream) {
        (**self).serialize(os);
        trace!("serialize Arc<T>");
    }
}

impl<Stream, T> BtcSerialize<Stream> for &T
where
    Stream: std::io::Write,
    T: BtcSerialize<Stream> + ?Sized,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        (*self).serialize(s);
    }
}

impl<Stream, T> BtcSerialize<Stream> for &mut T
where
    Stream: std::io::Write,
    T: BtcSerialize<Stream> + ?Sized,
{
    #[inline]
    fn serialize(&self, s: &mut Stream) {
        (**self).serialize(s);
    }
}

#[cfg(test)]
mod macros_and_serialize_tests {
    use super::*;
    use std::{io::Cursor, sync::Arc};
    use crate::imports::{HashMap, HashSet};

    #[traced_test]
    fn serialize_smoke_primitive_roundtrip_via_traits() {
        let original : u32 = 0x12345678;
        let mut buf  = Cursor::new(Vec::<u8>::new());
        serialize::BtcSerialize::serialize(&original, &mut buf);

        buf.set_position(0);
        let mut decoded = 0u32;
        decoded.unserialize(&mut buf);

        assert_eq!(decoded, original);
    }

    /* generic helper */
    fn roundtrip<T>(mut value: T)
    where
        T: Clone
        + PartialEq
        + std::fmt::Debug
        + Default
        + BtcSerialize<Cursor<Vec<u8>>>
        + BtcSerialize<crate::size_computer::SizeComputer>
        + BtcUnserialize<Cursor<Vec<u8>>>,
    {
        let mut buf = Cursor::new(Vec::<u8>::new());
        value.serialize(&mut buf);

        assert_eq!(get_serialize_size(&value, None), buf.get_ref().len());

        buf.set_position(0);
        let mut decoded = T::default();
        decoded.unserialize(&mut buf);

        assert_eq!(decoded, value);
    }

    /* --------------------------------------------------------------------- */
    /*  1.  Primitive & composite type coverage                              */
    /* --------------------------------------------------------------------- */
    #[traced_test]
    fn primitives_and_containers_roundtrip() {
        roundtrip(0i8);
        roundtrip(0xABu8);
        roundtrip(-0x1234i16);
        roundtrip(0xBEEFu16);
        roundtrip(-0x1234_5678i32);
        roundtrip(0xDEAD_BEEFu32);
        roundtrip(-0x1234_5678_9ABCi64);
        roundtrip(0x0123_4567_89AB_CDEFu64);
        roundtrip(true);
        roundtrip(false);
        roundtrip([0u8; 4]);
        roundtrip("hello‑βitcoin".to_string());

        /* Vec, Box, Arc ---------------------------------------------------- */
        roundtrip(vec![1u8, 2, 3, 4, 5]);
        roundtrip(Box::new(0x55AAu16));
        roundtrip(Arc::new(0x1122_3344u32));

        /* Tuple ------------------------------------------------------------ */
        roundtrip((0xAAu8, 0xBBBBu16));

        /* HashMap & HashSet ------------------------------------------------ */
        let mut hm: HashMap<u8, u8> = HashMap::new();
        hm.insert(1, 2);
        hm.insert(3, 4);
        roundtrip(hm);

        let mut hs: HashSet<u8> = HashSet::new();
        hs.insert(42);
        hs.insert(11);
        roundtrip(hs);
    }

    /* --------------------------------------------------------------------- */
    /*  2.  Macros: varint! / compactsize! / limited_string!                 */
    /* --------------------------------------------------------------------- */
    #[traced_test]
    fn varint_macro_roundtrip() {
        let mut n = 300u64;
        let mut buf = Cursor::new(Vec::<u8>::new());

        /* serialize via macro wrapper ------------------------------------- */
        varint!(&mut n).serialize(&mut buf);

        /* wipe & read back ------------------------------------------------ */
        n = 0;
        buf.set_position(0);
        varint!(&mut n).unserialize(&mut buf);
        assert_eq!(n, 300);
    }

    #[traced_test]
    fn compactsize_macro_roundtrip() {
        let mut n = crate::constants::MAX_SIZE; // large but allowed
        let mut buf = Cursor::new(Vec::<u8>::new());

        compactsize!(&mut n).serialize(&mut buf);
        n = 0;
        buf.set_position(0);
        compactsize!(&mut n).unserialize(&mut buf);
        assert_eq!(n, crate::constants::MAX_SIZE);
    }

    /* limited_string! borrow‑checker regression test */
    #[traced_test]
    fn limited_string_macro_roundtrip() {
        const LIMIT: usize = 16;
        let original = "hello".to_string();

        let mut buf = Cursor::new(Vec::<u8>::new());
        {
            let mut scratch = String::new();
            limited_string!(&mut scratch, LIMIT).ser(&mut buf, &original);
        }

        buf.set_position(0);
        let mut decoded = String::new();
        {
            let mut scratch = String::new();
            limited_string!(&mut scratch, LIMIT).unser(&mut buf, &mut decoded);
        }

        assert_eq!(decoded, original);
    }

    /* broad coverage identical to the macro layer (kept concise here) */
    #[traced_test]
    fn primitives_roundtrip() {
        roundtrip(0x1234_5678u32);
        roundtrip(true);
        roundtrip(vec![1u8, 2, 3]);
        let mut hm = HashMap::<u8, u8>::new();
        hm.insert(1, 1);
        roundtrip(hm);
        let mut hs = HashSet::<u8>::new();
        hs.insert(7);
        roundtrip(hs);
        roundtrip("bitcoin‑serialize".to_string());
        roundtrip(Arc::new(42u64));
    }

    /* --------------------------------------------------------------------- */
    /*  3.  Macros: readwrite! / ser_read! / ser_write!                      */
    /* --------------------------------------------------------------------- */
    #[traced_test]
    fn readwrite_macro_both_phases() {
        /* serialize phase */
        let mut s = Cursor::new(Vec::<u8>::new());
        let a:  u8  = 0x11;
        let b:  u16 = 0x2233;
        readwrite!(&mut s, SerActionSerialize {}, a, b);
        assert_eq!(s.get_ref().as_slice(), &[0x11, 0x33, 0x22]);

        /* unserialize phase */
        let mut s = Cursor::new(vec![0xAA, 0x55, 0x44]);
        let mut x: u8  = 0;
        let mut y: u16 = 0;
        readwrite!(&mut s, SerActionUnserialize {}, &mut x, &mut y);
        assert_eq!((x, y), (0xAA, 0x4455));
    }

    #[traced_test]
    fn ser_read_and_ser_write_macros() {
        /* prepare buffer with one byte ------------------------------------ */
        let mut buf = Cursor::new(Vec::<u8>::new());

        /* use ser_write! during serialize phase --------------------------- */
        ser_write!(
            &mut buf,
            SerActionSerialize {},
            0xFEu8,
            |stream, val| { ser_writedata8(stream, val); }
        );
        assert_eq!(buf.get_ref().as_slice(), &[0xFE]);

        /* use ser_read! during read phase --------------------------------- */
        /* read it back */
        buf.set_position(0);
        let mut out = 0u8;
        ser_read!(
            &mut buf,
            SerActionUnserialize {},
            &mut out,
            |stream, tgt| { *tgt = ser_readdata8(stream); }
        );
        assert_eq!(out, 0xFE);
    }
}
