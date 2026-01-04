// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_mul.rs ]
crate::ix!();

pub fn fe_mul(
    r: *mut Fe10x26,
    a: *const Fe10x26,
    b: *mut Fe10x26)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).magnitude <= 8);
            verify_check!((*b).magnitude <= 8);
            fe_verify(a);
            fe_verify(b);
            verify_check!(r != b);
            verify_check!(a != b);
        }

        fe_mul_inner((*r).n.as_mut_ptr(), (*a).n.as_ptr(), (*b).n.as_ptr());

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

/**
  | External assembler implementation
  |
  */
#[cfg(feature="secp256k1-use-external-asm")]
pub fn fe_mul_inner(
    r: *mut u32,
    a: *const u32,
    b: *const u32)  {

    extern "C" {
        fn secp256k1_fe_mul_inner(r: *mut u32, a: *const u32, b: *const u32);
    }

    unsafe {
        secp256k1_fe_mul_inner(r, a, b);
    }
}

#[cfg(not(feature="secp256k1-use-external-asm"))]
#[inline] fn fe_mul_inner(
    r: *mut u32,
    a: *const u32,
    b: *const u32)  {

    unsafe {
        let mut c: u64;
        let mut d: u64;
        let mut u0: u64;
        let mut u1: u64;
        let mut u2: u64;
        let mut u3: u64;
        let mut u4: u64;
        let mut u5: u64;
        let mut u6: u64;
        let mut u7: u64;
        let mut u8: u64;

        let mut t9: u32;
        let mut t1: u32;
        let mut t0: u32;
        let mut t2: u32;
        let mut t3: u32;
        let mut t4: u32;
        let mut t5: u32;
        let mut t6: u32;
        let mut t7: u32;

        const M: u32 = 0x3FFFFFFu32;
        const R0: u32 = 0x3D10u32;
        const R1: u32 = 0x400u32;

        verify_bits!(*a.add(0), 30);
        verify_bits!(*a.add(1), 30);
        verify_bits!(*a.add(2), 30);
        verify_bits!(*a.add(3), 30);
        verify_bits!(*a.add(4), 30);
        verify_bits!(*a.add(5), 30);
        verify_bits!(*a.add(6), 30);
        verify_bits!(*a.add(7), 30);
        verify_bits!(*a.add(8), 30);
        verify_bits!(*a.add(9), 26);

        verify_bits!(*b.add(0), 30);
        verify_bits!(*b.add(1), 30);
        verify_bits!(*b.add(2), 30);
        verify_bits!(*b.add(3), 30);
        verify_bits!(*b.add(4), 30);
        verify_bits!(*b.add(5), 30);
        verify_bits!(*b.add(6), 30);
        verify_bits!(*b.add(7), 30);
        verify_bits!(*b.add(8), 30);
        verify_bits!(*b.add(9), 26);

        let a0 = *a.add(0) as u64;
        let a1 = *a.add(1) as u64;
        let a2 = *a.add(2) as u64;
        let a3 = *a.add(3) as u64;
        let a4 = *a.add(4) as u64;
        let a5 = *a.add(5) as u64;
        let a6 = *a.add(6) as u64;
        let a7 = *a.add(7) as u64;
        let a8 = *a.add(8) as u64;
        let a9 = *a.add(9) as u64;

        let b0 = *b.add(0) as u64;
        let b1 = *b.add(1) as u64;
        let b2 = *b.add(2) as u64;
        let b3 = *b.add(3) as u64;
        let b4 = *b.add(4) as u64;
        let b5 = *b.add(5) as u64;
        let b6 = *b.add(6) as u64;
        let b7 = *b.add(7) as u64;
        let b8 = *b.add(8) as u64;
        let b9 = *b.add(9) as u64;

        /* [... a b c] is a shorthand for ... + a<<52 + b<<26 + c<<0 mod n.
         *  for 0 <= x <= 9, px is a shorthand for sum(a[i]*b[x-i], i=0..x).
         *  for 9 <= x <= 18, px is a shorthand for sum(a[i]*b[x-i], i=(x-9)..9)
         *  Note that [x 0 0 0 0 0 0 0 0 0 0] = [x*R1 x*R0].
         */
        d  = a0 * b9
           + a1 * b8
           + a2 * b7
           + a3 * b6
           + a4 * b5
           + a5 * b4
           + a6 * b3
           + a7 * b2
           + a8 * b1
           + a9 * b0;

        /* VERIFY_BITS(d, 64); */
        /* [d 0 0 0 0 0 0 0 0 0] = [p9 0 0 0 0 0 0 0 0 0] */
        t9 = (d & (M as u64)) as u32; d >>= 26;
        verify_bits!(t9, 26);
        verify_bits!(d, 38);
        /* [d t9 0 0 0 0 0 0 0 0 0] = [p9 0 0 0 0 0 0 0 0 0] */

        c  = a0 * b0;
        verify_bits!(c, 60);

        /* [d t9 0 0 0 0 0 0 0 0 c] = [p9 0 0 0 0 0 0 0 0 p0] */
        d += a1 * b9
           + a2 * b8
           + a3 * b7
           + a4 * b6
           + a5 * b5
           + a6 * b4
           + a7 * b3
           + a8 * b2
           + a9 * b1;
        verify_bits!(d, 63);

        /* [d t9 0 0 0 0 0 0 0 0 c] = [p10 p9 0 0 0 0 0 0 0 0 p0] */
        u0 = d & (M as u64); d >>= 26; c = c.wrapping_add(u0.wrapping_mul(R0 as u64));
        verify_bits!(u0, 26);
        verify_bits!(d, 37);
        verify_bits!(c, 61);

        /* [d u0 t9 0 0 0 0 0 0 0 0 c-u0*R0] = [p10 p9 0 0 0 0 0 0 0 0 p0] */
        t0 = (c & (M as u64)) as u32; c >>= 26; c = c.wrapping_add(u0.wrapping_mul(R1 as u64));
        verify_bits!(t0, 26);
        verify_bits!(c, 37);
        /* [d u0 t9 0 0 0 0 0 0 0 c-u0*R1 t0-u0*R0] = [p10 p9 0 0 0 0 0 0 0 0 p0] */
        /* [d 0 t9 0 0 0 0 0 0 0 c t0] = [p10 p9 0 0 0 0 0 0 0 0 p0] */

        c = c.wrapping_add(a0 * b1
           + a1 * b0);
        verify_bits!(c, 62);

        /* [d 0 t9 0 0 0 0 0 0 0 c t0] = [p10 p9 0 0 0 0 0 0 0 p1 p0] */
        d = d.wrapping_add(a2 * b9
           + a3 * b8
           + a4 * b7
           + a5 * b6
           + a6 * b5
           + a7 * b4
           + a8 * b3
           + a9 * b2);
        verify_bits!(d, 63);

        /* [d 0 t9 0 0 0 0 0 0 0 c t0] = [p11 p10 p9 0 0 0 0 0 0 0 p1 p0] */
        u1 = d & (M as u64); d >>= 26; c = c.wrapping_add(u1.wrapping_mul(R0 as u64));
        verify_bits!(u1, 26);
        verify_bits!(d, 37);
        verify_bits!(c, 63);

        /* [d u1 0 t9 0 0 0 0 0 0 0 c-u1*R0 t0] = [p11 p10 p9 0 0 0 0 0 0 0 p1 p0] */
        t1 = (c & (M as u64)) as u32; c >>= 26; c = c.wrapping_add(u1.wrapping_mul(R1 as u64));
        verify_bits!(t1, 26);
        verify_bits!(c, 38);
        /* [d u1 0 t9 0 0 0 0 0 0 c-u1*R1 t1-u1*R0 t0] = [p11 p10 p9 0 0 0 0 0 0 0 p1 p0] */
        /* [d 0 0 t9 0 0 0 0 0 0 c t1 t0] = [p11 p10 p9 0 0 0 0 0 0 0 p1 p0] */

        c = c.wrapping_add(a0 * b2
           + a1 * b1
           + a2 * b0);
        verify_bits!(c, 62);

        /* [d 0 0 t9 0 0 0 0 0 0 c t1 t0] = [p11 p10 p9 0 0 0 0 0 0 p2 p1 p0] */
        d = d.wrapping_add(a3 * b9
           + a4 * b8
           + a5 * b7
           + a6 * b6
           + a7 * b5
           + a8 * b4
           + a9 * b3);
        verify_bits!(d, 63);

        /* [d 0 0 t9 0 0 0 0 0 0 c t1 t0] = [p12 p11 p10 p9 0 0 0 0 0 0 p2 p1 p0] */
        u2 = d & (M as u64); d >>= 26; c = c.wrapping_add(u2.wrapping_mul(R0 as u64));
        verify_bits!(u2, 26);
        verify_bits!(d, 37);
        verify_bits!(c, 63);

        /* [d u2 0 0 t9 0 0 0 0 0 0 c-u2*R0 t1 t0] = [p12 p11 p10 p9 0 0 0 0 0 0 p2 p1 p0] */
        t2 = (c & (M as u64)) as u32; c >>= 26; c = c.wrapping_add(u2.wrapping_mul(R1 as u64));
        verify_bits!(t2, 26);
        verify_bits!(c, 38);
        /* [d u2 0 0 t9 0 0 0 0 0 c-u2*R1 t2-u2*R0 t1 t0] = [p12 p11 p10 p9 0 0 0 0 0 0 p2 p1 p0] */
        /* [d 0 0 0 t9 0 0 0 0 0 c t2 t1 t0] = [p12 p11 p10 p9 0 0 0 0 0 0 p2 p1 p0] */

        c = c.wrapping_add(a0 * b3
           + a1 * b2
           + a2 * b1
           + a3 * b0);
        verify_bits!(c, 63);
        /* [d 0 0 0 t9 0 0 0 0 0 c t2 t1 t0] = [p12 p11 p10 p9 0 0 0 0 0 p3 p2 p1 p0] */

        d = d.wrapping_add(a4 * b9
           + a5 * b8
           + a6 * b7
           + a7 * b6
           + a8 * b5
           + a9 * b4);
        verify_bits!(d, 63);

        /* [d 0 0 0 t9 0 0 0 0 0 c t2 t1 t0] = [p13 p12 p11 p10 p9 0 0 0 0 0 p3 p2 p1 p0] */
        u3 = d & (M as u64); d >>= 26; c = c.wrapping_add(u3.wrapping_mul(R0 as u64));
        verify_bits!(u3, 26);
        verify_bits!(d, 37);

        /* VERIFY_BITS(c, 64); */
        /* [d u3 0 0 0 t9 0 0 0 0 0 c-u3*R0 t2 t1 t0] = [p13 p12 p11 p10 p9 0 0 0 0 0 p3 p2 p1 p0] */
        t3 = (c & (M as u64)) as u32; c >>= 26; c = c.wrapping_add(u3.wrapping_mul(R1 as u64));
        verify_bits!(t3, 26);
        verify_bits!(c, 39);
        /* [d u3 0 0 0 t9 0 0 0 0 c-u3*R1 t3-u3*R0 t2 t1 t0] = [p13 p12 p11 p10 p9 0 0 0 0 0 p3 p2 p1 p0] */
        /* [d 0 0 0 0 t9 0 0 0 0 c t3 t2 t1 t0] = [p13 p12 p11 p10 p9 0 0 0 0 0 p3 p2 p1 p0] */

        c = c.wrapping_add(a0 * b4
           + a1 * b3
           + a2 * b2
           + a3 * b1
           + a4 * b0);
        verify_bits!(c, 63);

        /* [d 0 0 0 0 t9 0 0 0 0 c t3 t2 t1 t0] = [p13 p12 p11 p10 p9 0 0 0 0 p4 p3 p2 p1 p0] */
        d = d.wrapping_add(a5 * b9
           + a6 * b8
           + a7 * b7
           + a8 * b6
           + a9 * b5);
        verify_bits!(d, 62);

        /* [d 0 0 0 0 t9 0 0 0 0 c t3 t2 t1 t0] = [p14 p13 p12 p11 p10 p9 0 0 0 0 p4 p3 p2 p1 p0] */
        u4 = d & (M as u64); d >>= 26; c = c.wrapping_add(u4.wrapping_mul(R0 as u64));
        verify_bits!(u4, 26);
        verify_bits!(d, 36);
        /* VERIFY_BITS(c, 64); */
        /* [d u4 0 0 0 0 t9 0 0 0 0 c-u4*R0 t3 t2 t1 t0] = [p14 p13 p12 p11 p10 p9 0 0 0 0 p4 p3 p2 p1 p0] */

        t4 = (c & (M as u64)) as u32; c >>= 26; c = c.wrapping_add(u4.wrapping_mul(R1 as u64));
        verify_bits!(t4, 26);
        verify_bits!(c, 39);
        /* [d u4 0 0 0 0 t9 0 0 0 c-u4*R1 t4-u4*R0 t3 t2 t1 t0] = [p14 p13 p12 p11 p10 p9 0 0 0 0 p4 p3 p2 p1 p0] */
        /* [d 0 0 0 0 0 t9 0 0 0 c t4 t3 t2 t1 t0] = [p14 p13 p12 p11 p10 p9 0 0 0 0 p4 p3 p2 p1 p0] */

        c = c.wrapping_add(a0 * b5
           + a1 * b4
           + a2 * b3
           + a3 * b2
           + a4 * b1
           + a5 * b0);
        verify_bits!(c, 63);
        /* [d 0 0 0 0 0 t9 0 0 0 c t4 t3 t2 t1 t0] = [p14 p13 p12 p11 p10 p9 0 0 0 p5 p4 p3 p2 p1 p0] */

        d = d.wrapping_add(a6 * b9
           + a7 * b8
           + a8 * b7
           + a9 * b6);
        verify_bits!(d, 62);
        /* [d 0 0 0 0 0 t9 0 0 0 c t4 t3 t2 t1 t0] = [p15 p14 p13 p12 p11 p10 p9 0 0 0 p5 p4 p3 p2 p1 p0] */

        u5 = d & (M as u64); d >>= 26; c = c.wrapping_add(u5.wrapping_mul(R0 as u64));
        verify_bits!(u5, 26);
        verify_bits!(d, 36);
        /* VERIFY_BITS(c, 64); */
        /* [d u5 0 0 0 0 0 t9 0 0 0 c-u5*R0 t4 t3 t2 t1 t0] = [p15 p14 p13 p12 p11 p10 p9 0 0 0 p5 p4 p3 p2 p1 p0] */

        t5 = (c & (M as u64)) as u32; c >>= 26; c = c.wrapping_add(u5.wrapping_mul(R1 as u64));
        verify_bits!(t5, 26);
        verify_bits!(c, 39);
        /* [d u5 0 0 0 0 0 t9 0 0 c-u5*R1 t5-u5*R0 t4 t3 t2 t1 t0] = [p15 p14 p13 p12 p11 p10 p9 0 0 0 p5 p4 p3 p2 p1 p0] */
        /* [d 0 0 0 0 0 0 t9 0 0 c t5 t4 t3 t2 t1 t0] = [p15 p14 p13 p12 p11 p10 p9 0 0 0 p5 p4 p3 p2 p1 p0] */

        c = c.wrapping_add(a0 * b6
           + a1 * b5
           + a2 * b4
           + a3 * b3
           + a4 * b2
           + a5 * b1
           + a6 * b0);
        verify_bits!(c, 63);

        /* [d 0 0 0 0 0 0 t9 0 0 c t5 t4 t3 t2 t1 t0] = [p15 p14 p13 p12 p11 p10 p9 0 0 p6 p5 p4 p3 p2 p1 p0] */
        d = d.wrapping_add(a7 * b9
           + a8 * b8
           + a9 * b7);
        verify_bits!(d, 61);

        /* [d 0 0 0 0 0 0 t9 0 0 c t5 t4 t3 t2 t1 t0] = [p16 p15 p14 p13 p12 p11 p10 p9 0 0 p6 p5 p4 p3 p2 p1 p0] */
        u6 = d & (M as u64); d >>= 26; c = c.wrapping_add(u6.wrapping_mul(R0 as u64));
        verify_bits!(u6, 26);
        verify_bits!(d, 35);

        /* VERIFY_BITS(c, 64); */
        /* [d u6 0 0 0 0 0 0 t9 0 0 c-u6*R0 t5 t4 t3 t2 t1 t0] = [p16 p15 p14 p13 p12 p11 p10 p9 0 0 p6 p5 p4 p3 p2 p1 p0] */
        t6 = (c & (M as u64)) as u32; c >>= 26; c = c.wrapping_add(u6.wrapping_mul(R1 as u64));
        verify_bits!(t6, 26);
        verify_bits!(c, 39);
        /* [d u6 0 0 0 0 0 0 t9 0 c-u6*R1 t6-u6*R0 t5 t4 t3 t2 t1 t0] = [p16 p15 p14 p13 p12 p11 p10 p9 0 0 p6 p5 p4 p3 p2 p1 p0] */
        /* [d 0 0 0 0 0 0 0 t9 0 c t6 t5 t4 t3 t2 t1 t0] = [p16 p15 p14 p13 p12 p11 p10 p9 0 0 p6 p5 p4 p3 p2 p1 p0] */

        c = c.wrapping_add(a0 * b7
           + a1 * b6
           + a2 * b5
           + a3 * b4
           + a4 * b3
           + a5 * b2
           + a6 * b1
           + a7 * b0);
        /* VERIFY_BITS(c, 64); */
        verify_check!(c <= 0x8000007C00000007u64);

        /* [d 0 0 0 0 0 0 0 t9 0 c t6 t5 t4 t3 t2 t1 t0] = [p16 p15 p14 p13 p12 p11 p10 p9 0 p7 p6 p5 p4 p3 p2 p1 p0] */
        d = d.wrapping_add(a8 * b9
           + a9 * b8);
        verify_bits!(d, 58);

        /* [d 0 0 0 0 0 0 0 t9 0 c t6 t5 t4 t3 t2 t1 t0] = [p17 p16 p15 p14 p13 p12 p11 p10 p9 0 p7 p6 p5 p4 p3 p2 p1 p0] */
        u7 = d & (M as u64); d >>= 26; c = c.wrapping_add(u7.wrapping_mul(R0 as u64));
        verify_bits!(u7, 26);
        verify_bits!(d, 32);

        /* VERIFY_BITS(c, 64); */
        verify_check!(c <= 0x800001703FFFC2F7u64);

        /* [d u7 0 0 0 0 0 0 0 t9 0 c-u7*R0 t6 t5 t4 t3 t2 t1 t0] = [p17 p16 p15 p14 p13 p12 p11 p10 p9 0 p7 p6 p5 p4 p3 p2 p1 p0] */
        t7 = (c & (M as u64)) as u32; c >>= 26; c = c.wrapping_add(u7.wrapping_mul(R1 as u64));
        verify_bits!(t7, 26);
        verify_bits!(c, 38);
        /* [d u7 0 0 0 0 0 0 0 t9 c-u7*R1 t7-u7*R0 t6 t5 t4 t3 t2 t1 t0] = [p17 p16 p15 p14 p13 p12 p11 p10 p9 0 p7 p6 p5 p4 p3 p2 p1 p0] */
        /* [d 0 0 0 0 0 0 0 0 t9 c t7 t6 t5 t4 t3 t2 t1 t0] = [p17 p16 p15 p14 p13 p12 p11 p10 p9 0 p7 p6 p5 p4 p3 p2 p1 p0] */

        c = c.wrapping_add(a0 * b8
           + a1 * b7
           + a2 * b6
           + a3 * b5
           + a4 * b4
           + a5 * b3
           + a6 * b2
           + a7 * b1
           + a8 * b0);
        /* VERIFY_BITS(c, 64); */
        verify_check!(c <= 0x9000007B80000008u64);

        /* [d 0 0 0 0 0 0 0 0 t9 c t7 t6 t5 t4 t3 t2 t1 t0] = [p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        d = d.wrapping_add(a9 * b9);
        verify_bits!(d, 57);

        /* [d 0 0 0 0 0 0 0 0 t9 c t7 t6 t5 t4 t3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        u8 = d & (M as u64); d >>= 26; c = c.wrapping_add(u8.wrapping_mul(R0 as u64));
        verify_bits!(u8, 26);
        verify_bits!(d, 31);
        /* VERIFY_BITS(c, 64); */
        verify_check!(c <= 0x9000016FBFFFC2F8u64);
        /* [d u8 0 0 0 0 0 0 0 0 t9 c-u8*R0 t7 t6 t5 t4 t3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(3) = t3;
        verify_bits!(*r.add(3), 26);
        /* [d u8 0 0 0 0 0 0 0 0 t9 c-u8*R0 t7 t6 t5 t4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(4) = t4;
        verify_bits!(*r.add(4), 26);
        /* [d u8 0 0 0 0 0 0 0 0 t9 c-u8*R0 t7 t6 t5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(5) = t5;
        verify_bits!(*r.add(5), 26);
        /* [d u8 0 0 0 0 0 0 0 0 t9 c-u8*R0 t7 t6 r5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(6) = t6;
        verify_bits!(*r.add(6), 26);

        /* [d u8 0 0 0 0 0 0 0 0 t9 c-u8*R0 t7 r6 r5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        *r.add(7) = t7;
        verify_bits!(*r.add(7), 26);
        /* [d u8 0 0 0 0 0 0 0 0 t9 c-u8*R0 r7 r6 r5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(8) = (c & (M as u64)) as u32; c >>= 26; c = c.wrapping_add(u8.wrapping_mul(R1 as u64));
        verify_bits!(*r.add(8), 26);
        verify_bits!(c, 39);
        /* [d u8 0 0 0 0 0 0 0 0 t9+c-u8*R1 r8-u8*R0 r7 r6 r5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        /* [d 0 0 0 0 0 0 0 0 0 t9+c r8 r7 r6 r5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        c = c.wrapping_add(d.wrapping_mul(R0 as u64)).wrapping_add(t9 as u64);
        verify_bits!(c, 45);
        /* [d 0 0 0 0 0 0 0 0 0 c-d*R0 r8 r7 r6 r5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(9) = (c & ((M >> 4) as u64)) as u32; c >>= 22; c = c.wrapping_add(d.wrapping_mul((R1 << 4) as u64));
        verify_bits!(*r.add(9), 22);
        verify_bits!(c, 46);
        /* [d 0 0 0 0 0 0 0 0 r9+((c-d*R1<<4)<<22)-d*R0 r8 r7 r6 r5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        /* [d 0 0 0 0 0 0 0 -d*R1 r9+(c<<22)-d*R0 r8 r7 r6 r5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        /* [r9+(c<<22) r8 r7 r6 r5 r4 r3 t2 t1 t0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        d = c.wrapping_mul((R0 >> 4) as u64).wrapping_add(t0 as u64);
        verify_bits!(d, 56);
        /* [r9+(c<<22) r8 r7 r6 r5 r4 r3 t2 t1 d-c*R0>>4] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(0) = (d & (M as u64)) as u32; d >>= 26;
        verify_bits!(*r.add(0), 26);
        verify_bits!(d, 30);
        /* [r9+(c<<22) r8 r7 r6 r5 r4 r3 t2 t1+d r0-c*R0>>4] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        d = d.wrapping_add(c.wrapping_mul((R1 >> 4) as u64)).wrapping_add(t1 as u64);
        verify_bits!(d, 53);
        verify_check!(d <= 0x10000003FFFFBFu64);
        /* [r9+(c<<22) r8 r7 r6 r5 r4 r3 t2 d-c*R1>>4 r0-c*R0>>4] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        /* [r9 r8 r7 r6 r5 r4 r3 t2 d r0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(1) = (d & (M as u64)) as u32; d >>= 26;
        verify_bits!(*r.add(1), 26);
        verify_bits!(d, 27);
        verify_check!(d <= 0x4000000u64);
        /* [r9 r8 r7 r6 r5 r4 r3 t2+d r1 r0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        d = d.wrapping_add(t2 as u64);
        verify_bits!(d, 27);
        /* [r9 r8 r7 r6 r5 r4 r3 d r1 r0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        *r.add(2) = d as u32;
        verify_bits!(*r.add(2), 27);
        /* [r9 r8 r7 r6 r5 r4 r3 r2 r1 r0] = [p18 p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */
    }
}

#[cfg(test)]
mod fe_mul_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_mul_by_zero_is_zero() {
        info!("a*0 == 0");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let z = fe_from_be_bytes_checked(&BYTES_ZERO);

        let got = fe_mul_to_words_le_normalized(&a, &z);
        assert_eq!(got, [0u32; 8]);
    }

    #[traced_test]
    fn fe_mul_by_one_is_identity() {
        info!("a*1 == a");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let one = fe_from_be_bytes_checked(&BYTES_ONE);

        let got = fe_mul_to_words_le_normalized(&a, &one);
        let expected = words_le_from_be_bytes(&BYTES_PATTERN_A);

        debug!(?got, ?expected, "mul by one");
        assert_eq!(got, expected);
    }

    #[traced_test]
    fn fe_mul_commutes_for_representative_values() {
        info!("a*b == b*a");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let b = fe_from_be_bytes_checked(&BYTES_PATTERN_B);

        let ab = fe_mul_to_words_le_normalized(&a, &b);
        let ba = fe_mul_to_words_le_normalized(&b, &a);

        debug!(?ab, ?ba, "commutativity");
        assert_eq!(ab, ba);
    }

    #[traced_test]
    fn fe_mul_matches_reference_mod_p_for_known_vectors() {
        info!("fe_mul should match reference reduction for selected vectors");
        let a_words = words_le_from_be_bytes(&BYTES_PATTERN_A);
        let b_words = words_le_from_be_bytes(&BYTES_PATTERN_B);
        let expected = mul_mod_p(&a_words, &b_words);

        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let b = fe_from_be_bytes_checked(&BYTES_PATTERN_B);
        let got = fe_mul_to_words_le_normalized(&a, &b);

        trace!(?expected, ?got, "reference compare");
        assert_eq!(got, expected);
    }

    #[traced_test]
    fn fe_mul_by_p_minus_one_is_negation_mod_p() {
        info!("a*(p-1) == -a (mod p)");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let pm1 = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);

        let got = fe_mul_to_words_le_normalized(&a, &pm1);

        let a_words = words_le_from_be_bytes(&BYTES_PATTERN_A);
        let expected = neg_mod_p(&a_words);

        debug!(?got, ?expected, "mul by p-1");
        assert_eq!(got, expected);
    }

    #[traced_test]
    fn fe_mul_does_not_mutate_b_input_even_though_signature_is_mut_ptr() {
        info!("fe_mul should not mutate its b input");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let mut b = fe_from_be_bytes_checked(&BYTES_PATTERN_B);
        let mut b_before = fe_clone_value(&b);

        let mut r = Fe10x26::new();
        unsafe { fe_mul(&mut r as *mut Fe10x26, &a as *const Fe10x26, &mut b as *mut Fe10x26) };

        let out_b_before = fe_to_be_bytes_normalized(&mut b_before);
        let out_b_after = fe_to_be_bytes_normalized(&mut b);

        assert_eq!(out_b_after, out_b_before);
        assert_eq!(out_b_after, BYTES_PATTERN_B);
    }

    #[traced_test]
    fn fe_mul_distributes_over_addition_for_small_scalars() {
        info!("a*(b+c) == a*b + a*c (mod p) for representative vectors");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let b = fe_from_be_bytes_checked(&BYTES_THREE);
        let c = fe_from_be_bytes_checked(&BYTES_FIVE);

        let mut bc = fe_clone_value(&b);
        fe_add_in_place(&mut bc, &c);
        let bc_norm_words = fe_to_words_le_normalized(&mut bc);

        let left = fe_mul_to_words_le_normalized(&a, &bc);

        let ab = fe_mul_to_words_le_normalized(&a, &b);
        let ac = fe_mul_to_words_le_normalized(&a, &c);
        let right = add_mod_p(&ab, &ac);

        debug!(?bc_norm_words, ?left, ?right, "distributivity check");
        assert_eq!(left, right);
    }

    #[traced_test]
    fn fe_mul_sets_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_mul sets magnitude=1 normalized=0");
        let a = fe_from_be_bytes_checked(&BYTES_THREE);
        let mut b = fe_from_be_bytes_checked(&BYTES_FIVE);

        let mut r = Fe10x26::new();
        unsafe { fe_mul(&mut r as *mut Fe10x26, &a as *const Fe10x26, &mut b as *mut Fe10x26) };

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, 1);
            assert_eq!(r.normalized, 0);
        }
    }
}

#[cfg(test)]
mod fe_mul_magnitude_eight_boundary_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn fe_mul_accepts_inputs_with_effective_magnitude_eight_and_matches_reference() {
        info!("constructing an input via repeated doubling (magnitude 8 in verify builds) and comparing fe_mul output to reference");
        let base = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let b = fe_from_be_bytes_checked(&BYTES_PATTERN_B);

        let mut x = fe_clone_value(&base);
        let xptr = &mut x as *mut Fe10x26;

        unsafe { fe_add(xptr, xptr as *const Fe10x26) }; // *2
        unsafe { fe_add(xptr, xptr as *const Fe10x26) }; // *4
        unsafe { fe_add(xptr, xptr as *const Fe10x26) }; // *8

        #[cfg(feature = "secp256k1-verify")]
        {
            debug!(magnitude = x.magnitude, normalized = x.normalized, "constructed x");
            assert_eq!(x.magnitude, 8);
            assert_eq!(x.normalized, 0);
        }

        let mut x_norm = fe_clone_value(&x);
        let x_words = fe_to_words_le_normalized(&mut x_norm);

        let mut b_norm = fe_clone_value(&b);
        let b_words = fe_to_words_le_normalized(&mut b_norm);

        let expected = mul_mod_p(&x_words, &b_words);
        let got = fe_mul_to_words_le_normalized(&x, &b);

        trace!(x0 = x_words[0], b0 = b_words[0], got0 = got[0], exp0 = expected[0], "boundary mul snapshot");
        assert_eq!(got, expected);
    }
}
