// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_table_get_ge.rs ]
crate::ix!();

/// The following two macro retrieves a particular odd multiple from a table of precomputed
/// multiples.
///
#[macro_export]
macro_rules! ecmult_table_get_ge {
    ($r:ident, 
     $pre:ident, 
     $n:ident, 
     $w:ident) => {
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
    }
}

#[macro_export]
macro_rules! ecmult_table_get_ge_storage {
    ($r:ident, 
     $pre:ident, 
     $n:ident, 
     $w:ident) => {
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
    }
}
