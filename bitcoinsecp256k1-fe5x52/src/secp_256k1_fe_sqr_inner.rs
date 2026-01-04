// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/secp_256k1_fe_sqr_inner.rs ]
crate::ix!();

#[inline] pub fn secp_256k1_fe_sqr_inner(
        r: *mut u64,
        a: *const u64)  {

    unsafe {
        let mut c: u128;
        let mut d: u128;
        let mut t3: u64;
        let mut t4: u64;
        let mut tx: u64;
        let mut u0: u64;

        let mut a0: u64 = *a.add(0);
        let a1: u64 = *a.add(1);
        let a2: u64 = *a.add(2);
        let a3: u64 = *a.add(3);
        let mut a4: u64 = *a.add(4);

        const M: u64 = 0xFFFFFFFFFFFFF_u64;
        const R: u64 = 0x1000003D10_u64;

        verify_bits!(a0, 56);
        verify_bits!(a1, 56);
        verify_bits!(a2, 56);
        verify_bits!(a3, 56);
        verify_bits!(a4, 52);

        d = (a0 as u128) * ((a3 as u128) * 2u128)
          + (a1 as u128) * ((a2 as u128) * 2u128);
        verify_bits!(d, 114);
        /* [d 0 0 0] = [p3 0 0 0] */

        c = (a4 as u128) * (a4 as u128);
        verify_bits!(c, 112);
        /* [c 0 0 0 0 d 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */

        d = d.wrapping_add((c & (M as u128)) * (R as u128));
        c >>= 52;
        verify_bits!(d, 115);
        verify_bits!(c, 60);
        /* [c 0 0 0 0 0 d 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */

        t3 = (d & (M as u128)) as u64;
        d >>= 52;
        verify_bits!(t3, 52);
        verify_bits!(d, 63);
        /* [c 0 0 0 0 d t3 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */

        a4 = a4.wrapping_mul(2u64);

        d = d.wrapping_add((a0 as u128) * (a4 as u128)
          + (a1 as u128) * ((a3 as u128) * 2u128)
          + (a2 as u128) * (a2 as u128));
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

        c = (a0 as u128) * (a0 as u128);
        verify_bits!(c, 112);
        /* [d t4+(tx<<48) t3 0 0 c] = [p8 0 0 0 p4 p3 0 0 p0] */

        d = d.wrapping_add((a1 as u128) * (a4 as u128)
          + (a2 as u128) * ((a3 as u128) * 2u128));
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

        a0 = a0.wrapping_mul(2u64);

        c = c.wrapping_add((a0 as u128) * (a1 as u128));
        verify_bits!(c, 114);
        /* [d 0 t4 t3 0 c r0] = [p8 0 0 p5 p4 p3 0 p1 p0] */

        d = d.wrapping_add((a2 as u128) * (a4 as u128)
          + (a3 as u128) * (a3 as u128));
        verify_bits!(d, 114);
        /* [d 0 t4 t3 0 c r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */

        c = c.wrapping_add((d & (M as u128)) * (R as u128));
        d >>= 52;
        verify_bits!(c, 115);
        verify_bits!(d, 62);
        /* [d 0 0 t4 t3 0 c r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */

        *r.add(1) = (c & (M as u128)) as u64;
        c >>= 52;
        verify_bits!(*r.add(1), 52);
        verify_bits!(c, 63);
        /* [d 0 0 t4 t3 c r1 r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */

        c = c.wrapping_add((a0 as u128) * (a2 as u128)
          + (a1 as u128) * (a1 as u128));
        verify_bits!(c, 114);
        /* [d 0 0 t4 t3 c r1 r0] = [p8 0 p6 p5 p4 p3 p2 p1 p0] */

        d = d.wrapping_add((a3 as u128) * (a4 as u128));
        verify_bits!(d, 114);
        /* [d 0 0 t4 t3 c t1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        c = c.wrapping_add((d & (M as u128)) * (R as u128));
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
     *            rcx:rbx = d
     *            r10-r14 = a0-a4
     *            r15     = M (0xfffffffffffff)
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
        "movq $0xfffffffffffff,%%r15\n"

        /* d = (a0*2) * a3 */
        "leaq (%%r10,%%r10,1),%%rax\n"
        "mulq %%r13\n"
        "movq %%rax,%%rbx\n"
        "movq %%rdx,%%rcx\n"
        /* d += (a1*2) * a2 */
        "leaq (%%r11,%%r11,1),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* c = a4 * a4 */
        "movq %%r14,%%rax\n"
        "mulq %%r14\n"
        "movq %%rax,%%r8\n"
        "movq %%rdx,%%r9\n"
        /* d += (c & M) * R */
        "andq %%r15,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* c >>= 52 (%%r8 only) */
        "shrdq $52,%%r9,%%r8\n"
        /* t3 (tmp1) = d & M */
        "movq %%rbx,%%rsi\n"
        "andq %%r15,%%rsi\n"
        "movq %%rsi,%q1\n"
        /* d >>= 52 */
        "shrdq $52,%%rcx,%%rbx\n"
        "xorq %%rcx,%%rcx\n"
        /* a4 *= 2 */
        "addq %%r14,%%r14\n"
        /* d += a0 * a4 */
        "movq %%r10,%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d+= (a1*2) * a3 */
        "leaq (%%r11,%%r11,1),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d += a2 * a2 */
        "movq %%r12,%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d += c * R */
        "movq %%r8,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* t4 = d & M (%%rsi) */
        "movq %%rbx,%%rsi\n"
        "andq %%r15,%%rsi\n"
        /* d >>= 52 */
        "shrdq $52,%%rcx,%%rbx\n"
        "xorq %%rcx,%%rcx\n"
        /* tx = t4 >> 48 (tmp3) */
        "movq %%rsi,%%rax\n"
        "shrq $48,%%rax\n"
        "movq %%rax,%q3\n"
        /* t4 &= (M >> 4) (tmp2) */
        "movq $0xffffffffffff,%%rax\n"
        "andq %%rax,%%rsi\n"
        "movq %%rsi,%q2\n"
        /* c = a0 * a0 */
        "movq %%r10,%%rax\n"
        "mulq %%r10\n"
        "movq %%rax,%%r8\n"
        "movq %%rdx,%%r9\n"
        /* d += a1 * a4 */
        "movq %%r11,%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d += (a2*2) * a3 */
        "leaq (%%r12,%%r12,1),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* u0 = d & M (%%rsi) */
        "movq %%rbx,%%rsi\n"
        "andq %%r15,%%rsi\n"
        /* d >>= 52 */
        "shrdq $52,%%rcx,%%rbx\n"
        "xorq %%rcx,%%rcx\n"
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
        "andq %%r15,%%rax\n"
        "movq %%rax,0(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* a0 *= 2 */
        "addq %%r10,%%r10\n"
        /* c += a0 * a1 */
        "movq %%r10,%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d += a2 * a4 */
        "movq %%r12,%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d += a3 * a3 */
        "movq %%r13,%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* c += (d & M) * R */
        "movq %%rbx,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d >>= 52 */
        "shrdq $52,%%rcx,%%rbx\n"
        "xorq %%rcx,%%rcx\n"
        /* r[1] = c & M */
        "movq %%r8,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq %%rax,8(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += a0 * a2 (last use of %%r10) */
        "movq %%r10,%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* fetch t3 (%%r10, overwrites a0),t4 (%%rsi) */
        "movq %q2,%%rsi\n"
        "movq %q1,%%r10\n"
        /* c += a1 * a1 */
        "movq %%r11,%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d += a3 * a4 */
        "movq %%r13,%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* c += (d & M) * R */
        "movq %%rbx,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d >>= 52 (%%rbx only) */
        "shrdq $52,%%rcx,%%rbx\n"
        /* r[2] = c & M */
        "movq %%r8,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq %%rax,16(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += t3 */
        "addq %%r10,%%r8\n"
        /* c += d * R */
        "movq %%rbx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* r[3] = c & M */
        "movq %%r8,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq %%rax,24(%%rdi)\n"
        /* c >>= 52 (%%r8 only) */
        "shrdq $52,%%r9,%%r8\n"
        /* c += t4 (%%r8 only) */
        "addq %%rsi,%%r8\n"
        /* r[4] = c */
        "movq %%r8,32(%%rdi)\n"
    : "+S"(a), "=m"(tmp1), "=m"(tmp2), "=m"(tmp3)
    : "D"(r)
    : "%rax", "%rbx", "%rcx", "%rdx", "%r8", "%r9", "%r10", "%r11", "%r12", "%r13", "%r14", "%r15", "cc", "memory"
    );
        */
}

#[cfg(test)]
mod secp_256k1_fe_sqr_inner_rs_exhaustive_tests {
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
    fn sqr_inner_matches_mul_inner_self_and_high_level_fe_sqr_for_small_inputs() {
        tracing::info!("testing secp_256k1_fe_sqr_inner agreement with mul_inner(a,a) and fe_sqr");

        unsafe {
            let vals: [u64; 8] = [0, 1, 2, 3, 5, 7, 255, 65536];

            for &v in vals.iter() {
                tracing::debug!(value_u64 = v, "testing sqr_inner equivalence");
                let a = fe_from_u64(v);

                let mut sqr_out = [0u64; 5];
                crate::secp_256k1_fe_sqr_inner(sqr_out.as_mut_ptr(), a.n.as_ptr());

                let mut mul_out = [0u64; 5];
                crate::secp_256k1_fe_mul_inner(mul_out.as_mut_ptr(), a.n.as_ptr(), a.n.as_ptr());
                assert_eq!(sqr_out, mul_out);

                let mut sqr_fe = Fe5x52::new();
                sqr_fe.n = sqr_out;

                let mut high = Fe5x52::new();
                crate::fe_sqr(&mut high as *mut Fe5x52, &a as *const Fe5x52);

                let sqr_b = fe_to_b32_normalized(&mut sqr_fe);
                let high_b = fe_to_b32_normalized(&mut high);

                assert_eq!(sqr_b, high_b);

                let expected = (v as u128) * (v as u128);
                assert!(expected <= u128::from(u64::MAX));
                assert_eq!(sqr_b, u64_to_be32(expected as u64));
            }
        }
    }
}
