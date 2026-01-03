// ---------------- [ File: bitcoinsecp256k1-scalar/src/clear.rs ]
crate::ix!();

/**
  | Clear a scalar to prevent the leak of
  | sensitive data.
  |
  */
#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_clear(r: *mut Scalar)  {
    
    todo!();
        /*
            r->d[0] = 0;
        r->d[1] = 0;
        r->d[2] = 0;
        r->d[3] = 0;
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_clear(r: *mut Scalar)  {
    
    todo!();
        /*
            r->d[0] = 0;
        r->d[1] = 0;
        r->d[2] = 0;
        r->d[3] = 0;
        r->d[4] = 0;
        r->d[5] = 0;
        r->d[6] = 0;
        r->d[7] = 0;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
#[inline] pub fn scalar_clear(r: *mut Scalar)  {
    
    todo!();
        /*
            *r = 0;
        */
}


