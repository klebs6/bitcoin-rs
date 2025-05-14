crate::ix!();

impl<const BITS: usize> SubAssign<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// self -= other  =>  self = self + (-other)
    #[inline]
    fn sub_assign(&mut self, other: &BaseUInt<BITS>) {
        *self += &(-other.clone()); // We rely on our `Neg` impl.
    }
}

impl<const BITS: usize> SubAssign<u64> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// self -= u64  =>  self += -u64
    #[inline]
    fn sub_assign(&mut self, other: u64) {

        let mut b = Self::default();

        b.pn[0] = (other & 0xffff_ffff) as u32;

        if BITS / 32 > 1 {
            b.pn[1] = ((other >> 32) & 0xffff_ffff) as u32;
        }

        *self += &(-b);
    }
}
