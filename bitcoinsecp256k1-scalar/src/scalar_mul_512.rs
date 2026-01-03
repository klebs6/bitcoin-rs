// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_mul_512.rs ]
crate::ix!();

#[cfg(WIDEMUL_INT128)]
pub fn scalar_mul_512(
        l: [u64; 8],
        a: *const Scalar,
        b: *const Scalar)  {
    
    todo!();
        /*
            #ifdef USE_ASM_X86_64
        const uint64_t *pb = b->d;
        __asm__ __volatile__(
        /* Preload */
        "movq 0(%%rdi), %%r15\n"
        "movq 8(%%rdi), %%rbx\n"
        "movq 16(%%rdi), %%rcx\n"
        "movq 0(%%rdx), %%r11\n"
        "movq 8(%%rdx), %%r12\n"
        "movq 16(%%rdx), %%r13\n"
        "movq 24(%%rdx), %%r14\n"
        /* (rax,rdx) = a0 * b0 */
        "movq %%r15, %%rax\n"
        "mulq %%r11\n"
        /* Extract l0 */
        "movq %%rax, 0(%%rsi)\n"
        /* (r8,r9,r10) = (rdx) */
        "movq %%rdx, %%r8\n"
        "xorq %%r9, %%r9\n"
        "xorq %%r10, %%r10\n"
        /* (r8,r9,r10) += a0 * b1 */
        "movq %%r15, %%rax\n"
        "mulq %%r12\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        "adcq $0, %%r10\n"
        /* (r8,r9,r10) += a1 * b0 */
        "movq %%rbx, %%rax\n"
        "mulq %%r11\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        "adcq $0, %%r10\n"
        /* Extract l1 */
        "movq %%r8, 8(%%rsi)\n"
        "xorq %%r8, %%r8\n"
        /* (r9,r10,r8) += a0 * b2 */
        "movq %%r15, %%rax\n"
        "mulq %%r13\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* (r9,r10,r8) += a1 * b1 */
        "movq %%rbx, %%rax\n"
        "mulq %%r12\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* (r9,r10,r8) += a2 * b0 */
        "movq %%rcx, %%rax\n"
        "mulq %%r11\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* Extract l2 */
        "movq %%r9, 16(%%rsi)\n"
        "xorq %%r9, %%r9\n"
        /* (r10,r8,r9) += a0 * b3 */
        "movq %%r15, %%rax\n"
        "mulq %%r14\n"
        "addq %%rax, %%r10\n"
        "adcq %%rdx, %%r8\n"
        "adcq $0, %%r9\n"
        /* Preload a3 */
        "movq 24(%%rdi), %%r15\n"
        /* (r10,r8,r9) += a1 * b2 */
        "movq %%rbx, %%rax\n"
        "mulq %%r13\n"
        "addq %%rax, %%r10\n"
        "adcq %%rdx, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r10,r8,r9) += a2 * b1 */
        "movq %%rcx, %%rax\n"
        "mulq %%r12\n"
        "addq %%rax, %%r10\n"
        "adcq %%rdx, %%r8\n"
        "adcq $0, %%r9\n"
        /* (r10,r8,r9) += a3 * b0 */
        "movq %%r15, %%rax\n"
        "mulq %%r11\n"
        "addq %%rax, %%r10\n"
        "adcq %%rdx, %%r8\n"
        "adcq $0, %%r9\n"
        /* Extract l3 */
        "movq %%r10, 24(%%rsi)\n"
        "xorq %%r10, %%r10\n"
        /* (r8,r9,r10) += a1 * b3 */
        "movq %%rbx, %%rax\n"
        "mulq %%r14\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        "adcq $0, %%r10\n"
        /* (r8,r9,r10) += a2 * b2 */
        "movq %%rcx, %%rax\n"
        "mulq %%r13\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        "adcq $0, %%r10\n"
        /* (r8,r9,r10) += a3 * b1 */
        "movq %%r15, %%rax\n"
        "mulq %%r12\n"
        "addq %%rax, %%r8\n"
        "adcq %%rdx, %%r9\n"
        "adcq $0, %%r10\n"
        /* Extract l4 */
        "movq %%r8, 32(%%rsi)\n"
        "xorq %%r8, %%r8\n"
        /* (r9,r10,r8) += a2 * b3 */
        "movq %%rcx, %%rax\n"
        "mulq %%r14\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* (r9,r10,r8) += a3 * b2 */
        "movq %%r15, %%rax\n"
        "mulq %%r13\n"
        "addq %%rax, %%r9\n"
        "adcq %%rdx, %%r10\n"
        "adcq $0, %%r8\n"
        /* Extract l5 */
        "movq %%r9, 40(%%rsi)\n"
        /* (r10,r8) += a3 * b3 */
        "movq %%r15, %%rax\n"
        "mulq %%r14\n"
        "addq %%rax, %%r10\n"
        "adcq %%rdx, %%r8\n"
        /* Extract l6 */
        "movq %%r10, 48(%%rsi)\n"
        /* Extract l7 */
        "movq %%r8, 56(%%rsi)\n"
        : "+d"(pb)
        : "S"(l), "D"(a->d)
        : "rax", "rbx", "rcx", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "cc", "memory");
    #else
        /* 160 bit accumulator. */
        uint64_t c0 = 0, c1 = 0;
        uint32_t c2 = 0;

        /* l[0..7] = a[0..3] * b[0..3]. */
        muladd_fast(a->d[0], b->d[0]);
        extract_fast(l[0]);
        muladd(a->d[0], b->d[1]);
        muladd(a->d[1], b->d[0]);
        extract(l[1]);
        muladd(a->d[0], b->d[2]);
        muladd(a->d[1], b->d[1]);
        muladd(a->d[2], b->d[0]);
        extract(l[2]);
        muladd(a->d[0], b->d[3]);
        muladd(a->d[1], b->d[2]);
        muladd(a->d[2], b->d[1]);
        muladd(a->d[3], b->d[0]);
        extract(l[3]);
        muladd(a->d[1], b->d[3]);
        muladd(a->d[2], b->d[2]);
        muladd(a->d[3], b->d[1]);
        extract(l[4]);
        muladd(a->d[2], b->d[3]);
        muladd(a->d[3], b->d[2]);
        extract(l[5]);
        muladd_fast(a->d[3], b->d[3]);
        extract_fast(l[6]);
        VERIFY_CHECK(c1 == 0);
        l[7] = c0;
    #endif
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_mul_512(
        l: *mut u32,
        a: *const Scalar,
        b: *const Scalar)  {
    
    todo!();
        /*
            /* 96 bit accumulator. */
        uint32_t c0 = 0, c1 = 0, c2 = 0;

        /* l[0..15] = a[0..7] * b[0..7]. */
        muladd_fast(a->d[0], b->d[0]);
        extract_fast(l[0]);
        muladd(a->d[0], b->d[1]);
        muladd(a->d[1], b->d[0]);
        extract(l[1]);
        muladd(a->d[0], b->d[2]);
        muladd(a->d[1], b->d[1]);
        muladd(a->d[2], b->d[0]);
        extract(l[2]);
        muladd(a->d[0], b->d[3]);
        muladd(a->d[1], b->d[2]);
        muladd(a->d[2], b->d[1]);
        muladd(a->d[3], b->d[0]);
        extract(l[3]);
        muladd(a->d[0], b->d[4]);
        muladd(a->d[1], b->d[3]);
        muladd(a->d[2], b->d[2]);
        muladd(a->d[3], b->d[1]);
        muladd(a->d[4], b->d[0]);
        extract(l[4]);
        muladd(a->d[0], b->d[5]);
        muladd(a->d[1], b->d[4]);
        muladd(a->d[2], b->d[3]);
        muladd(a->d[3], b->d[2]);
        muladd(a->d[4], b->d[1]);
        muladd(a->d[5], b->d[0]);
        extract(l[5]);
        muladd(a->d[0], b->d[6]);
        muladd(a->d[1], b->d[5]);
        muladd(a->d[2], b->d[4]);
        muladd(a->d[3], b->d[3]);
        muladd(a->d[4], b->d[2]);
        muladd(a->d[5], b->d[1]);
        muladd(a->d[6], b->d[0]);
        extract(l[6]);
        muladd(a->d[0], b->d[7]);
        muladd(a->d[1], b->d[6]);
        muladd(a->d[2], b->d[5]);
        muladd(a->d[3], b->d[4]);
        muladd(a->d[4], b->d[3]);
        muladd(a->d[5], b->d[2]);
        muladd(a->d[6], b->d[1]);
        muladd(a->d[7], b->d[0]);
        extract(l[7]);
        muladd(a->d[1], b->d[7]);
        muladd(a->d[2], b->d[6]);
        muladd(a->d[3], b->d[5]);
        muladd(a->d[4], b->d[4]);
        muladd(a->d[5], b->d[3]);
        muladd(a->d[6], b->d[2]);
        muladd(a->d[7], b->d[1]);
        extract(l[8]);
        muladd(a->d[2], b->d[7]);
        muladd(a->d[3], b->d[6]);
        muladd(a->d[4], b->d[5]);
        muladd(a->d[5], b->d[4]);
        muladd(a->d[6], b->d[3]);
        muladd(a->d[7], b->d[2]);
        extract(l[9]);
        muladd(a->d[3], b->d[7]);
        muladd(a->d[4], b->d[6]);
        muladd(a->d[5], b->d[5]);
        muladd(a->d[6], b->d[4]);
        muladd(a->d[7], b->d[3]);
        extract(l[10]);
        muladd(a->d[4], b->d[7]);
        muladd(a->d[5], b->d[6]);
        muladd(a->d[6], b->d[5]);
        muladd(a->d[7], b->d[4]);
        extract(l[11]);
        muladd(a->d[5], b->d[7]);
        muladd(a->d[6], b->d[6]);
        muladd(a->d[7], b->d[5]);
        extract(l[12]);
        muladd(a->d[6], b->d[7]);
        muladd(a->d[7], b->d[6]);
        extract(l[13]);
        muladd_fast(a->d[7], b->d[7]);
        extract_fast(l[14]);
        VERIFY_CHECK(c1 == 0);
        l[15] = c0;
        */
}
