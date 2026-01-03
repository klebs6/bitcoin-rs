// ---------------- [ File: bitcoinsecp256k1-scalar/src/extract.rs ]
crate::ix!();

/**
  | Extract the lowest 64 bits of (c0,c1,c2)
  | into n, and left shift the number 64 bits.
  |
  */
#[cfg(WIDEMUL_INT128)]
#[macro_export] macro_rules! extract {
    ($n:ident) => {
        /*
                { 
            (n) = c0; 
            c0 = c1; 
            c1 = c2; 
            c2 = 0; 
        }
        */
    }
}

/**
  | Extract the lowest 64 bits of (c0,c1,c2)
  | into n, and left shift the number 64 bits.
  | c2 is required to be zero.
  |
  */
#[cfg(WIDEMUL_INT128)]
#[macro_export] macro_rules! extract_fast {
    ($n:ident) => {
        /*
                { 
            (n) = c0; 
            c0 = c1; 
            c1 = 0; 
            VERIFY_CHECK(c2 == 0); 
        }
        */
    }
}

/**
  | Extract the lowest 32 bits of (c0,c1,c2)
  | into n, and left shift the number 32 bits.
  |
  */
#[cfg(WIDEMUL_INT64)]
#[macro_export] macro_rules! extract {
    ($n:ident) => {
        /*
                { 
            (n) = c0; 
            c0 = c1; 
            c1 = c2; 
            c2 = 0; 
        }
        */
    }
}

/**
  | Extract the lowest 32 bits of (c0,c1,c2)
  | into n, and left shift the number 32 bits.
  | c2 is required to be zero.
  |
  */
#[cfg(WIDEMUL_INT64)]
#[macro_export] macro_rules! extract_fast {
    ($n:ident) => {
        /*
                { 
            (n) = c0; 
            c0 = c1; 
            c1 = 0; 
            VERIFY_CHECK(c2 == 0); 
        }
        */
    }
}

