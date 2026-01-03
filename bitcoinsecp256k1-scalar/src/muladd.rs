// ---------------- [ File: bitcoinsecp256k1-scalar/src/muladd.rs ]
crate::ix!();

/**
  | Add a*b to the number defined by (c0,c1,c2).
  | c2 must never overflow.
  |
  */
#[cfg(WIDEMUL_INT128)]
#[macro_export] macro_rules! muladd {
    ($a:ident, $b:ident) => {
        /*
                { 
            uint64_t tl, th; 
            { 
                uint128_t t = (uint128_t)a * b; 
                th = t >> 64;         /* at most 0xFFFFFFFFFFFFFFFE */ 
                tl = t; 
            } 
            c0 += tl;                 /* overflow is handled on the next line */ 
            th += (c0 < tl);          /* at most 0xFFFFFFFFFFFFFFFF */ 
            c1 += th;                 /* overflow is handled on the next line */ 
            c2 += (c1 < th);          /* never overflows by contract (verified in the next line) */ 
            VERIFY_CHECK((c1 >= th) || (c2 != 0)); 
        }
        */
    }
}

/**
  | Add a*b to the number defined by (c0,c1).
  | c1 must never overflow.
  |
  */
#[cfg(WIDEMUL_INT128)]
#[macro_export] macro_rules! muladd_fast {
    ($a:ident, $b:ident) => {
        /*
                { 
            uint64_t tl, th; 
            { 
                uint128_t t = (uint128_t)a * b; 
                th = t >> 64;         /* at most 0xFFFFFFFFFFFFFFFE */ 
                tl = t; 
            } 
            c0 += tl;                 /* overflow is handled on the next line */ 
            th += (c0 < tl);          /* at most 0xFFFFFFFFFFFFFFFF */ 
            c1 += th;                 /* never overflows by contract (verified in the next line) */ 
            VERIFY_CHECK(c1 >= th); 
        }
        */
    }
}

/*
  | Inspired by the macros in OpenSSL's
  | crypto/bn/asm/x86_64-gcc.c.
  |
  */

/**
  | Add a*b to the number defined by (c0,c1,c2).
  | c2 must never overflow.
  |
  */
#[cfg(WIDEMUL_INT64)]
#[macro_export] macro_rules! muladd {
    ($a:ident, $b:ident) => {
        /*
                { 
            uint32_t tl, th; 
            { 
                uint64_t t = (uint64_t)a * b; 
                th = t >> 32;         /* at most 0xFFFFFFFE */ 
                tl = t; 
            } 
            c0 += tl;                 /* overflow is handled on the next line */ 
            th += (c0 < tl);          /* at most 0xFFFFFFFF */ 
            c1 += th;                 /* overflow is handled on the next line */ 
            c2 += (c1 < th);          /* never overflows by contract (verified in the next line) */ 
            VERIFY_CHECK((c1 >= th) || (c2 != 0)); 
        }
        */
    }
}

/**
  | Add a*b to the number defined by (c0,c1).
  | c1 must never overflow.
  |
  */
#[cfg(WIDEMUL_INT64)]
#[macro_export] macro_rules! muladd_fast {
    ($a:ident, $b:ident) => {
        /*
                { 
            uint32_t tl, th; 
            { 
                uint64_t t = (uint64_t)a * b; 
                th = t >> 32;         /* at most 0xFFFFFFFE */ 
                tl = t; 
            } 
            c0 += tl;                 /* overflow is handled on the next line */ 
            th += (c0 < tl);          /* at most 0xFFFFFFFF */ 
            c1 += th;                 /* never overflows by contract (verified in the next line) */ 
            VERIFY_CHECK(c1 >= th); 
        }
        */
    }
}

