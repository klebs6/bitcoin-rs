crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_low.h]

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
pub type Scalar = u32;

#[macro_export] macro_rules! scalar_const {
    ($d7:ident, 
     $d6:ident, 
     $d5:ident, 
     $d4:ident, 
     $d3:ident, 
     $d2:ident, 
     $d1:ident, 
     $d0:ident) => {
        $d0
    }
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_low_impl.h]

#[inline] pub fn scalar_is_even(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return !(*a & 1);
        */
}

#[inline] pub fn scalar_clear(r: *mut Scalar)  {
    
    todo!();
        /*
            *r = 0;
        */
}

#[inline] pub fn scalar_set_int(
        r: *mut Scalar,
        v: u32)  {
    
    todo!();
        /*
            *r = v;
        */
}

#[inline] pub fn scalar_get_bits(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            if (offset < 32)
            return ((*a >> offset) & ((((uint32_t)1) << count) - 1));
        else
            return 0;
        */
}

#[inline] pub fn scalar_get_bits_var(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            return scalar_get_bits(a, offset, count);
        */
}

#[inline] pub fn scalar_check_overflow(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return *a >= EXHAUSTIVE_TEST_ORDER;
        */
}

pub fn scalar_add(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            *r = (*a + *b) % EXHAUSTIVE_TEST_ORDER;
        return *r < *b;
        */
}

pub fn scalar_cadd_bit(
        r:    *mut Scalar,
        bit:  u32,
        flag: i32)  {
    
    todo!();
        /*
            if (flag && bit < 32)
            *r += ((uint32_t)1 << bit);
    #ifdef VERIFY
        VERIFY_CHECK(bit < 32);
        /* Verify that adding (1 << bit) will not overflow any in-range scalar *r by overflowing the underlying uint32_t. */
        VERIFY_CHECK(((uint32_t)1 << bit) - 1 <= UINT32_MAX - EXHAUSTIVE_TEST_ORDER);
        VERIFY_CHECK(scalar_check_overflow(r) == 0);
    #endif
        */
}

pub fn scalar_set_b32(
        r:        *mut Scalar,
        b32:      *const u8,
        overflow: *mut i32)  {
    
    todo!();
        /*
            int i;
        int over = 0;
        *r = 0;
        for (i = 0; i < 32; i++) {
            *r = (*r * 0x100) + b32[i];
            if (*r >= EXHAUSTIVE_TEST_ORDER) {
                over = 1;
                *r %= EXHAUSTIVE_TEST_ORDER;
            }
        }
        if (overflow) *overflow = over;
        */
}

pub fn scalar_get_b32(
        bin: *mut u8,
        a:   *const Scalar)  {
    
    todo!();
        /*
            memset(bin, 0, 32);
        bin[28] = *a >> 24; bin[29] = *a >> 16; bin[30] = *a >> 8; bin[31] = *a;
        */
}

#[inline] pub fn scalar_is_zero(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return *a == 0;
        */
}

pub fn scalar_negate(
        r: *mut Scalar,
        a: *const Scalar)  {
    
    todo!();
        /*
            if (*a == 0) {
            *r = 0;
        } else {
            *r = EXHAUSTIVE_TEST_ORDER - *a;
        }
        */
}

#[inline] pub fn scalar_is_one(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return *a == 1;
        */
}

pub fn scalar_is_high(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return *a > EXHAUSTIVE_TEST_ORDER / 2;
        */
}

pub fn scalar_cond_negate(
        r:    *mut Scalar,
        flag: i32) -> i32 {
    
    todo!();
        /*
            if (flag) scalar_negate(r, r);
        return flag ? -1 : 1;
        */
}

pub fn scalar_mul(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar)  {
    
    todo!();
        /*
            *r = (*a * *b) % EXHAUSTIVE_TEST_ORDER;
        */
}

pub fn scalar_shr_int(
        r: *mut Scalar,
        n: i32) -> i32 {
    
    todo!();
        /*
            int ret;
        VERIFY_CHECK(n > 0);
        VERIFY_CHECK(n < 16);
        ret = *r & ((1 << n) - 1);
        *r >>= n;
        return ret;
        */
}

pub fn scalar_split_128(
        r1: *mut Scalar,
        r2: *mut Scalar,
        a:  *const Scalar)  {
    
    todo!();
        /*
            *r1 = *a;
        *r2 = 0;
        */
}

#[inline] pub fn scalar_eq(
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            return *a == *b;
        */
}

#[inline] pub fn scalar_cmov(
        r:    *mut Scalar,
        a:    *const Scalar,
        flag: i32)  {
    
    todo!();
        /*
            uint32_t mask0, mask1;
        VG_CHECK_VERIFY(r, sizeof(*r));
        mask0 = flag + ~((uint32_t)0);
        mask1 = ~mask0;
        *r = (*r & mask0) | (*a & mask1);
        */
}

pub fn scalar_inverse(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            int i;
        *r = 0;
        for (i = 0; i < EXHAUSTIVE_TEST_ORDER; i++)
            if ((i * *x) % EXHAUSTIVE_TEST_ORDER == 1)
                *r = i;
        /* If this VERIFY_CHECK triggers we were given a noninvertible scalar (and thus
         * have a composite group order; fix it in exhaustive_tests.c). */
        VERIFY_CHECK(*r != 0);
        */
}

pub fn scalar_inverse_var(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            scalar_inverse(r, x);
        */
}
