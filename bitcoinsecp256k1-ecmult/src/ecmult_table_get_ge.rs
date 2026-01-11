// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_table_get_ge.rs ]
crate::ix!();

/// The following two macro retrieves a particular odd multiple from a table of precomputed
/// multiples.
///
#[macro_export]
macro_rules! ecmult_table_get_ge {
    ($r:expr,
     $pre:expr,
     $n:expr,
     $w:expr) => {
        unsafe {
            let n: i32 = $n;
            let w: i32 = $w as i32;

            verify_check!(((n) & 1) == 1);
            verify_check!((n) >= -(((1i32) << ((w) - 1)) - 1));
            verify_check!((n) <=  (((1i32) << ((w) - 1)) - 1));

            if (n) > 0 {
                *($r) = *($pre).add(((n) - 1) as usize / 2usize);
            } else {
                *($r) = *($pre).add(((-(n)) - 1) as usize / 2usize);
                fe_negate(
                    ge_y_mut($r),
                    ge_y($r),
                    1
                );
            }
        }

        /*
        do { 
            VERIFY_CHECK(((n) & 1) == 1); 
            VERIFY_CHECK((n) >= -((1 << ((w)-1)) - 1)); 
            VERIFY_CHECK((n) <=  ((1 << ((w)-1)) - 1)); 
            if ((n) > 0) { 
                *(r) = (pre)[((n)-1)/2]; 
            } else { 
                *(r) = (pre)[(-(n)-1)/2]; 
                fe_negate(&((r)->y), &((r)->y), 1); 
            } 
        } while(0)
        */

    };
}

#[macro_export]
macro_rules! ecmult_table_get_ge_storage {
    ($r:expr,
     $pre:expr,
     $n:expr,
     $w:expr) => {
        unsafe {
            let n: i32 = $n;
            let w: i32 = $w as i32;

            verify_check!(((n) & 1) == 1);
            verify_check!((n) >= -(((1i32) << ((w) - 1)) - 1));
            verify_check!((n) <=  (((1i32) << ((w) - 1)) - 1));

            if (n) > 0 {
                ge_from_storage(($r), ($pre).add(((n) - 1) as usize / 2usize));
            } else {
                ge_from_storage(($r), ($pre).add(((-(n)) - 1) as usize / 2usize));
                fe_negate(
                    ge_y_mut($r),
                    ge_y($r),
                    1
                );
            }
        }
        /*
        do { 
            VERIFY_CHECK(((n) & 1) == 1); 
            VERIFY_CHECK((n) >= -((1 << ((w)-1)) - 1)); 
            VERIFY_CHECK((n) <=  ((1 << ((w)-1)) - 1)); 
            if ((n) > 0) { 
                ge_from_storage((r), &(pre)[((n)-1)/2]); 
            } else { 
                ge_from_storage((r), &(pre)[(-(n)-1)/2]); 
                fe_negate(&((r)->y), &((r)->y), 1); 
            } 
        } while(0)
        */

    };
}
