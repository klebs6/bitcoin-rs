// ---------------- [ File: bitcoinsecp256k1-group/src/gej_const.rs ]
crate::ix!();

#[macro_export]
macro_rules! gej_const {
    ($a:expr,
     $b:expr,
     $c:expr,
     $d:expr,
     $e:expr,
     $f:expr,
     $g:expr,
     $h:expr,
     $i:expr,
     $j:expr,
     $k:expr,
     $l:expr,
     $m:expr,
     $n:expr,
     $o:expr,
     $p:expr) => {
        Gej {
            x: fe_const!(($a), ($b), ($c), ($d), ($e), ($f), ($g), ($h)),
            y: fe_const!(($i), ($j), ($k), ($l), ($m), ($n), ($o), ($p)),
            z: fe_const!(0, 0, 0, 0, 0, 0, 0, 1),
            infinity: 0,
        }
    };
}

#[macro_export]
macro_rules! gej_const_infinity {
    () => {
        Gej {
            x: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            y: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            z: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            infinity: 1,
        }
    };
}

#[cfg(test)]
mod gej_const_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_const_macro_sets_expected_xy_unit_z_and_infinity_zero() {
        tracing::info!("Validating gej_const! macro sets x/y, z=1, infinity=0.");

        let p: Gej = gej_const!(
            0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 2
        );

        assert!(gej_is_infinity(core::ptr::addr_of!(p)) == 0);

        unsafe {
            let expected_x: Fe = crate::fe_const!(0, 0, 0, 0, 0, 0, 0, 1);
            let expected_y: Fe = crate::fe_const!(0, 0, 0, 0, 0, 0, 0, 2);
            let expected_z: Fe = secp256k1_group_exhaustive_test_support::fe_int(1);

            assert!(
                fe_equal_var(core::ptr::addr_of!(p.x), core::ptr::addr_of!(expected_x)) != 0
            );
            assert!(
                fe_equal_var(core::ptr::addr_of!(p.y), core::ptr::addr_of!(expected_y)) != 0
            );
            assert!(
                fe_equal_var(core::ptr::addr_of!(p.z), core::ptr::addr_of!(expected_z)) != 0
            );
        }
    }

    #[traced_test]
    fn gej_const_infinity_macro_sets_infinity_and_zero_coordinates() {
        tracing::info!("Validating gej_const_infinity! macro sets infinity=1 and zero xyz.");

        let inf: Gej = gej_const_infinity!();
        assert!(gej_is_infinity(core::ptr::addr_of!(inf)) != 0);

        assert!(fe_is_zero(core::ptr::addr_of!(inf.x)) != 0);
        assert!(fe_is_zero(core::ptr::addr_of!(inf.y)) != 0);
        assert!(fe_is_zero(core::ptr::addr_of!(inf.z)) != 0);
    }
}
