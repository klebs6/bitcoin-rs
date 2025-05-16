// ---------------- [ File: bitcoin-blob/src/impl_iter.rs ]
crate::ix!();

impl<const BITS: usize> BaseBlob<BITS>
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    /// Return an iterator over the bytes (by reference).
    pub fn iter(&self) -> core::slice::Iter<'_, u8> {
        trace!("iter => returning an iterator over bytes for BaseBlob<{}>", BITS);
        self.data.iter()
    }

    /// Return a mutable iterator over the bytes.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, u8> {
        trace!("iter_mut => returning a mutable iterator over bytes for BaseBlob<{}>", BITS);
        self.data.iter_mut()
    }
}

impl<const BITS: usize> IntoIterator for BaseBlob<BITS>
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    type Item = u8;
    type IntoIter = core::array::IntoIter<u8, { base_blob_width::<BITS>() }>;

    fn into_iter(self) -> Self::IntoIter {
        trace!("IntoIterator (by value) => BaseBlob<{}>", BITS);
        core::array::IntoIter::new(self.data)
    }
}

impl<'a, const BITS: usize> IntoIterator for &'a BaseBlob<BITS>
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        trace!("IntoIterator (by ref) => &BaseBlob<{}>", BITS);
        self.data.iter()
    }
}

impl<'a, const BITS: usize> IntoIterator for &'a mut BaseBlob<BITS>
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    type Item = &'a mut u8;
    type IntoIter = core::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        trace!("IntoIterator (by mut ref) => &mut BaseBlob<{}>", BITS);
        self.data.iter_mut()
    }
}

#[cfg(test)]
mod iteration_exhaustive_tests {
    use super::*;

    /// We will exhaustively test:
    ///   - `iter()` (returning `core::slice::Iter<'_, u8>`)
    ///   - `iter_mut()` (returning `core::slice::IterMut<'_, u8>`)
    ///   - `IntoIterator for BaseBlob<BITS>` (by value)
    ///   - `IntoIterator for &BaseBlob<BITS>` (by reference)
    ///   - `IntoIterator for &mut BaseBlob<BITS>` (by mutable reference)
    ///
    /// Each one is tested for BITS=8, BITS=64, and BITS=256 to ensure correctness
    /// across a range of widths. We fill the blob with a pattern, then verify
    /// the iterators produce the correct elements. For `iter_mut()` and
    /// `IntoIterator for &mut`, we'll confirm we can modify the data.

    #[traced_test]
    fn test_iter() {
        info!("Testing BaseBlob<BITS> .iter() for BITS=8, BITS=64, BITS=256...");
        test_iter_gen::<8>();
        test_iter_gen::<64>();
        test_iter_gen::<256>();
        info!(".iter() tests concluded successfully.");
    }

    fn test_iter_gen<const BITS: usize>()
        where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
    {
        let mut blob = BaseBlob::<BITS>::default();
        fill_pattern(&mut blob);

        // Collect the iteration results into a Vec
        let collected: Vec<u8> = blob.iter().copied().collect();
        // Compare with blob.data
        assert_eq!(
            collected,
            blob.data.to_vec(),
            "test_iter_gen<BITS={}> => mismatch between iter() and .data",
            BITS
        );
    }

    #[traced_test]
    fn test_iter_mut() {
        info!("Testing BaseBlob<BITS> .iter_mut() for BITS=8, BITS=64, BITS=256...");
        test_iter_mut_gen::<8>();
        test_iter_mut_gen::<64>();
        test_iter_mut_gen::<256>();
        info!(".iter_mut() tests concluded successfully.");
    }

    fn test_iter_mut_gen<const BITS: usize>()
        where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
    {
        // We'll fill the blob with a pattern, then iterate mutably and increment each byte by 1.
        let mut blob = BaseBlob::<BITS>::default();
        fill_pattern(&mut blob);

        for byte in blob.iter_mut() {
            *byte = byte.wrapping_add(1);
        }

        // Now compare with a reference: each original data[i] + 1
        for (i, &b) in blob.data.iter().enumerate() {
            let original = pattern_for_index(i);
            let expected = original.wrapping_add(1);
            assert_eq!(
                b, expected,
                "test_iter_mut_gen<BITS={}> => data[{}] mismatch after increment",
                BITS,
                i
            );
        }
    }

    #[traced_test]
    fn test_into_iter_by_value() {
        info!("Testing `IntoIterator for BaseBlob<BITS>` (by value) for BITS=8, BITS=64, BITS=256...");
        test_into_iter_by_value_gen::<8>();
        test_into_iter_by_value_gen::<64>();
        test_into_iter_by_value_gen::<256>();
        info!("IntoIterator (by value) tests concluded successfully.");
    }

    fn test_into_iter_by_value_gen<const BITS: usize>()
        where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
    {
        let mut blob = BaseBlob::<BITS>::default();
        fill_pattern(&mut blob);

        // into_iter => consume the blob, produce an iterator of u8
        let collected: Vec<u8> = blob.into_iter().collect();
        // Compare with the original data
        // but note: we can't use blob after we consume it by value
        assert_eq!(
            collected.len(),
            base_blob_width::<BITS>(),
            "BITS={} => length mismatch in by-value iteration",
            BITS
        );
        for (i, &val) in collected.iter().enumerate() {
            let expected = pattern_for_index(i);
            assert_eq!(
                val, expected,
                "test_into_iter_by_value_gen<BITS={}> => mismatch at index={}",
                BITS,
                i
            );
        }
    }

    #[traced_test]
    fn test_into_iter_by_ref() {
        info!("Testing `IntoIterator for &BaseBlob<BITS>` for BITS=8, BITS=64, BITS=256...");
        test_into_iter_by_ref_gen::<8>();
        test_into_iter_by_ref_gen::<64>();
        test_into_iter_by_ref_gen::<256>();
        info!("IntoIterator (by ref) tests concluded successfully.");
    }

    fn test_into_iter_by_ref_gen<const BITS: usize>()
        where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
    {
        let mut blob = BaseBlob::<BITS>::default();
        fill_pattern(&mut blob);

        // by ref => produce &u8 items, not consuming the blob
        let collected: Vec<u8> = (&blob).into_iter().copied().collect();

        // Compare with blob.data
        assert_eq!(
            collected,
            blob.data.to_vec(),
            "test_into_iter_by_ref_gen<BITS={}> => mismatch between &blob iter and .data",
            BITS
        );

        // ensure we still can use `blob` after iteration
        assert!(!blob.is_null(), "blob is still valid after &blob iteration, BITS={}", BITS);
    }

    #[traced_test]
    fn test_into_iter_by_mut_ref() {
        info!("Testing `IntoIterator for &mut BaseBlob<BITS>` for BITS=8, BITS=64, BITS=256...");
        test_into_iter_by_mut_ref_gen::<8>();
        test_into_iter_by_mut_ref_gen::<64>();
        test_into_iter_by_mut_ref_gen::<256>();
        info!("IntoIterator (by mut ref) tests concluded successfully.");
    }

    fn test_into_iter_by_mut_ref_gen<const BITS: usize>()
        where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
    {
        let mut blob = BaseBlob::<BITS>::default();
        fill_pattern(&mut blob);

        // by mut ref => produce &mut u8 items
        for (i, val) in (&mut blob).into_iter().enumerate() {
            // We'll do a simple transformation: val = val + (i as u8)
            let old = *val;
            *val = old.wrapping_add(i as u8);
        }

        // Now confirm the final values
        for (i, &b) in blob.data.iter().enumerate() {
            let orig = pattern_for_index(i);
            let expected = orig.wrapping_add(i as u8);
            assert_eq!(
                b, expected,
                "test_into_iter_by_mut_ref_gen<BITS={}> => mismatch at data[{}]",
                BITS,
                i
            );
        }
    }

    // ------------------------------------------------------------------------
    // Helper: fill the blob with a simple pattern: data[i] = pattern_for_index(i).
    // We'll just do i*3 ^ 0x5A or something. Must be consistent.
    fn fill_pattern<const BITS: usize>(blob: &mut BaseBlob<BITS>)
        where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
    {
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = pattern_for_index(i);
        }
    }

    fn pattern_for_index(i: usize) -> u8 {
        // A simple pattern that won't overflow or loop too soon.
        ((i as u8).wrapping_mul(3)) ^ 0x5A
    }
}

