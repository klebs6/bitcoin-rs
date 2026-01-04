// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_cmp_var.rs ]
crate::ix!();

pub fn fe_cmp_var(
    a: *const Fe10x26,
    b: *const Fe10x26) -> i32 {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).normalized != 0);
            verify_check!((*b).normalized != 0);
            fe_verify(a);
            fe_verify(b);
        }

        for i in (0..10).rev() {
            if (*a).n[i] > (*b).n[i] {
                return 1;
            }
            if (*a).n[i] < (*b).n[i] {
                return -1;
            }
        }

        0
    }
}

#[cfg(test)]
mod fe_cmp_var_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_cmp_var_equal_returns_zero() {
        info!("fe_cmp_var(a,a) == 0");
        let a = fe_from_be_bytes_checked(&BYTES_PATTERN_A);
        let cmp = unsafe { fe_cmp_var(&a as *const Fe10x26, &a as *const Fe10x26) };
        debug!(cmp, "cmp result");
        assert_eq!(cmp, 0);
    }

    #[traced_test]
    fn fe_cmp_var_orders_small_values_correctly() {
        info!("fe_cmp_var should order 0 < 1 < 2 < ...");
        let z = fe_from_be_bytes_checked(&BYTES_ZERO);
        let o = fe_from_be_bytes_checked(&BYTES_ONE);
        let t = fe_from_be_bytes_checked(&BYTES_TWO);

        assert_eq!(unsafe { fe_cmp_var(&z, &o) }, -1);
        assert_eq!(unsafe { fe_cmp_var(&o, &z) }, 1);

        assert_eq!(unsafe { fe_cmp_var(&o, &t) }, -1);
        assert_eq!(unsafe { fe_cmp_var(&t, &o) }, 1);
    }

    #[traced_test]
    fn fe_cmp_var_orders_near_prime_boundary() {
        info!("fe_cmp_var should order (p-2) < (p-1) and 1 < (p-1)");
        let pm2 = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_TWO_BYTES_BE);
        let pm1 = fe_from_be_bytes_checked(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
        let one = fe_from_be_bytes_checked(&BYTES_ONE);

        assert_eq!(unsafe { fe_cmp_var(&pm2, &pm1) }, -1);
        assert_eq!(unsafe { fe_cmp_var(&pm1, &pm2) }, 1);
        assert_eq!(unsafe { fe_cmp_var(&one, &pm1) }, -1);
        assert_eq!(unsafe { fe_cmp_var(&pm1, &one) }, 1);
    }
}
