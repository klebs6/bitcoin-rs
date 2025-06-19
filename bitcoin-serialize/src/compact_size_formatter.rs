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
