// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/secp_256k1_fe_mul_inner.rs ]
crate::ix!();

/**
  | Changelog:
  | 
  | - March 2013, Diederik Huys: original
  | version
  | 
  | - November 2014, Pieter Wuille: updated
  | to use Peter Dettman's parallel multiplication
  | algorithm
  | 
  | - December 2014, Pieter Wuille: converted
  | from YASM to GCC inline assembly
  |
  */
#[inline] pub fn secp_256k1_fe_mul_inner(
        r: *mut u64,
        a: *const u64,
        b: *const u64)  {

    unsafe {
        let mut c: u128;
        let mut d: u128;
        let mut t3: u64;
        let mut t4: u64;
        let mut tx: u64;
        let mut u0: u64;

        let a0: u64 = *a.add(0);
        let a1: u64 = *a.add(1);
        let a2: u64 = *a.add(2);
        let a3: u64 = *a.add(3);
        let a4: u64 = *a.add(4);

        let b0: u64 = *b.add(0);
        let b1: u64 = *b.add(1);
        let b2: u64 = *b.add(2);
        let b3: u64 = *b.add(3);
        let b4: u64 = *b.add(4);

        const M: u64 = 0xFFFFFFFFFFFFF_u64;
        const R: u64 = 0x1000003D10_u64;

        verify_bits!(a0, 56);
        verify_bits!(a1, 56);
        verify_bits!(a2, 56);
        verify_bits!(a3, 56);
        verify_bits!(a4, 52);
        verify_bits!(b0, 56);
        verify_bits!(b1, 56);
        verify_bits!(b2, 56);
        verify_bits!(b3, 56);
        verify_bits!(b4, 52);
        verify_check!(r != b as *mut u64);
        verify_check!(a != b);

        /* [... a b c] is a shorthand for ... + a<<104 + b<<52 + c<<0 mod n.
         * for 0 <= x <= 4, px is a shorthand for sum(a[i]*b[x-i], i=0..x).
         * for 4 <= x <= 8, px is a shorthand for sum(a[i]*b[x-i], i=(x-4)..4)
         * Note that [x 0 0 0 0 0] = [x*R].
         */

        d = (a0 as u128) * (b3 as u128)
          + (a1 as u128) * (b2 as u128)
          + (a2 as u128) * (b1 as u128)
          + (a3 as u128) * (b0 as u128);
        verify_bits!(d, 114);
        /* [d 0 0 0] = [p3 0 0 0] */

        c = (a4 as u128) * (b4 as u128);
        verify_bits!(c, 112);
        /* [c 0 0 0 0 d 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */

        d = d.wrapping_add(((c & (M as u128)) * (R as u128)));
        c >>= 52;
        verify_bits!(d, 115);
        verify_bits!(c, 60);
        /* [c 0 0 0 0 0 d 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */

        t3 = (d & (M as u128)) as u64;
        d >>= 52;
        verify_bits!(t3, 52);
        verify_bits!(d, 63);
        /* [c 0 0 0 0 d t3 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */

        d = d.wrapping_add((a0 as u128) * (b4 as u128)
          + (a1 as u128) * (b3 as u128)
          + (a2 as u128) * (b2 as u128)
          + (a3 as u128) * (b1 as u128)
          + (a4 as u128) * (b0 as u128));
        verify_bits!(d, 115);
        /* [c 0 0 0 0 d t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */

        d = d.wrapping_add(c.wrapping_mul(R as u128));
        verify_bits!(d, 116);
        /* [d t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */

        t4 = (d & (M as u128)) as u64;
        d >>= 52;
        verify_bits!(t4, 52);
        verify_bits!(d, 64);
        /* [d t4 t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */

        tx = t4 >> 48;
        t4 &= (M >> 4);
        verify_bits!(tx, 4);
        verify_bits!(t4, 48);
        /* [d t4+(tx<<48) t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */

        c = (a0 as u128) * (b0 as u128);
        verify_bits!(c, 112);
        /* [d t4+(tx<<48) t3 0 0 c] = [p8 0 0 0 p4 p3 0 0 p0] */

        d = d.wrapping_add((a1 as u128) * (b4 as u128)
          + (a2 as u128) * (b3 as u128)
          + (a3 as u128) * (b2 as u128)
          + (a4 as u128) * (b1 as u128));
        verify_bits!(d, 115);
        /* [d t4+(tx<<48) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */

        u0 = (d & (M as u128)) as u64;
        d >>= 52;
        verify_bits!(u0, 52);
        verify_bits!(d, 63);
        /* [d u0 t4+(tx<<48) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        /* [d 0 t4+(tx<<48)+(u0<<52) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */

        u0 = (u0 << 4) | tx;
        verify_bits!(u0, 56);
        /* [d 0 t4+(u0<<48) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */

        c = c.wrapping_add((u0 as u128) * ((R >> 4) as u128));
        verify_bits!(c, 115);
        /* [d 0 t4 t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */

        *r.add(0) = (c & (M as u128)) as u64;
        c >>= 52;
        verify_bits!(*r.add(0), 52);
        verify_bits!(c, 61);
        /* [d 0 t4 t3 0 c r0] = [p8 0 0 p5 p4 p3 0 0 p0] */

        c = c.wrapping_add((a0 as u128) * (b1 as u128)
          + (a1 as u128) * (b0 as u128));
        verify_bits!(c, 114);
        /* [d 0 t4 t3 0 c r0] = [p8 0 0 p5 p4 p3 0 p1 p0] */

        d = d.wrapping_add((a2 as u128) * (b4 as u128)
          + (a3 as u128) * (b3 as u128)
          + (a4 as u128) * (b2 as u128));
        verify_bits!(d, 114);
        /* [d 0 t4 t3 0 c r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */

        c = c.wrapping_add(((d & (M as u128)) * (R as u128)));
        d >>= 52;
        verify_bits!(c, 115);
        verify_bits!(d, 62);
        /* [d 0 0 t4 t3 0 c r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */

        *r.add(1) = (c & (M as u128)) as u64;
        c >>= 52;
        verify_bits!(*r.add(1), 52);
        verify_bits!(c, 63);
        /* [d 0 0 t4 t3 c r1 r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */

        c = c.wrapping_add((a0 as u128) * (b2 as u128)
          + (a1 as u128) * (b1 as u128)
          + (a2 as u128) * (b0 as u128));
        verify_bits!(c, 114);
        /* [d 0 0 t4 t3 c r1 r0] = [p8 0 p6 p5 p4 p3 p2 p1 p0] */

        d = d.wrapping_add((a3 as u128) * (b4 as u128)
          + (a4 as u128) * (b3 as u128));
        verify_bits!(d, 114);
        /* [d 0 0 t4 t3 c t1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        c = c.wrapping_add(((d & (M as u128)) * (R as u128)));
        d >>= 52;
        verify_bits!(c, 115);
        verify_bits!(d, 62);
        /* [d 0 0 0 t4 t3 c r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(2) = (c & (M as u128)) as u64;
        c >>= 52;
        verify_bits!(*r.add(2), 52);
        verify_bits!(c, 63);
        /* [d 0 0 0 t4 t3+c r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        c = c.wrapping_add(d.wrapping_mul(R as u128)).wrapping_add(t3 as u128);
        verify_bits!(c, 100);
        /* [t4 c r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(3) = (c & (M as u128)) as u64;
        c >>= 52;
        verify_bits!(*r.add(3), 52);
        verify_bits!(c, 48);
        /* [t4+c r3 r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        c = c.wrapping_add(t4 as u128);
        verify_bits!(c, 49);
        *r.add(4) = c as u64;
        verify_bits!(*r.add(4), 49);
    }

        /*
            /**
     * Registers: rdx:rax = multiplication accumulator
     *            r9:r8   = c
     *            r15:rcx = d
     *            r10-r14 = a0-a4
     *            rbx     = b
     *            rdi     = r
     *            rsi     = a / t?
     */
      uint64_t tmp1, tmp2, tmp3;
    __asm__ __volatile__(
        "movq 0(%%rsi),%%r10\n"
        "movq 8(%%rsi),%%r11\n"
        "movq 16(%%rsi),%%r12\n"
        "movq 24(%%rsi),%%r13\n"
        "movq 32(%%rsi),%%r14\n"

        /* d += a3 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "movq %%rax,%%rcx\n"
        "movq %%rdx,%%r15\n"
        /* d += a2 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a1 * b2 */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d = a0 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* c = a4 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "movq %%rax,%%r8\n"
        "movq %%rdx,%%r9\n"
        /* d += (c & M) * R */
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* c >>= 52 (%%r8 only) */
        "shrdq $52,%%r9,%%r8\n"
        /* t3 (tmp1) = d & M */
        "movq %%rcx,%%rsi\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rsi\n"
        "movq %%rsi,%q1\n"
        /* d >>= 52 */
        "shrdq $52,%%r15,%%rcx\n"
        "xorq %%r15,%%r15\n"
        /* d += a4 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a3 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a2 * b2 */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a1 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a0 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += c * R */
        "movq %%r8,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* t4 = d & M (%%rsi) */
        "movq %%rcx,%%rsi\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rsi\n"
        /* d >>= 52 */
        "shrdq $52,%%r15,%%rcx\n"
        "xorq %%r15,%%r15\n"
        /* tx = t4 >> 48 (tmp3) */
        "movq %%rsi,%%rax\n"
        "shrq $48,%%rax\n"
        "movq %%rax,%q3\n"
        /* t4 &= (M >> 4) (tmp2) */
        "movq $0xffffffffffff,%%rax\n"
        "andq %%rax,%%rsi\n"
        "movq %%rsi,%q2\n"
        /* c = a0 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "movq %%rax,%%r8\n"
        "movq %%rdx,%%r9\n"
        /* d += a4 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a3 * b2 */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a2 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a1 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* u0 = d & M (%%rsi) */
        "movq %%rcx,%%rsi\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rsi\n"
        /* d >>= 52 */
        "shrdq $52,%%r15,%%rcx\n"
        "xorq %%r15,%%r15\n"
        /* u0 = (u0 << 4) | tx (%%rsi) */
        "shlq $4,%%rsi\n"
        "movq %q3,%%rax\n"
        "orq %%rax,%%rsi\n"
        /* c += u0 * (R >> 4) */
        "movq $0x1000003d1,%%rax\n"
        "mulq %%rsi\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* r[0] = c & M */
        "movq %%r8,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq %%rax,0(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += a1 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* c += a0 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d += a4 * b2 */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a3 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a2 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* c += (d & M) * R */
        "movq %%rcx,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d >>= 52 */
        "shrdq $52,%%r15,%%rcx\n"
        "xorq %%r15,%%r15\n"
        /* r[1] = c & M */
        "movq %%r8,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq %%rax,8(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += a2 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* c += a1 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* c += a0 * b2 (last use of %%r10 = a0) */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* fetch t3 (%%r10, overwrites a0), t4 (%%rsi) */
        "movq %q2,%%rsi\n"
        "movq %q1,%%r10\n"
        /* d += a4 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a3 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* c += (d & M) * R */
        "movq %%rcx,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d >>= 52 (%%rcx only) */
        "shrdq $52,%%r15,%%rcx\n"
        /* r[2] = c & M */
        "movq %%r8,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq %%rax,16(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += t3 */
        "addq %%r10,%%r8\n"
        /* c += d * R */
        "movq %%rcx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* r[3] = c & M */
        "movq %%r8,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq %%rax,24(%%rdi)\n"
        /* c >>= 52 (%%r8 only) */
        "shrdq $52,%%r9,%%r8\n"
        /* c += t4 (%%r8 only) */
        "addq %%rsi,%%r8\n"
        /* r[4] = c */
        "movq %%r8,32(%%rdi)\n"
    : "+S"(a), "=m"(tmp1), "=m"(tmp2), "=m"(tmp3)
    : "b"(b), "D"(r)
    : "%rax", "%rcx", "%rdx", "%r8", "%r9", "%r10", "%r11", "%r12", "%r13", "%r14", "%r15", "cc", "memory"
    );
        */
}

#[cfg(test)]
mod secp_256k1_fe_mul_inner_rs_exhaustive_tests {
    use super::*;

    fn u64_to_be32(v: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out
    }

    unsafe fn fe_from_u64(v: u64) -> Fe5x52 {
        let mut fe = Fe5x52::new();
        let bytes = u64_to_be32(v);
        let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, bytes.as_ptr());
        assert_eq!(ret, 1);
        fe
    }

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    #[traced_test]
    fn mul_inner_matches_high_level_fe_mul_for_small_inputs_and_is_commutative() {
        tracing::info!("testing secp_256k1_fe_mul_inner agreement with fe_mul for small inputs");

        unsafe {
            let cases: &[(u64, u64)] = &[
                (0, 0),
                (0, 1),
                (1, 0),
                (1, 1),
                (2, 3),
                (7, 11),
                (17, 19),
                (255, 256),
            ];

            for &(x, y) in cases.iter() {
                tracing::debug!(x_u64 = x, y_u64 = y, "comparing mul_inner with fe_mul");
                let a = fe_from_u64(x);
                let b = fe_from_u64(y);

                let mut inner_out = [0u64; 5];
                crate::secp_256k1_fe_mul_inner(inner_out.as_mut_ptr(), a.n.as_ptr(), b.n.as_ptr());
                let mut inner_fe = Fe5x52::new();
                inner_fe.n = inner_out;

                let mut high = Fe5x52::new();
                crate::fe_mul(&mut high as *mut Fe5x52, &a as *const Fe5x52, &b as *const Fe5x52);

                let inner_b = fe_to_b32_normalized(&mut inner_fe);
                let high_b = fe_to_b32_normalized(&mut high);

                assert_eq!(inner_b, high_b);

                let mut inner_out_sw = [0u64; 5];
                crate::secp_256k1_fe_mul_inner(inner_out_sw.as_mut_ptr(), b.n.as_ptr(), a.n.as_ptr());
                assert_eq!(inner_out, inner_out_sw);
            }
        }
    }
}
