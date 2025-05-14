crate::ix!();

impl<const BITS: usize> Add<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    /// self + &other => new BaseUInt
    fn add(self, other: &BaseUInt<BITS>) -> Self::Output {
        let mut ret = self.clone();
        ret += other; // uses our AddAssign<&BaseUInt<BITS>>
        ret
    }
}

impl<const BITS: usize> Sub<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    /// self - &other => new BaseUInt
    fn sub(self, other: &BaseUInt<BITS>) -> Self::Output {
        let mut ret = self.clone();
        ret -= other; // uses our SubAssign<&BaseUInt<BITS>>
        ret
    }
}

impl<const BITS: usize> Mul<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    /// self * &other => new BaseUInt
    fn mul(self, other: &BaseUInt<BITS>) -> Self::Output {
        let mut ret = self.clone();
        // We'll assume `MulAssign<&BaseUInt<BITS>>` is already defined
        ret *= other;
        ret
    }
}

impl<const BITS: usize> Div<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    /// self / &other => new BaseUInt
    fn div(self, other: &BaseUInt<BITS>) -> Self::Output {
        let mut ret = self.clone();
        // We'll assume `DivAssign<&BaseUInt<BITS>>` is already defined
        ret /= other;
        ret
    }
}
