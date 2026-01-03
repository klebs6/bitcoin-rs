// ---------------- [ File: bitcoinsecp256k1-scratch/src/vg_check.rs ]
crate::ix!();

/**
  | Define `VG_UNDEF` and `VG_CHECK` when
  | VALGRIND is defined
  |
  */
#[cfg(not(VG_CHECK))]
lazy_static!{
    /*
    # if defined(VALGRIND)
    #  include <valgrind/memcheck.h>
    #  define VG_UNDEF(x,y) VALGRIND_MAKE_MEM_UNDEFINED((x),(y))
    #  define VG_CHECK(x,y) VALGRIND_CHECK_MEM_IS_DEFINED((x),(y))
    # else
    #  define VG_UNDEF(x,y)
    #  define VG_CHECK(x,y)
    # endif
    */
}

/**
  | Like `VG_CHECK` but on VERIFY only
  |
  */
#[cfg(VERIFY)]
macro_rules! vg_check_verify {
    ($x:ident, $y:ident) => {
        /*
                VG_CHECK((x), (y))
        */
    }
}

#[cfg(not(VERIFY))]
macro_rules! vg_check_verify { ($x:ident, $y:ident) => { } }
