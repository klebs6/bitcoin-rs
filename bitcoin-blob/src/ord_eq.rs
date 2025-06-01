// ---------------- [ File: bitcoin-blob/src/ord_eq.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_base_blob_ord_eq {
    (
        $blob_ty:ident,
        $bits:expr,
        $bytes:expr
    ) => {

        impl PartialEq<$blob_ty> for $blob_ty {
            fn eq(&self, other: &$blob_ty) -> bool {
                self.compare(other) == 0
            }
        }
        impl Eq for $blob_ty {}

        impl Ord for $blob_ty {
            fn cmp(&self, other: &$blob_ty) -> Ordering {
                let x = self.compare(other);
                match x {
                    _ if x < 0  => Ordering::Less,
                    _ if x == 0 => Ordering::Equal,
                    _ if x > 0  => Ordering::Greater,
                    _ => unreachable!(),
                }
            }
        }

        impl PartialOrd<$blob_ty> for $blob_ty {
            fn partial_cmp(&self, other: &$blob_ty) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
    }
}

#[cfg(test)]
mod eq_ord_exhaustive_tests {
    use super::*;
    use tracing::{info, trace, debug};

    #[traced_test]
    fn test_eq_ord_extremes() {
        info!("Testing extreme values (all-zeros, all-ones, partial) for B=8,64,256...");
        test_eq_ord_extremes_8();
        test_eq_ord_extremes_64();
        test_eq_ord_extremes_256();
        info!("extremes test concluded successfully.");
    }

    fn test_eq_ord_extremes_8() {
        // zero
        let zero_blob = BaseBlob8::default();
        // ones
        let mut ones_blob = BaseBlob8::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }
        // mid => 0x7F
        let mut mid_blob = BaseBlob8::default();
        mid_blob.data[0] = 0x7F;

        assert!(zero_blob < mid_blob, "B=8: zero < mid");
        assert!(mid_blob < ones_blob, "B=8: mid < ones");
        assert!(zero_blob < ones_blob, "B=8: zero < ones");

        // reflexivity
        assert_eq!(zero_blob, zero_blob, "B=8: zero == zero");
        assert_eq!(mid_blob, mid_blob, "B=8: mid == mid");
        assert_eq!(ones_blob, ones_blob, "B=8: ones == ones");
    }

    fn test_eq_ord_extremes_64() {
        // zero
        let zero_blob = BaseBlob64::default();
        // ones
        let mut ones_blob = BaseBlob64::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }
        // "mid": half 0, half 0xFF => but it's only 8 bytes, so let's do the top 4 = 0, bottom 4=0xFF
        let mut mid_blob = BaseBlob64::default();
        let half = 8 / 2;
        for b in mid_blob.data[half..].iter_mut() {
            *b = 0xFF;
        }

        assert!(zero_blob < mid_blob, "B=64: zero < mid");
        assert!(mid_blob < ones_blob, "B=64: mid < ones");
        assert!(zero_blob < ones_blob, "B=64: zero < ones");

        // reflexivity
        assert_eq!(zero_blob, zero_blob, "B=64: zero == zero");
        assert_eq!(mid_blob, mid_blob, "B=64: mid == mid");
        assert_eq!(ones_blob, ones_blob, "B=64: ones == ones");
    }

    fn test_eq_ord_extremes_256() {
        // zero
        let zero_blob = BaseBlob256::default();
        // ones
        let mut ones_blob = BaseBlob256::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }
        // mid => half 0, half 0xFF (16 zero, 16 FF)
        let mut mid_blob = BaseBlob256::default();
        let half = 32 / 2;
        for b in mid_blob.data[half..].iter_mut() {
            *b = 0xFF;
        }

        assert!(zero_blob < mid_blob, "B=256: zero < mid");
        assert!(mid_blob < ones_blob, "B=256: mid < ones");
        assert!(zero_blob < ones_blob, "B=256: zero < ones");

        // reflexivity
        assert_eq!(zero_blob, zero_blob, "B=256: zero == zero");
        assert_eq!(mid_blob, mid_blob, "B=256: mid == mid");
        assert_eq!(ones_blob, ones_blob, "B=256: ones == ones");
    }

    #[traced_test]
    fn test_eq_ord_random() {
        info!("Testing random pairs for B=8, B=64, B=256...");
        test_eq_ord_random_8();
        test_eq_ord_random_64();
        test_eq_ord_random_256();
        info!("random eq/ord test concluded successfully.");
    }

    fn test_eq_ord_random_8() {
        let mut rng = crate::simple_rng::SimpleRng::new(0xABCDEF0123456789);
        for _i in 0..20 {
            let mut buf1 = [0u8; 1];
            let mut buf2 = [0u8; 1];
            rng.fill_bytes(&mut buf1);
            rng.fill_bytes(&mut buf2);
            let blob1 = BaseBlob8 { data: buf1 };
            let blob2 = BaseBlob8 { data: buf2 };

            let cmp_slices = buf1.cmp(&buf2);
            let blob_eq = (blob1 == blob2);
            let blob_cmp = blob1.cmp(&blob2);
            let slices_eq = (cmp_slices == core::cmp::Ordering::Equal);
            assert_eq!(
                blob_eq, slices_eq,
                "B=8 => mismatch in equality"
            );
            assert_eq!(
                blob_cmp, cmp_slices,
                "B=8 => mismatch in ordering"
            );
        }
    }

    fn test_eq_ord_random_64() {
        let mut rng = crate::simple_rng::SimpleRng::new(0xABCDEF0123456789);
        for _i in 0..20 {
            let mut buf1 = [0u8; 8];
            let mut buf2 = [0u8; 8];
            rng.fill_bytes(&mut buf1);
            rng.fill_bytes(&mut buf2);
            let blob1 = BaseBlob64 { data: buf1 };
            let blob2 = BaseBlob64 { data: buf2 };

            let cmp_slices = buf1.cmp(&buf2);
            let blob_eq = (blob1 == blob2);
            let blob_cmp = blob1.cmp(&blob2);
            let slices_eq = (cmp_slices == core::cmp::Ordering::Equal);
            assert_eq!(
                blob_eq, slices_eq,
                "B=64 => mismatch in equality"
            );
            assert_eq!(
                blob_cmp, cmp_slices,
                "B=64 => mismatch in ordering"
            );
        }
    }

    fn test_eq_ord_random_256() {
        let mut rng = crate::simple_rng::SimpleRng::new(0xABCDEF0123456789);
        for _i in 0..20 {
            let mut buf1 = [0u8; 32];
            let mut buf2 = [0u8; 32];
            rng.fill_bytes(&mut buf1);
            rng.fill_bytes(&mut buf2);
            let blob1 = BaseBlob256 { data: buf1 };
            let blob2 = BaseBlob256 { data: buf2 };

            let cmp_slices = buf1.cmp(&buf2);
            let blob_eq = (blob1 == blob2);
            let blob_cmp = blob1.cmp(&blob2);
            let slices_eq = (cmp_slices == core::cmp::Ordering::Equal);
            assert_eq!(
                blob_eq, slices_eq,
                "B=256 => mismatch in equality"
            );
            assert_eq!(
                blob_cmp, cmp_slices,
                "B=256 => mismatch in ordering"
            );
        }
    }
}
