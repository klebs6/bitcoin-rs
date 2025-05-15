crate::ix!();

impl<const BITS: usize> PartialEq<BaseBlob<BITS>> for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    fn eq(&self, other: &BaseBlob<BITS>) -> bool {
        self.compare(other) == 0
    }
}

impl<const BITS: usize> Eq for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{}

impl<const BITS: usize> Ord for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    fn cmp(&self, other: &BaseBlob<BITS>) -> Ordering {

        let x = self.compare(other);

        match x {
            _ if x < 0  => Ordering::Less,
            _ if x == 0 => Ordering::Equal,
            _ if x > 0  => Ordering::Greater,
            _ => unreachable![],
        }
    }
}

impl<const BITS: usize> PartialOrd<BaseBlob<BITS>> for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    fn partial_cmp(&self, other: &BaseBlob<BITS>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
