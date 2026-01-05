// ---------------- [ File: bitcoinsecp256k1-ecmultconst/src/ecmult_const_table_get_ge.rs ]
crate::ix!();

/**
  | This is like `ECMULT_TABLE_GET_GE`
  | but is constant time
  |
  */
#[macro_export]
macro_rules! ecmult_const_table_get_ge {
    ($r:ident, $pre:ident, $n:ident, $w:ident) => {
        {
            let mut m: i32 = 0;
            /* Extract the sign-bit for a constant time absolute-value. */
            let mask: i32 = ($n) >> ((core::mem::size_of_val(&($n)) * 8 - 1) as u32);
            let abs_n: i32 = (($n) + mask) ^ mask;
            let idx_n: i32 = abs_n >> 1;
            let mut neg_y: Fe = unsafe { core::mem::zeroed() };

            verify_check!((($n) & 1) == 1);
            verify_check!(($n) >= -(((1i32) << (($w) - 1)) - 1));
            verify_check!(($n) <= (((1i32) << (($w) - 1)) - 1));

            unsafe {
                VERIFY_SETUP!(fe_clear(&mut (*($r)).x));
                VERIFY_SETUP!(fe_clear(&mut (*($r)).y));

                /* Unconditionally set r->x = (pre)[m].x. r->y = (pre)[m].y. because it's either the correct one
                 * or will get replaced in the later iterations, this is needed to make sure `r` is initialized. */
                (*($r)).x = ($pre)[m as usize].x;
                (*($r)).y = ($pre)[m as usize].y;

                m = 1;
                while (m as usize) < ECMULT_TABLE_SIZE!($w) {
                    /* This loop is used to avoid secret data in array indices. See
                     * the comment in ecmult_gen_impl.h for rationale. */
                    fe_cmov(
                        &mut (*($r)).x,
                        &($pre)[m as usize].x,
                        (m == idx_n) as i32,
                    );
                    fe_cmov(
                        &mut (*($r)).y,
                        &($pre)[m as usize].y,
                        (m == idx_n) as i32,
                    );
                    m += 1;
                }

                (*($r)).infinity = 0;
                fe_negate(&mut neg_y, &(*($r)).y, 1);
                fe_cmov(&mut (*($r)).y, &neg_y, (($n) != abs_n) as i32);
            }
        }
    };
}
