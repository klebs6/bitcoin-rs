// ---------------- [ File: bitcoin-blob/src/ord_eq.rs ]
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

#[cfg(test)]
mod eq_ord_exhaustive_tests {
    use super::*;

    /// We'll perform an exhaustive set of checks for:
    ///
    /// - `PartialEq` and `Eq`
    /// - `Ord` and `PartialOrd`
    ///
    /// We confirm that:
    ///   1) Zero vs. ones vs. a "mid" pattern yields the expected ordering.
    ///   2) Self-comparison => equality (reflexive).
    ///   3) A random set of pairs each match their slice-comparison order.
    ///
    /// We do this for B=8, B=64, and B=256 to cover small, typical, and large sizes.

    #[traced_test]
    fn test_eq_ord_extremes() {
        info!("Testing extreme values (all-zeros, all-ones, partial) for B=8,64,256...");
        test_eq_ord_extremes_gen::<8>();
        test_eq_ord_extremes_gen::<64>();
        test_eq_ord_extremes_gen::<256>();
        info!("extremes test concluded successfully.");
    }

    /// For a given B, we test:
    ///   - zero_blob < mid_blob < ones_blob
    ///   - zero_blob == zero_blob
    ///   - ones_blob == ones_blob
    ///   - mid_blob == mid_blob
    fn test_eq_ord_extremes_gen<const B: usize>()
    where
        [(); base_blob_width::<B>()]:,
        [u8; (B % 8) + usize::MAX]:,
    {
        let width = base_blob_width::<B>();

        // 1) zero
        let zero_blob = BaseBlob::<B>::default();

        // 2) ones
        let mut ones_blob = BaseBlob::<B>::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }

        // 3) mid: for one byte (B=8), let's pick 0x7F. Otherwise, half 0x00, half 0xFF.
        let mut mid_blob = BaseBlob::<B>::default();
        if width == 1 {
            mid_blob.data[0] = 0x7F;
        } else {
            let half = width / 2;
            for b in mid_blob.data[half..].iter_mut() {
                *b = 0xFF;
            }
        }

        // Check order
        assert!(zero_blob < mid_blob, "B={}: zero < mid", B);
        assert!(mid_blob < ones_blob, "B={}: mid < ones", B);
        assert!(zero_blob < ones_blob, "B={}: zero < ones", B);

        // reflexivity
        assert_eq!(zero_blob, zero_blob, "B={}: zero == zero", B);
        assert_eq!(mid_blob, mid_blob, "B={}: mid == mid", B);
        assert_eq!(ones_blob, ones_blob, "B={}: ones == ones", B);
    }

    #[traced_test]
    fn test_eq_ord_random() {
        info!("Testing random pairs for B=8, B=64, B=256...");
        test_eq_ord_random_gen::<8>();
        test_eq_ord_random_gen::<64>();
        test_eq_ord_random_gen::<256>();
        info!("random eq/ord test concluded successfully.");
    }

    /// We'll generate random data for two blobs, compare them bytewise to see which is smaller,
    /// then confirm the BaseBlob<B>::cmp => matches that.
    fn test_eq_ord_random_gen<const B: usize>()
    where
        [(); base_blob_width::<B>()]:,
        [u8; (B % 8) + usize::MAX]:,
    {
        let mut rng = SimpleRng::new(0xABCDEF0123456789);

        // We'll do 20 random pairs
        for i in 0..20 {
            // build random data
            let width = base_blob_width::<B>();
            let mut buf1 = vec![0u8; width];
            let mut buf2 = vec![0u8; width];
            rng.fill_bytes(&mut buf1);
            rng.fill_bytes(&mut buf2);

            // create BaseBlob<B>
            let blob1 = make_blob::<B>(&buf1);
            let blob2 = make_blob::<B>(&buf2);

            // do a direct slice comparison
            let cmp_slices = buf1.cmp(&buf2);
            let blob_eq = (blob1 == blob2);
            let blob_cmp = blob1.cmp(&blob2);

            // check eq vs. eq_slices
            let slices_eq = (cmp_slices == core::cmp::Ordering::Equal);
            assert_eq!(
                blob_eq, slices_eq,
                "B={}, i={} => mismatch in equality: \n buf1={:X?}\n buf2={:X?}",
                B, i, buf1, buf2
            );

            // check the ordering
            assert_eq!(
                blob_cmp, cmp_slices,
                "B={}, i={} => mismatch in ordering: \n buf1={:X?}\n buf2={:X?}",
                B, i, buf1, buf2
            );

            // partial_cmp should match
            let blob_pcmp = blob1.partial_cmp(&blob2).unwrap();
            assert_eq!(
                blob_pcmp, blob_cmp,
                "B={}, i={} => partial_cmp mismatch with cmp",
                B, i
            );
        }
    }

    // A simple helper to build a BaseBlob<B> from a slice
    fn make_blob<const B: usize>(src: &[u8]) -> BaseBlob<B>
    where
        [(); base_blob_width::<B>()]:,
        [u8; (B % 8) + usize::MAX]:,
    {
        assert_eq!(
            src.len(),
            base_blob_width::<B>(),
            "Source length must match base_blob_width<{}>",
            B
        );
        let mut blob = BaseBlob::<B>::default();
        blob.data.copy_from_slice(src);
        blob
    }

    // A simple random generator from earlier code
    struct SimpleRng(u64);
    impl SimpleRng {
        fn new(seed: u64) -> Self {
            Self(seed)
        }
        fn next_u64(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1);
            self.0
        }
        fn fill_bytes(&mut self, buf: &mut [u8]) {
            for chunk in buf.chunks_mut(8) {
                let rnd = self.next_u64().to_le_bytes();
                let n = chunk.len();
                chunk.copy_from_slice(&rnd[..n]);
            }
        }
    }
}
