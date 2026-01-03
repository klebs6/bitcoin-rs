// ---------------- [ File: bitcoinsecp256k1-scratch/src/util_misc.rs ]
crate::ix!();

/**
  | Macro for restrict, when available
  | and not in a VERIFY build.
  |
  */
lazy_static!{
    /*
    #if defined(BUILD) && defined(VERIFY)
    # define 
    #else
    # if (!defined(__STDC_VERSION__) || (__STDC_VERSION__ < 199901L) )
    #  if GNUC_PREREQ(3,0)
    #   define  __restrict__
    #  elif (defined(_MSC_VER) && _MSC_VER >= 1400)
    #   define  __restrict
    #  else
    #   define 
    #  endif
    # else
    #  define  restrict
    # endif
    #endif
    */
}

lazy_static!{
    /*
    #if defined(_WIN32)
    # define I64FORMAT "I64d"
    # define I64uFORMAT "I64u"
    #else
    # define I64FORMAT "lld"
    # define I64uFORMAT "llu"
    #endif
    */
}

/**
  | If {LITTLE,BIG}_ENDIAN is not explicitly
  | provided, infer from various other
  | system macros.
  |
  */
lazy_static!{
    /*
    #if !defined(LITTLE_ENDIAN) && !defined(BIG_ENDIAN)
    /* Inspired by https://github.com/rofl0r/endianness.h/blob/9853923246b065a3b52d2c43835f3819a62c7199/endianness.h#L52L73 */
    # if (defined(__BYTE_ORDER__) && defined(__ORDER_LITTLE_ENDIAN__) && __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__) || \
         defined(_X86_) || defined(__x86_64__) || defined(__i386__) || \
         defined(__i486__) || defined(__i586__) || defined(__i686__) || \
         defined(__MIPSEL) || defined(_MIPSEL) || defined(MIPSEL) || \
         defined(__ARMEL__) || defined(__AARCH64EL__) || \
         (defined(__LITTLE_ENDIAN__) && __LITTLE_ENDIAN__ == 1) || \
         (defined(_LITTLE_ENDIAN) && _LITTLE_ENDIAN == 1) || \
         defined(_M_IX86) || defined(_M_AMD64) || defined(_M_ARM) /* MSVC */
    #  define LITTLE_ENDIAN
    # endif
    # if (defined(__BYTE_ORDER__) && defined(__ORDER_BIG_ENDIAN__) && __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__) || \
         defined(__MIPSEB) || defined(_MIPSEB) || defined(MIPSEB) || \
         defined(__MICROBLAZEEB__) || defined(__ARMEB__) || defined(__AARCH64EB__) || \
         (defined(__BIG_ENDIAN__) && __BIG_ENDIAN__ == 1) || \
         (defined(_BIG_ENDIAN) && _BIG_ENDIAN == 1)
    #  define BIG_ENDIAN
    # endif
    #endif
    */
}

/**
  | If USE_FORCE_WIDEMUL_{INT128,INT64}
  | is set, use that wide multiplication
  | implementation.
  | 
  | Otherwise use the presence of __SIZEOF_INT128__
  | to decide.
  |
  */
lazy_static!{
    /*
    #if defined(USE_FORCE_WIDEMUL_INT128)
    # define WIDEMUL_INT128 1
    #elif defined(USE_FORCE_WIDEMUL_INT64)
    # define WIDEMUL_INT64 1
    #elif defined(UINT128_MAX) || defined(__SIZEOF_INT128__)
    # define WIDEMUL_INT128 1
    #else
    # define WIDEMUL_INT64 1
    #endif
    */
}


#[cfg(WIDEMUL_INT128)]
lazy_static!{
    /*
    # if !defined(UINT128_MAX) && defined(__SIZEOF_INT128__)
    GNUC_EXT typedef unsigned __int128 uint128_t;
    GNUC_EXT typedef __int128 int128_t;
    #define UINT128_MAX ((uint128_t)(-1))
    #define INT128_MAX ((int128_t)(UINT128_MAX >> 1))
    #define INT128_MIN (-INT128_MAX - 1)
    /* No (U)INT128_C macros because compilers providing __int128 do not support 128-bit literals.  */
    # endif
    */
}
