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
