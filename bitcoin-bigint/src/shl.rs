crate::ix!();

/// For reference-based shifts, we clamp the shift to BITS (i.e. if shift > BITS, it's all zero).
impl<const BITS: usize> core::ops::Shl<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    fn shl(self, rhs: &BaseUInt<BITS>) -> Self::Output {
        // We'll interpret only the lower 32 bits of rhs for the shift amount, then clamp to BITS
        let shift_raw = rhs.pn[0];
        let shift_bits = shift_raw.min(BITS as u32);

        let mut ret = self.clone();
        ret <<= shift_bits;
        ret
    }
}
