// ---------------- [ File: bitcoinsecp256k1-scalar/src/lib.rs ]
#[macro_use] mod imports; use imports::*;
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_8x32.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_8x32_impl.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_4x64.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_4x64_impl.h]

x!{constants}
x!{widemul_macros}
x!{limbs}
x!{scalar}
x!{scalar_add}
x!{scalar_cadd_bit}
x!{scalar_check_overflow}
x!{scalar_clear}
x!{scalar_cmov}
x!{scalar_cond_negate}
x!{scalar_const}
x!{scalar_eq}
x!{scalar_from_signed30}
x!{scalar_from_signed62}
x!{scalar_get_b32}
x!{scalar_get_bits}
x!{scalar_get_bits_var}
x!{scalar_inverse}
x!{scalar_inverse_var}
x!{scalar_is_even}
x!{scalar_is_high}
x!{scalar_is_one}
x!{scalar_is_zero}
x!{scalar_mul}
x!{scalar_mul_512}
x!{scalar_mul_shift}
x!{scalar_negate}
x!{scalar_reduce}
x!{scalar_reduce_512}
x!{scalar_set_b32}
x!{scalar_set_b32_seckey}
x!{scalar_set_int}
x!{scalar_shr}
x!{scalar_split}
x!{scalar_split_lambda}
x!{scalar_split_lambda_verify}
x!{scalar_to_signed30}
x!{scalar_to_signed62}

#[cfg(test)]
x!{scalar_test_support}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar.h]

lazy_static!{
    /*
    #if defined(EXHAUSTIVE_TEST_ORDER)
    #include "scalar_low.h"
    #elif defined(WIDEMUL_INT128)
    #include "scalar_4x64.h"
    #elif defined(WIDEMUL_INT64)
    #include "scalar_8x32.h"
    #else
    #error "Please select wide multiplication implementation"
    #endif
    */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_impl.h]

lazy_static!{
    /*
    #if defined(EXHAUSTIVE_TEST_ORDER)
    #include "scalar_low_impl.h"
    #elif defined(WIDEMUL_INT128)
    #include "scalar_4x64_impl.h"
    #elif defined(WIDEMUL_INT64)
    #include "scalar_8x32_impl.h"
    #else
    #error "Please select wide multiplication implementation"
    #endif
    */
}


/**
  | These parameters are generated using
  | sage/gen_exhaustive_groups.sage.
  |
  */
#[cfg(EXHAUSTIVE_TEST_ORDER)]
lazy_static!{
    /*
    #  if EXHAUSTIVE_TEST_ORDER == 13
    #    define EXHAUSTIVE_TEST_LAMBDA 9
    #  elif EXHAUSTIVE_TEST_ORDER == 199
    #    define EXHAUSTIVE_TEST_LAMBDA 92
    #  else
    #    error No known lambda for the specified exhaustive test group order.
    #  endif
    */
}
