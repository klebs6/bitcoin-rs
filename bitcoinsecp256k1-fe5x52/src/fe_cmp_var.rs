// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_cmp_var.rs ]
crate::ix!();

pub fn fe_cmp_var(
        a: *const Fe5x52,
        b: *const Fe5x52) -> i32 {

    unsafe {
        let mut i: i32;

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).normalized != 0);
            verify_check!((*b).normalized != 0);
            fe_verify(a);
            fe_verify(b);
        }

        i = 4;
        while i >= 0 {
            let idx = i as usize;
            if (*a).n[idx] > (*b).n[idx] {
                return 1;
            }
            if (*a).n[idx] < (*b).n[idx] {
                return -1;
            }
            i -= 1;
        }

        0
    }
}
