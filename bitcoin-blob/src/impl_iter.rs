// ---------------- [ File: bitcoin-blob/src/impl_iter.rs ]
crate::ix!();

impl<const BITS: usize> BaseBlob<BITS>
where
    [(); base_blob_width::<BITS>()]:,
{
    /// Return an iterator over the bytes (by reference).
    pub fn iter(&self) -> core::slice::Iter<'_, u8> {
        tracing::trace!("iter => returning an iterator over bytes for BaseBlob<{}>", BITS);
        self.data.iter()
    }

    /// Return a mutable iterator over the bytes.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, u8> {
        tracing::trace!("iter_mut => returning a mutable iterator over bytes for BaseBlob<{}>", BITS);
        self.data.iter_mut()
    }
}

impl<const BITS: usize> IntoIterator for BaseBlob<BITS>
where
    [(); base_blob_width::<BITS>()]:,
{
    type Item = u8;
    type IntoIter = core::array::IntoIter<u8, { base_blob_width::<BITS>() }>;

    fn into_iter(self) -> Self::IntoIter {
        tracing::trace!("IntoIterator (by value) => BaseBlob<{}>", BITS);
        core::array::IntoIter::new(self.data)
    }
}

impl<'a, const BITS: usize> IntoIterator for &'a BaseBlob<BITS>
where
    [(); base_blob_width::<BITS>()]:,
{
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        tracing::trace!("IntoIterator (by ref) => &BaseBlob<{}>", BITS);
        self.data.iter()
    }
}

impl<'a, const BITS: usize> IntoIterator for &'a mut BaseBlob<BITS>
where
    [(); base_blob_width::<BITS>()]:,
{
    type Item = &'a mut u8;
    type IntoIter = core::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        tracing::trace!("IntoIterator (by mut ref) => &mut BaseBlob<{}>", BITS);
        self.data.iter_mut()
    }
}
