// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_reduce_512.rs ]
crate::ix!();

#[cfg(WIDEMUL_INT128)]
pub fn scalar_reduce_512(
        r: *mut Scalar,
        l: *const u64)  {
    
    todo!();
        /*
            #ifdef USE_ASM_X86_64
        /* Reduce 512 bits into 385. */
        uint64_t m0, m1, m2, m3, m4, m5, m6;
        uint64_t p0, p1, p2, p3, p4;
        uint64_t c;

        __asm__ __volatile__(
        /* Preload. */
        "movq 32(%%rsi), %%r11\n"
        "movq 40(%%rsi), %%r12\n"
        "movq 48(%%rsi), %%r13\n"
        "movq 56(%%rsi), %%r14\n"
        /* Initialize r8,r9,r10 */
        "movq 0(%%rsi), %%r8\n"
        "xorq %%r9, %%r9\n"
        "xorq %%r10, %%r10\n"
        /* (r8,r9) += n0 * c0 */
        "movq %8, %%rax\n"
        "mulq %%r11\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        /* extract m0 */
        "movq %%r8, %q0\n"
        "xorq %%r8, %%r8\n"
        /* (r9,r10) += l1 */
        "addq 8(%%rsi), %%r9\n"
        "adcq $0, %%r10\n"
        /* (r9,r10,r8) += n1 * c0 */
        "movq %8, %%rax\n"
        "mulq %%r12\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* (r9,r10,r8) += n0 * c1 */
        "movq %9, %%rax\n"
        "mulq %%r11\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* extract m1 */
        "movq %%r9, %q1\n"
        "xorq %%r9, %%r9\n"
        /* (r10,r8,r9) += l2 */
        "addq 16(%%rsi), %%r10\n"
        "adcq $0, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r10,r8,r9) += n2 * c0 */
        "movq %8, %%rax\n"
        "mulq %%r13\n"
        "addq %%rax, %%r10\n"
        "adcq %%rdx, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r10,r8,r9) += n1 * c1 */
        "movq %9, %%rax\n"
        "mulq %%r12\n"
        "addq %%rax, %%r10\n"
        "adcq %%rdx, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r10,r8,r9) += n0 */
        "addq %%r11, %%r10\n"
        "adcq $0, %%r8\n"
        "adcq $0, %%r9\n"
        /* extract m2 */
        "movq %%r10, %q2\n"
        "xorq %%r10, %%r10\n"
        /* (r8,r9,r10) += l3 */
        "addq 24(%%rsi), %%r8\n"
        "adcq $0, %%r9\n"
        "adcq $0, %%r10\n"
        /* (r8,r9,r10) += n3 * c0 */
        "movq %8, %%rax\n"
        "mulq %%r14\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        "adcq $0, %%r10\n"
        /* (r8,r9,r10) += n2 * c1 */
        "movq %9, %%rax\n"
        "mulq %%r13\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        "adcq $0, %%r10\n"
        /* (r8,r9,r10) += n1 */
        "addq %%r12, %%r8\n"
        "adcq $0, %%r9\n"
        "adcq $0, %%r10\n"
        /* extract m3 */
        "movq %%r8, %q3\n"
        "xorq %%r8, %%r8\n"
        /* (r9,r10,r8) += n3 * c1 */
        "movq %9, %%rax\n"
        "mulq %%r14\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* (r9,r10,r8) += n2 */
        "addq %%r13, %%r9\n"
        "adcq $0, %%r10\n"
        "adcq $0, %%r8\n"
        /* extract m4 */
        "movq %%r9, %q4\n"
        /* (r10,r8) += n3 */
        "addq %%r14, %%r10\n"
        "adcq $0, %%r8\n"
        /* extract m5 */
        "movq %%r10, %q5\n"
        /* extract m6 */
        "movq %%r8, %q6\n"
        : "=g"(m0), "=g"(m1), "=g"(m2), "=g"(m3), "=g"(m4), "=g"(m5), "=g"(m6)
        : "S"(l), "i"(N_C_0), "i"(N_C_1)
        : "rax", "rdx", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "cc");

        /* Reduce 385 bits into 258. */
        __asm__ __volatile__(
        /* Preload */
        "movq %q9, %%r11\n"
        "movq %q10, %%r12\n"
        "movq %q11, %%r13\n"
        /* Initialize (r8,r9,r10) */
        "movq %q5, %%r8\n"
        "xorq %%r9, %%r9\n"
        "xorq %%r10, %%r10\n"
        /* (r8,r9) += m4 * c0 */
        "movq %12, %%rax\n"
        "mulq %%r11\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        /* extract p0 */
        "movq %%r8, %q0\n"
        "xorq %%r8, %%r8\n"
        /* (r9,r10) += m1 */
        "addq %q6, %%r9\n"
        "adcq $0, %%r10\n"
        /* (r9,r10,r8) += m5 * c0 */
        "movq %12, %%rax\n"
        "mulq %%r12\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* (r9,r10,r8) += m4 * c1 */
        "movq %13, %%rax\n"
        "mulq %%r11\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* extract p1 */
        "movq %%r9, %q1\n"
        "xorq %%r9, %%r9\n"
        /* (r10,r8,r9) += m2 */
        "addq %q7, %%r10\n"
        "adcq $0, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r10,r8,r9) += m6 * c0 */
        "movq %12, %%rax\n"
        "mulq %%r13\n"
        "addq %%rax, %%r10\n"
        "adcq %%rdx, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r10,r8,r9) += m5 * c1 */
        "movq %13, %%rax\n"
        "mulq %%r12\n"
        "addq %%rax, %%r10\n"
        "adcq %%rdx, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r10,r8,r9) += m4 */
        "addq %%r11, %%r10\n"
        "adcq $0, %%r8\n"
        "adcq $0, %%r9\n"
        /* extract p2 */
        "movq %%r10, %q2\n"
        /* (r8,r9) += m3 */
        "addq %q8, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r8,r9) += m6 * c1 */
        "movq %13, %%rax\n"
        "mulq %%r13\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        /* (r8,r9) += m5 */
        "addq %%r12, %%r8\n"
        "adcq $0, %%r9\n"
        /* extract p3 */
        "movq %%r8, %q3\n"
        /* (r9) += m6 */
        "addq %%r13, %%r9\n"
        /* extract p4 */
        "movq %%r9, %q4\n"
        : "=&g"(p0), "=&g"(p1), "=&g"(p2), "=g"(p3), "=g"(p4)
        : "g"(m0), "g"(m1), "g"(m2), "g"(m3), "g"(m4), "g"(m5), "g"(m6), "i"(N_C_0), "i"(N_C_1)
        : "rax", "rdx", "r8", "r9", "r10", "r11", "r12", "r13", "cc");

        /* Reduce 258 bits into 256. */
        __asm__ __volatile__(
        /* Preload */
        "movq %q5, %%r10\n"
        /* (rax,rdx) = p4 * c0 */
        "movq %7, %%rax\n"
        "mulq %%r10\n"
        /* (rax,rdx) += p0 */
        "addq %q1, %%rax\n"
        "adcq $0, %%rdx\n"
        /* extract r0 */
        "movq %%rax, 0(%q6)\n"
        /* Move to (r8,r9) */
        "movq %%rdx, %%r8\n"
        "xorq %%r9, %%r9\n"
        /* (r8,r9) += p1 */
        "addq %q2, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r8,r9) += p4 * c1 */
        "movq %8, %%rax\n"
        "mulq %%r10\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        /* Extract r1 */
        "movq %%r8, 8(%q6)\n"
        "xorq %%r8, %%r8\n"
        /* (r9,r8) += p4 */
        "addq %%r10, %%r9\n"
        "adcq $0, %%r8\n"
        /* (r9,r8) += p2 */
        "addq %q3, %%r9\n"
        "adcq $0, %%r8\n"
        /* Extract r2 */
        "movq %%r9, 16(%q6)\n"
        "xorq %%r9, %%r9\n"
        /* (r8,r9) += p3 */
        "addq %q4, %%r8\n"
        "adcq $0, %%r9\n"
        /* Extract r3 */
        "movq %%r8, 24(%q6)\n"
        /* Extract c */
        "movq %%r9, %q0\n"
        : "=g"(c)
        : "g"(p0), "g"(p1), "g"(p2), "g"(p3), "g"(p4), "D"(r), "i"(N_C_0), "i"(N_C_1)
        : "rax", "rdx", "r8", "r9", "r10", "cc", "memory");
    #else
        uint128_t c;
        uint64_t c0, c1, c2;
        uint64_t n0 = l[4], n1 = l[5], n2 = l[6], n3 = l[7];
        uint64_t m0, m1, m2, m3, m4, m5;
        uint32_t m6;
        uint64_t p0, p1, p2, p3;
        uint32_t p4;

        /* Reduce 512 bits into 385. */
        /* m[0..6] = l[0..3] + n[0..3] * N_C. */
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
        sumadd(n0);
        extract(m2);
        sumadd(l[3]);
        muladd(n3, N_C_0);
        muladd(n2, N_C_1);
        sumadd(n1);
        extract(m3);
        muladd(n3, N_C_1);
        sumadd(n2);
        extract(m4);
        sumadd_fast(n3);
        extract_fast(m5);
        VERIFY_CHECK(c0 <= 1);
        m6 = c0;

        /* Reduce 385 bits into 258. */
        /* p[0..4] = m[0..3] + m[4..6] * N_C. */
        c0 = m0; c1 = 0; c2 = 0;
        muladd_fast(m4, N_C_0);
        extract_fast(p0);
        sumadd_fast(m1);
        muladd(m5, N_C_0);
        muladd(m4, N_C_1);
        extract(p1);
        sumadd(m2);
        muladd(m6, N_C_0);
        muladd(m5, N_C_1);
        sumadd(m4);
        extract(p2);
        sumadd_fast(m3);
        muladd_fast(m6, N_C_1);
        sumadd_fast(m5);
        extract_fast(p3);
        p4 = c0 + m6;
        VERIFY_CHECK(p4 <= 2);

        /* Reduce 258 bits into 256. */
        /* r[0..3] = p[0..3] + p[4] * N_C. */
        c = p0 + (uint128_t)N_C_0 * p4;
        r->d[0] = c & 0xFFFFFFFFFFFFFFFFULL; c >>= 64;
        c += p1 + (uint128_t)N_C_1 * p4;
        r->d[1] = c & 0xFFFFFFFFFFFFFFFFULL; c >>= 64;
        c += p2 + (uint128_t)p4;
        r->d[2] = c & 0xFFFFFFFFFFFFFFFFULL; c >>= 64;
        c += p3;
        r->d[3] = c & 0xFFFFFFFFFFFFFFFFULL; c >>= 64;
    #endif

        /* Final reduction of r. */
        scalar_reduce(r, c + scalar_check_overflow(r));
        */
}

#[cfg(WIDEMUL_INT64)]
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

