// ---------------- [ File: bitcoinsecp256k1-scratch/src/verify.rs ]
/**
  | Like assert(), but when VERIFY is defined,
  | and side-effect safe.
  |
  */
lazy_static!{
    /*
    #if defined(COVERAGE)
    #define VERIFY_CHECK(check)
    #define VERIFY_SETUP(stmt)
    #elif defined(VERIFY)
    #define VERIFY_CHECK CHECK
    #define VERIFY_SETUP(stmt) do { stmt; } while(0)
    #else
    #define VERIFY_CHECK(cond) do { (c_void)(cond); } while(0)
    #define VERIFY_SETUP(stmt)
    #endif
    */
}
