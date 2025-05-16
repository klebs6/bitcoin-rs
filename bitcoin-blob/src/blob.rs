// ---------------- [ File: bitcoin-blob/src/blob.rs ]
crate::ix!();

/**
  | Template base class for fixed-sized
  | opaque blobs.
  |
  */
#[derive(Clone,Debug,Hash)]
pub struct BaseBlob<const BITS: usize> 
where [u8; base_blob_width::<BITS>()]:
{
    pub data: [u8; base_blob_width::<BITS>()],
}

//------------------------------
unsafe impl<const BITS: usize> Send for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{}

unsafe impl<const BITS: usize> Sync for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{}

//------------------------------
impl<const BITS: usize> Default for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{

    /**
      | construct 0 value by default
      |
      */
    fn default() -> Self {
    
        Self {
            data: [0; base_blob_width::<BITS>()],
        }
    }
}

impl<const BITS: usize> BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    pub const ZERO: Self = Self::zero();
    pub const ONE:  Self = Self::one();

    pub const fn zero() -> Self {
        // all bytes = 0
        Self {
            data: [0; base_blob_width::<BITS>()],
        }
    }

    pub const fn one() -> Self {
        // all bytes = 0, except the first byte=1
        let mut x = Self {
            data: [0; base_blob_width::<BITS>()],
        };
        // We can do this in const fn if BITS>0, but it's stable in modern Rust:
        x.data[0] = 1;
        x
    }
}

#[cfg(test)]
mod base_blob_exhaustive_tests {
    use super::*;

    /// Test the `base_blob_width` const-fn for some example bit-sizes.
    #[traced_test]
    fn test_base_blob_width_invariants() {
        info!("Testing base_blob_width for various BITS.");

        let w64 = base_blob_width::<64>();
        trace!("width for 64 bits => {}", w64);
        assert_eq!(w64, 8, "64 bits => 8 bytes");

        let w256 = base_blob_width::<256>();
        trace!("width for 256 bits => {}", w256);
        assert_eq!(w256, 32, "256 bits => 32 bytes");

        let w8 = base_blob_width::<8>();
        trace!("width for 8 bits => {}", w8);
        assert_eq!(w8, 1, "8 bits => 1 byte");

        let w1 = base_blob_width::<1>();
        trace!("width for 1 bit => {}", w1);
        assert_eq!(w1, 0, "1 bit => 0 bytes (integer division)");

        info!("base_blob_width tests concluded successfully.");
    }

    /// Test Default for various bit sizes, ensuring we get arrays of 0.
    #[traced_test]
    fn test_base_blob_default() {
        info!("Testing `Default` impl for BaseBlob<BITS>.");

        let default64 = BaseBlob::<64>::default();
        for &b in default64.data.iter() {
            assert_eq!(b, 0, "All bytes should be zero for default 64-bit blob");
        }

        let default256 = BaseBlob::<256>::default();
        for &b in default256.data.iter() {
            assert_eq!(b, 0, "All bytes should be zero for default 256-bit blob");
        }

        let default8 = BaseBlob::<8>::default();
        assert_eq!(default8.data.len(), 1, "8 bits => 1 byte array");
        assert_eq!(default8.data[0], 0, "All zeros for default 8-bit blob");

        info!("Default creation checks complete.");
    }

    #[traced_test]
    fn test_eq_ord_random() {
        use super::*;
        use core::cmp::Ordering;
        use tracing::{debug, error, info, trace};
        info!("Testing Eq/Ord with random BaseBlob data.");

        let mut rng = SimpleRng::new(0xDEAD_BEEF);

        // We'll do multiple random comparisons for 128 bits and 256 bits.
        // Because Rust can't infer the const generic from a runtime variable,
        // we match on bits to pick the correct BaseBlob<B> type.
        for bits in [128, 256] {
            info!("Subtest for BITS={}", bits);

            // We'll run 20 trials for each size
            for _i in 0..20 {
                // First, fill random buffers large enough for 256 bits (32 bytes).
                // Then we’ll just slice off the exact length for 128 or 256 below.
                let mut buf1 = vec![0u8; 32];
                let mut buf2 = vec![0u8; 32];

                rng.fill_bytes(&mut buf1);
                rng.fill_bytes(&mut buf2);

                // For 128 bits => 16 bytes, for 256 bits => 32 bytes
                let width = bits / 8; 
                let data1 = &buf1[..width];
                let data2 = &buf2[..width];

                // Now we pick the type at compile time via match:
                match bits {
                    128 => {
                        let blob1: BaseBlob<128> = make_blob::<128>(data1);
                        let blob2: BaseBlob<128> = make_blob::<128>(data2);

                        // Compare them
                        let eq_std = (data1 == data2);
                        let eq_blob = (blob1 == blob2);
                        assert_eq!(
                            eq_std, eq_blob,
                            "Eq mismatch: BITS={}, data1={:X?}, data2={:X?}",
                            bits, data1, data2
                        );

                        let cmp_std = data1.cmp(data2);
                        let cmp_blob = blob1.cmp(&blob2);
                        assert_eq!(
                            cmp_std, cmp_blob,
                            "Ordering mismatch: BITS={}, data1={:X?}, data2={:X?}",
                            bits, data1, data2
                        );
                    }
                    256 => {
                        let blob1: BaseBlob<256> = make_blob::<256>(data1);
                        let blob2: BaseBlob<256> = make_blob::<256>(data2);

                        let eq_std = (data1 == data2);
                        let eq_blob = (blob1 == blob2);
                        assert_eq!(
                            eq_std, eq_blob,
                            "Eq mismatch: BITS={}, data1={:X?}, data2={:X?}",
                            bits, data1, data2
                        );

                        let cmp_std = data1.cmp(data2);
                        let cmp_blob = blob1.cmp(&blob2);
                        assert_eq!(
                            cmp_std, cmp_blob,
                            "Ordering mismatch: BITS={}, data1={:X?}, data2={:X?}",
                            bits, data1, data2
                        );
                    }
                    _ => {
                        panic!("Unsupported bits={}", bits);
                    }
                }
            }
        }

        info!("Eq and Ord random tests completed successfully.");
    }

    /// This test specifically checks that "all ones" vs. "all zeros" ordering, plus
    /// a few partial checks, is correct for BITS=256.
    #[traced_test]
    fn test_base_blob_cmp_extremes_256() {
        info!("Testing extremes: all-zeros vs. all-ones for BITS=256.");

        let mut zero_blob = BaseBlob::<256>::default();
        let mut ones_blob = BaseBlob::<256>::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }

        // zero vs. ones => zero < ones
        assert!(zero_blob < ones_blob, "All-zero < all-ones for 256 bits");
        assert!(ones_blob > zero_blob, "All-ones > all-zero for 256 bits");
        assert_ne!(zero_blob, ones_blob, "Zero != ones, obviously");

        // flip a single byte in zero_blob to 0xFF => now compare
        zero_blob.data[31] = 0xFF; // the last byte (big-end? little-end? doesn't matter, we just want difference)
        assert!(zero_blob < ones_blob, "A single trailing 0xFF is still < 32 x 0xFF for BITS=256");

        // make them identical => eq
        zero_blob.data.copy_from_slice(&ones_blob.data);
        assert!(zero_blob == ones_blob, "Now identical => eq for 256 bits");
        info!("Checked extremes for BITS=256 comparisons successfully.");
    }

    /// Check that BaseBlob<BITS> is `Send + Sync` in a practical sense by moving it between threads.
    #[traced_test]
    fn test_base_blob_send_sync_64() {
        info!("Testing that BaseBlob<64> can be sent across threads (Send + Sync).");

        let mut some_blob = BaseBlob::<64>::default();
        // fill it with something
        some_blob.data.copy_from_slice(&[1,2,3,4,5,6,7,8]);

        // Move it into a closure, spawn a thread, move it back
        let handle = std::thread::spawn(move || {
            debug!("Thread: got the blob => data={:X?}", some_blob.data);
            some_blob
        });

        let returned = handle.join().expect("Thread panicked?");
        debug!("test: returned from thread => data={:X?}", returned.data);
        assert_eq!(&returned.data[..8], &[1,2,3,4,5,6,7,8],
                   "Should match the data we initially set");
        info!("Send+Sync test for 64-bit succeeded.");
    }
}
