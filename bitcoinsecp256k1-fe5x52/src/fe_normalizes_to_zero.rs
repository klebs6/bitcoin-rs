// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_normalizes_to_zero.rs ]
crate::ix!();

pub fn fe_normalizes_to_zero(r: *const Fe5x52) -> i32 {

    unsafe {
        let mut t0: u64 = (*r).n[0];
        let mut t1: u64 = (*r).n[1];
        let mut t2: u64 = (*r).n[2];
        let mut t3: u64 = (*r).n[3];
        let mut t4: u64 = (*r).n[4];

        /* z0 tracks a possible raw value of 0, z1 tracks a possible raw value of P */
        let mut z0: u64;
        let mut z1: u64;

        /* Reduce t4 at the start so there will be at most a single carry from the first pass */
        let x: u64 = t4 >> 48;
        t4 &= 0x0FFFFFFFFFFFF_u64;

        /* The first pass ensures the magnitude is 1, ... */
        t0 = t0.wrapping_add(x.wrapping_mul(0x1000003D1_u64));
        t1 = t1.wrapping_add(t0 >> 52); t0 &= 0xFFFFFFFFFFFFF_u64;
        z0 = t0;
        z1 = t0 ^ 0x1000003D0_u64;
        t2 = t2.wrapping_add(t1 >> 52); t1 &= 0xFFFFFFFFFFFFF_u64; z0 |= t1; z1 &= t1;
        t3 = t3.wrapping_add(t2 >> 52); t2 &= 0xFFFFFFFFFFFFF_u64; z0 |= t2; z1 &= t2;
        t4 = t4.wrapping_add(t3 >> 52); t3 &= 0xFFFFFFFFFFFFF_u64; z0 |= t3; z1 &= t3;
        z0 |= t4; z1 &= t4 ^ 0xF000000000000_u64;

        /* ... except for a possible carry at bit 48 of t4 (i.e. bit 256 of the field element) */
        verify_check!((t4 >> 49) == 0);

        (((z0 == 0) | (z1 == 0xFFFFFFFFFFFFF_u64)) as i32)
    }
}

#[cfg(test)]
mod fe_normalizes_to_zero_rs_exhaustive_tests {
    use super::*;

    const P0: u64 = 0xFFFFEFFFFFC2F_u64;
    const P1: u64 = 0xFFFFFFFFFFFFF_u64;
    const P2: u64 = 0xFFFFFFFFFFFFF_u64;
    const P3: u64 = 0xFFFFFFFFFFFFF_u64;
    const P4: u64 = 0x0FFFFFFFFFFFF_u64;

    #[traced_test]
    fn fe_normalizes_to_zero_identifies_zero_and_field_characteristic_and_multiples() {
        tracing::info!("testing fe_normalizes_to_zero on raw 0, raw p, and 2p, plus nonzero near them");

        unsafe {
            let mut z = Fe5x52::new();
            z.n = [0, 0, 0, 0, 0];
            assert_eq!(crate::fe_normalizes_to_zero(&z as *const Fe5x52), 1);

            let mut p = Fe5x52::new();
            p.n[0] = P0; p.n[1] = P1; p.n[2] = P2; p.n[3] = P3; p.n[4] = P4;
            assert_eq!(crate::fe_normalizes_to_zero(&p as *const Fe5x52), 1);

            let mut p1 = p;
            p1.n[0] = p1.n[0].wrapping_add(1);
            assert_eq!(crate::fe_normalizes_to_zero(&p1 as *const Fe5x52), 0);

            let mut two_p = p;
            crate::fe_add(&mut two_p as *mut Fe5x52, &p as *const Fe5x52);
            assert_eq!(crate::fe_normalizes_to_zero(&two_p as *const Fe5x52), 1);

            let mut two_p_plus_one = two_p;
            two_p_plus_one.n[0] = two_p_plus_one.n[0].wrapping_add(1);
            assert_eq!(crate::fe_normalizes_to_zero(&two_p_plus_one as *const Fe5x52), 0);

            let mut with_2pow256 = p;
            with_2pow256.n[4] = with_2pow256.n[4].wrapping_add(1u64 << 48);
            assert_eq!(crate::fe_normalizes_to_zero(&with_2pow256 as *const Fe5x52), 0);
        }
    }
}
