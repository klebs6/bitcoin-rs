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

/*
  | Variable-length integers: bytes are
  | a MSB base-128 encoding of the number.
  | 
  | The high bit in each byte signifies whether
  | another digit follows. To make sure
  | the encoding is one-to-one, one is subtracted
  | from all but the last digit.
  | 
  | Thus, the byte sequence a[] with length
  | len, where all but the last byte has bit
  | 128 set, encodes the number:
  | 
  | -----------
  | @code
  | 
  | (a[len-1] & 0x7F) + sum(i=1..len-1, 128^i*((a[len-i-1] & 0x7F)+1))
  | 
  | Properties:
  | 
  | - Very small (0-127: 1 byte, 128-16511:
  | 2 bytes, 16512-2113663: 3 bytes)
  | 
  | - Every integer has exactly one encoding
  | 
  | - Encoding does not depend on size of
  | original integer type
  | 
  | - No redundancy: every (infinite) byte
  | sequence corresponds to a list of encoded
  | integers.
  | ----------
  | @code
  | 
  | 0:         [0x00]  256:        [0x81 0x00]
  | 1:         [0x01]  16383:      [0xFE 0x7F]
  | 127:       [0x7F]  16384:      [0xFF 0x00]
  | 128:  [0x80 0x00]  16511:      [0xFF 0x7F]
  | 255:  [0x80 0x7F]  65535: [0x82 0xFE 0x7F]
  | 2^32:           [0x8E 0xFE 0xFE 0xFF 0x00]
  |
  */

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
/// 
#[inline]
pub fn using<'a, Formatter, T>(t: &'a mut T) -> Wrapper<'a, Formatter, &'a mut T> {
    Wrapper::new(t)
}

/**
  | If none of the specialized versions
  | above matched, default to calling member
  | function.
  |
  */
/*
   impls conflict with concrete type impls

impl<Stream,T> Serialize<Stream> for T {
    #[inline] fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                a.Serialize(os);
            */
    }
}
*/

impl<Stream> Serialize<Stream> for String {

    fn serialize(&self, os:   &mut Stream)  {

        todo!();
            /*
                WriteCompactSize(os, self.size());
            if (!self.empty())
                os.write((char*)self.data(), self.size() * sizeof(C));
            */
    }
}

impl<Stream, T: Default, const N: usize> Serialize<Stream> for PreVector<T,N> {

    #[inline] fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
            fn _serialize_impl_u8<Stream>(
                    os: &mut Stream,
                    v:  &PreVector<T,N>,
                    _2: &u8)  {

                todo!();
                    /*
                        WriteCompactSize(os, v.size());
                    if (!v.empty())
                        os.write((char*)v.data(), v.size() * sizeof(T));
                    */
            }

            fn _serialize_impl<Stream,V>(
                    os: &mut Stream,
                    v:  &PreVector<T,N>,
                    _2: &V)  {

                todo!();
                    /*
                        Serialize(os, Using<VectorFormatter<DefaultFormatter>>(v));
                    */
            }
                Serialize_impl(os, v, T());
            */
    }
}

impl<Stream, T, A: Allocator> Serialize<Stream> for Vec<T,A> {

    #[inline] fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
            #[inline] fn _serialize_impl_u8<Stream>(
                    os: &mut Stream,
                    v:  &Vec<T,A>,
                    _2: &u8)  {

                todo!();
                    /*
                        WriteCompactSize(os, v.size());
                    if (!v.empty())
                        os.write((char*)v.data(), v.size() * sizeof(T));
                    */
            }

            #[inline] fn _serialize_impl_bool<Stream>(
                    os: &mut Stream,
                    v:  &Vec<T,A>,
                    _2: &bool)  {

                todo!();
                    /*
                        // A special case for std::vector<bool>, as dereferencing
                    // std::vector<bool>::const_iterator does not result in a const bool&
                    // due to std::vector's special casing for bool arguments.
                    WriteCompactSize(os, v.size());
                    for (bool elem : v) {
                        ::Serialize(os, elem);
                    }
                    */
            }

            #[inline] fn _serialize_impl<Stream,V>(
                    os: &mut Stream,
                    v:  &Vec<T,A>,
                    _2: &V)  {

                todo!();
                    /*
                        Serialize(os, Using<VectorFormatter<DefaultFormatter>>(v));
                    */
            }
                Serialize_impl(os, v, T());
            */
    }
}

impl<Stream,K,T> Serialize<Stream> for (K,T) {
    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                Serialize(os, item.first);
            Serialize(os, item.second);
            */
    }
}

impl<Stream,K,T> Serialize<Stream> for HashMap<K,T> {
    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                WriteCompactSize(os, m.size());
            for (const auto& entry : m)
                Serialize(os, entry);
            */
    }
}

impl<Stream,K> Serialize<Stream> for HashSet<K> {
    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                WriteCompactSize(os, m.size());
            for (typename std::set<K, Pred, A>::const_iterator it = m.begin(); it != m.end(); ++it)
                Serialize(os, (*it));
            */
    }
}

impl<Stream,T> Serialize<Stream> for Box<T> {

    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                Serialize(os, *p);
            */
    }
}

impl<Stream,T> Serialize<Stream> for Arc<T> {
    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                Serialize(os, *p);
            */
    }

}
