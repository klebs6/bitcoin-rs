// ---------------- [ File: bitcoin-serialize/src/read_write.rs ]
crate::ix!();

/// Execute a *read* operation (`fn_`) during a **serialize** phase.  
/// Exists for parity with the C++ helpers; the closure performs the
/// actual work.
#[inline]
pub fn ser_read_with_action_serialize<Stream, T, F>(
    s: &mut Stream,
    _ser_action: SerActionSerialize,
    obj: T,
    fn_: F,
) where
    Stream: Write,
    F: FnOnce(&mut Stream, T),
{
    trace!("ser_read_with_action_serialize");
    fn_(s, obj);
}

/// Execute a *read* operation (`fn_`) during an **unserialize** phase.
#[inline]
pub fn ser_read<Stream, T, F>(
    s: &mut Stream,
    _ser_action: SerActionUnserialize,
    obj: T,
    fn_: F,
) where
    Stream: Read,
    F: FnOnce(&mut Stream, T),
{
    trace!("ser_read");
    fn_(s, obj);
}

/// Execute a *write* operation (`fn_`) during a **serialize** phase.
#[inline]
pub fn ser_write_with_action_serialize<Stream, T, F>(
    s: &mut Stream,
    _ser_action: SerActionSerialize,
    obj: T,
    fn_: F,
) where
    Stream: Write,
    F: FnOnce(&mut Stream, T),
{
    trace!("ser_write_with_action_serialize");
    fn_(s, obj);
}

/// Execute a *write* operation (`fn_`) during an **unserialize** phase.  
/// This is typically a no‑op but is defined for completeness.
#[inline]
pub fn ser_write_with_action_unserialize<Stream, T, F>(
    _s: &mut Stream,
    _ser_action: SerActionUnserialize,
    _obj: T,
    _fn_: F,
) where
    Stream: Read,
    F: FnOnce(&mut Stream, T),
{
    trace!("ser_write_with_action_unserialize (noop)");
}

#[cfg(test)]
mod read_write_phase_tests {
    use super::*;
    use std::io::Cursor;

    #[traced_test]
    fn serialize_phase_invokes_closure() {
        let mut buf = Cursor::new(Vec::<u8>::new());
        let value   = 0xAAu8;

        let mut hit = false;
        ser_read_with_action_serialize(
            &mut buf,
            SerActionSerialize {},
            value,
            |s, v| {
                hit = true;
                crate::read_write_data::ser_writedata8(s, v);
            },
        );
        assert!(hit);
        assert_eq!(buf.get_ref().as_slice(), &[0xAA]);
    }

    #[traced_test]
    fn unserialize_phase_invokes_closure() {
        let mut buf = Cursor::new(vec![0xBB]);
        let mut out = 0u8;
        ser_read(
            &mut buf,
            SerActionUnserialize {},
            &mut out,
            |s, tgt| {
                *tgt = crate::read_write_data::ser_readdata8(s);
            },
        );
        assert_eq!(out, 0xBB);
        assert_eq!(buf.position(), 1);
    }

    #[traced_test]
    fn write_helpers_roundtrip() {
        let mut buf = Cursor::new(Vec::<u8>::new());
        ser_write_with_action_serialize(
            &mut buf,
            SerActionSerialize {},
            0xCCu8,
            |s, v| crate::read_write_data::ser_writedata8(s, v),
        );
        buf.set_position(0);

        ser_write_with_action_unserialize(
            &mut buf,
            SerActionUnserialize {},
            (),
            |_s, _| {
                // no‑op – the API explicitly does nothing on read‑phase.
            },
        );
    }
}
