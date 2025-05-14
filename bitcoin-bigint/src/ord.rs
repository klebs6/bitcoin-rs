crate::ix!();

impl<const BITS: usize> Ord for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // We want the numeric comparison. The high limb is at pn[WIDTH-1], the low limb is at pn[0].
        // So we compare from the top down.
        for i in (0..(BITS / 32)).rev() {
            if self.pn[i] < other.pn[i] {
                return Ordering::Less;
            } else if self.pn[i] > other.pn[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl<const BITS: usize> PartialOrd<BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    fn partial_cmp(&self, other: &BaseUInt<BITS>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BITS: usize> PartialEq for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    fn eq(&self, other: &Self) -> bool {
        // Compare the entire array for equality. In C++ we had memcmp, but in Rust we can just compare slices.
        self.pn == other.pn
    }
}


impl<const BITS: usize> Eq for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{}
