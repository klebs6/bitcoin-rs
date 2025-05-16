// ---------------- [ File: bitcoin-blob/src/from_bytes.rs ]
crate::ix!();

impl<const BITS: usize> From<u8> for BaseBlob<BITS>
where
    [u8; (BITS % 8) + usize::MAX]: ,
    [(); base_blob_width::<BITS>()]:
{
    /**
      | constructor for constants between
      | 1 and 255
      |
    */
    fn from(v: u8) -> Self {
        debug!(
            "Constructing BaseBlob<{}> from u8=0x{:02X}; only the first byte is set to v",
            BITS,
            v
        );

        let mut out = Self::default();
        out.data[0] = v;
        out
    }
}

impl<const BITS: usize> From<&Vec<u8>> for BaseBlob<BITS>
where
    [u8; (BITS % 8) + usize::MAX]: ,
    [(); base_blob_width::<BITS>()]:
{
    fn from(vch: &Vec<u8>) -> Self {
        debug!(
            "Constructing BaseBlob<{}> from &Vec<u8> of length={}",
            BITS,
            vch.len()
        );

        let expected_len = base_blob_width::<BITS>();
        assert_eq!(
            vch.len(),
            expected_len,
            "Input Vec<u8> must match base_blob_width for BITS={}",
            BITS
        );

        let mut out = Self::default();
        out.data.copy_from_slice(&vch[..]);
        out
    }
}

#[cfg(test)]
mod from_bytes_exhaustive_tests {
    use super::*;

    /// We'll test the `From<u8>` and `From<&Vec<u8>>` implementations.
    /// Specifically, we cover:
    /// 1) Converting a random `u8` => BaseBlob<BITS>, checking only the first byte is set.
    /// 2) Converting a `Vec<u8>` => BaseBlob<BITS>:
    ///    - correct length => success
    ///    - wrong length => panic
    ///
    /// We do this for `B=8`, `B=64`, and `B=256`.
    #[traced_test]
    fn test_from_u8() {
        info!("Testing From<u8> => BaseBlob<B> for B=8, B=64, B=256...");
        test_from_u8_gen::<8>();
        test_from_u8_gen::<64>();
        test_from_u8_gen::<256>();
        info!("From<u8> tests concluded successfully.");
    }

    #[traced_test]
    fn test_from_vec_u8() {
        info!("Testing From<&Vec<u8>> => BaseBlob<B> for B=8, B=64, B=256...");
        test_from_vec_gen::<8>();
        test_from_vec_gen::<64>();
        test_from_vec_gen::<256>();
        info!("From<&Vec<u8>> tests concluded successfully.");
    }

    /// For a given B, pick multiple random `u8` values (including 0) and verify:
    /// - `blob.data[0] == the_u8`
    /// - All other bytes are zero
    /// - If the_u8 == 0 => `blob.is_null() == true`, else => `false`.
    fn test_from_u8_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        info!("Subtest: From<u8> => BaseBlob<{}>", B);

        let test_values = [0u8, 1, 127, 128, 255];
        for &val in test_values.iter() {
            debug!("Trying val=0x{:02X}", val);

            let blob = BaseBlob::<B>::from(val);
            assert_eq!(
                blob.data[0], val,
                "First byte should match input val for B={}",
                B
            );
            // All other bytes must be 0:
            let width = base_blob_width::<B>();
            for i in 1..width {
                assert_eq!(
                    blob.data[i],
                    0,
                    "All bytes except data[0] should be 0 for B={}",
                    B
                );
            }

            // If val=0 => is_null => true, else => false
            if val == 0 {
                assert!(
                    blob.is_null(),
                    "If val=0, is_null() should be true for B={}",
                    B
                );
            } else {
                assert!(
                    !blob.is_null(),
                    "If val !=0, is_null() should be false for B={}",
                    B
                );
            }
        }
    }

    /// For a given B, we do:
    /// - "happy path": correct-length Vec => verify it copies all bytes
    /// - "sad path": wrong-length Vec => assert it panics
    fn test_from_vec_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        info!("Subtest: From<&Vec<u8>> => BaseBlob<{}>", B);

        let width = base_blob_width::<B>();

        // 1) "happy path": correct-length
        let mut good_vec = vec![0u8; width];
        // fill with a deterministic pattern
        for (i, b) in good_vec.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(11); // just a simple pattern
        }

        let blob_good = BaseBlob::<B>::from(&good_vec);
        // verify data matches
        for i in 0..width {
            assert_eq!(
                blob_good.data[i], good_vec[i],
                "blob.data[{}] mismatch for B={}",
                i, B
            );
        }

        // 2) "sad path": too short
        if width > 0 {
            let shorter_len = width - 1;
            let short_vec = vec![0u8; shorter_len];
            let caught_short = std::panic::catch_unwind(|| {
                let _blob_short = BaseBlob::<B>::from(&short_vec);
            });
            assert!(
                caught_short.is_err(),
                "Expected panic from short Vec of len={} vs width={}, B={}",
                shorter_len,
                width,
                B
            );
        }

        // 3) "sad path": too long
        let longer_len = width + 1; // at least 1 bigger
        let long_vec = vec![0u8; longer_len];
        let caught_long = std::panic::catch_unwind(|| {
            let _blob_long = BaseBlob::<B>::from(&long_vec);
        });
        assert!(
            caught_long.is_err(),
            "Expected panic from long Vec of len={} vs width={}, B={}",
            longer_len,
            width,
            B
        );
    }
}
