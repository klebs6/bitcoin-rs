crate::ix!();

impl<const BITS: usize> Not for BaseUInt<BITS>
where
    [(); BITS / 32]:
{
    type Output = BaseUInt<BITS>;

    #[inline]
    fn not(self) -> Self::Output {
        // Bitwise NOT (~) each 32-bit limb
        let mut ret = Self::default();
        for (i, &val) in self.pn.iter().enumerate() {
            ret.pn[i] = !val;
        }
        ret
    }
}

impl<const BITS: usize> Neg for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = Self;

    /// Two's complement negation.
    /// In C++: `ret = ~self; ++ret;`
    /// This is effectively  `-x = (~x + 1)`
    #[inline]
    fn neg(self) -> Self::Output {
        let mut ret = Self::default();
        // Invert bits
        for (i, &val) in self.pn.iter().enumerate() {
            ret.pn[i] = !val;
        }
        // Now add 1 to complete two's complement
        let mut carry = 1u64;
        for limb in ret.pn.iter_mut() {
            let sum = *limb as u64 + carry;
            *limb = (sum & 0xffffffff) as u32;
            carry = sum >> 32;
            if carry == 0 {
                break; // No more carry, done.
            }
        }
        ret
    }
}
