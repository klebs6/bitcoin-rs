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
pub fn scalar_reduce_512(
        r: *mut Scalar,
        l: *const u32)  {
    
    todo!();
        /*
            uint64_t c;
        uint32_t n0 = l[8], n1 = l[9], n2 = l[10], n3 = l[11], n4 = l[12], n5 = l[13], n6 = l[14], n7 = l[15];
        uint32_t m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12;
        uint32_t p0, p1, p2, p3, p4, p5, p6, p7, p8;

        /* 96 bit accumulator. */
        uint32_t c0, c1, c2;

        /* Reduce 512 bits into 385. */
        /* m[0..12] = l[0..7] + n[0..7] * N_C. */
        c0 = l[0]; c1 = 0; c2 = 0;
        muladd_fast(n0, N_C_0);
        extract_fast(m0);
        sumadd_fast(l[1]);
        muladd(n1, N_C_0);
        muladd(n0, N_C_1);
        extract(m1);
        sumadd(l[2]);
        muladd(n2, N_C_0);
        muladd(n1, N_C_1);
        muladd(n0, N_C_2);
        extract(m2);
        sumadd(l[3]);
        muladd(n3, N_C_0);
        muladd(n2, N_C_1);
        muladd(n1, N_C_2);
        muladd(n0, N_C_3);
        extract(m3);
        sumadd(l[4]);
        muladd(n4, N_C_0);
        muladd(n3, N_C_1);
        muladd(n2, N_C_2);
        muladd(n1, N_C_3);
        sumadd(n0);
        extract(m4);
        sumadd(l[5]);
        muladd(n5, N_C_0);
        muladd(n4, N_C_1);
        muladd(n3, N_C_2);
        muladd(n2, N_C_3);
        sumadd(n1);
        extract(m5);
        sumadd(l[6]);
        muladd(n6, N_C_0);
        muladd(n5, N_C_1);
        muladd(n4, N_C_2);
        muladd(n3, N_C_3);
        sumadd(n2);
        extract(m6);
        sumadd(l[7]);
        muladd(n7, N_C_0);
        muladd(n6, N_C_1);
        muladd(n5, N_C_2);
        muladd(n4, N_C_3);
        sumadd(n3);
        extract(m7);
        muladd(n7, N_C_1);
        muladd(n6, N_C_2);
        muladd(n5, N_C_3);
        sumadd(n4);
        extract(m8);
        muladd(n7, N_C_2);
        muladd(n6, N_C_3);
        sumadd(n5);
        extract(m9);
        muladd(n7, N_C_3);
        sumadd(n6);
        extract(m10);
        sumadd_fast(n7);
        extract_fast(m11);
        VERIFY_CHECK(c0 <= 1);
        m12 = c0;

        /* Reduce 385 bits into 258. */
        /* p[0..8] = m[0..7] + m[8..12] * N_C. */
        c0 = m0; c1 = 0; c2 = 0;
        muladd_fast(m8, N_C_0);
        extract_fast(p0);
        sumadd_fast(m1);
        muladd(m9, N_C_0);
        muladd(m8, N_C_1);
        extract(p1);
        sumadd(m2);
        muladd(m10, N_C_0);
        muladd(m9, N_C_1);
        muladd(m8, N_C_2);
        extract(p2);
        sumadd(m3);
        muladd(m11, N_C_0);
        muladd(m10, N_C_1);
        muladd(m9, N_C_2);
        muladd(m8, N_C_3);
        extract(p3);
        sumadd(m4);
        muladd(m12, N_C_0);
        muladd(m11, N_C_1);
        muladd(m10, N_C_2);
        muladd(m9, N_C_3);
        sumadd(m8);
        extract(p4);
        sumadd(m5);
        muladd(m12, N_C_1);
        muladd(m11, N_C_2);
        muladd(m10, N_C_3);
        sumadd(m9);
        extract(p5);
        sumadd(m6);
        muladd(m12, N_C_2);
        muladd(m11, N_C_3);
        sumadd(m10);
        extract(p6);
        sumadd_fast(m7);
        muladd_fast(m12, N_C_3);
        sumadd_fast(m11);
        extract_fast(p7);
        p8 = c0 + m12;
        VERIFY_CHECK(p8 <= 2);

        /* Reduce 258 bits into 256. */
        /* r[0..7] = p[0..7] + p[8] * N_C. */
        c = p0 + (uint64_t)N_C_0 * p8;
        r->d[0] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p1 + (uint64_t)N_C_1 * p8;
        r->d[1] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p2 + (uint64_t)N_C_2 * p8;
        r->d[2] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p3 + (uint64_t)N_C_3 * p8;
        r->d[3] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p4 + (uint64_t)p8;
        r->d[4] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p5;
        r->d[5] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p6;
        r->d[6] = c & 0xFFFFFFFFUL; c >>= 32;
        c += p7;
        r->d[7] = c & 0xFFFFFFFFUL; c >>= 32;

        /* Final reduction of r. */
        scalar_reduce(r, c + scalar_check_overflow(r));
        */
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
        info!("validating scalar_reduce_512 (widemul-int64) against reference mod-n reduction");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);
            for (j, b_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
                let b = scalar_from_be_bytes(b_be);

                let mut l = [0u32; 16];
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
}
