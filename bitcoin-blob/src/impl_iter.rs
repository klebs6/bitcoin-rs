// ---------------- [ File: bitcoin-blob/src/impl_iter.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_base_blob_iter {
    (
        $blob_ty:ident,
        $bits:expr,
        $bytes:expr
    ) => {

        impl $blob_ty {
            /// Return an iterator over the bytes (by reference).
            pub fn iter(&self) -> core::slice::Iter<'_, u8> {
                trace!("iter => returning an iterator over bytes for BaseBlob<{}>", $bits);
                self.data.iter()
            }

            /// Return a mutable iterator over the bytes.
            pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, u8> {
                trace!("iter_mut => returning a mutable iterator over bytes for BaseBlob<{}>", $bits);
                self.data.iter_mut()
            }
        }

        impl IntoIterator for $blob_ty {
            type Item = u8;
            type IntoIter = core::array::IntoIter<u8, $bytes>;

            fn into_iter(self) -> Self::IntoIter {
                trace!("IntoIterator (by value) => BaseBlob<{}>", $bits);
                core::array::IntoIter::new(self.data)
            }
        }

        impl<'a> IntoIterator for &'a $blob_ty {
            type Item = &'a u8;
            type IntoIter = core::slice::Iter<'a, u8>;

            fn into_iter(self) -> Self::IntoIter {
                trace!("IntoIterator (by ref) => &BaseBlob<{}>", $bits);
                self.data.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $blob_ty {
            type Item = &'a mut u8;
            type IntoIter = core::slice::IterMut<'a, u8>;

            fn into_iter(self) -> Self::IntoIter {
                trace!("IntoIterator (by mut ref) => &mut BaseBlob<{}>", $bits);
                self.data.iter_mut()
            }
        }
    }
}

#[cfg(test)]
mod iteration_exhaustive_tests {
    use super::*;
    use tracing::{info, debug, warn, error, trace};

    // We'll define convenience constants:
    const SIZE_8:   usize = 1;
    const SIZE_64:  usize = 8;
    const SIZE_256: usize = 32;

    // We replicate the original test structure and commentary,
    // but adapt it to call our three concrete types: BaseBlob8,
    // BaseBlob64, and BaseBlob256.

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
        test_iter_8();
        test_iter_64();
        test_iter_256();
        info!(".iter() tests concluded successfully.");
    }

    fn test_iter_8() {
        let mut blob = BaseBlob8::default();
        fill_pattern_8(&mut blob);
        let collected: Vec<u8> = blob.iter().copied().collect();
        assert_eq!(
            collected,
            blob.data.to_vec(),
            "test_iter_8 => mismatch between iter() and .data"
        );
    }
    fn test_iter_64() {
        let mut blob = BaseBlob64::default();
        fill_pattern_64(&mut blob);
        let collected: Vec<u8> = blob.iter().copied().collect();
        assert_eq!(
            collected,
            blob.data.to_vec(),
            "test_iter_64 => mismatch between iter() and .data"
        );
    }
    fn test_iter_256() {
        let mut blob = BaseBlob256::default();
        fill_pattern_256(&mut blob);
        let collected: Vec<u8> = blob.iter().copied().collect();
        assert_eq!(
            collected,
            blob.data.to_vec(),
            "test_iter_256 => mismatch between iter() and .data"
        );
    }

    #[traced_test]
    fn test_iter_mut() {
        info!("Testing BaseBlob<BITS> .iter_mut() for BITS=8, BITS=64, BITS=256...");
        test_iter_mut_8();
        test_iter_mut_64();
        test_iter_mut_256();
        info!(".iter_mut() tests concluded successfully.");
    }

    fn test_iter_mut_8() {
        let mut blob = BaseBlob8::default();
        fill_pattern_8(&mut blob);
        for byte in blob.iter_mut() {
            *byte = byte.wrapping_add(1);
        }
        for (i, &b) in blob.data.iter().enumerate() {
            let original = pattern_for_index(i);
            let expected = original.wrapping_add(1);
            assert_eq!(
                b, expected,
                "test_iter_mut_8 => data[{}] mismatch after increment",
                i
            );
        }
    }
    fn test_iter_mut_64() {
        let mut blob = BaseBlob64::default();
        fill_pattern_64(&mut blob);
        for byte in blob.iter_mut() {
            *byte = byte.wrapping_add(1);
        }
        for (i, &b) in blob.data.iter().enumerate() {
            let original = pattern_for_index(i);
            let expected = original.wrapping_add(1);
            assert_eq!(
                b, expected,
                "test_iter_mut_64 => data[{}] mismatch after increment",
                i
            );
        }
    }
    fn test_iter_mut_256() {
        let mut blob = BaseBlob256::default();
        fill_pattern_256(&mut blob);
        for byte in blob.iter_mut() {
            *byte = byte.wrapping_add(1);
        }
        for (i, &b) in blob.data.iter().enumerate() {
            let original = pattern_for_index(i);
            let expected = original.wrapping_add(1);
            assert_eq!(
                b, expected,
                "test_iter_mut_256 => data[{}] mismatch after increment",
                i
            );
        }
    }

    #[traced_test]
    fn test_into_iter_by_value() {
        info!("Testing `IntoIterator for BaseBlob<BITS>` (by value) for BITS=8, BITS=64, BITS=256...");
        test_into_iter_by_value_8();
        test_into_iter_by_value_64();
        test_into_iter_by_value_256();
        info!("IntoIterator (by value) tests concluded successfully.");
    }

    fn test_into_iter_by_value_8() {
        let mut blob = BaseBlob8::default();
        fill_pattern_8(&mut blob);
        let collected: Vec<u8> = blob.into_iter().collect();
        assert_eq!(
            collected.len(),
            SIZE_8,
            "BITS=8 => length mismatch in by-value iteration"
        );
        for (i, &val) in collected.iter().enumerate() {
            let expected = pattern_for_index(i);
            assert_eq!(
                val, expected,
                "test_into_iter_by_value_8 => mismatch at index={}",
                i
            );
        }
    }
    fn test_into_iter_by_value_64() {
        let mut blob = BaseBlob64::default();
        fill_pattern_64(&mut blob);
        let collected: Vec<u8> = blob.into_iter().collect();
        assert_eq!(
            collected.len(),
            SIZE_64,
            "BITS=64 => length mismatch in by-value iteration"
        );
        for (i, &val) in collected.iter().enumerate() {
            let expected = pattern_for_index(i);
            assert_eq!(
                val, expected,
                "test_into_iter_by_value_64 => mismatch at index={}",
                i
            );
        }
    }
    fn test_into_iter_by_value_256() {
        let mut blob = BaseBlob256::default();
        fill_pattern_256(&mut blob);
        let collected: Vec<u8> = blob.into_iter().collect();
        assert_eq!(
            collected.len(),
            SIZE_256,
            "BITS=256 => length mismatch in by-value iteration"
        );
        for (i, &val) in collected.iter().enumerate() {
            let expected = pattern_for_index(i);
            assert_eq!(
                val, expected,
                "test_into_iter_by_value_256 => mismatch at index={}",
                i
            );
        }
    }

    #[traced_test]
    fn test_into_iter_by_ref() {
        info!("Testing `IntoIterator for &BaseBlob<BITS>` for BITS=8, BITS=64, BITS=256...");
        test_into_iter_by_ref_8();
        test_into_iter_by_ref_64();
        test_into_iter_by_ref_256();
        info!("IntoIterator (by ref) tests concluded successfully.");
    }

    fn test_into_iter_by_ref_8() {
        let mut blob = BaseBlob8::default();
        fill_pattern_8(&mut blob);
        let collected: Vec<u8> = (&blob).into_iter().copied().collect();
        assert_eq!(
            collected,
            blob.data.to_vec(),
            "test_into_iter_by_ref_8 => mismatch between &blob iter and .data"
        );
        assert!(!blob.is_null(), "blob is still valid after &blob iteration, BITS=8");
    }
    fn test_into_iter_by_ref_64() {
        let mut blob = BaseBlob64::default();
        fill_pattern_64(&mut blob);
        let collected: Vec<u8> = (&blob).into_iter().copied().collect();
        assert_eq!(
            collected,
            blob.data.to_vec(),
            "test_into_iter_by_ref_64 => mismatch between &blob iter and .data"
        );
        assert!(!blob.is_null(), "blob is still valid after &blob iteration, BITS=64");
    }
    fn test_into_iter_by_ref_256() {
        let mut blob = BaseBlob256::default();
        fill_pattern_256(&mut blob);
        let collected: Vec<u8> = (&blob).into_iter().copied().collect();
        assert_eq!(
            collected,
            blob.data.to_vec(),
            "test_into_iter_by_ref_256 => mismatch between &blob iter and .data"
        );
        assert!(!blob.is_null(), "blob is still valid after &blob iteration, BITS=256");
    }

    #[traced_test]
    fn test_into_iter_by_mut_ref() {
        info!("Testing `IntoIterator for &mut BaseBlob<BITS>` for BITS=8, BITS=64, BITS=256...");
        test_into_iter_by_mut_ref_8();
        test_into_iter_by_mut_ref_64();
        test_into_iter_by_mut_ref_256();
        info!("IntoIterator (by mut ref) tests concluded successfully.");
    }

    fn test_into_iter_by_mut_ref_8() {
        let mut blob = BaseBlob8::default();
        fill_pattern_8(&mut blob);
        for (i, val) in (&mut blob).into_iter().enumerate() {
            let old = *val;
            *val = old.wrapping_add(i as u8);
        }
        for (i, &b) in blob.data.iter().enumerate() {
            let orig = pattern_for_index(i);
            let expected = orig.wrapping_add(i as u8);
            assert_eq!(
                b, expected,
                "test_into_iter_by_mut_ref_8 => mismatch at data[{}]",
                i
            );
        }
    }
    fn test_into_iter_by_mut_ref_64() {
        let mut blob = BaseBlob64::default();
        fill_pattern_64(&mut blob);
        for (i, val) in (&mut blob).into_iter().enumerate() {
            let old = *val;
            *val = old.wrapping_add(i as u8);
        }
        for (i, &b) in blob.data.iter().enumerate() {
            let orig = pattern_for_index(i);
            let expected = orig.wrapping_add(i as u8);
            assert_eq!(
                b, expected,
                "test_into_iter_by_mut_ref_64 => mismatch at data[{}]",
                i
            );
        }
    }
    fn test_into_iter_by_mut_ref_256() {
        let mut blob = BaseBlob256::default();
        fill_pattern_256(&mut blob);
        for (i, val) in (&mut blob).into_iter().enumerate() {
            let old = *val;
            *val = old.wrapping_add(i as u8);
        }
        for (i, &b) in blob.data.iter().enumerate() {
            let orig = pattern_for_index(i);
            let expected = orig.wrapping_add(i as u8);
            assert_eq!(
                b, expected,
                "test_into_iter_by_mut_ref_256 => mismatch at data[{}]",
                i
            );
        }
    }

    // Helper: fill the blob with a simple pattern: data[i] = pattern_for_index(i).
    fn fill_pattern_8(blob: &mut BaseBlob8) {
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = pattern_for_index(i);
        }
    }
    fn fill_pattern_64(blob: &mut BaseBlob64) {
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = pattern_for_index(i);
        }
    }
    fn fill_pattern_256(blob: &mut BaseBlob256) {
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = pattern_for_index(i);
        }
    }

    fn pattern_for_index(i: usize) -> u8 {
        ((i as u8).wrapping_mul(3)) ^ 0x5A
    }
}
