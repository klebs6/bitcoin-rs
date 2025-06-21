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


#[cfg(test)]
mod size_computer_helper_tests {
    use super::*;

    #[traced_test]
    fn write_compact_size_with_sc_counts_correctly() {
        for &len in &[0u64, 252, 253, 65_536, 4_000_000_000] {
            let mut sc = SizeComputer::new(0);
            write_compact_size_with_size_computer(&mut sc, len);

            let expect = get_size_of_compact_size(len) as usize;
            assert_eq!(sc.size(), expect, "len={len}");
        }
    }

    #[traced_test]
    fn write_var_int_with_sc_counts_correctly() {
        for &n in &[0u64, 1, 0x7F, 0x80, 0x3FFF, 0x4000, u32::MAX as u64] {
            let mut sc = SizeComputer::new(0);
            write_var_int_with_size_computer(&mut sc, n);

            let expect =
                get_size_of_var_int::<u64, { VarIntMode::Default }>(n) as usize;
            assert_eq!(sc.size(), expect, "n={n}");
        }
    }

    /// Cross‑check the helpers against *real* serialization.
    #[traced_test]
    fn helpers_match_actual_byte_stream() {
        let value = 123_456u64;

        // ── Real bytes written ──
        let mut cur = std::io::Cursor::new(Vec::<u8>::new());
        crate::compact_size::write_compact_size(&mut cur, value);
        write_var_int::<_, u64, { VarIntMode::Default }>(&mut cur, value);
        let actual_bytes = cur.into_inner().len();

        // ── Simulated bytes written ──
        let mut sc = SizeComputer::new(0);
        write_compact_size_with_size_computer(&mut sc, value);
        write_var_int_with_size_computer(&mut sc, value);

        assert_eq!(sc.size(), actual_bytes);
    }
}
