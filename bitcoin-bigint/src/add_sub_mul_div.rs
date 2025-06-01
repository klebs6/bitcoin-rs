// ---------------- [ File: bitcoin-bigint/src/add_sub_mul_div.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_baseuint_add_sub_mul_div {
    ($uint_type:ident, $bits:expr, $limbs:expr) => {

        impl core::ops::Add<&$uint_type> for $uint_type {
            type Output = $uint_type;
            fn add(self, other: &$uint_type) -> Self::Output {
                let mut ret = self.clone();
                ret += other;
                ret
            }
        }

        impl core::ops::Sub<&$uint_type> for $uint_type {
            type Output = $uint_type;
            fn sub(self, other: &$uint_type) -> Self::Output {
                let mut ret = self.clone();
                ret -= other; // we rely on SubAssign
                ret
            }
        }

        impl core::ops::Mul<&$uint_type> for $uint_type {
            type Output = $uint_type;
            fn mul(self, other: &$uint_type) -> Self::Output {
                let mut ret = self.clone();
                ret *= other; // we rely on MulAssign
                ret
            }
        }

        impl core::ops::Div<&$uint_type> for $uint_type {
            type Output = $uint_type;
            fn div(self, other: &$uint_type) -> Self::Output {
                let mut ret = self.clone();
                ret /= other; // we rely on DivAssign
                ret
            }
        }
    }
}

#[cfg(test)]
mod add_sub_mul_div_tests {
    use super::*;
    use tracing::{info, debug, trace};

    #[traced_test]
    fn test_add_sub_64_bits() {
        info!("Testing Add and Sub operators with 64-bit BaseUInt.");

        type U64 = BaseUInt64;

        let x = {
            let mut tmp = U64::default();
            tmp.pn[0] = 10;
            tmp
        };
        let y = {
            let mut tmp = U64::default();
            tmp.pn[0] = 3;
            tmp
        };

        let z1 = x.clone() + &y; 
        assert_eq!(z1.pn[0], 13);
        assert_eq!(z1.pn[1], 0);

        let z2 = x.clone() - &y;
        assert_eq!(z2.pn[0], 7);
        assert_eq!(z2.pn[1], 0);

        // Wrap-around
        let a = {
            let mut tmp = U64::default();
            tmp.pn[0] = 1;
            tmp
        };
        let b = {
            let mut tmp = U64::default();
            tmp.pn[0] = 2;
            tmp
        };
        let c = a - &b;
        assert_eq!(c.pn[0], 0xffff_ffff);
        assert_eq!(c.pn[1], 0xffff_ffff);

        info!("Add/Sub operator tests passed.");
    }

    #[traced_test]
    fn test_mul_div_64_bits() {
        info!("Testing Mul and Div operators with 64-bit BaseUInt.");

        type U64 = BaseUInt64;

        let x = {
            let mut tmp = U64::default();
            tmp.pn[0] = 6;
            tmp
        };
        let y = {
            let mut tmp = U64::default();
            tmp.pn[0] = 7;
            tmp
        };

        let z = x.clone() * &y; 
        assert_eq!(z.pn[0], 42);
        assert_eq!(z.pn[1], 0);

        let w = z.clone() / &x; 
        assert_eq!(w.pn[0], 7);
        assert_eq!(w.pn[1], 0);

        info!("Mul/Div operator tests passed.");
    }
}
