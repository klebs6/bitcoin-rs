// ---------------- [ File: bitcoin-serialize/src/write_with_size_computer.rs ]
crate::ix!();

#[inline]
pub fn write_compact_size_with_size_computer(s: &mut SizeComputer, n_size: u64) {
    let sz = get_size_of_compact_size(n_size);
    trace!(bytes = sz, "write_compact_size_with_size_computer");
    s.seek(sz as usize);
}

#[inline]
pub fn write_var_int_with_size_computer<I>(s: &mut SizeComputer, n: I)
where
    I: Into<u128> + Copy,
    (): ModeConstraint<{ VarIntMode::Default }, I>,
{
    let sz = get_size_of_var_int::<I, { VarIntMode::Default }>(n);
    trace!(bytes = sz, "write_var_int_with_size_computer");
    s.seek(sz as usize);
}
