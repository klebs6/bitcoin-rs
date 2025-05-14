crate::ix!();

/// Similarly for right shifts: read lower 32 bits, clamp, then do the normal shr_assign.
impl<const BITS: usize> core::ops::Shr<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    fn shr(self, rhs: &BaseUInt<BITS>) -> Self::Output {

        let shift_raw  = rhs.pn[0];
        let shift_bits = shift_raw.min(BITS as u32);

        let mut ret = self.clone();
        ret >>= shift_bits;
        ret
    }
}

