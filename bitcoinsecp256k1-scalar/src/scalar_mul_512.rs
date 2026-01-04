// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_mul_512.rs ]
crate::ix!();

#[cfg(feature = "widemul-int128")]
pub fn scalar_mul_512(l: *mut u64, a: *const Scalar, b: *const Scalar) {
    unsafe {
        #[cfg(all(feature = "asm", target_arch = "x86_64"))]
        {
            let mut pb: *const u64 = (*b).d.as_ptr();
            let ad: *const u64 = (*a).d.as_ptr();

            core::arch::asm!(
                /* Preload */
                "movq 0(%rdi), %r15",
                "movq 8(%rdi), %rbx",
                "movq 16(%rdi), %rcx",
                "movq 0(%rdx), %r11",
                "movq 8(%rdx), %r12",
                "movq 16(%rdx), %r13",
                "movq 24(%rdx), %r14",
                /* (rax,rdx) = a0 * b0 */
                "movq %r15, %rax",
                "mulq %r11",
                /* Extract l0 */
                "movq %rax, 0(%rsi)",
                /* (r8,r9,r10) = (rdx) */
                "movq %rdx, %r8",
                "xorq %r9, %r9",
                "xorq %r10, %r10",
                /* (r8,r9,r10) += a0 * b1 */
                "movq %r15, %rax",
                "mulq %r12",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                "adcq $0, %r10",
                /* (r8,r9,r10) += a1 * b0 */
                "movq %rbx, %rax",
                "mulq %r11",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                "adcq $0, %r10",
                /* Extract l1 */
                "movq %r8, 8(%rsi)",
                "xorq %r8, %r8",
                /* (r9,r10,r8) += a0 * b2 */
                "movq %r15, %rax",
                "mulq %r13",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* (r9,r10,r8) += a1 * b1 */
                "movq %rbx, %rax",
                "mulq %r12",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* (r9,r10,r8) += a2 * b0 */
                "movq %rcx, %rax",
                "mulq %r11",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* Extract l2 */
                "movq %r9, 16(%rsi)",
                "xorq %r9, %r9",
                /* (r10,r8,r9) += a0 * b3 */
                "movq %r15, %rax",
                "mulq %r14",
                "addq %rax, %r10",
                "adcq %rdx, %r8",
                "adcq $0, %r9",
                /* Preload a3 */
                "movq 24(%rdi), %r15",
                /* (r10,r8,r9) += a1 * b2 */
                "movq %rbx, %rax",
                "mulq %r13",
                "addq %rax, %r10",
                "adcq %rdx, %r8",
                "adcq $0, %r9",
                /* (r10,r8,r9) += a2 * b1 */
                "movq %rcx, %rax",
                "mulq %r12",
                "addq %rax, %r10",
                "adcq %rdx, %r8",
                "adcq $0, %r9",
                /* (r10,r8,r9) += a3 * b0 */
                "movq %r15, %rax",
                "mulq %r11",
                "addq %rax, %r10",
                "adcq %rdx, %r8",
                "adcq $0, %r9",
                /* Extract l3 */
                "movq %r10, 24(%rsi)",
                "xorq %r10, %r10",
                /* (r8,r9,r10) += a1 * b3 */
                "movq %rbx, %rax",
                "mulq %r14",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                "adcq $0, %r10",
                /* (r8,r9,r10) += a2 * b2 */
                "movq %rcx, %rax",
                "mulq %r13",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                "adcq $0, %r10",
                /* (r8,r9,r10) += a3 * b1 */
                "movq %r15, %rax",
                "mulq %r12",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                "adcq $0, %r10",
                /* Extract l4 */
                "movq %r8, 32(%rsi)",
                "xorq %r8, %r8",
                /* (r9,r10,r8) += a2 * b3 */
                "movq %rcx, %rax",
                "mulq %r14",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* (r9,r10,r8) += a3 * b2 */
                "movq %r15, %rax",
                "mulq %r13",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* Extract l5 */
                "movq %r9, 40(%rsi)",
                /* (r10,r8) += a3 * b3 */
                "movq %r15, %rax",
                "mulq %r14",
                "addq %rax, %r10",
                "adcq %rdx, %r8",
                /* Extract l6 */
                "movq %r10, 48(%rsi)",
                /* Extract l7 */
                "movq %r8, 56(%rsi)",
                in("rsi") l,
                in("rdi") ad,
                pb = inout("rdx") pb => _,
                lateout("rax") _,
                lateout("rbx") _,
                lateout("rcx") _,
                lateout("r8") _,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r14") _,
                lateout("r15") _,
                options(att_syntax, nostack)
            );
        }

        #[cfg(not(all(feature = "asm", target_arch = "x86_64")))]
        {
            /* 160 bit accumulator. */
            let mut c0: u64 = 0;
            let mut c1: u64 = 0;
            let mut c2: u32 = 0;

            /* l[0..7] = a[0..3] * b[0..3]. */
            muladd_fast!((*a).d[0], (*b).d[0]);
            extract_fast!(*l.add(0));
            muladd!((*a).d[0], (*b).d[1]);
            muladd!((*a).d[1], (*b).d[0]);
            extract!(*l.add(1));
            muladd!((*a).d[0], (*b).d[2]);
            muladd!((*a).d[1], (*b).d[1]);
            muladd!((*a).d[2], (*b).d[0]);
            extract!(*l.add(2));
            muladd!((*a).d[0], (*b).d[3]);
            muladd!((*a).d[1], (*b).d[2]);
            muladd!((*a).d[2], (*b).d[1]);
            muladd!((*a).d[3], (*b).d[0]);
            extract!(*l.add(3));
            muladd!((*a).d[1], (*b).d[3]);
            muladd!((*a).d[2], (*b).d[2]);
            muladd!((*a).d[3], (*b).d[1]);
            extract!(*l.add(4));
            muladd!((*a).d[2], (*b).d[3]);
            muladd!((*a).d[3], (*b).d[2]);
            extract!(*l.add(5));
            muladd_fast!((*a).d[3], (*b).d[3]);
            extract_fast!(*l.add(6));
            VERIFY_CHECK!(c1 == 0);
            *l.add(7) = c0;
        }
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_mul_512(l: *mut u32, a: *const Scalar, b: *const Scalar) {
    unsafe {
        /* 96 bit accumulator. */
        let mut c0: u32 = 0;
        let mut c1: u32 = 0;
        let mut c2: u32 = 0;

        define_widemul_accum_macros!(c0, c1, c2);

        /* l[0..15] = a[0..7] * b[0..7]. */
        muladd_fast!((*a).d[0], (*b).d[0]);
        extract_fast!(*l.add(0));
        muladd!((*a).d[0], (*b).d[1]);
        muladd!((*a).d[1], (*b).d[0]);
        extract!(*l.add(1));
        muladd!((*a).d[0], (*b).d[2]);
        muladd!((*a).d[1], (*b).d[1]);
        muladd!((*a).d[2], (*b).d[0]);
        extract!(*l.add(2));
        muladd!((*a).d[0], (*b).d[3]);
        muladd!((*a).d[1], (*b).d[2]);
        muladd!((*a).d[2], (*b).d[1]);
        muladd!((*a).d[3], (*b).d[0]);
        extract!(*l.add(3));
        muladd!((*a).d[0], (*b).d[4]);
        muladd!((*a).d[1], (*b).d[3]);
        muladd!((*a).d[2], (*b).d[2]);
        muladd!((*a).d[3], (*b).d[1]);
        muladd!((*a).d[4], (*b).d[0]);
        extract!(*l.add(4));
        muladd!((*a).d[0], (*b).d[5]);
        muladd!((*a).d[1], (*b).d[4]);
        muladd!((*a).d[2], (*b).d[3]);
        muladd!((*a).d[3], (*b).d[2]);
        muladd!((*a).d[4], (*b).d[1]);
        muladd!((*a).d[5], (*b).d[0]);
        extract!(*l.add(5));
        muladd!((*a).d[0], (*b).d[6]);
        muladd!((*a).d[1], (*b).d[5]);
        muladd!((*a).d[2], (*b).d[4]);
        muladd!((*a).d[3], (*b).d[3]);
        muladd!((*a).d[4], (*b).d[2]);
        muladd!((*a).d[5], (*b).d[1]);
        muladd!((*a).d[6], (*b).d[0]);
        extract!(*l.add(6));
        muladd!((*a).d[0], (*b).d[7]);
        muladd!((*a).d[1], (*b).d[6]);
        muladd!((*a).d[2], (*b).d[5]);
        muladd!((*a).d[3], (*b).d[4]);
        muladd!((*a).d[4], (*b).d[3]);
        muladd!((*a).d[5], (*b).d[2]);
        muladd!((*a).d[6], (*b).d[1]);
        muladd!((*a).d[7], (*b).d[0]);
        extract!(*l.add(7));
        muladd!((*a).d[1], (*b).d[7]);
        muladd!((*a).d[2], (*b).d[6]);
        muladd!((*a).d[3], (*b).d[5]);
        muladd!((*a).d[4], (*b).d[4]);
        muladd!((*a).d[5], (*b).d[3]);
        muladd!((*a).d[6], (*b).d[2]);
        muladd!((*a).d[7], (*b).d[1]);
        extract!(*l.add(8));
        muladd!((*a).d[2], (*b).d[7]);
        muladd!((*a).d[3], (*b).d[6]);
        muladd!((*a).d[4], (*b).d[5]);
        muladd!((*a).d[5], (*b).d[4]);
        muladd!((*a).d[6], (*b).d[3]);
        muladd!((*a).d[7], (*b).d[2]);
        extract!(*l.add(9));
        muladd!((*a).d[3], (*b).d[7]);
        muladd!((*a).d[4], (*b).d[6]);
        muladd!((*a).d[5], (*b).d[5]);
        muladd!((*a).d[6], (*b).d[4]);
        muladd!((*a).d[7], (*b).d[3]);
        extract!(*l.add(10));
        muladd!((*a).d[4], (*b).d[7]);
        muladd!((*a).d[5], (*b).d[6]);
        muladd!((*a).d[6], (*b).d[5]);
        muladd!((*a).d[7], (*b).d[4]);
        extract!(*l.add(11));
        muladd!((*a).d[5], (*b).d[7]);
        muladd!((*a).d[6], (*b).d[6]);
        muladd!((*a).d[7], (*b).d[5]);
        extract!(*l.add(12));
        muladd!((*a).d[6], (*b).d[7]);
        muladd!((*a).d[7], (*b).d[6]);
        extract!(*l.add(13));
        muladd_fast!((*a).d[7], (*b).d[7]);
        extract_fast!(*l.add(14));
        verify_check!(c1 == 0);
        *l.add(15) = c0;
    }
}

#[cfg(test)]
mod scalar_mul_512_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    #[cfg(feature = "widemul-int128")]
    fn scalar_mul_512_matches_reference_512_bit_product_u64_limbs() {
        info!("validating scalar_mul_512 (widemul-int128) against reference 512-bit product");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);
            for (j, b_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
                let b = scalar_from_be_bytes(b_be);

                let mut l = [0u64; 8];
                unsafe {
                    scalar_mul_512(l.as_mut_ptr(), &a as *const Scalar, &b as *const Scalar);
                }

                // Convert l (little-endian u64 limbs) to big-endian 64 bytes.
                let mut got = [0u8; 64];
                for limb in 0..8 {
                    let be = l[7 - limb].to_be_bytes();
                    got[limb * 8..limb * 8 + 8].copy_from_slice(&be);
                }

                let expected = be_mul_256(a_be, b_be);
                trace!(i, j, ?expected, ?got, "mul_512 case");
                assert_eq!(got, expected);
            }
        }

        debug!("scalar_mul_512 reference match completed");
    }

    #[traced_test]
    #[cfg(feature = "widemul-int64")]
    fn scalar_mul_512_matches_reference_512_bit_product_u32_limbs() {
        info!("validating scalar_mul_512 (widemul-int64) against reference 512-bit product");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);
            for (j, b_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
                let b = scalar_from_be_bytes(b_be);

                let mut l = [0u32; 16];
                unsafe {
                    scalar_mul_512(l.as_mut_ptr(), &a as *const Scalar, &b as *const Scalar);
                }

                // Convert l (little-endian u32 limbs) to big-endian 64 bytes.
                let mut got = [0u8; 64];
                for limb in 0..16 {
                    let be = l[15 - limb].to_be_bytes();
                    got[limb * 4..limb * 4 + 4].copy_from_slice(&be);
                }

                let expected = be_mul_256(a_be, b_be);
                trace!(i, j, ?expected, ?got, "mul_512 case");
                assert_eq!(got, expected);
            }
        }

        debug!("scalar_mul_512 reference match completed");
    }
}
