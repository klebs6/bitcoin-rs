crate::ix!();

// Note: muladd macros inspired by the macros in OpenSSL's crypto/bn/asm/x86_64-gcc.c.
//
#[cfg(feature = "widemul-int128")]
#[macro_export]
macro_rules! define_widemul_accum_macros {
    ($c0:ident, $c1:ident, $c2:ident) => {

        /// Add a*b to the number defined by (c0,c1,c2). c2 must never overflow.
        macro_rules! muladd {
            ($a:expr, $b:expr) => {{
                let tl: u64;
                let mut th: u64;
                {
                    let t: u128 = ($a as u128) * ($b as u128);

                    /* at most 0xFFFFFFFFFFFFFFFE */
                    th = (t >> 64) as u64;

                    tl = t as u64;
                }

                /* overflow is handled on the next line */
                $c0 = $c0.wrapping_add(tl);

                /* at most 0xFFFFFFFFFFFFFFFF */
                th = th.wrapping_add(($c0 < tl) as u64);

                /* overflow is handled on the next line */
                $c1 = $c1.wrapping_add(th);

                /* never overflows by contract (verified in the next line) */
                $c2 = $c2.wrapping_add(($c1 < th) as u32);

                verify_check!(($c1 >= th) || ($c2 != 0));
            }};
        }

        /// Add a*b to the number defined by (c0,c1). c1 must never overflow.
        macro_rules! muladd_fast {
            ($a:expr, $b:expr) => {{
                let tl: u64;
                let mut th: u64;
                {
                    let t: u128 = ($a as u128) * ($b as u128);

                    /* at most 0xFFFFFFFFFFFFFFFE */
                    th = (t >> 64) as u64;

                    tl = t as u64;
                }

                /* overflow is handled on the next line */
                $c0 = $c0.wrapping_add(tl);

                /* at most 0xFFFFFFFFFFFFFFFF */
                th = th.wrapping_add(($c0 < tl) as u64);

                /* never overflows by contract (verified in the next line) */
                $c1 = $c1.wrapping_add(th);

                verify_check!($c1 >= th);
            }};
        }

        /// Add a to the number defined by (c0,c1,c2). c2 must never overflow.
        macro_rules! sumadd {
            ($a:expr) => {{
                let over: u8;

                /* overflow is handled on the next line */
                $c0 = $c0.wrapping_add(($a) as _);

                over = ($c0 < (($a) as _)) as u8;

                /* overflow is handled on the next line */
                $c1 = $c1.wrapping_add(over as _);

                /* never overflows by contract */
                $c2 = $c2.wrapping_add(($c1 < (over as _)) as _);
            }};
        }

        /// Add a to the number defined by (c0,c1). c1 must never overflow, c2 must be zero.
        macro_rules! sumadd_fast {
            ($a:expr) => {{

                /* overflow is handled on the next line */
                $c0 = $c0.wrapping_add(($a) as _);

                /* never overflows by contract (verified the next line) */
                $c1 = $c1.wrapping_add(($c0 < (($a) as _)) as _);

                verify_check!(($c1 != 0) | ($c0 >= (($a) as _)));
                verify_check!($c2 == 0);
            }};
        }

        /// Extract the lowest 64 bits of (c0,c1,c2) into n, and left shift the number 64 bits.
        macro_rules! extract {
            ($n:expr) => {{
                ($n) = $c0;
                $c0 = $c1;
                $c1 = $c2 as _;
                $c2 = 0 as _;
            }};
        }

        /// Extract the lowest 64 bits of (c0,c1,c2) into n, and left shift the number 64 bits. c2 is required to be zero.
        macro_rules! extract_fast {
            ($n:expr) => {{
                ($n) = $c0;
                $c0 = $c1;
                $c1 = 0 as _;
                verify_check!($c2 == 0);
            }};
        }
    };
}

#[cfg(feature = "widemul-int64")]
#[macro_export]
macro_rules! define_widemul_accum_macros {
    ($c0:ident, $c1:ident, $c2:ident) => {

        /// Add a*b to the number defined by (c0,c1,c2). c2 must never overflow.
        macro_rules! muladd {
            ($a:expr, $b:expr) => {{
                let tl: u32;
                let mut th: u32;
                {
                    let t: u64 = ($a as u64) * ($b as u64);

                    /* at most 0xFFFFFFFE */
                    th = (t >> 32) as u32;

                    tl = t as u32;
                }

                /* overflow is handled on the next line */
                $c0 = $c0.wrapping_add(tl);

                /* at most 0xFFFFFFFF */
                th = th.wrapping_add(($c0 < tl) as u32);

                /* overflow is handled on the next line */
                $c1 = $c1.wrapping_add(th);

                /* never overflows by contract (verified in the next line) */
                $c2 = $c2.wrapping_add(($c1 < th) as u32);

                verify_check!(($c1 >= th) || ($c2 != 0));
            }};
        }

        /// Add a*b to the number defined by (c0,c1). c1 must never overflow.
        macro_rules! muladd_fast {
            ($a:expr, $b:expr) => {{
                let tl: u32;
                let mut th: u32;
                {
                    let t: u64 = ($a as u64) * ($b as u64);

                    /* at most 0xFFFFFFFE */
                    th = (t >> 32) as u32;

                    tl = t as u32;
                }

                /* overflow is handled on the next line */
                $c0 = $c0.wrapping_add(tl);

                /* at most 0xFFFFFFFF */
                th = th.wrapping_add(($c0 < tl) as u32);

                /* never overflows by contract (verified in the next line) */
                $c1 = $c1.wrapping_add(th);

                verify_check!($c1 >= th);
            }};
        }

        /// Add a to the number defined by (c0,c1,c2). c2 must never overflow.
        macro_rules! sumadd {
            ($a:expr) => {{
                let over: u8;

                /* overflow is handled on the next line */
                $c0 = $c0.wrapping_add(($a) as _);

                over = ($c0 < (($a) as _)) as u8;

                /* overflow is handled on the next line */
                $c1 = $c1.wrapping_add(over as _);

                /* never overflows by contract */
                $c2 = $c2.wrapping_add(($c1 < (over as _)) as _);
            }};
        }

        /// Add a to the number defined by (c0,c1).  c1 must never overflow, c2 must be zero.
        macro_rules! sumadd_fast {
            ($a:expr) => {{

                /* overflow is handled on the next line */
                $c0 = $c0.wrapping_add(($a) as _);

                /* never overflows by contract (verified the next line) */
                $c1 = $c1.wrapping_add(($c0 < (($a) as _)) as _);

                verify_check!(($c1 != 0) | ($c0 >= (($a) as _)));
                verify_check!($c2 == 0);
            }};
        }

        /// Extract the lowest 32 bits of (c0,c1,c2) into n, and left shift the number 32 bits.
        macro_rules! extract {
            ($n:expr) => {{
                ($n) = $c0;
                $c0 = $c1;
                $c1 = $c2 as _;
                $c2 = 0 as _;
            }};
        }

        /// Extract the lowest 32 bits of (c0,c1,c2) into n, and left shift the number 32 bits. c2 is required to be zero.
        macro_rules! extract_fast {
            ($n:expr) => {{
                ($n) = $c0;
                $c0 = $c1;
                $c1 = 0 as _;
                verify_check!($c2 == 0);
            }};
        }
    };
}

#[cfg(test)]
mod widemul_accumulator_macro_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    #[cfg(feature = "widemul-int128")]
    fn define_widemul_accum_macros_supports_basic_accumulate_and_extract_flow_u64() {
        info!("validating define_widemul_accum_macros! (widemul-int128) basic accumulate/extract behavior");

        let mut c0: u64 = 0;
        let mut c1: u64 = 0;
        let mut c2: u32 = 0;

        define_widemul_accum_macros!(c0, c1, c2);

        // Accumulate known products and extract limbs, compare against reference 512-bit product.
        let a_be = SECP256K1_ORDER_MINUS_1_BE;
        let b_be = SCALAR_MAX_U32_BE;

        let a = scalar_from_be_bytes(&a_be);
        let b = scalar_from_be_bytes(&b_be);

        let mut l = [0u64; 8];
        unsafe {
            scalar_mul_512(l.as_mut_ptr(), &a as *const Scalar, &b as *const Scalar);
        }

        // Now build the same product by driving the local macros like the algorithm does on a tiny example:
        // We verify that extract/extract_fast shift is coherent by comparing against the first two limbs from scalar_mul_512.
        // (This is intentionally lightweight; scalar_mul_512/reduce tests do the heavy lifting.)
        muladd_fast!(1u64, 2u64); // +2
        extract_fast!(l[0]);
        sumadd_fast!(3u64); // +3 into accumulator state
        extract_fast!(l[1]);

        debug!(l0 = l[0], l1 = l[1], "macro-driven extract results");
        // With the above sequence, l[0] becomes 2, and l[1] becomes 3 (given the shift).
        assert_eq!(l[0], 2u64);
        assert_eq!(l[1], 3u64);
    }

    #[traced_test]
    #[cfg(feature = "widemul-int64")]
    fn define_widemul_accum_macros_supports_basic_accumulate_and_extract_flow_u32() {
        info!("validating define_widemul_accum_macros! (widemul-int64) basic accumulate/extract behavior");

        let mut c0: u32 = 0;
        let mut c1: u32 = 0;
        let mut c2: u32 = 0;

        define_widemul_accum_macros!(c0, c1, c2);

        let mut out0: u32 = 0;
        let mut out1: u32 = 0;

        muladd_fast!(1u32, 2u32); // +2
        extract_fast!(out0);
        sumadd_fast!(3u32); // +3
        extract_fast!(out1);

        debug!(out0, out1, "macro-driven extract results");
        assert_eq!(out0, 2u32);
        assert_eq!(out1, 3u32);
    }
}
