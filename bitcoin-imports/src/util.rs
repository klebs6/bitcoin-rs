// ---------------- [ File: bitcoin-imports/src/util.rs ]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/util.h]

pub struct Callback {
    pub fn_:  fn(text: *const u8, data: *mut libc::c_void),
    pub data: *const libc::c_void,
}

#[inline] pub fn callback_call(
        cb:   *const Callback,
        text: *const u8)  {
    
    todo!();
        /*
            cb->fn(text, (c_void*)cb->data);
        */
}

#[cfg(DETERMINISTIC)]
macro_rules! test_failure {
    ($msg:ident) => {
        /*
                do { 
            fprintf(stderr, "%s\n", msg); 
            abort(); 
        } while(0);
        */
    }
}

#[cfg(not(DETERMINISTIC))]
macro_rules! test_failure {
    ($msg:ident) => {
        /*
                do { 
            fprintf(stderr, "%s:%d: %s\n", __FILE__, __LINE__, msg); 
            abort(); 
        } while(0)
        */
    }
}

#[cfg(DETERMINISTIC)]
macro_rules! check {
    ($cond:ident) => {
        /*
                do { 
            if (EXPECT(!(cond), 0)) { 
                TEST_FAILURE("test condition failed"); 
            } 
        } while(0)
        */
    }
}

#[cfg(not(DETERMINISTIC))]
macro_rules! check {
    ($cond:ident) => {
        /*
                do { 
            if (EXPECT(!(cond), 0)) { 
                TEST_FAILURE("test condition failed: " #cond); 
            } 
        } while(0)
        */
    }
}

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

#[inline] pub fn checked_malloc(
        cb:   *const Callback,
        size: usize)  {
    
    todo!();
        /*
            c_void *ret = malloc(size);
        if (ret == NULL) {
            callback_call(cb, "Out of memory");
        }
        return ret;
        */
}

#[inline] pub fn checked_realloc(
        cb:   *const Callback,
        ptr:  *mut libc::c_void,
        size: usize)  {
    
    todo!();
        /*
            c_void *ret = realloc(ptr, size);
        if (ret == NULL) {
            callback_call(cb, "Out of memory");
        }
        return ret;
        */
}

#[cfg(__BIGGEST_ALIGNMENT__)]
pub const ALIGNMENT: usize = __BIGGEST_ALIGNMENT__;

/**
  | Using 16 bytes alignment because common
  | architectures never have alignment
  | requirements above 8 for any of the types
  | we care about. In addition we leave some
  | room because currently we don't care
  | about a few bytes.
  |
  */
#[cfg(not(__BIGGEST_ALIGNMENT__))]
pub const ALIGNMENT: usize = 16;

#[macro_export] macro_rules! round_to_align {
    ($size:expr) => {
        ((($size) + ALIGNMENT - 1) / ALIGNMENT) * ALIGNMENT
    }
}

/**
  | Assume there is a contiguous memory
  | object with bounds [base, base + max_size)
  | of which the memory range [base, *prealloc_ptr)
  | is already allocated for usage, where
  | *prealloc_ptr is an aligned pointer.
  | In that setting, this functions reserves
  | the subobject [*prealloc_ptr, *prealloc_ptr
  | + alloc_size) of alloc_size bytes by
  | increasing *prealloc_ptr accordingly,
  | taking into account alignment requirements.
  | 
  | The function returns an aligned pointer
  | to the newly allocated subobject.
  | 
  | This is useful for manual memory management:
  | if we're simply given a block [base,
  | base + max_size), the caller can use
  | this function to allocate memory in
  | this block and keep track of the current
  | allocation state with *prealloc_ptr.
  | 
  | It is VERIFY_CHECKed that there is enough
  | space left in the memory object and *prealloc_ptr
  | is aligned relative to base.
  |
  */
#[inline] pub fn manual_alloc(
        prealloc_ptr: *mut *mut libc::c_void,
        alloc_size:   usize,
        base:         *mut libc::c_void,
        max_size:     usize)  {
    
    todo!();
        /*
            size_t aligned_alloc_size = ROUND_TO_ALIGN(alloc_size);
        c_void* ret;
        VERIFY_CHECK(prealloc_ptr != NULL);
        VERIFY_CHECK(*prealloc_ptr != NULL);
        VERIFY_CHECK(base != NULL);
        VERIFY_CHECK((unsigned char*)*prealloc_ptr >= (unsigned char*)base);
        VERIFY_CHECK(((unsigned char*)*prealloc_ptr - (unsigned char*)base) % ALIGNMENT == 0);
        VERIFY_CHECK((unsigned char*)*prealloc_ptr - (unsigned char*)base + aligned_alloc_size <= max_size);
        ret = *prealloc_ptr;
        *prealloc_ptr = (unsigned char*)*prealloc_ptr + aligned_alloc_size;
        return ret;
        */
}

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
  | Zero memory if flag == 1. Flag must be
  | 0 or 1. Constant time.
  |
  */
#[inline] pub fn memczero(
        s:    *mut libc::c_void,
        len:  usize,
        flag: i32)  {
    
    todo!();
        /*
            unsigned char *p = (unsigned char *)s;
        /* Access flag with a volatile-qualified lvalue.
           This prevents clang from figuring out (after inlining) that flag can
           take only be 0 or 1, which leads to variable time code. */
        volatile int vflag = flag;
        unsigned char mask = -(unsigned char) vflag;
        while (len) {
            *p &= ~mask;
            p++;
            len--;
        }
        */
}

/**
  | Semantics like memcmp. Variable-time.
  | 
  | We use this to avoid possible compiler
  | bugs with memcmp, e.g. https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189
  |
  */
#[inline] pub fn memcmp_var(
        s1: *const libc::c_void,
        s2: *const libc::c_void,
        n:  usize) -> i32 {
    
    todo!();
        /*
            const unsigned char *p1 = s1, *p2 = s2;
        size_t i;

        for (i = 0; i < n; i++) {
            int diff = p1[i] - p2[i];
            if (diff != 0) {
                return diff;
            }
        }
        return 0;
        */
}

/**
  | If flag is true, set *r equal to *a; otherwise
  | leave it. Constant-time. Both *r and
  | *a must be initialized and non-negative.
  |
  */
#[inline] pub fn int_cmov(
        r:    *mut i32,
        a:    *const i32,
        flag: i32)  {
    
    todo!();
        /*
            unsigned int mask0, mask1, r_masked, a_masked;
        /* Access flag with a volatile-qualified lvalue.
           This prevents clang from figuring out (after inlining) that flag can
           take only be 0 or 1, which leads to variable time code. */
        volatile int vflag = flag;

        /* Casting a negative int to unsigned and back to int is implementation defined behavior */
        VERIFY_CHECK(*r >= 0 && *a >= 0);

        mask0 = (unsigned int)vflag + ~0u;
        mask1 = ~mask0;
        r_masked = ((unsigned int)*r & mask0);
        a_masked = ((unsigned int)*a & mask1);

        *r = (int)(r_masked | a_masked);
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

/**
  | Determine the number of trailing zero
  | bits in a (non-zero) 32-bit x.
  | 
  | This function is only intended to be
  | used as fallback for ctz32_var, but
  | permits it to be tested separately.
  |
  */
#[inline] pub fn ctz32_var_debruijn(x: u32) -> i32 {
    
    todo!();
        /*
            static const uint8_t debruijn[32] = {
            0x00, 0x01, 0x02, 0x18, 0x03, 0x13, 0x06, 0x19, 0x16, 0x04, 0x14, 0x0A,
            0x10, 0x07, 0x0C, 0x1A, 0x1F, 0x17, 0x12, 0x05, 0x15, 0x09, 0x0F, 0x0B,
            0x1E, 0x11, 0x08, 0x0E, 0x1D, 0x0D, 0x1C, 0x1B
        };
        return debruijn[((x & -x) * 0x04D7651F) >> 27];
        */
}

/**
  | Determine the number of trailing zero
  | bits in a (non-zero) 64-bit x.
  | 
  | This function is only intended to be
  | used as fallback for ctz64_var, but
  | permits it to be tested separately.
  |
  */
#[inline] pub fn ctz64_var_debruijn(x: u64) -> i32 {
    
    todo!();
        /*
            static const uint8_t debruijn[64] = {
            0, 1, 2, 53, 3, 7, 54, 27, 4, 38, 41, 8, 34, 55, 48, 28,
            62, 5, 39, 46, 44, 42, 22, 9, 24, 35, 59, 56, 49, 18, 29, 11,
            63, 52, 6, 26, 37, 40, 33, 47, 61, 45, 43, 21, 23, 58, 17, 10,
            51, 25, 36, 32, 60, 20, 57, 16, 50, 31, 19, 15, 30, 14, 13, 12
        };
        return debruijn[((x & -x) * 0x022FDD63CC95386D) >> 58];
        */
}

/**
  | Determine the number of trailing zero
  | bits in a (non-zero) 32-bit x.
  |
  */
#[inline] pub fn ctz32_var(x: u32) -> i32 {
    
    todo!();
        /*
            VERIFY_CHECK(x != 0);
    #if (__has_builtin(__builtin_ctz) || GNUC_PREREQ(3,4))
        /* If the unsigned type is sufficient to represent the largest uint32_t, consider __builtin_ctz. */
        if (((unsigned)UINT32_MAX) == UINT32_MAX) {
            return __builtin_ctz(x);
        }
    #endif
    #if (__has_builtin(__builtin_ctzl) || GNUC_PREREQ(3,4))
        /* Otherwise consider __builtin_ctzl (the unsigned long type is always at least 32 bits). */
        return __builtin_ctzl(x);
    #else
        /* If no suitable CTZ builtin is available, use a (variable time) software emulation. */
        return ctz32_var_debruijn(x);
    #endif
        */
}

/**
  | Determine the number of trailing zero
  | bits in a (non-zero) 64-bit x.
  |
  */
#[inline] pub fn ctz64_var(x: u64) -> i32 {
    
    todo!();
        /*
            VERIFY_CHECK(x != 0);
    #if (__has_builtin(__builtin_ctzl) || GNUC_PREREQ(3,4))
        /* If the unsigned long type is sufficient to represent the largest uint64_t, consider __builtin_ctzl. */
        if (((unsigned long)UINT64_MAX) == UINT64_MAX) {
            return __builtin_ctzl(x);
        }
    #endif
    #if (__has_builtin(__builtin_ctzll) || GNUC_PREREQ(3,4))
        /* Otherwise consider __builtin_ctzll (the unsigned long long type is always at least 64 bits). */
        return __builtin_ctzll(x);
    #else
        /* If no suitable CTZ builtin is available, use a (variable time) software emulation. */
        return ctz64_var_debruijn(x);
    #endif
        */
}
