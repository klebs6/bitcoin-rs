// ---------------- [ File: bitcoin-blob/src/blob.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_base_blob_struct {
    (
        $blob_ty:ident,
        $bits:expr,
        $bytes:expr
    ) => {
        /**
          | Template base class for fixed-sized
          | opaque blobs (concretized via macro).
          |
        */
        #[derive(Clone,Debug,Hash)]
        pub struct $blob_ty {
            pub(crate) data: [u8; $bytes],
        }

        // Safety as in the original code
        unsafe impl Send for $blob_ty {}
        unsafe impl Sync for $blob_ty {}

        impl Default for $blob_ty {
            fn default() -> Self {
                Self {
                    data: [0; $bytes],
                }
            }
        }

        impl $blob_ty {
            /// A pure `const fn` constructor copying in `arr`.
            pub const fn from_bytes(arr: [u8; $bytes]) -> Self {
                Self { data: arr }
            }

            pub fn zero() -> Self {
                Self {
                    data: [0; $bytes],
                }
            }

            pub fn one() -> Self {
                let mut x = Self {
                    data: [0; $bytes],
                };
                if $bytes > 0 {
                    x.data[0] = 1;
                }
                x
            }
        }
    }
}

#[cfg(test)]
mod base_blob_exhaustive_tests {
    use super::*;
    use crate::simple_rng::SimpleRng;
    use tracing::{info, debug, trace};

    /// Test that we get correct `.size()` for each concrete BaseBlob type.
    #[traced_test]
    fn test_base_blob_width_invariants() {
        info!("Testing known widths for each base blob type...");

        // 64 bits => 8 bytes
        assert_eq!(BaseBlob64::default().size(), 8, "BaseBlob64 => 8 bytes");

        // 256 bits => 32 bytes
        assert_eq!(BaseBlob256::default().size(), 32, "BaseBlob256 => 32 bytes");

        // 8 bits => 1 byte
        assert_eq!(BaseBlob8::default().size(), 1, "BaseBlob8 => 1 byte");

        // 128 bits => 16 bytes
        assert_eq!(BaseBlob128::default().size(), 16, "BaseBlob128 => 16 bytes");

        // 160 bits => 20 bytes
        assert_eq!(BaseBlob160::default().size(), 20, "BaseBlob160 => 20 bytes");

        info!("Done checking widths for each base blob type.");
    }

    /// Test the `Default` impl on a few base-blob widths to ensure zeroed arrays.
    #[traced_test]
    fn test_base_blob_default() {
        info!("Testing `Default` for various BaseBlob types.");

        // 64-bit => 8 bytes
        let default64 = BaseBlob64::default();
        for &b in default64.data.iter() {
            assert_eq!(b, 0, "All bytes should be zero for BaseBlob64 default");
        }

        // 256-bit => 32 bytes
        let default256 = BaseBlob256::default();
        for &b in default256.data.iter() {
            assert_eq!(b, 0, "All bytes should be zero for BaseBlob256 default");
        }

        // 8-bit => 1 byte
        let default8 = BaseBlob8::default();
        assert_eq!(default8.data.len(), 1, "BaseBlob8 => data[] length=1");
        assert_eq!(default8.data[0], 0, "All zeros for default 8-bit blob");

        // 128-bit => 16 bytes
        let default128 = BaseBlob128::default();
        for &b in default128.data.iter() {
            assert_eq!(b, 0, "All zeros for default 128-bit blob");
        }

        // 160-bit => 20 bytes
        let default160 = BaseBlob160::default();
        for &b in default160.data.iter() {
            assert_eq!(b, 0, "All zeros for default 160-bit blob");
        }

        info!("Default creation checks complete.");
    }

    /// Check random comparisons for BaseBlob128 and BaseBlob256
    #[traced_test]
    fn test_eq_ord_random() {
        info!("Testing random Eq/Ord checks for BaseBlob128 and BaseBlob256.");
        let mut rng = SimpleRng::new(0xDEAD_BEEF);

        // Test 128-bit random comparisons
        for _ in 0..20 {
            let mut buf1 = [0u8; 16];
            let mut buf2 = [0u8; 16];
            rng.fill_bytes(&mut buf1);
            rng.fill_bytes(&mut buf2);

            let mut blob1 = BaseBlob128::default();
            blob1.data.copy_from_slice(&buf1);
            let mut blob2 = BaseBlob128::default();
            blob2.data.copy_from_slice(&buf2);

            let eq_std = (buf1 == buf2);
            let eq_blob = (blob1 == blob2);
            assert_eq!(eq_std, eq_blob, "Eq mismatch in 128-bit random test");

            let cmp_std = buf1.cmp(&buf2);
            let cmp_blob = blob1.cmp(&blob2);
            assert_eq!(cmp_std, cmp_blob, "Ord mismatch in 128-bit random test");
        }

        // Test 256-bit random comparisons
        for _ in 0..20 {
            let mut buf1 = [0u8; 32];
            let mut buf2 = [0u8; 32];
            rng.fill_bytes(&mut buf1);
            rng.fill_bytes(&mut buf2);

            let mut blob1 = BaseBlob256::default();
            blob1.data.copy_from_slice(&buf1);
            let mut blob2 = BaseBlob256::default();
            blob2.data.copy_from_slice(&buf2);

            let eq_std = (buf1 == buf2);
            let eq_blob = (blob1 == blob2);
            assert_eq!(eq_std, eq_blob, "Eq mismatch in 256-bit random test");

            let cmp_std = buf1.cmp(&buf2);
            let cmp_blob = blob1.cmp(&blob2);
            assert_eq!(cmp_std, cmp_blob, "Ord mismatch in 256-bit random test");
        }

        info!("Random eq/ord tests completed for 128-bit & 256-bit.");
    }

    /// Check extremes for 256 bits (zero vs. all-ones).
    #[traced_test]
    fn test_base_blob_cmp_extremes_256() {
        info!("Testing extremes: all-zeros vs. all-ones for BaseBlob256.");

        let mut zero_blob = BaseBlob256::default();
        let mut ones_blob = BaseBlob256::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }

        assert!(zero_blob < ones_blob, "All-zero < all-ones in 256 bits");
        assert!(ones_blob > zero_blob, "All-ones > all-zero in 256 bits");
        assert_ne!(zero_blob, ones_blob, "Zero != ones obviously.");

        // Make them identical => eq
        zero_blob.data.copy_from_slice(&ones_blob.data);
        assert_eq!(zero_blob, ones_blob, "Now identical => eq for 256 bits");
        info!("Checked extremes for BaseBlob256 comparisons successfully.");
    }

    /// Check that `BaseBlob64` is Send+Sync by sending it to another thread
    #[traced_test]
    fn test_base_blob_send_sync_64() {
        info!("Testing that BaseBlob64 can be sent across threads (Send + Sync).");

        let mut some_blob = BaseBlob64::default();
        some_blob.data.copy_from_slice(&[1,2,3,4,5,6,7,8]);

        // Move it into a closure, spawn, then get it back
        let handle = std::thread::spawn(move || {
            debug!("Thread: got the blob => data={:X?}", some_blob.data);
            some_blob
        });

        let returned = handle.join().expect("Thread panicked?");
        debug!("test: returned => data={:X?}", returned.data);
        assert_eq!(&returned.data[..8], &[1,2,3,4,5,6,7,8],
            "Should match the data we initially set");
        info!("Send+Sync test for BaseBlob64 succeeded.");
    }
}
