// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_reduce_512.rs ]
crate::ix!();

#[cfg(feature = "widemul-int128")]
pub fn scalar_reduce_512(r: *mut Scalar, l: *const u64) {
    unsafe {

        #[cfg(all(feature = "asm", target_arch = "x86_64"))]
        {
            /* Reduce 512 bits into 385. */
            let mut m: [u64; 7] = [0u64; 7];

            core::arch::asm!(
                /* Preload. */
                "movq 32(%rsi), %r11",
                "movq 40(%rsi), %r12",
                "movq 48(%rsi), %r13",
                "movq 56(%rsi), %r14",
                /* Initialize r8,r9,r10 */
                "movq 0(%rsi), %r8",
                "xorq %r9, %r9",
                "xorq %r10, %r10",
                /* (r8,r9) += n0 * c0 */
                "movq ${nc0}, %rax",
                "mulq %r11",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                /* extract m0 */
                "movq %r8, 0(%rdi)",
                "xorq %r8, %r8",
                /* (r9,r10) += l1 */
                "addq 8(%rsi), %r9",
                "adcq $0, %r10",
                /* (r9,r10,r8) += n1 * c0 */
                "movq ${nc0}, %rax",
                "mulq %r12",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* (r9,r10,r8) += n0 * c1 */
                "movq ${nc1}, %rax",
                "mulq %r11",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* extract m1 */
                "movq %r9, 8(%rdi)",
                "xorq %r9, %r9",
                /* (r10,r8,r9) += l2 */
                "addq 16(%rsi), %r10",
                "adcq $0, %r8",
                "adcq $0, %r9",
                /* (r10,r8,r9) += n2 * c0 */
                "movq ${nc0}, %rax",
                "mulq %r13",
                "addq %rax, %r10",
                "adcq %rdx, %r8",
                "adcq $0, %r9",
                /* (r10,r8,r9) += n1 * c1 */
                "movq ${nc1}, %rax",
                "mulq %r12",
                "addq %rax, %r10",
                "adcq %rdx, %r8",
                "adcq $0, %r9",
                /* (r10,r8,r9) += n0 */
                "addq %r11, %r10",
                "adcq $0, %r8",
                "adcq $0, %r9",
                /* extract m2 */
                "movq %r10, 16(%rdi)",
                "xorq %r10, %r10",
                /* (r8,r9,r10) += l3 */
                "addq 24(%rsi), %r8",
                "adcq $0, %r9",
                "adcq $0, %r10",
                /* (r8,r9,r10) += n3 * c0 */
                "movq ${nc0}, %rax",
                "mulq %r14",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                "adcq $0, %r10",
                /* (r8,r9,r10) += n2 * c1 */
                "movq ${nc1}, %rax",
                "mulq %r13",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                "adcq $0, %r10",
                /* (r8,r9,r10) += n1 */
                "addq %r12, %r8",
                "adcq $0, %r9",
                "adcq $0, %r10",
                /* extract m3 */
                "movq %r8, 24(%rdi)",
                "xorq %r8, %r8",
                /* (r9,r10,r8) += n3 * c1 */
                "movq ${nc1}, %rax",
                "mulq %r14",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* (r9,r10,r8) += n2 */
                "addq %r13, %r9",
                "adcq $0, %r10",
                "adcq $0, %r8",
                /* extract m4 */
                "movq %r9, 32(%rdi)",
                /* (r10,r8) += n3 */
                "addq %r14, %r10",
                "adcq $0, %r8",
                /* extract m5 */
                "movq %r10, 40(%rdi)",
                /* extract m6 */
                "movq %r8, 48(%rdi)",
                nc0 = const N_C_0,
                nc1 = const N_C_1,
                in("rsi") l,
                in("rdi") m.as_mut_ptr(),
                lateout("rax") _,
                lateout("rdx") _,
                lateout("r8") _,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r14") _,
                options(att_syntax, nostack)
            );

            /* Reduce 385 bits into 258. */
            let mut p: [u64; 5] = [0u64; 5];

            core::arch::asm!(
                /* Preload */
                "movq 32(%rsi), %r11",
                "movq 40(%rsi), %r12",
                "movq 48(%rsi), %r13",
                /* Initialize (r8,r9,r10) */
                "movq 0(%rsi), %r8",
                "xorq %r9, %r9",
                "xorq %r10, %r10",
                /* (r8,r9) += m4 * c0 */
                "movq ${nc0}, %rax",
                "mulq %r11",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                /* extract p0 */
                "movq %r8, 0(%rdi)",
                "xorq %r8, %r8",
                /* (r9,r10) += m1 */
                "addq 8(%rsi), %r9",
                "adcq $0, %r10",
                /* (r9,r10,r8) += m5 * c0 */
                "movq ${nc0}, %rax",
                "mulq %r12",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* (r9,r10,r8) += m4 * c1 */
                "movq ${nc1}, %rax",
                "mulq %r11",
                "addq %rax, %r9",
                "adcq %rdx, %r10",
                "adcq $0, %r8",
                /* extract p1 */
                "movq %r9, 8(%rdi)",
                "xorq %r9, %r9",
                /* (r10,r8,r9) += m2 */
                "addq 16(%rsi), %r10",
                "adcq $0, %r8",
                "adcq $0, %r9",
                /* (r10,r8,r9) += m6 * c0 */
                "movq ${nc0}, %rax",
                "mulq %r13",
                "addq %rax, %r10",
                "adcq %rdx, %r8",
                "adcq $0, %r9",
                /* (r10,r8,r9) += m5 * c1 */
                "movq ${nc1}, %rax",
                "mulq %r12",
                "addq %rax, %r10",
                "adcq %rdx, %r8",
                "adcq $0, %r9",
                /* (r10,r8,r9) += m4 */
                "addq %r11, %r10",
                "adcq $0, %r8",
                "adcq $0, %r9",
                /* extract p2 */
                "movq %r10, 16(%rdi)",
                /* (r8,r9) += m3 */
                "addq 24(%rsi), %r8",
                "adcq $0, %r9",
                /* (r8,r9) += m6 * c1 */
                "movq ${nc1}, %rax",
                "mulq %r13",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                /* (r8,r9) += m5 */
                "addq %r12, %r8",
                "adcq $0, %r9",
                /* extract p3 */
                "movq %r8, 24(%rdi)",
                /* (r9) += m6 */
                "addq %r13, %r9",
                /* extract p4 */
                "movq %r9, 32(%rdi)",
                nc0 = const N_C_0,
                nc1 = const N_C_1,
                in("rsi") m.as_ptr(),
                in("rdi") p.as_mut_ptr(),
                lateout("rax") _,
                lateout("rdx") _,
                lateout("r8") _,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                lateout("r12") _,
                lateout("r13") _,
                options(att_syntax, nostack)
            );

            /* Reduce 258 bits into 256. */
            let mut c: u64;

            core::arch::asm!(
                /* Preload */
                "movq 32(%rsi), %r10",
                /* (rax,rdx) = p4 * c0 */
                "movq ${nc0}, %rax",
                "mulq %r10",
                /* (rax,rdx) += p0 */
                "addq 0(%rsi), %rax",
                "adcq $0, %rdx",
                /* extract r0 */
                "movq %rax, 0(%rdi)",
                /* Move to (r8,r9) */
                "movq %rdx, %r8",
                "xorq %r9, %r9",
                /* (r8,r9) += p1 */
                "addq 8(%rsi), %r8",
                "adcq $0, %r9",
                /* (r8,r9) += p4 * c1 */
                "movq ${nc1}, %rax",
                "mulq %r10",
                "addq %rax, %r8",
                "adcq %rdx, %r9",
                /* Extract r1 */
                "movq %r8, 8(%rdi)",
                "xorq %r8, %r8",
                /* (r9,r8) += p4 */
                "addq %r10, %r9",
                "adcq $0, %r8",
                /* (r9,r8) += p2 */
                "addq 16(%rsi), %r9",
                "adcq $0, %r8",
                /* Extract r2 */
                "movq %r9, 16(%rdi)",
                "xorq %r9, %r9",
                /* (r8,r9) += p3 */
                "addq 24(%rsi), %r8",
                "adcq $0, %r9",
                /* Extract r3 */
                "movq %r8, 24(%rdi)",
                /* Extract c */
                "movq %r9, {c_out}",
                c_out = out(reg) c,
                nc0 = const N_C_0,
                nc1 = const N_C_1,
                in("rsi") p.as_ptr(),
                in("rdi") r,
                lateout("rax") _,
                lateout("rdx") _,
                lateout("r8") _,
                lateout("r9") _,
                lateout("r10") _,
                options(att_syntax, nostack)
            );

            /* Final reduction of r. */
            scalar_reduce(
                r,
                c.wrapping_add(scalar_check_overflow(r as *const Scalar) as u64),
            );
        }

        #[cfg(not(all(feature = "asm", target_arch = "x86_64")))]
        {
            let mut c0: u64;
            let mut c1: u64;
            let mut c2: u64;

            let n0 = *l.add(4);
            let n1 = *l.add(5);
            let n2 = *l.add(6);
            let n3 = *l.add(7);

            let mut m0: u64 = 0;
            let mut m1: u64 = 0;
            let mut m2: u64 = 0;
            let mut m3: u64 = 0;
            let mut m4: u64 = 0;
            let mut m5: u64 = 0;
            let mut m6: u32 = 0;

            let mut p0: u64 = 0;
            let mut p1: u64 = 0;
            let mut p2: u64 = 0;
            let mut p3: u64 = 0;
            let mut p4: u32 = 0;

            /* Reduce 512 bits into 385. */
            /* m[0..6] = l[0..3] + n[0..3] * N_C. */
            c0 = *l.add(0);
            c1 = 0;
            c2 = 0;
            muladd_fast!(n0, N_C_0);
            extract_fast!(m0);
            sumadd_fast!(*l.add(1));
            muladd!(n1, N_C_0);
            muladd!(n0, N_C_1);
            extract!(m1);
            sumadd!(*l.add(2));
            muladd!(n2, N_C_0);
            muladd!(n1, N_C_1);
            sumadd!(n0);
            extract!(m2);
            sumadd!(*l.add(3));
            muladd!(n3, N_C_0);
            muladd!(n2, N_C_1);
            sumadd!(n1);
            extract!(m3);
            muladd!(n3, N_C_1);
            sumadd!(n2);
            extract!(m4);
            sumadd_fast!(n3);
            extract_fast!(m5);
            VERIFY_CHECK!(c0 <= 1);
            m6 = c0 as u32;

            /* Reduce 385 bits into 258. */
            /* p[0..4] = m[0..3] + m[4..6] * N_C. */
            c0 = m0;
            c1 = 0;
            c2 = 0;
            muladd_fast!(m4, N_C_0);
            extract_fast!(p0);
            sumadd_fast!(m1);
            muladd!(m5, N_C_0);
            muladd!(m4, N_C_1);
            extract!(p1);
            sumadd!(m2);
            muladd!(m6 as u64, N_C_0);
            muladd!(m5, N_C_1);
            sumadd!(m4);
            extract!(p2);
            sumadd_fast!(m3);
            muladd_fast!(m6 as u64, N_C_1);
            sumadd_fast!(m5);
            extract_fast!(p3);
            p4 = (c0.wrapping_add(m6 as u64)) as u32;
            VERIFY_CHECK!(p4 <= 2);

            /* Reduce 258 bits into 256. */
            /* r[0..3] = p[0..3] + p[4] * N_C. */
            let mut c: u128 = (p0 as u128).wrapping_add((N_C_0 as u128).wrapping_mul(p4 as u128));
            (*r).d[0] = (c & 0xFFFFFFFFFFFFFFFFu128) as u64;
            c >>= 64;
            c = c.wrapping_add(p1 as u128)
                .wrapping_add((N_C_1 as u128).wrapping_mul(p4 as u128));
            (*r).d[1] = (c & 0xFFFFFFFFFFFFFFFFu128) as u64;
            c >>= 64;
            c = c.wrapping_add(p2 as u128).wrapping_add(p4 as u128);
            (*r).d[2] = (c & 0xFFFFFFFFFFFFFFFFu128) as u64;
            c >>= 64;
            c = c.wrapping_add(p3 as u128);
            (*r).d[3] = (c & 0xFFFFFFFFFFFFFFFFu128) as u64;
            c >>= 64;

            /* Final reduction of r. */
            scalar_reduce(
                r,
                (c as u64).wrapping_add(scalar_check_overflow(r as *const Scalar) as u64),
            );
        }
    }
}

#[cfg(feature = "widemul-int64")]
pub fn scalar_reduce_512(r: *mut Scalar, l: *const u32) {
    unsafe {
        let mut c: u64;

        let n0: u32 = *l.add(8);
        let n1: u32 = *l.add(9);
        let n2: u32 = *l.add(10);
        let n3: u32 = *l.add(11);
        let n4: u32 = *l.add(12);
        let n5: u32 = *l.add(13);
        let n6: u32 = *l.add(14);
        let n7: u32 = *l.add(15);

        let mut m0: u32 = 0;
        let mut m1: u32 = 0;
        let mut m2: u32 = 0;
        let mut m3: u32 = 0;
        let mut m4: u32 = 0;
        let mut m5: u32 = 0;
        let mut m6: u32 = 0;
        let mut m7: u32 = 0;
        let mut m8: u32 = 0;
        let mut m9: u32 = 0;
        let mut m10: u32 = 0;
        let mut m11: u32 = 0;
        let mut m12: u32 = 0;

        let mut p0: u32 = 0;
        let mut p1: u32 = 0;
        let mut p2: u32 = 0;
        let mut p3: u32 = 0;
        let mut p4: u32 = 0;
        let mut p5: u32 = 0;
        let mut p6: u32 = 0;
        let mut p7: u32 = 0;
        let mut p8: u32 = 0;

        /* 96 bit accumulator. */
        let mut c0: u32 = 0;
        let mut c1: u32 = 0;
        let mut c2: u32 = 0;

        define_widemul_accum_macros!(c0, c1, c2);

        /* Reduce 512 bits into 385. */
        /* m[0..12] = l[0..7] + n[0..7] * N_C. */
        c0 = *l.add(0);
        c1 = 0;
        c2 = 0;
        muladd_fast!(n0, N_C_0);
        extract_fast!(m0);
        sumadd_fast!(*l.add(1));
        muladd!(n1, N_C_0);
        muladd!(n0, N_C_1);
        extract!(m1);
        sumadd!(*l.add(2));
        muladd!(n2, N_C_0);
        muladd!(n1, N_C_1);
        muladd!(n0, N_C_2);
        extract!(m2);
        sumadd!(*l.add(3));
        muladd!(n3, N_C_0);
        muladd!(n2, N_C_1);
        muladd!(n1, N_C_2);
        muladd!(n0, N_C_3);
        extract!(m3);
        sumadd!(*l.add(4));
        muladd!(n4, N_C_0);
        muladd!(n3, N_C_1);
        muladd!(n2, N_C_2);
        muladd!(n1, N_C_3);
        sumadd!(n0);
        extract!(m4);
        sumadd!(*l.add(5));
        muladd!(n5, N_C_0);
        muladd!(n4, N_C_1);
        muladd!(n3, N_C_2);
        muladd!(n2, N_C_3);
        sumadd!(n1);
        extract!(m5);
        sumadd!(*l.add(6));
        muladd!(n6, N_C_0);
        muladd!(n5, N_C_1);
        muladd!(n4, N_C_2);
        muladd!(n3, N_C_3);
        sumadd!(n2);
        extract!(m6);
        sumadd!(*l.add(7));
        muladd!(n7, N_C_0);
        muladd!(n6, N_C_1);
        muladd!(n5, N_C_2);
        muladd!(n4, N_C_3);
        sumadd!(n3);
        extract!(m7);
        muladd!(n7, N_C_1);
        muladd!(n6, N_C_2);
        muladd!(n5, N_C_3);
        sumadd!(n4);
        extract!(m8);
        muladd!(n7, N_C_2);
        muladd!(n6, N_C_3);
        sumadd!(n5);
        extract!(m9);
        muladd!(n7, N_C_3);
        sumadd!(n6);
        extract!(m10);
        sumadd_fast!(n7);
        extract_fast!(m11);
        verify_check!(c0 <= 1);
        m12 = c0;

        /* Reduce 385 bits into 258. */
        /* p[0..8] = m[0..7] + m[8..12] * N_C. */
        c0 = m0;
        c1 = 0;
        c2 = 0;
        muladd_fast!(m8, N_C_0);
        extract_fast!(p0);
        sumadd_fast!(m1);
        muladd!(m9, N_C_0);
        muladd!(m8, N_C_1);
        extract!(p1);
        sumadd!(m2);
        muladd!(m10, N_C_0);
        muladd!(m9, N_C_1);
        muladd!(m8, N_C_2);
        extract!(p2);
        sumadd!(m3);
        muladd!(m11, N_C_0);
        muladd!(m10, N_C_1);
        muladd!(m9, N_C_2);
        muladd!(m8, N_C_3);
        extract!(p3);
        sumadd!(m4);
        muladd!(m12, N_C_0);
        muladd!(m11, N_C_1);
        muladd!(m10, N_C_2);
        muladd!(m9, N_C_3);
        sumadd!(m8);
        extract!(p4);
        sumadd!(m5);
        muladd!(m12, N_C_1);
        muladd!(m11, N_C_2);
        muladd!(m10, N_C_3);
        sumadd!(m9);
        extract!(p5);
        sumadd!(m6);
        muladd!(m12, N_C_2);
        muladd!(m11, N_C_3);
        sumadd!(m10);
        extract!(p6);
        sumadd_fast!(m7);
        muladd_fast!(m12, N_C_3);
        sumadd_fast!(m11);
        extract_fast!(p7);
        p8 = c0.wrapping_add(m12);
        verify_check!(p8 <= 2);

        /* Reduce 258 bits into 256. */
        /* r[0..7] = p[0..7] + p[8] * N_C. */
        c = (p0 as u64).wrapping_add((N_C_0 as u64).wrapping_mul(p8 as u64));
        (*r).d[0] = (c & 0xFFFF_FFFFu64) as u32;
        c >>= 32;
        c = c
            .wrapping_add(p1 as u64)
            .wrapping_add((N_C_1 as u64).wrapping_mul(p8 as u64));
        (*r).d[1] = (c & 0xFFFF_FFFFu64) as u32;
        c >>= 32;
        c = c
            .wrapping_add(p2 as u64)
            .wrapping_add((N_C_2 as u64).wrapping_mul(p8 as u64));
        (*r).d[2] = (c & 0xFFFF_FFFFu64) as u32;
        c >>= 32;
        c = c
            .wrapping_add(p3 as u64)
            .wrapping_add((N_C_3 as u64).wrapping_mul(p8 as u64));
        (*r).d[3] = (c & 0xFFFF_FFFFu64) as u32;
        c >>= 32;
        c = c.wrapping_add(p4 as u64).wrapping_add(p8 as u64);
        (*r).d[4] = (c & 0xFFFF_FFFFu64) as u32;
        c >>= 32;
        c = c.wrapping_add(p5 as u64);
        (*r).d[5] = (c & 0xFFFF_FFFFu64) as u32;
        c >>= 32;
        c = c.wrapping_add(p6 as u64);
        (*r).d[6] = (c & 0xFFFF_FFFFu64) as u32;
        c >>= 32;
        c = c.wrapping_add(p7 as u64);
        (*r).d[7] = (c & 0xFFFF_FFFFu64) as u32;
        c >>= 32;

        /* Final reduction of r. */
        scalar_reduce(
            r,
            (c as u32).wrapping_add(scalar_check_overflow(r as *const Scalar) as u32),
        );
    }

}

#[cfg(test)]
mod scalar_reduce_512_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    #[cfg(feature = "widemul-int128")]
    fn scalar_reduce_512_matches_reference_mod_n_for_u64_limb_input() {
        info!("validating scalar_reduce_512 (widemul-int128) against reference mod-n reduction");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);
            for (j, b_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
                let b = scalar_from_be_bytes(b_be);

                let mut l = [0u64; 8];
                unsafe {
                    scalar_mul_512(l.as_mut_ptr(), &a as *const Scalar, &b as *const Scalar);
                }

                let mut r = scalar_zero_value();
                unsafe {
                    scalar_reduce_512(&mut r as *mut Scalar, l.as_ptr());
                }
                let got = scalar_to_be_bytes(&r);

                let prod = be_mul_256(a_be, b_be);
                let expected = be_mod_512_by_order_n(&prod);

                trace!(i, j, ?got, ?expected, "reduce_512 case");
                assert_eq!(got, expected);
                assert!(scalar_is_normalized_bytes(&got));
            }
        }

        debug!("scalar_reduce_512 reference match completed");
    }

    #[traced_test]
    #[cfg(feature = "widemul-int64")]
    fn scalar_reduce_512_matches_reference_mod_n_for_u32_limb_input() {
        tracing::info!("validating scalar_reduce_512 (widemul-int64) against reference mod-n reduction");

        const MOD_N: [u64; 4] = [
            0xBFD25E8CD0364141u64,
            0xBAAEDCE6AF48A03Bu64,
            0xFFFFFFFFFFFFFFFEu64,
            0xFFFFFFFFFFFFFFFFu64,
        ];

        fn u64x4_le_ge(a: &[u64; 4], b: &[u64; 4]) -> bool {
            for i in (0..4).rev() {
                if a[i] > b[i] {
                    return true;
                }
                if a[i] < b[i] {
                    return false;
                }
            }
            true
        }

        fn u64x4_le_add(a: &[u64; 4], b: &[u64; 4]) -> ([u64; 4], u64) {
            let mut out = [0u64; 4];
            let mut carry: u128 = 0;
            for i in 0..4 {
                let sum = a[i] as u128 + b[i] as u128 + carry;
                out[i] = sum as u64;
                carry = sum >> 64;
            }
            (out, carry as u64)
        }

        fn u64x4_le_sub_wrapping(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
            let mut out = [0u64; 4];
            let mut borrow: u128 = 0;
            for i in 0..4 {
                let ai = a[i] as u128;
                let bi = b[i] as u128 + borrow;
                if ai >= bi {
                    out[i] = (ai - bi) as u64;
                    borrow = 0;
                } else {
                    out[i] = ((1u128 << 64) + ai - bi) as u64;
                    borrow = 1;
                }
            }
            out
        }

        fn u64x4_le_add_mod_n(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
            let (sum, carry) = u64x4_le_add(&a, &b);
            if carry != 0 || u64x4_le_ge(&sum, &MOD_N) {
                u64x4_le_sub_wrapping(&sum, &MOD_N)
            } else {
                sum
            }
        }

        fn u64x4_le_to_be_bytes(limbs: &[u64; 4]) -> [u8; 32] {
            let mut out = [0u8; 32];
            for i in 0..4 {
                let be = limbs[3 - i].to_be_bytes();
                out[i * 8..(i + 1) * 8].copy_from_slice(&be);
            }
            out
        }

        fn reference_reduce_512_mod_n_u32_limbs(l: &[u32; 16]) -> [u8; 32] {
            let mut rem = [0u64; 4];

            for idx in (0..16usize).rev() {
                for _ in 0..32 {
                    rem = u64x4_le_add_mod_n(rem, rem);
                }
                let add = [l[idx] as u64, 0u64, 0u64, 0u64];
                rem = u64x4_le_add_mod_n(rem, add);
            }

            u64x4_le_to_be_bytes(&rem)
        }

        unsafe fn scalar_from_be_bytes(bytes: &[u8; 32]) -> Scalar {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow: i32 = 0;
            scalar_set_b32(&mut s, bytes.as_ptr(), &mut overflow);
            if overflow != 0 {
                tracing::warn!(overflow, "scalar_set_b32 reported overflow for canonical-vector input");
            }
            s
        }

        unsafe fn scalar_to_be_bytes(s: &Scalar) -> [u8; 32] {
            let mut out = [0u8; 32];
            scalar_get_b32(out.as_mut_ptr(), s);
            out
        }

        let canonical_vectors: [[u8; 32]; 10] = {
            let v0 = [0u8; 32];

            let mut v1 = [0u8; 32];
            v1[31] = 1;

            let mut v2 = [0u8; 32];
            v2[31] = 2;

            let mut v3 = [0u8; 32];
            v3[31] = 3;

            let mut v4 = [0u8; 32];
            v4[28..32].copy_from_slice(&[0xFFu8; 4]);

            let mut v5 = [0u8; 32];
            v5[15] = 0x80;

            let mut v6 = [0u8; 32];
            v6[15] = 0x01;

            let mut v7 = [0u8; 32];
            v7[0] = 0x80;

            let v8: [u8; 32] = [
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0x5D, 0x57, 0x6E, 0x73, 0x57, 0xA4, 0x50, 0x1D, 0xDF, 0xE9, 0x2F, 0x46, 0x68, 0x1B,
                0x20, 0xA0,
            ];

            let v9: [u8; 32] = [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36,
                0x41, 0x40,
            ];

            [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9]
        };

        for (i, a_bytes) in canonical_vectors.iter().enumerate() {
            for (j, b_bytes) in canonical_vectors.iter().enumerate() {
                let (l, got) = unsafe {
                    let a = scalar_from_be_bytes(a_bytes);
                    let b = scalar_from_be_bytes(b_bytes);

                    let mut wide = [0u32; 16];
                    scalar_mul_512(wide.as_mut_ptr(), &a, &b);

                    let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
                    scalar_reduce_512(&mut r, wide.as_ptr());

                    (wide, scalar_to_be_bytes(&r))
                };

                let expected = reference_reduce_512_mod_n_u32_limbs(&l);

                tracing::trace!(i, j, l = ?l, got = ?got, expected = ?expected, "reduce_512 case");
                assert_eq!(got, expected);
            }
        }
    }

    #[traced_test]
    fn scalar_reduce_512_attack_surface_matches_reference_for_wide_edge_patterns_and_random_inputs() {
        tracing::info!(
            "adversarial reduction coverage: scalar_reduce_512 vs independent mod-n reference for edge patterns and random wide inputs"
        );

        const MOD_N: [u64; 4] = [
            0xBFD25E8CD0364141u64,
            0xBAAEDCE6AF48A03Bu64,
            0xFFFFFFFFFFFFFFFEu64,
            0xFFFFFFFFFFFFFFFFu64,
        ];

        const N_BYTES: [u8; 32] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36,
            0x41, 0x41,
        ];

        const N_MINUS_ONE_BYTES: [u8; 32] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36,
            0x41, 0x40,
        ];

        fn u64x4_le_ge(a: &[u64; 4], b: &[u64; 4]) -> bool {
            for i in (0..4).rev() {
                if a[i] > b[i] {
                    return true;
                }
                if a[i] < b[i] {
                    return false;
                }
            }
            true
        }

        fn u64x4_le_add(a: &[u64; 4], b: &[u64; 4]) -> ([u64; 4], u64) {
            let mut out = [0u64; 4];
            let mut carry: u128 = 0;
            for i in 0..4 {
                let sum = a[i] as u128 + b[i] as u128 + carry;
                out[i] = sum as u64;
                carry = sum >> 64;
            }
            (out, carry as u64)
        }

        fn u64x4_le_sub_wrapping(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
            let mut out = [0u64; 4];
            let mut borrow: u128 = 0;
            for i in 0..4 {
                let ai = a[i] as u128;
                let bi = b[i] as u128 + borrow;
                if ai >= bi {
                    out[i] = (ai - bi) as u64;
                    borrow = 0;
                } else {
                    out[i] = ((1u128 << 64) + ai - bi) as u64;
                    borrow = 1;
                }
            }
            out
        }

        fn u64x4_le_add_mod_n(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
            let (sum, carry) = u64x4_le_add(&a, &b);
            if carry != 0 || u64x4_le_ge(&sum, &MOD_N) {
                u64x4_le_sub_wrapping(&sum, &MOD_N)
            } else {
                sum
            }
        }

        fn u64x4_le_to_be_bytes(limbs: &[u64; 4]) -> [u8; 32] {
            let mut out = [0u8; 32];
            for i in 0..4 {
                let be = limbs[3 - i].to_be_bytes();
                out[i * 8..(i + 1) * 8].copy_from_slice(&be);
            }
            out
        }

        fn reference_reduce_512_mod_n_u32_limbs(l: &[u32; 16]) -> [u8; 32] {
            let mut rem = [0u64; 4];

            for idx in (0..16usize).rev() {
                for _ in 0..32 {
                    rem = u64x4_le_add_mod_n(rem, rem);
                }
                rem = u64x4_le_add_mod_n(rem, [l[idx] as u64, 0u64, 0u64, 0u64]);
            }

            u64x4_le_to_be_bytes(&rem)
        }

        fn bytes_be_to_u32_words_le(bytes: &[u8; 32]) -> [u32; 8] {
            let mut words = [0u32; 8];
            for i in 0..8 {
                let start = 32 - 4 * (i + 1);
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[start..start + 4]);
                words[i] = u32::from_be_bytes(buf);
            }
            words
        }

        fn prng_next_u64(state: &mut u64) -> u64 {
            let mut x = *state;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            *state = x;
            x.wrapping_mul(2685821657736338717u64)
        }

        fn prng_fill_u32x16(state: &mut u64) -> [u32; 16] {
            let mut out = [0u32; 16];
            for i in 0..8 {
                let v = prng_next_u64(state);
                out[i * 2] = v as u32;
                out[i * 2 + 1] = (v >> 32) as u32;
            }
            out
        }

        unsafe fn scalar_to_be_bytes(s: &Scalar) -> [u8; 32] {
            let mut out = [0u8; 32];
            scalar_get_b32(out.as_mut_ptr(), s);
            out
        }

        unsafe fn reduce_512_via_api(l: &[u32; 16]) -> [u8; 32] {
            let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            scalar_reduce_512(&mut r, l.as_ptr());
            scalar_to_be_bytes(&r)
        }

        unsafe fn assert_canonical_bytes(bytes: &[u8; 32]) {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow: i32 = 0;
            scalar_set_b32(&mut s, bytes.as_ptr(), &mut overflow);
            assert_eq!(overflow, 0);

            let mut roundtrip = [0u8; 32];
            scalar_get_b32(roundtrip.as_mut_ptr(), &s);
            assert_eq!(roundtrip, *bytes);
        }

        let mut n_plus_one_bytes = N_BYTES;
        n_plus_one_bytes[31] = n_plus_one_bytes[31].wrapping_add(1);

        let n_words = bytes_be_to_u32_words_le(&N_BYTES);
        let n_minus_one_words = bytes_be_to_u32_words_le(&N_MINUS_ONE_BYTES);
        let n_plus_one_words = bytes_be_to_u32_words_le(&n_plus_one_bytes);

        let mut patterns: Vec<(&'static str, [u32; 16])> = Vec::new();

        patterns.push(("all_zero", [0u32; 16]));
        patterns.push(("all_ones", [0xFFFF_FFFFu32; 16]));
        patterns.push(("all_aaaa", [0xAAAA_AAAAu32; 16]));
        patterns.push(("all_5555", [0x5555_5555u32; 16]));

        let mut alternating = [0u32; 16];
        for i in 0..16 {
            alternating[i] = if (i & 1) == 0 { 0xAAAA_AAAAu32 } else { 0x5555_5555u32 };
        }
        patterns.push(("alternating_aaaa_5555", alternating));

        let mut low_half_ones = [0u32; 16];
        for i in 0..8 {
            low_half_ones[i] = 0xFFFF_FFFFu32;
        }
        patterns.push(("low_half_ones", low_half_ones));

        let mut high_half_ones = [0u32; 16];
        for i in 8..16 {
            high_half_ones[i] = 0xFFFF_FFFFu32;
        }
        patterns.push(("high_half_ones", high_half_ones));

        let mut increasing = [0u32; 16];
        for i in 0..16 {
            increasing[i] = i as u32;
        }
        patterns.push(("increasing_words", increasing));

        let mut decreasing = [0u32; 16];
        for i in 0..16 {
            decreasing[i] = (15 - i) as u32;
        }
        patterns.push(("decreasing_words", decreasing));

        let mut single_lsb = [0u32; 16];
        single_lsb[0] = 1;
        patterns.push(("single_lsb", single_lsb));

        let mut single_msb = [0u32; 16];
        single_msb[15] = 1;
        patterns.push(("single_msb", single_msb));

        let mut n_in_low_half = [0u32; 16];
        for i in 0..8 {
            n_in_low_half[i] = n_words[i];
        }
        patterns.push(("n_in_low_half", n_in_low_half));

        let mut n_minus_one_in_low_half = [0u32; 16];
        for i in 0..8 {
            n_minus_one_in_low_half[i] = n_minus_one_words[i];
        }
        patterns.push(("n_minus_one_in_low_half", n_minus_one_in_low_half));

        let mut n_plus_one_in_low_half = [0u32; 16];
        for i in 0..8 {
            n_plus_one_in_low_half[i] = n_plus_one_words[i];
        }
        patterns.push(("n_plus_one_in_low_half", n_plus_one_in_low_half));

        let mut n_in_high_half = [0u32; 16];
        for i in 0..8 {
            n_in_high_half[i + 8] = n_words[i];
        }
        patterns.push(("n_in_high_half", n_in_high_half));

        let mut n_minus_one_in_high_half = [0u32; 16];
        for i in 0..8 {
            n_minus_one_in_high_half[i + 8] = n_minus_one_words[i];
        }
        patterns.push(("n_minus_one_in_high_half", n_minus_one_in_high_half));

        let mut n_plus_one_in_high_half = [0u32; 16];
        for i in 0..8 {
            n_plus_one_in_high_half[i + 8] = n_plus_one_words[i];
        }
        patterns.push(("n_plus_one_in_high_half", n_plus_one_in_high_half));

        tracing::info!(pattern_count = patterns.len(), "running wide pattern reduction cases");

        for (name, wide) in patterns.iter() {
            let expected = reference_reduce_512_mod_n_u32_limbs(wide);
            let got = unsafe { reduce_512_via_api(wide) };

            tracing::debug!(name = *name, "scalar_reduce_512 pattern case");
            tracing::trace!(name = *name, wide = ?wide, got = ?got, expected = ?expected, "reduce_512 pattern details");

            if got != expected {
                tracing::error!(name = *name, wide = ?wide, got = ?got, expected = ?expected, "scalar_reduce_512 mismatch on pattern");
            }
            assert_eq!(got, expected);
            unsafe { assert_canonical_bytes(&got) };
        }

        tracing::info!("running randomized wide reduction cases");

        const RANDOM_CASES: usize = 256;
        let mut rng_state: u64 = 0x9E37_79B9_7F4A_7C15u64;

        for iter in 0..RANDOM_CASES {
            if (iter & 63) == 0 {
                tracing::debug!(iter, "randomized scalar_reduce_512 sweep progress");
            }

            let wide = prng_fill_u32x16(&mut rng_state);
            let expected = reference_reduce_512_mod_n_u32_limbs(&wide);
            let got = unsafe { reduce_512_via_api(&wide) };

            tracing::trace!(
                iter,
                w0 = wide[0],
                w15 = wide[15],
                got0 = got[0],
                got31 = got[31],
                "reduce_512 randomized sample"
            );

            if got != expected {
                tracing::error!(
                    iter,
                    wide = ?wide,
                    got = ?got,
                    expected = ?expected,
                    "scalar_reduce_512 mismatch on random wide input"
                );
            }
            assert_eq!(got, expected);
            unsafe { assert_canonical_bytes(&got) };
        }

        tracing::debug!("scalar_reduce_512 adversarial coverage complete");
    }

    #[traced_test]
    fn scalar_mul_and_reduce_512_attack_surface_remain_consistent_for_randomized_pairs() {
        tracing::info!(
            "adversarial consistency coverage: scalar_mul must match scalar_mul_512 + scalar_reduce_512 for randomized canonical inputs"
        );

        fn prng_next_u64(state: &mut u64) -> u64 {
            let mut x = *state;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            *state = x;
            x.wrapping_mul(2685821657736338717u64)
        }

        fn prng_fill_b32(state: &mut u64) -> [u8; 32] {
            let mut out = [0u8; 32];
            for i in 0..4 {
                let v = prng_next_u64(state);
                out[i * 8..(i + 1) * 8].copy_from_slice(&v.to_be_bytes());
            }
            out
        }

        unsafe fn scalar_to_be_bytes(s: &Scalar) -> [u8; 32] {
            let mut out = [0u8; 32];
            scalar_get_b32(out.as_mut_ptr(), s);
            out
        }

        unsafe fn canonical_scalar_from_bytes(bytes: &[u8; 32]) -> (Scalar, [u8; 32]) {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow: i32 = 0;
            scalar_set_b32(&mut s, bytes.as_ptr(), &mut overflow);
            tracing::trace!(overflow, "canonicalize scalar_set_b32 overflow flag");

            let canonical = scalar_to_be_bytes(&s);

            let mut s2 = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow2: i32 = 0;
            scalar_set_b32(&mut s2, canonical.as_ptr(), &mut overflow2);
            assert_eq!(overflow2, 0);

            (s2, canonical)
        }

        unsafe fn assert_canonical_bytes(bytes: &[u8; 32]) {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow: i32 = 0;
            scalar_set_b32(&mut s, bytes.as_ptr(), &mut overflow);
            assert_eq!(overflow, 0);

            let mut roundtrip = [0u8; 32];
            scalar_get_b32(roundtrip.as_mut_ptr(), &s);
            assert_eq!(roundtrip, *bytes);
        }

        unsafe fn scalar_mul_to_bytes(a: &Scalar, b: &Scalar) -> [u8; 32] {
            let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            scalar_mul(&mut r, a, b);
            scalar_to_be_bytes(&r)
        }

        unsafe fn scalar_mul_512_then_reduce_to_bytes(a: &Scalar, b: &Scalar) -> [u8; 32] {
            let mut wide = [0u32; 16];
            scalar_mul_512(wide.as_mut_ptr(), a, b);

            let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            scalar_reduce_512(&mut r, wide.as_ptr());

            let out = scalar_to_be_bytes(&r);
            tracing::trace!(wide0 = wide[0], wide15 = wide[15], out0 = out[0], out31 = out[31], "mul_512+reduce sample");
            out
        }

        const ITERATIONS: usize = 256;
        let mut rng_state: u64 = 0x4F1B_9D25_3C8A_77E1u64;

        for iter in 0..ITERATIONS {
            if (iter & 63) == 0 {
                tracing::debug!(iter, "mul vs mul_512+reduce sweep progress");
            }

            let a_raw = prng_fill_b32(&mut rng_state);
            let b_raw = prng_fill_b32(&mut rng_state);

            let (a, a_bytes) = unsafe { canonical_scalar_from_bytes(&a_raw) };
            let (b, b_bytes) = unsafe { canonical_scalar_from_bytes(&b_raw) };

            let got_mul = unsafe { scalar_mul_to_bytes(&a, &b) };
            let got_reduce = unsafe { scalar_mul_512_then_reduce_to_bytes(&a, &b) };

            tracing::trace!(
                iter,
                a0 = a_bytes[0],
                a31 = a_bytes[31],
                b0 = b_bytes[0],
                b31 = b_bytes[31],
                mul0 = got_mul[0],
                mul31 = got_mul[31],
                red0 = got_reduce[0],
                red31 = got_reduce[31],
                "consistency sample"
            );

            if got_mul != got_reduce {
                tracing::error!(
                    iter,
                    a = ?a_bytes,
                    b = ?b_bytes,
                    mul = ?got_mul,
                    reduced = ?got_reduce,
                    "scalar_mul != scalar_mul_512+scalar_reduce_512"
                );
            }

            assert_eq!(got_mul, got_reduce);

            unsafe { assert_canonical_bytes(&got_mul) };
            unsafe { assert_canonical_bytes(&got_reduce) };
        }

        tracing::debug!("mul vs mul_512+reduce adversarial consistency coverage complete");
    }
}
