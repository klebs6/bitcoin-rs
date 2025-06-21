// ---------------- [ File: bitcoin-serialize/src/many.rs ]
crate::ix!();

/// Base case – nothing to serialise.
#[inline]
pub fn serialize_many_base<Stream>(_s: &mut Stream) {}

/// Serialise the first argument, then recurse.  This emulates the C++
/// variadic template `SerializeMany`.
pub fn serialize_many<Stream, A, Rest>(
    s: &mut Stream,
    arg: &A,
    rest: &Rest,
) where
    Stream: Write,
    A: BtcSerialize<Stream>,
    Rest: SerializeMany<Stream>,
{
    arg.serialize(s);
    rest.serialize_many(s);
}

/// Trait to provide a blanket implementation for tuples up to a
/// reasonable arity (here 4 – extend if needed).
pub trait SerializeMany<Stream> {
    fn serialize_many(&self, s: &mut Stream);
}

/* ---- tuple impls ((), (T,), (T,U), …) ---- */

impl<Stream> SerializeMany<Stream> for () {
    #[inline] fn serialize_many(&self, _s: &mut Stream) {}
}

impl<Stream, A> SerializeMany<Stream> for (A,)
where
    Stream: Write,
    A: BtcSerialize<Stream>,
{
    fn serialize_many(&self, s: &mut Stream) {
        self.0.serialize(s);
    }
}

impl<Stream, A, B> SerializeMany<Stream> for (A, B)
where
    Stream: Write,
    A: BtcSerialize<Stream>,
    B: BtcSerialize<Stream>,
{
    fn serialize_many(&self, s: &mut Stream) {
        self.0.serialize(s);
        self.1.serialize(s);
    }
}

impl<Stream, A, B, C> SerializeMany<Stream> for (A, B, C)
where
    Stream: Write,
    A: BtcSerialize<Stream>,
    B: BtcSerialize<Stream>,
    C: BtcSerialize<Stream>,
{
    fn serialize_many(&self, s: &mut Stream) {
        self.0.serialize(s);
        self.1.serialize(s);
        self.2.serialize(s);
    }
}

/// Base case for unserialisation.
#[inline]
pub fn unserialize_many_base<Stream>(_s: &mut Stream) {}

/// Unserialise the first argument, then recurse.
#[inline]
pub fn unserialize_many<Stream, A, Rest>(
    s: &mut Stream,
    arg: &mut A,
    rest: &mut Rest,
) where
    Stream: Read,
    A: BtcUnserialize<Stream>,
    Rest: UnserializeMany<Stream>,
{
    arg.unserialize(s);
    rest.unserialize_many(s);
}

/// Mirror of `SerializeMany` for reading.
pub trait UnserializeMany<Stream> {
    fn unserialize_many(&mut self, s: &mut Stream);
}

/* ---- tuple impls ---- */

impl<Stream> UnserializeMany<Stream> for () {
    #[inline] fn unserialize_many(&mut self, _s: &mut Stream) {}
}

impl<Stream, A> UnserializeMany<Stream> for (A,)
where
    Stream: Read,
    A: BtcUnserialize<Stream>,
{
    fn unserialize_many(&mut self, s: &mut Stream) {
        self.0.unserialize(s);
    }
}

impl<Stream, A, B> UnserializeMany<Stream> for (A, B)
where
    Stream: Read,
    A: BtcUnserialize<Stream>,
    B: BtcUnserialize<Stream>,
{
    fn unserialize_many(&mut self, s: &mut Stream) {
        self.0.unserialize(s);
        self.1.unserialize(s);
    }
}

impl<Stream, A, B, C> UnserializeMany<Stream> for (A, B, C)
where
    Stream: Read,
    A: BtcUnserialize<Stream>,
    B: BtcUnserialize<Stream>,
    C: BtcUnserialize<Stream>,
{
    fn unserialize_many(&mut self, s: &mut Stream) {
        self.0.unserialize(s);
        self.1.unserialize(s);
        self.2.unserialize(s);
    }
}

/* -------- convenience wrappers matching Bitcoin Core helpers -------- */

#[inline]
pub fn ser_read_write_many_with_action_serialize<Stream, T>(
    s: &mut Stream,
    _act: SerActionSerialize,
    args: &T,
) where
    Stream: Write,
    T: SerializeMany<Stream>,
{
    trace!("ser_read_write_many_with_action_serialize");
    args.serialize_many(s);
}

#[inline]
pub fn ser_read_write_many_with_action_unserialize<Stream, T>(
    s: &mut Stream,
    _act: SerActionUnserialize,
    args: &mut T,
) where
    Stream: Read,
    T: UnserializeMany<Stream>,
{
    trace!("ser_read_write_many_with_action_unserialize");
    args.unserialize_many(s);
}

impl<'a, Stream, T> SerializeMany<Stream> for &'a T
where
    Stream: Write,
    T: SerializeMany<Stream> + ?Sized,
{
    #[inline]
    fn serialize_many(&self, s: &mut Stream) {
        (**self).serialize_many(s);
    }
}

impl<'a, Stream, T> SerializeMany<Stream> for &'a mut T
where
    Stream: Write,
    T: SerializeMany<Stream> + ?Sized,
{
    #[inline]
    fn serialize_many(&self, s: &mut Stream) {
        (**self).serialize_many(s);
    }
}

impl<'a, Stream, T> UnserializeMany<Stream> for &'a mut T
where
    Stream: Read,
    T: UnserializeMany<Stream> + ?Sized,
{
    #[inline]
    fn unserialize_many(&mut self, s: &mut Stream) {
        (**self).unserialize_many(s);
    }
}

#[cfg(test)]
mod many_tests {
    use super::*;
    use std::io::Cursor;

    #[traced_test]
    fn roundtrip_two_elements() {
        let original = (0xAAu8, 0xBBCCu16);

        let mut buf = Cursor::new(Vec::<u8>::new());
        ser_read_write_many_with_action_serialize(
            &mut buf,
            crate::action::SerActionSerialize {},
            &original,
        );

        buf.set_position(0);
        let mut decoded = (0u8, 0u16);
        ser_read_write_many_with_action_unserialize(
            &mut buf,
            crate::action::SerActionUnserialize {},
            &mut decoded,
        );

        assert_eq!(decoded, original);
    }

    #[traced_test]
    fn serialize_size_many_matches_actual() {
        let triple = (1u8, 2u16, false);

        let mut cur = Cursor::new(Vec::<u8>::new());
        BtcSerialize::serialize(&triple.0, &mut cur);
        BtcSerialize::serialize(&triple.1, &mut cur);
        BtcSerialize::serialize(&triple.2, &mut cur);
        let manual = cur.get_ref().len();

        let helper = crate::get_serialize_size_many(0, &triple);
        assert_eq!(manual, helper);
    }
}
