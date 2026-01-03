// ---------------- [ File: bitcoinsecp256k1-scalar/src/sumadd.rs ]
crate::ix!();

/**
  | Add a to the number defined by (c0,c1,c2).
  | c2 must never overflow.
  |
  */
#[cfg(WIDEMUL_INT128)]
#[macro_export] macro_rules! sumadd {
    ($a:ident) => {
        /*
                { 
            unsigned int over; 
            c0 += (a);                  /* overflow is handled on the next line */ 
            over = (c0 < (a));         
            c1 += over;                 /* overflow is handled on the next line */ 
            c2 += (c1 < over);          /* never overflows by contract */ 
        }
        */
    }
}

/**
  | Add a to the number defined by (c0,c1).
  | c1 must never overflow, c2 must be zero.
  |
  */
#[cfg(WIDEMUL_INT128)]
#[macro_export] macro_rules! sumadd_fast {
    ($a:ident) => {
        /*
                { 
            c0 += (a);                 /* overflow is handled on the next line */ 
            c1 += (c0 < (a));          /* never overflows by contract (verified the next line) */ 
            VERIFY_CHECK((c1 != 0) | (c0 >= (a))); 
            VERIFY_CHECK(c2 == 0); 
        }
        */
    }
}

/**
  | Add a to the number defined by (c0,c1,c2).
  | c2 must never overflow.
  |
  */
#[cfg(WIDEMUL_INT64)]
#[macro_export] macro_rules! sumadd {
    ($a:ident) => {
        /*
                { 
            unsigned int over; 
            c0 += (a);                  /* overflow is handled on the next line */ 
            over = (c0 < (a)); 
            c1 += over;                 /* overflow is handled on the next line */ 
            c2 += (c1 < over);          /* never overflows by contract */ 
        }
        */
    }
}

/**
  | Add a to the number defined by (c0,c1).
  | c1 must never overflow, c2 must be zero.
  |
  */
#[cfg(WIDEMUL_INT64)]
#[macro_export] macro_rules! sumadd_fast {
    ($a:ident) => {
        /*
                { 
            c0 += (a);                 /* overflow is handled on the next line */ 
            c1 += (c0 < (a));          /* never overflows by contract (verified the next line) */ 
            VERIFY_CHECK((c1 != 0) | (c0 >= (a))); 
            VERIFY_CHECK(c2 == 0); 
        }
        */
    }
}
