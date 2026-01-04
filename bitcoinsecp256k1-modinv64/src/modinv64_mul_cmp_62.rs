// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_mul_cmp_62.rs ]
crate::ix!();

/// Return -1 for a<b*factor, 0 for a==b*factor,
/// 1 for a>b*factor. A has alen limbs; b has 5.
/// 
#[cfg(VERIFY)]
pub fn modinv64_mul_cmp_62(
        a:      *const ModInv64Signed62,
        alen:   i32,
        b:      *const ModInv64Signed62,
        factor: i64) -> i32 {

    let mut am = ModInv64Signed62::from_limbs([0, 0, 0, 0, 0]);
    let mut bm = ModInv64Signed62::from_limbs([0, 0, 0, 0, 0]);
    let mut i: i32;

    modinv64_mul_62(&mut am as *mut _, a, alen, 1); /* Normalize all but the top limb of a. */
    modinv64_mul_62(&mut bm as *mut _, b, 5, factor);

    i = 0;
    while i < 4 {
        /* Verify that all but the top limb of a and b are normalized. */
        verify_check!(am.v()[i as usize] >> 62 == 0);
        verify_check!(bm.v()[i as usize] >> 62 == 0);
        i += 1;
    }
    i = 4;
    loop {
        if am.v()[i as usize] < bm.v()[i as usize] { return -1; }
        if am.v()[i as usize] > bm.v()[i as usize] { return 1; }
        if i == 0 { break; }
        i -= 1;
    }
    0
}

#[cfg(all(test, VERIFY))]
mod modinv64_mul_cmp_62_contract {
    use super::*;

    #[traced_test]
    fn mul_cmp_62_matches_small_integer_reference_ordering() {
        let mut seed: u64 = 0x0BAD_F00D_DEAD_BEEF;
        let base: i128 = 1i128 << LIMB_BITS;

        let mut i: usize = 0;
        while i < 512 {
            let a0: i128 = (splitmix128_next(&mut seed) & 0x7FFF_FFFF) as i128;
            let a1: i128 = (splitmix128_next(&mut seed) & 0x000F_FFFF) as i128;
            let b0: i128 = (splitmix128_next(&mut seed) & 0x7FFF_FFFF) as i128;
            let b1: i128 = (splitmix128_next(&mut seed) & 0x000F_FFFF) as i128;

            let alen: i32 = if (splitmix128_next(&mut seed) & 1) == 0 { 1 } else { 2 };
            let a_val: i128 = if alen == 1 { a0 } else { a0 + a1 * base };
            let b_val: i128 = b0 + b1 * base; /* always 2 limbs in this test */

            let factor: i64 = ((splitmix128_next(&mut seed) as i64) % 31) - 15;

            let a = signed62_from_i128(a_val);
            let b = signed62_from_i128(b_val);

            trace!(iter = i, a_val = a_val, b_val = b_val, alen = alen, factor = factor);

            let cmp = modinv64_mul_cmp_62(&a as *const _, alen, &b as *const _, factor);

            let rhs: i128 = b_val * (factor as i128);
            let expected = if a_val < rhs { -1 } else if a_val > rhs { 1 } else { 0 };

            trace!(iter = i, cmp = cmp, expected = expected);
            assert!(cmp == expected);

            i += 1;
        }
    }
}
