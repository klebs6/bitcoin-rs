// ---------------- [ File: bitcoinsecp256k1-group/src/ge_const.rs ]
crate::ix!();

#[macro_export]
macro_rules! ge_const {
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
        Ge {
            x: fe_const!(($a), ($b), ($c), ($d), ($e), ($f), ($g), ($h)),
            y: fe_const!(($i), ($j), ($k), ($l), ($m), ($n), ($o), ($p)),
            infinity: 0,
        }
    };
}

#[macro_export]
macro_rules! ge_const_infinity {
    () => {
        Ge {
            x: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            y: fe_const!(0, 0, 0, 0, 0, 0, 0, 0),
            infinity: 1,
        }
    };
}

#[cfg(test)]
mod ge_const_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_const_macro_sets_expected_x_y_and_infinity_zero() {
        tracing::info!("Validating ge_const! macro populates coordinates and sets infinity=0.");

        let p: Ge = ge_const!(
            0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 2
        );

        assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);

        unsafe {
            let expected_x: Fe = crate::fe_const!(0, 0, 0, 0, 0, 0, 0, 1);
            let expected_y: Fe = crate::fe_const!(0, 0, 0, 0, 0, 0, 0, 2);

            assert!(
                fe_equal_var(core::ptr::addr_of!(p.x), core::ptr::addr_of!(expected_x)) != 0
            );
            assert!(
                fe_equal_var(core::ptr::addr_of!(p.y), core::ptr::addr_of!(expected_y)) != 0
            );
        }
    }

    #[traced_test]
    fn ge_const_infinity_macro_sets_infinity_and_zero_coordinates() {
        tracing::info!("Validating ge_const_infinity! macro sets infinity=1 with x=y=0.");

        let inf: Ge = ge_const_infinity!();
        assert!(ge_is_infinity(core::ptr::addr_of!(inf)) != 0);

        assert!(fe_is_zero(core::ptr::addr_of!(inf.x)) != 0);
        assert!(fe_is_zero(core::ptr::addr_of!(inf.y)) != 0);
    }
}
