crate::ix!();

impl<const BITS: usize> MulAssign<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    #[inline]
    fn mul_assign(&mut self, b: &BaseUInt<BITS>) {
        // The original C++ snippet was:
        //   BaseUInt<BITS> a;
        //   for j in [0..WIDTH):
        //       uint64_t carry = 0;
        //       for i in [0..WIDTH-j):
        //           uint64_t n = carry + a.pn[i+j] + (uint64_t)pn[j]*b.pn[i];
        //           a.pn[i+j] = n & 0xffffffff;
        //           carry = n >> 32;
        //   *this = a;
        let mut a = BaseUInt::<BITS>::default();

        for j in 0..(BITS / 32) {
            let mut carry = 0u64;
            for i in 0..(BITS / 32 - j) {
                let idx = i + j;
                let n = carry
                    + (a.pn[idx] as u64)
                    + (self.pn[j] as u64) * (b.pn[i] as u64);
                a.pn[idx] = (n & 0xffff_ffff) as u32;
                carry = n >> 32;
            }
        }

        *self = a;
    }
}

impl<const BITS: usize> MulAssign<u32> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    #[inline]
    fn mul_assign(&mut self, b32: u32) {
        // Equivalent of:
        //   uint64_t carry = 0;
        //   for i in [0..WIDTH):
        //       uint64_t n = carry + (uint64_t)b32 * pn[i];
        //       pn[i] = n & 0xffffffff;
        //       carry = n >> 32;
        let mut carry = 0u64;
        for i in 0..(BITS / 32) {
            let n = carry + (b32 as u64) * (self.pn[i] as u64);
            self.pn[i] = (n & 0xffff_ffff) as u32;
            carry = n >> 32;
        }
    }
}
