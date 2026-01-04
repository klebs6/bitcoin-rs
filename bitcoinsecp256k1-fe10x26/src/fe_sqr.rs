// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_sqr.rs ]
crate::ix!();

pub fn fe_sqr(
    r: *mut Fe10x26,
    a: *const Fe10x26)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).magnitude <= 8);
            fe_verify(a);
        }

        fe_sqr_inner((*r).n.as_mut_ptr(), (*a).n.as_ptr());

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 0;
            fe_verify(r);
        }
    }
}

#[cfg(feature="secp256k1-use-external-asm")]
pub fn fe_sqr_inner(
    r: *mut u32,
    a: *const u32)  {

    extern "C" {
        fn secp256k1_fe_sqr_inner(r: *mut u32, a: *const u32);
    }

    unsafe {
        secp256k1_fe_sqr_inner(r, a);
    }
}

#[cfg(not(feature="secp256k1-use-external-asm"))]
#[inline] pub fn fe_sqr_inner(
    r: *mut u32,
    a: *const u32)  {

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
        let mut t0: u32;
        let mut t1: u32;
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

        /* [... a b c] is a shorthand for ... + a<<52 + b<<26 + c<<0 mod n.
         *  px is a shorthand for sum(a[i]*a[x-i], i=0..x).
         *  Note that [x 0 0 0 0 0 0 0 0 0 0] = [x*R1 x*R0].
         */

        d  = (u64::from((*a.add(0)).wrapping_mul(2))) * u64::from(*a.add(9))
           + (u64::from((*a.add(1)).wrapping_mul(2))) * u64::from(*a.add(8))
           + (u64::from((*a.add(2)).wrapping_mul(2))) * u64::from(*a.add(7))
           + (u64::from((*a.add(3)).wrapping_mul(2))) * u64::from(*a.add(6))
           + (u64::from((*a.add(4)).wrapping_mul(2))) * u64::from(*a.add(5));
        /* VERIFY_BITS(d, 64); */
        /* [d 0 0 0 0 0 0 0 0 0] = [p9 0 0 0 0 0 0 0 0 0] */
        t9 = (d & (M as u64)) as u32; d >>= 26;
        verify_bits!(t9, 26);
        verify_bits!(d, 38);
        /* [d t9 0 0 0 0 0 0 0 0 0] = [p9 0 0 0 0 0 0 0 0 0] */

        c  = u64::from(*a.add(0)) * u64::from(*a.add(0));
        verify_bits!(c, 60);
        /* [d t9 0 0 0 0 0 0 0 0 c] = [p9 0 0 0 0 0 0 0 0 p0] */
        d = d.wrapping_add(
            (u64::from((*a.add(1)).wrapping_mul(2))) * u64::from(*a.add(9))
          + (u64::from((*a.add(2)).wrapping_mul(2))) * u64::from(*a.add(8))
          + (u64::from((*a.add(3)).wrapping_mul(2))) * u64::from(*a.add(7))
          + (u64::from((*a.add(4)).wrapping_mul(2))) * u64::from(*a.add(6))
          + (u64::from(*a.add(5))) * u64::from(*a.add(5))
        );
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

        c = c.wrapping_add((u64::from((*a.add(0)).wrapping_mul(2))) * u64::from(*a.add(1)));
        verify_bits!(c, 62);
        /* [d 0 t9 0 0 0 0 0 0 0 c t0] = [p10 p9 0 0 0 0 0 0 0 p1 p0] */
        d = d.wrapping_add(
            (u64::from((*a.add(2)).wrapping_mul(2))) * u64::from(*a.add(9))
          + (u64::from((*a.add(3)).wrapping_mul(2))) * u64::from(*a.add(8))
          + (u64::from((*a.add(4)).wrapping_mul(2))) * u64::from(*a.add(7))
          + (u64::from((*a.add(5)).wrapping_mul(2))) * u64::from(*a.add(6))
        );
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

        c = c.wrapping_add(
            (u64::from((*a.add(0)).wrapping_mul(2))) * u64::from(*a.add(2))
          + (u64::from(*a.add(1))) * u64::from(*a.add(1))
        );
        verify_bits!(c, 62);
        /* [d 0 0 t9 0 0 0 0 0 0 c t1 t0] = [p11 p10 p9 0 0 0 0 0 0 p2 p1 p0] */
        d = d.wrapping_add(
            (u64::from((*a.add(3)).wrapping_mul(2))) * u64::from(*a.add(9))
          + (u64::from((*a.add(4)).wrapping_mul(2))) * u64::from(*a.add(8))
          + (u64::from((*a.add(5)).wrapping_mul(2))) * u64::from(*a.add(7))
          + (u64::from(*a.add(6))) * u64::from(*a.add(6))
        );
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

        c = c.wrapping_add(
            (u64::from((*a.add(0)).wrapping_mul(2))) * u64::from(*a.add(3))
          + (u64::from((*a.add(1)).wrapping_mul(2))) * u64::from(*a.add(2))
        );
        verify_bits!(c, 63);
        /* [d 0 0 0 t9 0 0 0 0 0 c t2 t1 t0] = [p12 p11 p10 p9 0 0 0 0 0 p3 p2 p1 p0] */
        d = d.wrapping_add(
            (u64::from((*a.add(4)).wrapping_mul(2))) * u64::from(*a.add(9))
          + (u64::from((*a.add(5)).wrapping_mul(2))) * u64::from(*a.add(8))
          + (u64::from((*a.add(6)).wrapping_mul(2))) * u64::from(*a.add(7))
        );
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

        c = c.wrapping_add(
            (u64::from((*a.add(0)).wrapping_mul(2))) * u64::from(*a.add(4))
          + (u64::from((*a.add(1)).wrapping_mul(2))) * u64::from(*a.add(3))
          + (u64::from(*a.add(2))) * u64::from(*a.add(2))
        );
        verify_bits!(c, 63);
        /* [d 0 0 0 0 t9 0 0 0 0 c t3 t2 t1 t0] = [p13 p12 p11 p10 p9 0 0 0 0 p4 p3 p2 p1 p0] */
        d = d.wrapping_add(
            (u64::from((*a.add(5)).wrapping_mul(2))) * u64::from(*a.add(9))
          + (u64::from((*a.add(6)).wrapping_mul(2))) * u64::from(*a.add(8))
          + (u64::from(*a.add(7))) * u64::from(*a.add(7))
        );
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

        c = c.wrapping_add(
            (u64::from((*a.add(0)).wrapping_mul(2))) * u64::from(*a.add(5))
          + (u64::from((*a.add(1)).wrapping_mul(2))) * u64::from(*a.add(4))
          + (u64::from((*a.add(2)).wrapping_mul(2))) * u64::from(*a.add(3))
        );
        verify_bits!(c, 63);
        /* [d 0 0 0 0 0 t9 0 0 0 c t4 t3 t2 t1 t0] = [p14 p13 p12 p11 p10 p9 0 0 0 p5 p4 p3 p2 p1 p0] */
        d = d.wrapping_add(
            (u64::from((*a.add(6)).wrapping_mul(2))) * u64::from(*a.add(9))
          + (u64::from((*a.add(7)).wrapping_mul(2))) * u64::from(*a.add(8))
        );
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

        c = c.wrapping_add(
            (u64::from((*a.add(0)).wrapping_mul(2))) * u64::from(*a.add(6))
          + (u64::from((*a.add(1)).wrapping_mul(2))) * u64::from(*a.add(5))
          + (u64::from((*a.add(2)).wrapping_mul(2))) * u64::from(*a.add(4))
          + (u64::from(*a.add(3))) * u64::from(*a.add(3))
        );
        verify_bits!(c, 63);
        /* [d 0 0 0 0 0 0 t9 0 0 c t5 t4 t3 t2 t1 t0] = [p15 p14 p13 p12 p11 p10 p9 0 0 p6 p5 p4 p3 p2 p1 p0] */
        d = d.wrapping_add(
            (u64::from((*a.add(7)).wrapping_mul(2))) * u64::from(*a.add(9))
          + (u64::from(*a.add(8))) * u64::from(*a.add(8))
        );
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

        c = c.wrapping_add(
            (u64::from((*a.add(0)).wrapping_mul(2))) * u64::from(*a.add(7))
          + (u64::from((*a.add(1)).wrapping_mul(2))) * u64::from(*a.add(6))
          + (u64::from((*a.add(2)).wrapping_mul(2))) * u64::from(*a.add(5))
          + (u64::from((*a.add(3)).wrapping_mul(2))) * u64::from(*a.add(4))
        );
        /* VERIFY_BITS(c, 64); */
        verify_check!(c <= 0x8000007C00000007u64);
        /* [d 0 0 0 0 0 0 0 t9 0 c t6 t5 t4 t3 t2 t1 t0] = [p16 p15 p14 p13 p12 p11 p10 p9 0 p7 p6 p5 p4 p3 p2 p1 p0] */
        d = d.wrapping_add((u64::from((*a.add(8)).wrapping_mul(2))) * u64::from(*a.add(9)));
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

        c = c.wrapping_add(
            (u64::from((*a.add(0)).wrapping_mul(2))) * u64::from(*a.add(8))
          + (u64::from((*a.add(1)).wrapping_mul(2))) * u64::from(*a.add(7))
          + (u64::from((*a.add(2)).wrapping_mul(2))) * u64::from(*a.add(6))
          + (u64::from((*a.add(3)).wrapping_mul(2))) * u64::from(*a.add(5))
          + (u64::from(*a.add(4))) * u64::from(*a.add(4))
        );
        /* VERIFY_BITS(c, 64); */
        verify_check!(c <= 0x9000007B80000008u64);
        /* [d 0 0 0 0 0 0 0 0 t9 c t7 t6 t5 t4 t3 t2 t1 t0] = [p17 p16 p15 p14 p13 p12 p11 p10 p9 p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        d = d.wrapping_add(u64::from(*a.add(9)) * u64::from(*a.add(9)));
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

        d    = c.wrapping_mul((R0 >> 4) as u64).wrapping_add(t0 as u64);
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
mod fe_sqr_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn fe_sqr_of_zero_is_zero() {
        info!("0^2 == 0");
        let a = fe_from_be_bytes_checked(&BYTES_ZERO);
        let got = fe_sqr_to_words_le_normalized(&a);
        assert_eq!(got, [0u32; 8]);
    }

    #[traced_test]
    fn fe_sqr_of_one_is_one() {
        info!("1^2 == 1");
        let a = fe_from_be_bytes_checked(&BYTES_ONE);
        let got = fe_sqr_to_words_le_normalized(&a);
        assert_eq!(got, words_le_from_be_bytes(&BYTES_ONE));
    }

    #[traced_test]
    fn fe_sqr_of_p_minus_one_is_one() {
        info!("(p-1)^2 == 1");
        let a = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let got = fe_sqr_to_words_le_normalized(&a);
        assert_eq!(got, words_le_from_be_bytes(&BYTES_ONE));
    }

    #[traced_test]
    fn fe_sqr_matches_fe_mul_self_and_reference() {
        info!("fe_sqr(a) should equal fe_mul(a,a) and reference sqr_mod_p");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

        let got_sqr = fe_sqr_to_words_le_normalized(&a);
        let got_mul = fe_mul_to_words_le_normalized(&a, &a);

        let a_words = words_le_from_be_bytes(&BYTES_PATTERN_A);
        let expected = sqr_mod_p(&a_words);

        debug!(?got_sqr, ?got_mul, ?expected, "sqr comparisons");
        assert_eq!(got_sqr, got_mul);
        assert_eq!(got_sqr, expected);
    }

    #[traced_test]
    fn fe_sqr_sets_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_sqr sets magnitude=1 normalized=0");
        let a = fe_from_be_bytes_checked(&BYTES_THREE);
        let mut r = Fe10x26::new();
        unsafe { fe_sqr(&mut r as *mut Fe10x26, &a as *const Fe10x26) };

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(r.magnitude, 1);
            assert_eq!(r.normalized, 0);
        }
    }
}

#[cfg(test)]
mod fe_sqr_binomial_expansion_identity_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info, trace};

    fn compute_square_bytes(x: &Fe10x26) -> [u8; 32] {
        let mut r = Fe10x26::new();
        unsafe { fe_sqr(&mut r as *mut Fe10x26, x as *const Fe10x26) };
        fe_to_be_bytes_normalized(&mut r)
    }

    fn compute_mul_bytes(a: &Fe10x26, b: &Fe10x26) -> [u8; 32] {
        let mut r = Fe10x26::new();
        let mut b_mut = fe_clone_value(b);
        unsafe { fe_mul(&mut r as *mut Fe10x26, a as *const Fe10x26, &mut b_mut as *mut Fe10x26) };
        fe_to_be_bytes_normalized(&mut r)
    }

    #[traced_test]
    fn binomial_expansion_matches_for_representative_pairs() {
        info!("checking (a+b)^2 == a^2 + 2ab + b^2 (mod p) for representative pairs");
        let pairs: [(&[u8; 32], &[u8; 32]); 3] = [
            (&BYTES_PATTERN_A, &BYTES_PATTERN_B),
            (&BYTES_THREE, &BYTES_FIVE),
            (&FIELD_PRIME_MINUS_ONE_BYTES_BE, &BYTES_TWO),
        ];

        for (idx, (a_bytes, b_bytes)) in pairs.iter().enumerate() {
            let a = fe_from_be_bytes_checked(a_bytes);
            let b = fe_from_be_bytes_checked(b_bytes);

            // left = (a+b)^2
            let mut a_plus_b = fe_clone_value(&a);
            fe_add_in_place(&mut a_plus_b, &b);
            let left = compute_square_bytes(&a_plus_b);

            // right = a^2 + 2ab + b^2
            let a2 = compute_square_bytes(&a);
            let b2 = compute_square_bytes(&b);

            let mut ab = Fe10x26::new();
            let mut b_mut = fe_clone_value(&b);
            unsafe { fe_mul(&mut ab as *mut Fe10x26, &a as *const Fe10x26, &mut b_mut as *mut Fe10x26) };

            let ab_copy = fe_clone_value(&ab);
            fe_add_in_place(&mut ab, &ab_copy); // 2ab

            let mut right_acc = fe_from_be_bytes_checked(&a2);
            let two_ab_bytes = fe_to_be_bytes_normalized(&mut ab);
            let two_ab_fe = fe_from_be_bytes_checked(&two_ab_bytes);
            fe_add_in_place(&mut right_acc, &two_ab_fe);

            let b2_fe = fe_from_be_bytes_checked(&b2);
            fe_add_in_place(&mut right_acc, &b2_fe);

            let right = fe_to_be_bytes_normalized(&mut right_acc);

            trace!(pair_index = idx, ?left, ?right, "binomial identity comparison");
            assert_eq!(left, right);
        }
    }
}

#[cfg(test)]
mod fe_sqr_magnitude_eight_boundary_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn fe_sqr_accepts_inputs_with_effective_magnitude_eight_and_matches_reference() {
        info!("constructing an input via repeated doubling (magnitude 8 in verify builds) and comparing fe_sqr output to reference");
        let base = fe_from_be_bytes_checked(&BYTES_PATTERN_A);

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
        let expected = sqr_mod_p(&x_words);

        let got = fe_sqr_to_words_le_normalized(&x);

        trace!(x0 = x_words[0], got0 = got[0], exp0 = expected[0], "boundary square snapshot");
        assert_eq!(got, expected);
    }
}
