// ---------------- [ File: bitcoin-serialize/src/compact_size_formatter.rs ]
crate::ix!();

/**
  | Formatter for integers in CompactSize
  | format.
  |
  */
pub struct CompactSizeFormatter<const RangeCheck: bool> { }

impl<const RangeCheck: bool> CompactSizeFormatter<RangeCheck> {
    /// Counterpart to `compactsize!` macro.
    #[inline]
    pub fn new<'a, T>(item: &'a mut T) -> crate::wrapper::Wrapper<'a, Self, T> {
        crate::wrapper::Wrapper::new(item)
    }
}

impl<const RangeCheck: bool> Default for CompactSizeFormatter<RangeCheck> {
    #[inline]
    fn default() -> Self {
        Self {}
    }
}

impl<const RangeCheck: bool, I> ValueFormatter<I> for CompactSizeFormatter<RangeCheck>
where
    I: Into<u64> + TryFrom<u64> + Copy + std::fmt::Debug,
    <I as TryFrom<u64>>::Error: std::fmt::Debug,
{
    fn ser<S: Write>(&mut self, s: &mut S, v: &I) {
        write_compact_size(s, (*v).into());
    }

    fn unser<S: Read>(&mut self, s: &mut S, v: &mut I) {
        let n = read_compact_size(s, Some(RangeCheck));
        *v = I::try_from(n).expect("CompactSize exceeds type range");
    }
}


#[cfg(test)]
mod compact_size_formatter_tests {
    use super::*;
    use std::io::Cursor;

    type FmtChecked  = CompactSizeFormatter<true>;
    type FmtUnchecked = CompactSizeFormatter<false>;

    const BIG_VALUE: u64 = crate::constants::MAX_SIZE + 42;

    #[traced_test]
    fn roundtrip_in_range() {
        let value = 123_456_u64;
        let mut cur = Cursor::new(Vec::<u8>::new());
        let mut fmt = FmtChecked::default();

        fmt.ser(&mut cur, &value);
        cur.set_position(0);

        let mut decoded = 0u64;
        fmt.unser(&mut cur, &mut decoded);
        assert_eq!(decoded, value);
    }

    /// Unchecked formatter must allow values above `MAX_SIZE`.
    #[traced_test]
    fn allow_large_without_range_check() {
        let mut cur = Cursor::new(Vec::<u8>::new());
        let mut fmt = FmtUnchecked::default();

        fmt.ser(&mut cur, &BIG_VALUE);
        cur.set_position(0);

        let mut decoded = 0u64;
        fmt.unser(&mut cur, &mut decoded);
        assert_eq!(decoded, BIG_VALUE);
    }

    /// Checked formatter must panic when the encoded integer exceeds
    /// `MAX_SIZE`.  We do **not** assert on the panic message because it
    /// is not part of the public contract.
    #[test]
    #[should_panic] 
    fn range_check_panics_on_oversize() {
        let mut cur = Cursor::new(Vec::<u8>::new());
        let mut fmt = FmtChecked::default();

        fmt.ser(&mut cur, &BIG_VALUE);
        cur.set_position(0);

        let mut _decoded = 0u64;
        fmt.unser(&mut cur, &mut _decoded);
    }
}
