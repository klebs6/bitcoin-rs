// ---------------- [ File: bitcoin-serialize/src/macros.rs ]
crate::ix!();

#[macro_export]
macro_rules! varint_mode {
    ($obj:expr, $mode:ident) => {
        $crate::var_int_formatter::VarIntFormatter::<{
            $crate::var_int_mode::VarIntMode::$mode
        }>::new($obj)
    };
}

#[macro_export]
macro_rules! varint {
    ($obj:expr) => {
        $crate::var_int_formatter::VarIntFormatter::<{
            $crate::var_int_mode::VarIntMode::Default
        }>::new($obj)
    };
}

#[macro_export]
macro_rules! compactsize {
    ($obj:expr) => {
        $crate::compact_size_formatter::CompactSizeFormatter::<true>::new($obj)
    };
}

#[macro_export]
macro_rules! limited_string {
    ($obj:expr, $n:ident) => {
        $crate::limited_string_formatter::LimitedStringFormatter::<$n> { item: $obj }
    };
}

/* ─── internal helper: branch‑aware forwarding ────────────────────────── */
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_read_or_write {
    ($s:expr, $marker:expr, $tuple_mut:expr, $tuple_const:expr) => {{
        if $marker.for_read() {
            /* read‑phase → forward **mutable** tuple */
            $crate::many::ser_read_write_many_with_action_unserialize(
                $s,
                $crate::action::SerActionUnserialize {},
                &mut $tuple_mut,
            )
        } else {
            /* write‑phase → forward **immutable** tuple */
            $crate::many::ser_read_write_many_with_action_serialize(
                $s,
                $crate::action::SerActionSerialize {},
                &$tuple_const,
            )
        }
    }};
}

/* ─── READWRITE – preferred explicit form plus legacy fallback ──── */
#[macro_export]
macro_rules! readwrite {
    ( $s:expr, $marker:expr, $( $elem:expr ),* $(,)? ) => {{
        // One tuple – may itself contain mutable references
        let mut __rw_tuple = ( $( $elem ),* );

        if $marker.for_read() {
            /* read phase -------------------------------------------------- */
            $crate::many::ser_read_write_many_with_action_unserialize(
                $s,
                $crate::action::SerActionUnserialize {},
                &mut __rw_tuple,
            )
        } else {
            /* write phase ------------------------------------------------- */
            $crate::many::ser_read_write_many_with_action_serialize(
                $s,
                $crate::action::SerActionSerialize {},
                &__rw_tuple,       // immutable borrow is enough
            )
        }
    }};
}

/* ─── READWRITEAS ───────────────────────────────────────────────── */
#[macro_export]
macro_rules! readwriteas {
    ( $s:expr, $marker:expr, $ty:ty, $obj:expr ) => {{
        let mut __tmp: $ty = unsafe { core::mem::transmute_copy(&$obj) };
        readwrite!($s, $marker, __tmp);
        $obj = unsafe { core::mem::transmute_copy(&__tmp) };
    }};
}

/* ─── SER_READ / SER_WRITE – closure‑based & hygienic ───────────── */

#[macro_export]
macro_rules! ser_read {
    ( $s:expr, $marker:expr, $obj:expr, |$stm:ident, $val:ident| $body:block ) => {{
        $crate::read_write::ser_read($s, $marker, $obj, |$stm, mut $val| $body)
    }};
}

#[macro_export]
macro_rules! ser_write {
    ( $s:expr, $marker:expr, $obj:expr, |$stm:ident, $val:ident| $body:block ) => {{
        $crate::read_write::ser_write_with_action_serialize(
            $s,
            $marker,
            $obj,
            |$stm, $val| $body,
        )
    }};
}

/**
  | Implement the Ser and Unser methods
  | needed for implementing a formatter
  | (see Using below).
  | 
  | Both Ser and Unser are delegated to a
  | single static method SerializationOps,
  | which is polymorphic in the serialized/deserialized
  | type (allowing it to be const when serializing,
  | and non-const when deserializing).
  | 
  | Example use:
  | 
  | -----------
  | @code
  | 
  | struct FooFormatter {
  |   FORMATTER_METHODS(Class, obj) { READWRITE(obj.val1, VARINT(obj.val2)); }
  | }
  |
  | would define a class FooFormatter that
  | defines a serialization of Class objects
  | consisting of serializing its val1
  | member using the default serialization,
  | and its val2 member using
  | 
  | VARINT serialization. That FooFormatter
  | can then be used in statements like
  | 
  | READWRITE(Using<FooFormatter>(obj.bla)).
  |
  */
#[macro_export]
macro_rules! formatter_methods {
    ($cls:ident, $obj:ident) => {
        fn ser<Stream: std::io::Write>(&self, s: &mut Stream) {
            Self::serialization_ops(self, s, $crate::action::SerActionSerialize {})
        }
        fn unser<Stream: std::io::Read>(&mut self, s: &mut Stream) {
            Self::serialization_ops(self, s, $crate::action::SerActionUnserialize {})
        }
        fn serialization_ops<Stream, Op>(
            &mut self,
            s: &mut Stream,
            ser_action: Op,
        ) where
            Stream: std::io::Read + std::io::Write,
            Op: crate::action::SerActionSerialize + crate::action::SerActionUnserialize,
        {
            let $obj = self;
            // user‑supplied body follows the macro invocation
        }
    };
}

/**
  | Implement the BtcSerialize and BtcUnserialize
  | methods by delegating to a single templated
  | static method that takes the to-be-(de)serialized
  | object as a parameter.
  | 
  | This approach has the advantage that
  | the constness of the object becomes
  | a template parameter, and thus allows
  | a single implementation that sees the
  | object as const for serializing and
  | non-const for deserializing, without
  | casts.
  |
  */
#[macro_export]
macro_rules! serialize_methods {
    ($cls:ident, $obj:ident) => {
        impl<Stream: std::io::Write> $crate::serialize::BtcSerialize<Stream> for $cls {
            fn serialize(&self, s: &mut Stream) {
                let mut me = self.clone();
                me.ser(s);
            }
        }
        impl<Stream: std::io::Read> $crate::unserialize::BtcUnserialize<Stream> for $cls {
            fn unserialize(&mut self, s: &mut Stream) {
                self.unser(s);
            }
        }
    };
}

#[cfg(test)]
mod macros_and_serialize_tests {
    use super::*;
    use std::{io::Cursor, sync::Arc};
    use crate::imports::{HashMap, HashSet};

    fn comprehensive_roundtrip<T>(mut value: T)
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

    /// Explicit READ ↔ WRITE round‑trip exercising the `readwrite!` macro.
    #[traced_test]
    fn readwrite_macro_explicit() {
        /* ── serialize (write) phase ──────────────────────────────────── */
        let mut stream = Cursor::new(Vec::<u8>::new());
        readwrite!(
            &mut stream,
            SerActionSerialize {},
            0x11u8,
            0x2233u16
        );
        assert_eq!(stream.get_ref().as_slice(), &[0x11, 0x33, 0x22]);

        /* ── unserialize (read) phase ─────────────────────────────────── */
        let mut stream = Cursor::new(vec![0xAA, 0x55, 0x44]);
        let (mut x, mut y) = (0u8, 0u16);
        readwrite!(
            &mut stream,
            SerActionUnserialize {},
            &mut x,
            &mut y
        );
        assert_eq!((x, y), (0xAA, 0x4455));
    }

    #[traced_test]
    fn explicit_ser_read_and_write() {
        let mut buf = Cursor::new(Vec::<u8>::new());

        /* write one byte */
        ser_write!(
            &mut buf,
            SerActionSerialize {},
            0xFEu8,
            |stream, val| { ser_writedata8(stream, val); }
        );
        assert_eq!(buf.get_ref().as_slice(), &[0xFE]);

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

    #[traced_test]
    fn limited_string_macro_roundtrip() {
        const LIMIT: usize = 32;
        let original = "limited‑string‑macro!".to_string();

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

    #[traced_test]
    fn primitives_and_containers_roundtrip() {
        comprehensive_roundtrip(0i8);
        comprehensive_roundtrip(0xABu8);
        comprehensive_roundtrip(-0x1234i16);
        comprehensive_roundtrip(0xBEEFu16);
        comprehensive_roundtrip(-0x1234_5678i32);
        comprehensive_roundtrip(0xDEAD_BEEFu32);
        comprehensive_roundtrip(-0x1234_5678_9ABCi64);
        comprehensive_roundtrip(0x0123_4567_89AB_CDEFu64);
        comprehensive_roundtrip(true);
        comprehensive_roundtrip(false);
        comprehensive_roundtrip([0u8; 4]);
        comprehensive_roundtrip("rust‑bitcoin".to_string());

        comprehensive_roundtrip(vec![1u8, 2, 3, 4, 5]);
        comprehensive_roundtrip(Box::new(0x55AAu16));
        comprehensive_roundtrip(Arc::new(0x1122_3344u32));
        comprehensive_roundtrip((0xAAu8, 0xBBBBu16));

        let mut hm = HashMap::<u8, u8>::new();
        hm.insert(1, 2);
        hm.insert(3, 4);
        comprehensive_roundtrip(hm);

        let mut hs = HashSet::<u8>::new();
        hs.insert(42);
        hs.insert(11);
        comprehensive_roundtrip(hs);
    }
}
