/*!
  | Implements arithmetic modulo FFFFFFFF FFFFFFFF
  |  FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE
  |  FFFFFC2F, represented as 5 uint64_t's in base
  |  2^52. 
  |
  |  The values are allowed to contain >52 each. In
  |  particular, each FieldElem has a 'magnitude'
  |  associated with it. 
  |
  |  Internally, a magnitude M means each element
  |  is at most M*(2^53-1), except the most
  |  significant one, which is limited to
  |  M*(2^49-1). 
  |
  |  All operations accept any input with magnitude
  |  at most M, and have different rules for
  |  propagating magnitude to their output.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_5x52.h]

pub struct Fe {

    /**
      | X = sum(i=0..4, n[i]*2^(i*52)) mod
      | p where p = 2^256 - 0x1000003D1
      |
      */
    pub n:          [u64; 5],

    #[cfg(VERIFY)]
    pub magnitude:  i32,

    #[cfg(VERIFY)]
    pub normalized: i32,
}

impl Fe {

    pub const fn new() -> Self {
        Self {
            n: [0; 5],
            #[cfg(VERIFY)] magnitude:  0,
            #[cfg(VERIFY)] normalized: 0,
        }
    }
}

/**
  | Unpacks a constant into a overlapping
  | multi-limbed FE element.
  |
  */
macro_rules! fe_const_inner {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        [ 
            ($d0) | ((($d1 as u64) & 0xFFFFF) << 32), 
            (($d1 as u64) >> 20) | (($d2 as u64) << 12) | ((($d3 as u64) & 0xFF) << 44), 
            (($d3 as u64) >> 8)  | ((($d4 as u64) & 0xFFFFFFF) << 24), 
            (($d4 as u64) >> 28) | (($d5 as u64) << 4) | ((($d6 as u64) & 0xFFFF) << 36), 
            (($d6 as u64) >> 16) | (($d7 as u64) << 16) 
        ]
    }
}

#[cfg(VERIFY)]
macro_rules! fe_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        Fe {
            n: fe_const_inner!{
                $d7, 
                $d6, 
                $d5, 
                $d4, 
                $d3, 
                $d2, 
                $d1, 
                $d0
            },
            magnitude:  1, 
            normalized: 1
        }
    }
}

#[cfg(not(VERIFY))]
macro_rules! fe_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        Fe {
            n: fe_const_inner!{
                $d7, 
                $d6, 
                $d5, 
                $d4, 
                $d3, 
                $d2, 
                $d1, 
                $d0
            }
        }
    }
}

pub struct FeStorage {
    n: [u64; 4],
}

macro_rules! fe_storage_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        todo!();
        /*
                {{ 
            (d0) | (((uint64_t)(d1)) << 32), 
            (d2) | (((uint64_t)(d3)) << 32), 
            (d4) | (((uint64_t)(d5)) << 32), 
            (d6) | (((uint64_t)(d7)) << 32) 
        }}
        */
    }
}

macro_rules! fe_storage_const_get {
    ($d:expr) => {
        todo!();
        /*
        
            (uint32_t)(d.n[3] >> 32), (uint32_t)d.n[3], 
            (uint32_t)(d.n[2] >> 32), (uint32_t)d.n[2], 
            (uint32_t)(d.n[1] >> 32), (uint32_t)d.n[1], 
            (uint32_t)(d.n[0] >> 32), (uint32_t)d.n[0]
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_5x52_impl.h]

#[cfg(VERIFY)]
pub fn fe_verify(a: *const Fe)  {
    
    todo!();
        /*
            const uint64_t *d = a->n;
        int m = a->normalized ? 1 : 2 * a->magnitude, r = 1;
       /* secp256k1 'p' value defined in "Standards for Efficient Cryptography" (SEC2) 2.7.1. */
        r &= (d[0] <= 0xFFFFFFFFFFFFFULL * m);
        r &= (d[1] <= 0xFFFFFFFFFFFFFULL * m);
        r &= (d[2] <= 0xFFFFFFFFFFFFFULL * m);
        r &= (d[3] <= 0xFFFFFFFFFFFFFULL * m);
        r &= (d[4] <= 0x0FFFFFFFFFFFFULL * m);
        r &= (a->magnitude >= 0);
        r &= (a->magnitude <= 2048);
        if (a->normalized) {
            r &= (a->magnitude <= 1);
            if (r && (d[4] == 0x0FFFFFFFFFFFFULL) && ((d[3] & d[2] & d[1]) == 0xFFFFFFFFFFFFFULL)) {
                r &= (d[0] < 0xFFFFEFFFFFC2FULL);
            }
        }
        VERIFY_CHECK(r == 1);
        */
}

pub fn fe_normalize(r: *mut Fe)  {
    
    todo!();
        /*
            uint64_t t0 = r->n[0], t1 = r->n[1], t2 = r->n[2], t3 = r->n[3], t4 = r->n[4];

        /* Reduce t4 at the start so there will be at most a single carry from the first pass */
        uint64_t m;
        uint64_t x = t4 >> 48; t4 &= 0x0FFFFFFFFFFFFULL;

        /* The first pass ensures the magnitude is 1, ... */
        t0 += x * 0x1000003D1ULL;
        t1 += (t0 >> 52); t0 &= 0xFFFFFFFFFFFFFULL;
        t2 += (t1 >> 52); t1 &= 0xFFFFFFFFFFFFFULL; m = t1;
        t3 += (t2 >> 52); t2 &= 0xFFFFFFFFFFFFFULL; m &= t2;
        t4 += (t3 >> 52); t3 &= 0xFFFFFFFFFFFFFULL; m &= t3;

        /* ... except for a possible carry at bit 48 of t4 (i.e. bit 256 of the field element) */
        VERIFY_CHECK(t4 >> 49 == 0);

        /* At most a single final reduction is needed; check if the value is >= the field characteristic */
        x = (t4 >> 48) | ((t4 == 0x0FFFFFFFFFFFFULL) & (m == 0xFFFFFFFFFFFFFULL)
            & (t0 >= 0xFFFFEFFFFFC2FULL));

        /* Apply the final reduction (for constant-time behaviour, we do it always) */
        t0 += x * 0x1000003D1ULL;
        t1 += (t0 >> 52); t0 &= 0xFFFFFFFFFFFFFULL;
        t2 += (t1 >> 52); t1 &= 0xFFFFFFFFFFFFFULL;
        t3 += (t2 >> 52); t2 &= 0xFFFFFFFFFFFFFULL;
        t4 += (t3 >> 52); t3 &= 0xFFFFFFFFFFFFFULL;

        /* If t4 didn't carry to bit 48 already, then it should have after any final reduction */
        VERIFY_CHECK(t4 >> 48 == x);

        /* Mask off the possible multiple of 2^256 from the final reduction */
        t4 &= 0x0FFFFFFFFFFFFULL;

        r->n[0] = t0; r->n[1] = t1; r->n[2] = t2; r->n[3] = t3; r->n[4] = t4;

    #ifdef VERIFY
        r->magnitude = 1;
        r->normalized = 1;
        fe_verify(r);
    #endif
        */
}

pub fn fe_normalize_weak(r: *mut Fe)  {
    
    todo!();
        /*
            uint64_t t0 = r->n[0], t1 = r->n[1], t2 = r->n[2], t3 = r->n[3], t4 = r->n[4];

        /* Reduce t4 at the start so there will be at most a single carry from the first pass */
        uint64_t x = t4 >> 48; t4 &= 0x0FFFFFFFFFFFFULL;

        /* The first pass ensures the magnitude is 1, ... */
        t0 += x * 0x1000003D1ULL;
        t1 += (t0 >> 52); t0 &= 0xFFFFFFFFFFFFFULL;
        t2 += (t1 >> 52); t1 &= 0xFFFFFFFFFFFFFULL;
        t3 += (t2 >> 52); t2 &= 0xFFFFFFFFFFFFFULL;
        t4 += (t3 >> 52); t3 &= 0xFFFFFFFFFFFFFULL;

        /* ... except for a possible carry at bit 48 of t4 (i.e. bit 256 of the field element) */
        VERIFY_CHECK(t4 >> 49 == 0);

        r->n[0] = t0; r->n[1] = t1; r->n[2] = t2; r->n[3] = t3; r->n[4] = t4;

    #ifdef VERIFY
        r->magnitude = 1;
        fe_verify(r);
    #endif
        */
}

pub fn fe_normalize_var(r: *mut Fe)  {
    
    todo!();
        /*
            uint64_t t0 = r->n[0], t1 = r->n[1], t2 = r->n[2], t3 = r->n[3], t4 = r->n[4];

        /* Reduce t4 at the start so there will be at most a single carry from the first pass */
        uint64_t m;
        uint64_t x = t4 >> 48; t4 &= 0x0FFFFFFFFFFFFULL;

        /* The first pass ensures the magnitude is 1, ... */
        t0 += x * 0x1000003D1ULL;
        t1 += (t0 >> 52); t0 &= 0xFFFFFFFFFFFFFULL;
        t2 += (t1 >> 52); t1 &= 0xFFFFFFFFFFFFFULL; m = t1;
        t3 += (t2 >> 52); t2 &= 0xFFFFFFFFFFFFFULL; m &= t2;
        t4 += (t3 >> 52); t3 &= 0xFFFFFFFFFFFFFULL; m &= t3;

        /* ... except for a possible carry at bit 48 of t4 (i.e. bit 256 of the field element) */
        VERIFY_CHECK(t4 >> 49 == 0);

        /* At most a single final reduction is needed; check if the value is >= the field characteristic */
        x = (t4 >> 48) | ((t4 == 0x0FFFFFFFFFFFFULL) & (m == 0xFFFFFFFFFFFFFULL)
            & (t0 >= 0xFFFFEFFFFFC2FULL));

        if (x) {
            t0 += 0x1000003D1ULL;
            t1 += (t0 >> 52); t0 &= 0xFFFFFFFFFFFFFULL;
            t2 += (t1 >> 52); t1 &= 0xFFFFFFFFFFFFFULL;
            t3 += (t2 >> 52); t2 &= 0xFFFFFFFFFFFFFULL;
            t4 += (t3 >> 52); t3 &= 0xFFFFFFFFFFFFFULL;

            /* If t4 didn't carry to bit 48 already, then it should have after any final reduction */
            VERIFY_CHECK(t4 >> 48 == x);

            /* Mask off the possible multiple of 2^256 from the final reduction */
            t4 &= 0x0FFFFFFFFFFFFULL;
        }

        r->n[0] = t0; r->n[1] = t1; r->n[2] = t2; r->n[3] = t3; r->n[4] = t4;

    #ifdef VERIFY
        r->magnitude = 1;
        r->normalized = 1;
        fe_verify(r);
    #endif
        */
}

pub fn fe_normalizes_to_zero(r: *const Fe) -> i32 {
    
    todo!();
        /*
            uint64_t t0 = r->n[0], t1 = r->n[1], t2 = r->n[2], t3 = r->n[3], t4 = r->n[4];

        /* z0 tracks a possible raw value of 0, z1 tracks a possible raw value of P */
        uint64_t z0, z1;

        /* Reduce t4 at the start so there will be at most a single carry from the first pass */
        uint64_t x = t4 >> 48; t4 &= 0x0FFFFFFFFFFFFULL;

        /* The first pass ensures the magnitude is 1, ... */
        t0 += x * 0x1000003D1ULL;
        t1 += (t0 >> 52); t0 &= 0xFFFFFFFFFFFFFULL; z0  = t0; z1  = t0 ^ 0x1000003D0ULL;
        t2 += (t1 >> 52); t1 &= 0xFFFFFFFFFFFFFULL; z0 |= t1; z1 &= t1;
        t3 += (t2 >> 52); t2 &= 0xFFFFFFFFFFFFFULL; z0 |= t2; z1 &= t2;
        t4 += (t3 >> 52); t3 &= 0xFFFFFFFFFFFFFULL; z0 |= t3; z1 &= t3;
                                                    z0 |= t4; z1 &= t4 ^ 0xF000000000000ULL;

        /* ... except for a possible carry at bit 48 of t4 (i.e. bit 256 of the field element) */
        VERIFY_CHECK(t4 >> 49 == 0);

        return (z0 == 0) | (z1 == 0xFFFFFFFFFFFFFULL);
        */
}

pub fn fe_normalizes_to_zero_var(r: *const Fe) -> i32 {
    
    todo!();
        /*
            uint64_t t0, t1, t2, t3, t4;
        uint64_t z0, z1;
        uint64_t x;

        t0 = r->n[0];
        t4 = r->n[4];

        /* Reduce t4 at the start so there will be at most a single carry from the first pass */
        x = t4 >> 48;

        /* The first pass ensures the magnitude is 1, ... */
        t0 += x * 0x1000003D1ULL;

        /* z0 tracks a possible raw value of 0, z1 tracks a possible raw value of P */
        z0 = t0 & 0xFFFFFFFFFFFFFULL;
        z1 = z0 ^ 0x1000003D0ULL;

        /* Fast return path should catch the majority of cases */
        if ((z0 != 0ULL) & (z1 != 0xFFFFFFFFFFFFFULL)) {
            return 0;
        }

        t1 = r->n[1];
        t2 = r->n[2];
        t3 = r->n[3];

        t4 &= 0x0FFFFFFFFFFFFULL;

        t1 += (t0 >> 52);
        t2 += (t1 >> 52); t1 &= 0xFFFFFFFFFFFFFULL; z0 |= t1; z1 &= t1;
        t3 += (t2 >> 52); t2 &= 0xFFFFFFFFFFFFFULL; z0 |= t2; z1 &= t2;
        t4 += (t3 >> 52); t3 &= 0xFFFFFFFFFFFFFULL; z0 |= t3; z1 &= t3;
                                                    z0 |= t4; z1 &= t4 ^ 0xF000000000000ULL;

        /* ... except for a possible carry at bit 48 of t4 (i.e. bit 256 of the field element) */
        VERIFY_CHECK(t4 >> 49 == 0);

        return (z0 == 0) | (z1 == 0xFFFFFFFFFFFFFULL);
        */
}

#[inline] pub fn fe_set_int(
        r: *mut Fe,
        a: i32)  {
    
    todo!();
        /*
            r->n[0] = a;
        r->n[1] = r->n[2] = r->n[3] = r->n[4] = 0;
    #ifdef VERIFY
        r->magnitude = 1;
        r->normalized = 1;
        fe_verify(r);
    #endif
        */
}

#[inline] pub fn fe_is_zero(a: *const Fe) -> i32 {
    
    todo!();
        /*
            const uint64_t *t = a->n;
    #ifdef VERIFY
        VERIFY_CHECK(a->normalized);
        fe_verify(a);
    #endif
        return (t[0] | t[1] | t[2] | t[3] | t[4]) == 0;
        */
}

#[inline] pub fn fe_is_odd(a: *const Fe) -> i32 {
    
    todo!();
        /*
            #ifdef VERIFY
        VERIFY_CHECK(a->normalized);
        fe_verify(a);
    #endif
        return a->n[0] & 1;
        */
}

#[inline] pub fn fe_clear(a: *mut Fe)  {
    
    todo!();
        /*
            int i;
    #ifdef VERIFY
        a->magnitude = 0;
        a->normalized = 1;
    #endif
        for (i=0; i<5; i++) {
            a->n[i] = 0;
        }
        */
}

pub fn fe_cmp_var(
        a: *const Fe,
        b: *const Fe) -> i32 {
    
    todo!();
        /*
            int i;
    #ifdef VERIFY
        VERIFY_CHECK(a->normalized);
        VERIFY_CHECK(b->normalized);
        fe_verify(a);
        fe_verify(b);
    #endif
        for (i = 4; i >= 0; i--) {
            if (a->n[i] > b->n[i]) {
                return 1;
            }
            if (a->n[i] < b->n[i]) {
                return -1;
            }
        }
        return 0;
        */
}

pub fn fe_set_b32(
        r: *mut Fe,
        a: *const u8) -> i32 {
    
    todo!();
        /*
            int ret;
        r->n[0] = (uint64_t)a[31]
                | ((uint64_t)a[30] << 8)
                | ((uint64_t)a[29] << 16)
                | ((uint64_t)a[28] << 24)
                | ((uint64_t)a[27] << 32)
                | ((uint64_t)a[26] << 40)
                | ((uint64_t)(a[25] & 0xF)  << 48);
        r->n[1] = (uint64_t)((a[25] >> 4) & 0xF)
                | ((uint64_t)a[24] << 4)
                | ((uint64_t)a[23] << 12)
                | ((uint64_t)a[22] << 20)
                | ((uint64_t)a[21] << 28)
                | ((uint64_t)a[20] << 36)
                | ((uint64_t)a[19] << 44);
        r->n[2] = (uint64_t)a[18]
                | ((uint64_t)a[17] << 8)
                | ((uint64_t)a[16] << 16)
                | ((uint64_t)a[15] << 24)
                | ((uint64_t)a[14] << 32)
                | ((uint64_t)a[13] << 40)
                | ((uint64_t)(a[12] & 0xF) << 48);
        r->n[3] = (uint64_t)((a[12] >> 4) & 0xF)
                | ((uint64_t)a[11] << 4)
                | ((uint64_t)a[10] << 12)
                | ((uint64_t)a[9]  << 20)
                | ((uint64_t)a[8]  << 28)
                | ((uint64_t)a[7]  << 36)
                | ((uint64_t)a[6]  << 44);
        r->n[4] = (uint64_t)a[5]
                | ((uint64_t)a[4] << 8)
                | ((uint64_t)a[3] << 16)
                | ((uint64_t)a[2] << 24)
                | ((uint64_t)a[1] << 32)
                | ((uint64_t)a[0] << 40);
        ret = !((r->n[4] == 0x0FFFFFFFFFFFFULL) & ((r->n[3] & r->n[2] & r->n[1]) == 0xFFFFFFFFFFFFFULL) & (r->n[0] >= 0xFFFFEFFFFFC2FULL));
    #ifdef VERIFY
        r->magnitude = 1;
        if (ret) {
            r->normalized = 1;
            fe_verify(r);
        } else {
            r->normalized = 0;
        }
    #endif
        return ret;
        */
}

/**
  | Convert a field element to a 32-byte
  | big endian value. Requires the input
  | to be normalized
  |
  */
pub fn fe_get_b32(
        r: *mut u8,
        a: *const Fe)  {
    
    todo!();
        /*
            #ifdef VERIFY
        VERIFY_CHECK(a->normalized);
        fe_verify(a);
    #endif
        r[0] = (a->n[4] >> 40) & 0xFF;
        r[1] = (a->n[4] >> 32) & 0xFF;
        r[2] = (a->n[4] >> 24) & 0xFF;
        r[3] = (a->n[4] >> 16) & 0xFF;
        r[4] = (a->n[4] >> 8) & 0xFF;
        r[5] = a->n[4] & 0xFF;
        r[6] = (a->n[3] >> 44) & 0xFF;
        r[7] = (a->n[3] >> 36) & 0xFF;
        r[8] = (a->n[3] >> 28) & 0xFF;
        r[9] = (a->n[3] >> 20) & 0xFF;
        r[10] = (a->n[3] >> 12) & 0xFF;
        r[11] = (a->n[3] >> 4) & 0xFF;
        r[12] = ((a->n[2] >> 48) & 0xF) | ((a->n[3] & 0xF) << 4);
        r[13] = (a->n[2] >> 40) & 0xFF;
        r[14] = (a->n[2] >> 32) & 0xFF;
        r[15] = (a->n[2] >> 24) & 0xFF;
        r[16] = (a->n[2] >> 16) & 0xFF;
        r[17] = (a->n[2] >> 8) & 0xFF;
        r[18] = a->n[2] & 0xFF;
        r[19] = (a->n[1] >> 44) & 0xFF;
        r[20] = (a->n[1] >> 36) & 0xFF;
        r[21] = (a->n[1] >> 28) & 0xFF;
        r[22] = (a->n[1] >> 20) & 0xFF;
        r[23] = (a->n[1] >> 12) & 0xFF;
        r[24] = (a->n[1] >> 4) & 0xFF;
        r[25] = ((a->n[0] >> 48) & 0xF) | ((a->n[1] & 0xF) << 4);
        r[26] = (a->n[0] >> 40) & 0xFF;
        r[27] = (a->n[0] >> 32) & 0xFF;
        r[28] = (a->n[0] >> 24) & 0xFF;
        r[29] = (a->n[0] >> 16) & 0xFF;
        r[30] = (a->n[0] >> 8) & 0xFF;
        r[31] = a->n[0] & 0xFF;
        */
}

#[inline] pub fn fe_negate(
        r: *mut Fe,
        a: *const Fe,
        m: i32)  {
    
    todo!();
        /*
            #ifdef VERIFY
        VERIFY_CHECK(a->magnitude <= m);
        fe_verify(a);
    #endif
        r->n[0] = 0xFFFFEFFFFFC2FULL * 2 * (m + 1) - a->n[0];
        r->n[1] = 0xFFFFFFFFFFFFFULL * 2 * (m + 1) - a->n[1];
        r->n[2] = 0xFFFFFFFFFFFFFULL * 2 * (m + 1) - a->n[2];
        r->n[3] = 0xFFFFFFFFFFFFFULL * 2 * (m + 1) - a->n[3];
        r->n[4] = 0x0FFFFFFFFFFFFULL * 2 * (m + 1) - a->n[4];
    #ifdef VERIFY
        r->magnitude = m + 1;
        r->normalized = 0;
        fe_verify(r);
    #endif
        */
}

#[inline] pub fn fe_mul_int(
        r: *mut Fe,
        a: i32)  {
    
    todo!();
        /*
            r->n[0] *= a;
        r->n[1] *= a;
        r->n[2] *= a;
        r->n[3] *= a;
        r->n[4] *= a;
    #ifdef VERIFY
        r->magnitude *= a;
        r->normalized = 0;
        fe_verify(r);
    #endif
        */
}

#[inline] pub fn fe_add(
        r: *mut Fe,
        a: *const Fe)  {
    
    todo!();
        /*
            #ifdef VERIFY
        fe_verify(a);
    #endif
        r->n[0] += a->n[0];
        r->n[1] += a->n[1];
        r->n[2] += a->n[2];
        r->n[3] += a->n[3];
        r->n[4] += a->n[4];
    #ifdef VERIFY
        r->magnitude += a->magnitude;
        r->normalized = 0;
        fe_verify(r);
    #endif
        */
}

pub fn fe_mul(
        r: *mut Fe,
        a: *const Fe,
        b: *const Fe)  {
    
    todo!();
        /*
            #ifdef VERIFY
        VERIFY_CHECK(a->magnitude <= 8);
        VERIFY_CHECK(b->magnitude <= 8);
        fe_verify(a);
        fe_verify(b);
        VERIFY_CHECK(r != b);
        VERIFY_CHECK(a != b);
    #endif
        fe_mul_inner(r->n, a->n, b->n);
    #ifdef VERIFY
        r->magnitude = 1;
        r->normalized = 0;
        fe_verify(r);
    #endif
        */
}

pub fn fe_sqr(
        r: *mut Fe,
        a: *const Fe)  {
    
    todo!();
        /*
            #ifdef VERIFY
        VERIFY_CHECK(a->magnitude <= 8);
        fe_verify(a);
    #endif
        fe_sqr_inner(r->n, a->n);
    #ifdef VERIFY
        r->magnitude = 1;
        r->normalized = 0;
        fe_verify(r);
    #endif
        */
}

#[inline] pub fn fe_cmov(
        r:    *mut Fe,
        a:    *const Fe,
        flag: i32)  {
    
    todo!();
        /*
            uint64_t mask0, mask1;
        VG_CHECK_VERIFY(r->n, sizeof(r->n));
        mask0 = flag + ~((uint64_t)0);
        mask1 = ~mask0;
        r->n[0] = (r->n[0] & mask0) | (a->n[0] & mask1);
        r->n[1] = (r->n[1] & mask0) | (a->n[1] & mask1);
        r->n[2] = (r->n[2] & mask0) | (a->n[2] & mask1);
        r->n[3] = (r->n[3] & mask0) | (a->n[3] & mask1);
        r->n[4] = (r->n[4] & mask0) | (a->n[4] & mask1);
    #ifdef VERIFY
        if (flag) {
            r->magnitude = a->magnitude;
            r->normalized = a->normalized;
        }
    #endif
        */
}


#[inline] pub fn fe_storage_cmov(
        r:    *mut FeStorage,
        a:    *const FeStorage,
        flag: i32)  {
    
    todo!();
        /*
            uint64_t mask0, mask1;
        VG_CHECK_VERIFY(r->n, sizeof(r->n));
        mask0 = flag + ~((uint64_t)0);
        mask1 = ~mask0;
        r->n[0] = (r->n[0] & mask0) | (a->n[0] & mask1);
        r->n[1] = (r->n[1] & mask0) | (a->n[1] & mask1);
        r->n[2] = (r->n[2] & mask0) | (a->n[2] & mask1);
        r->n[3] = (r->n[3] & mask0) | (a->n[3] & mask1);
        */
}

pub fn fe_to_storage(
        r: *mut FeStorage,
        a: *const Fe)  {
    
    todo!();
        /*
            #ifdef VERIFY
        VERIFY_CHECK(a->normalized);
    #endif
        r->n[0] = a->n[0] | a->n[1] << 52;
        r->n[1] = a->n[1] >> 12 | a->n[2] << 40;
        r->n[2] = a->n[2] >> 24 | a->n[3] << 28;
        r->n[3] = a->n[3] >> 36 | a->n[4] << 16;
        */
}

#[inline] pub fn fe_from_storage(
        r: *mut Fe,
        a: *const FeStorage)  {
    
    todo!();
        /*
            r->n[0] = a->n[0] & 0xFFFFFFFFFFFFFULL;
        r->n[1] = a->n[0] >> 52 | ((a->n[1] << 12) & 0xFFFFFFFFFFFFFULL);
        r->n[2] = a->n[1] >> 40 | ((a->n[2] << 24) & 0xFFFFFFFFFFFFFULL);
        r->n[3] = a->n[2] >> 28 | ((a->n[3] << 36) & 0xFFFFFFFFFFFFFULL);
        r->n[4] = a->n[3] >> 16;
    #ifdef VERIFY
        r->magnitude = 1;
        r->normalized = 1;
    #endif
        */
}

pub fn fe_from_signed62(
        r: *mut Fe,
        a: *const ModInv64Signed62)  {
    
    todo!();
        /*
            const uint64_t M52 = UINT64_MAX >> 12;
        const uint64_t a0 = a->v[0], a1 = a->v[1], a2 = a->v[2], a3 = a->v[3], a4 = a->v[4];

        /* The output from modinv64{_var} should be normalized to range [0,modulus), and
         * have limbs in [0,2^62). The modulus is < 2^256, so the top limb must be below 2^(256-62*4).
         */
        VERIFY_CHECK(a0 >> 62 == 0);
        VERIFY_CHECK(a1 >> 62 == 0);
        VERIFY_CHECK(a2 >> 62 == 0);
        VERIFY_CHECK(a3 >> 62 == 0);
        VERIFY_CHECK(a4 >> 8 == 0);

        r->n[0] =  a0                   & M52;
        r->n[1] = (a0 >> 52 | a1 << 10) & M52;
        r->n[2] = (a1 >> 42 | a2 << 20) & M52;
        r->n[3] = (a2 >> 32 | a3 << 30) & M52;
        r->n[4] = (a3 >> 22 | a4 << 40);

    #ifdef VERIFY
        r->magnitude = 1;
        r->normalized = 1;
        fe_verify(r);
    #endif
        */
}

pub fn fe_to_signed62(
        r: *mut ModInv64Signed62,
        a: *const Fe)  {
    
    todo!();
        /*
            const uint64_t M62 = UINT64_MAX >> 2;
        const uint64_t a0 = a->n[0], a1 = a->n[1], a2 = a->n[2], a3 = a->n[3], a4 = a->n[4];

    #ifdef VERIFY
        VERIFY_CHECK(a->normalized);
    #endif

        r->v[0] = (a0       | a1 << 52) & M62;
        r->v[1] = (a1 >> 10 | a2 << 42) & M62;
        r->v[2] = (a2 >> 20 | a3 << 32) & M62;
        r->v[3] = (a3 >> 30 | a4 << 22) & M62;
        r->v[4] =  a4 >> 40;
        */
}

lazy_static!{
    /*
    static const modinv64_modinfo const_modinfo_fe = {
        {{-0x1000003D1LL, 0, 0, 0, 256}},
        0x27C7F6E22DDACACFLL
    };
    */
}

pub fn fe_inv(
        r: *mut Fe,
        x: *const Fe)  {
    
    todo!();
        /*
            Fe tmp;
        modinv64_signed62 s;

        tmp = *x;
        fe_normalize(&tmp);
        fe_to_signed62(&s, &tmp);
        modinv64(&s, &const_modinfo_fe);
        fe_from_signed62(r, &s);

    #ifdef VERIFY
        VERIFY_CHECK(fe_normalizes_to_zero(r) == fe_normalizes_to_zero(&tmp));
    #endif
        */
}

pub fn fe_inv_var(
        r: *mut Fe,
        x: *const Fe)  {
    
    todo!();
        /*
            Fe tmp;
        modinv64_signed62 s;

        tmp = *x;
        fe_normalize_var(&tmp);
        fe_to_signed62(&s, &tmp);
        modinv64_var(&s, &const_modinfo_fe);
        fe_from_signed62(r, &s);

    #ifdef VERIFY
        VERIFY_CHECK(fe_normalizes_to_zero(r) == fe_normalizes_to_zero(&tmp));
    #endif
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_5x52_asm_impl.h]

/**
  | Changelog:
  | 
  | - March 2013, Diederik Huys: original
  | version
  | 
  | - November 2014, Pieter Wuille: updated
  | to use Peter Dettman's parallel multiplication
  | algorithm
  | 
  | - December 2014, Pieter Wuille: converted
  | from YASM to GCC inline assembly
  |
  */
#[inline] pub fn secp_256k1_fe_mul_inner(
        r: *mut u64,
        a: *const u64,
        b: *const u64)  {
    
    todo!();
        /*
            /**
     * Registers: rdx:rax = multiplication accumulator
     *            r9:r8   = c
     *            r15:rcx = d
     *            r10-r14 = a0-a4
     *            rbx     = b
     *            rdi     = r
     *            rsi     = a / t?
     */
      uint64_t tmp1, tmp2, tmp3;
    __asm__ __volatile__(
        "movq 0(%%rsi),%%r10\n"
        "movq 8(%%rsi),%%r11\n"
        "movq 16(%%rsi),%%r12\n"
        "movq 24(%%rsi),%%r13\n"
        "movq 32(%%rsi),%%r14\n"

        /* d += a3 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "movq %%rax,%%rcx\n"
        "movq %%rdx,%%r15\n"
        /* d += a2 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a1 * b2 */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d = a0 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* c = a4 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "movq %%rax,%%r8\n"
        "movq %%rdx,%%r9\n"
        /* d += (c & M) * R */
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* c >>= 52 (%%r8 only) */
        "shrdq $52,%%r9,%%r8\n"
        /* t3 (tmp1) = d & M */
        "movq %%rcx,%%rsi\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rsi\n"
        "movq %%rsi,%q1\n"
        /* d >>= 52 */
        "shrdq $52,%%r15,%%rcx\n"
        "xorq %%r15,%%r15\n"
        /* d += a4 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a3 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a2 * b2 */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a1 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a0 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += c * R */
        "movq %%r8,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* t4 = d & M (%%rsi) */
        "movq %%rcx,%%rsi\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rsi\n"
        /* d >>= 52 */
        "shrdq $52,%%r15,%%rcx\n"
        "xorq %%r15,%%r15\n"
        /* tx = t4 >> 48 (tmp3) */
        "movq %%rsi,%%rax\n"
        "shrq $48,%%rax\n"
        "movq %%rax,%q3\n"
        /* t4 &= (M >> 4) (tmp2) */
        "movq $0xffffffffffff,%%rax\n"
        "andq %%rax,%%rsi\n"
        "movq %%rsi,%q2\n"
        /* c = a0 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "movq %%rax,%%r8\n"
        "movq %%rdx,%%r9\n"
        /* d += a4 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a3 * b2 */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a2 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a1 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* u0 = d & M (%%rsi) */
        "movq %%rcx,%%rsi\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rsi\n"
        /* d >>= 52 */
        "shrdq $52,%%r15,%%rcx\n"
        "xorq %%r15,%%r15\n"
        /* u0 = (u0 << 4) | tx (%%rsi) */
        "shlq $4,%%rsi\n"
        "movq %q3,%%rax\n"
        "orq %%rax,%%rsi\n"
        /* c += u0 * (R >> 4) */
        "movq $0x1000003d1,%%rax\n"
        "mulq %%rsi\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* r[0] = c & M */
        "movq %%r8,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq %%rax,0(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += a1 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* c += a0 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d += a4 * b2 */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a3 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a2 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* c += (d & M) * R */
        "movq %%rcx,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d >>= 52 */
        "shrdq $52,%%r15,%%rcx\n"
        "xorq %%r15,%%r15\n"
        /* r[1] = c & M */
        "movq %%r8,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq %%rax,8(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += a2 * b0 */
        "movq 0(%%rbx),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* c += a1 * b1 */
        "movq 8(%%rbx),%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* c += a0 * b2 (last use of %%r10 = a0) */
        "movq 16(%%rbx),%%rax\n"
        "mulq %%r10\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* fetch t3 (%%r10, overwrites a0), t4 (%%rsi) */
        "movq %q2,%%rsi\n"
        "movq %q1,%%r10\n"
        /* d += a4 * b3 */
        "movq 24(%%rbx),%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* d += a3 * b4 */
        "movq 32(%%rbx),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rcx\n"
        "adcq %%rdx,%%r15\n"
        /* c += (d & M) * R */
        "movq %%rcx,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d >>= 52 (%%rcx only) */
        "shrdq $52,%%r15,%%rcx\n"
        /* r[2] = c & M */
        "movq %%r8,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq %%rax,16(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += t3 */
        "addq %%r10,%%r8\n"
        /* c += d * R */
        "movq %%rcx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* r[3] = c & M */
        "movq %%r8,%%rax\n"
        "movq $0xfffffffffffff,%%rdx\n"
        "andq %%rdx,%%rax\n"
        "movq %%rax,24(%%rdi)\n"
        /* c >>= 52 (%%r8 only) */
        "shrdq $52,%%r9,%%r8\n"
        /* c += t4 (%%r8 only) */
        "addq %%rsi,%%r8\n"
        /* r[4] = c */
        "movq %%r8,32(%%rdi)\n"
    : "+S"(a), "=m"(tmp1), "=m"(tmp2), "=m"(tmp3)
    : "b"(b), "D"(r)
    : "%rax", "%rcx", "%rdx", "%r8", "%r9", "%r10", "%r11", "%r12", "%r13", "%r14", "%r15", "cc", "memory"
    );
        */
}

#[inline] pub fn secp_256k1_fe_sqr_inner(
        r: *mut u64,
        a: *const u64)  {
    
    todo!();
        /*
            /**
     * Registers: rdx:rax = multiplication accumulator
     *            r9:r8   = c
     *            rcx:rbx = d
     *            r10-r14 = a0-a4
     *            r15     = M (0xfffffffffffff)
     *            rdi     = r
     *            rsi     = a / t?
     */
      uint64_t tmp1, tmp2, tmp3;
    __asm__ __volatile__(
        "movq 0(%%rsi),%%r10\n"
        "movq 8(%%rsi),%%r11\n"
        "movq 16(%%rsi),%%r12\n"
        "movq 24(%%rsi),%%r13\n"
        "movq 32(%%rsi),%%r14\n"
        "movq $0xfffffffffffff,%%r15\n"

        /* d = (a0*2) * a3 */
        "leaq (%%r10,%%r10,1),%%rax\n"
        "mulq %%r13\n"
        "movq %%rax,%%rbx\n"
        "movq %%rdx,%%rcx\n"
        /* d += (a1*2) * a2 */
        "leaq (%%r11,%%r11,1),%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* c = a4 * a4 */
        "movq %%r14,%%rax\n"
        "mulq %%r14\n"
        "movq %%rax,%%r8\n"
        "movq %%rdx,%%r9\n"
        /* d += (c & M) * R */
        "andq %%r15,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* c >>= 52 (%%r8 only) */
        "shrdq $52,%%r9,%%r8\n"
        /* t3 (tmp1) = d & M */
        "movq %%rbx,%%rsi\n"
        "andq %%r15,%%rsi\n"
        "movq %%rsi,%q1\n"
        /* d >>= 52 */
        "shrdq $52,%%rcx,%%rbx\n"
        "xorq %%rcx,%%rcx\n"
        /* a4 *= 2 */
        "addq %%r14,%%r14\n"
        /* d += a0 * a4 */
        "movq %%r10,%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d+= (a1*2) * a3 */
        "leaq (%%r11,%%r11,1),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d += a2 * a2 */
        "movq %%r12,%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d += c * R */
        "movq %%r8,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* t4 = d & M (%%rsi) */
        "movq %%rbx,%%rsi\n"
        "andq %%r15,%%rsi\n"
        /* d >>= 52 */
        "shrdq $52,%%rcx,%%rbx\n"
        "xorq %%rcx,%%rcx\n"
        /* tx = t4 >> 48 (tmp3) */
        "movq %%rsi,%%rax\n"
        "shrq $48,%%rax\n"
        "movq %%rax,%q3\n"
        /* t4 &= (M >> 4) (tmp2) */
        "movq $0xffffffffffff,%%rax\n"
        "andq %%rax,%%rsi\n"
        "movq %%rsi,%q2\n"
        /* c = a0 * a0 */
        "movq %%r10,%%rax\n"
        "mulq %%r10\n"
        "movq %%rax,%%r8\n"
        "movq %%rdx,%%r9\n"
        /* d += a1 * a4 */
        "movq %%r11,%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d += (a2*2) * a3 */
        "leaq (%%r12,%%r12,1),%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* u0 = d & M (%%rsi) */
        "movq %%rbx,%%rsi\n"
        "andq %%r15,%%rsi\n"
        /* d >>= 52 */
        "shrdq $52,%%rcx,%%rbx\n"
        "xorq %%rcx,%%rcx\n"
        /* u0 = (u0 << 4) | tx (%%rsi) */
        "shlq $4,%%rsi\n"
        "movq %q3,%%rax\n"
        "orq %%rax,%%rsi\n"
        /* c += u0 * (R >> 4) */
        "movq $0x1000003d1,%%rax\n"
        "mulq %%rsi\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* r[0] = c & M */
        "movq %%r8,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq %%rax,0(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* a0 *= 2 */
        "addq %%r10,%%r10\n"
        /* c += a0 * a1 */
        "movq %%r10,%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d += a2 * a4 */
        "movq %%r12,%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* d += a3 * a3 */
        "movq %%r13,%%rax\n"
        "mulq %%r13\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* c += (d & M) * R */
        "movq %%rbx,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d >>= 52 */
        "shrdq $52,%%rcx,%%rbx\n"
        "xorq %%rcx,%%rcx\n"
        /* r[1] = c & M */
        "movq %%r8,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq %%rax,8(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += a0 * a2 (last use of %%r10) */
        "movq %%r10,%%rax\n"
        "mulq %%r12\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* fetch t3 (%%r10, overwrites a0),t4 (%%rsi) */
        "movq %q2,%%rsi\n"
        "movq %q1,%%r10\n"
        /* c += a1 * a1 */
        "movq %%r11,%%rax\n"
        "mulq %%r11\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d += a3 * a4 */
        "movq %%r13,%%rax\n"
        "mulq %%r14\n"
        "addq %%rax,%%rbx\n"
        "adcq %%rdx,%%rcx\n"
        /* c += (d & M) * R */
        "movq %%rbx,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* d >>= 52 (%%rbx only) */
        "shrdq $52,%%rcx,%%rbx\n"
        /* r[2] = c & M */
        "movq %%r8,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq %%rax,16(%%rdi)\n"
        /* c >>= 52 */
        "shrdq $52,%%r9,%%r8\n"
        "xorq %%r9,%%r9\n"
        /* c += t3 */
        "addq %%r10,%%r8\n"
        /* c += d * R */
        "movq %%rbx,%%rax\n"
        "movq $0x1000003d10,%%rdx\n"
        "mulq %%rdx\n"
        "addq %%rax,%%r8\n"
        "adcq %%rdx,%%r9\n"
        /* r[3] = c & M */
        "movq %%r8,%%rax\n"
        "andq %%r15,%%rax\n"
        "movq %%rax,24(%%rdi)\n"
        /* c >>= 52 (%%r8 only) */
        "shrdq $52,%%r9,%%r8\n"
        /* c += t4 (%%r8 only) */
        "addq %%rsi,%%r8\n"
        /* r[4] = c */
        "movq %%r8,32(%%rdi)\n"
    : "+S"(a), "=m"(tmp1), "=m"(tmp2), "=m"(tmp3)
    : "D"(r)
    : "%rax", "%rbx", "%rcx", "%rdx", "%r8", "%r9", "%r10", "%r11", "%r12", "%r13", "%r14", "%r15", "cc", "memory"
    );
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_5x52_int128_impl.h]

#[cfg(VERIFY)]
macro_rules! verify_bits {
    ($x:ident, $n:ident) => {
        /*
                VERIFY_CHECK(((x) >> (n)) == 0)
        */
    }
}

#[cfg(not(VERIFY))]
macro_rules! verify_bits {
    ($x:ident, $n:ident) => {
        /*
                do { } while(0)
        */
    }
}

#[inline] pub fn fe_mul_inner(
        r: *mut u64,
        a: *const u64,
        b: *const u64)  {
    
    todo!();
        /*
            uint128_t c, d;
        uint64_t t3, t4, tx, u0;
        uint64_t a0 = a[0], a1 = a[1], a2 = a[2], a3 = a[3], a4 = a[4];
        const uint64_t M = 0xFFFFFFFFFFFFFULL, R = 0x1000003D10ULL;

        VERIFY_BITS(a[0], 56);
        VERIFY_BITS(a[1], 56);
        VERIFY_BITS(a[2], 56);
        VERIFY_BITS(a[3], 56);
        VERIFY_BITS(a[4], 52);
        VERIFY_BITS(b[0], 56);
        VERIFY_BITS(b[1], 56);
        VERIFY_BITS(b[2], 56);
        VERIFY_BITS(b[3], 56);
        VERIFY_BITS(b[4], 52);
        VERIFY_CHECK(r != b);
        VERIFY_CHECK(a != b);

        /*  [... a b c] is a shorthand for ... + a<<104 + b<<52 + c<<0 mod n.
         *  for 0 <= x <= 4, px is a shorthand for sum(a[i]*b[x-i], i=0..x).
         *  for 4 <= x <= 8, px is a shorthand for sum(a[i]*b[x-i], i=(x-4)..4)
         *  Note that [x 0 0 0 0 0] = [x*R].
         */

        d  = (uint128_t)a0 * b[3]
           + (uint128_t)a1 * b[2]
           + (uint128_t)a2 * b[1]
           + (uint128_t)a3 * b[0];
        VERIFY_BITS(d, 114);
        /* [d 0 0 0] = [p3 0 0 0] */
        c  = (uint128_t)a4 * b[4];
        VERIFY_BITS(c, 112);
        /* [c 0 0 0 0 d 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */
        d += (c & M) * R; c >>= 52;
        VERIFY_BITS(d, 115);
        VERIFY_BITS(c, 60);
        /* [c 0 0 0 0 0 d 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */
        t3 = d & M; d >>= 52;
        VERIFY_BITS(t3, 52);
        VERIFY_BITS(d, 63);
        /* [c 0 0 0 0 d t3 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */

        d += (uint128_t)a0 * b[4]
           + (uint128_t)a1 * b[3]
           + (uint128_t)a2 * b[2]
           + (uint128_t)a3 * b[1]
           + (uint128_t)a4 * b[0];
        VERIFY_BITS(d, 115);
        /* [c 0 0 0 0 d t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */
        d += c * R;
        VERIFY_BITS(d, 116);
        /* [d t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */
        t4 = d & M; d >>= 52;
        VERIFY_BITS(t4, 52);
        VERIFY_BITS(d, 64);
        /* [d t4 t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */
        tx = (t4 >> 48); t4 &= (M >> 4);
        VERIFY_BITS(tx, 4);
        VERIFY_BITS(t4, 48);
        /* [d t4+(tx<<48) t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */

        c  = (uint128_t)a0 * b[0];
        VERIFY_BITS(c, 112);
        /* [d t4+(tx<<48) t3 0 0 c] = [p8 0 0 0 p4 p3 0 0 p0] */
        d += (uint128_t)a1 * b[4]
           + (uint128_t)a2 * b[3]
           + (uint128_t)a3 * b[2]
           + (uint128_t)a4 * b[1];
        VERIFY_BITS(d, 115);
        /* [d t4+(tx<<48) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        u0 = d & M; d >>= 52;
        VERIFY_BITS(u0, 52);
        VERIFY_BITS(d, 63);
        /* [d u0 t4+(tx<<48) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        /* [d 0 t4+(tx<<48)+(u0<<52) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        u0 = (u0 << 4) | tx;
        VERIFY_BITS(u0, 56);
        /* [d 0 t4+(u0<<48) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        c += (uint128_t)u0 * (R >> 4);
        VERIFY_BITS(c, 115);
        /* [d 0 t4 t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        r[0] = c & M; c >>= 52;
        VERIFY_BITS(r[0], 52);
        VERIFY_BITS(c, 61);
        /* [d 0 t4 t3 0 c r0] = [p8 0 0 p5 p4 p3 0 0 p0] */

        c += (uint128_t)a0 * b[1]
           + (uint128_t)a1 * b[0];
        VERIFY_BITS(c, 114);
        /* [d 0 t4 t3 0 c r0] = [p8 0 0 p5 p4 p3 0 p1 p0] */
        d += (uint128_t)a2 * b[4]
           + (uint128_t)a3 * b[3]
           + (uint128_t)a4 * b[2];
        VERIFY_BITS(d, 114);
        /* [d 0 t4 t3 0 c r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */
        c += (d & M) * R; d >>= 52;
        VERIFY_BITS(c, 115);
        VERIFY_BITS(d, 62);
        /* [d 0 0 t4 t3 0 c r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */
        r[1] = c & M; c >>= 52;
        VERIFY_BITS(r[1], 52);
        VERIFY_BITS(c, 63);
        /* [d 0 0 t4 t3 c r1 r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */

        c += (uint128_t)a0 * b[2]
           + (uint128_t)a1 * b[1]
           + (uint128_t)a2 * b[0];
        VERIFY_BITS(c, 114);
        /* [d 0 0 t4 t3 c r1 r0] = [p8 0 p6 p5 p4 p3 p2 p1 p0] */
        d += (uint128_t)a3 * b[4]
           + (uint128_t)a4 * b[3];
        VERIFY_BITS(d, 114);
        /* [d 0 0 t4 t3 c t1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        c += (d & M) * R; d >>= 52;
        VERIFY_BITS(c, 115);
        VERIFY_BITS(d, 62);
        /* [d 0 0 0 t4 t3 c r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        /* [d 0 0 0 t4 t3 c r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        r[2] = c & M; c >>= 52;
        VERIFY_BITS(r[2], 52);
        VERIFY_BITS(c, 63);
        /* [d 0 0 0 t4 t3+c r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        c   += d * R + t3;
        VERIFY_BITS(c, 100);
        /* [t4 c r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        r[3] = c & M; c >>= 52;
        VERIFY_BITS(r[3], 52);
        VERIFY_BITS(c, 48);
        /* [t4+c r3 r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        c   += t4;
        VERIFY_BITS(c, 49);
        /* [c r3 r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        r[4] = c;
        VERIFY_BITS(r[4], 49);
        /* [r4 r3 r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        */
}

#[inline] pub fn fe_sqr_inner(
        r: *mut u64,
        a: *const u64)  {
    
    todo!();
        /*
            uint128_t c, d;
        uint64_t a0 = a[0], a1 = a[1], a2 = a[2], a3 = a[3], a4 = a[4];
        int64_t t3, t4, tx, u0;
        const uint64_t M = 0xFFFFFFFFFFFFFULL, R = 0x1000003D10ULL;

        VERIFY_BITS(a[0], 56);
        VERIFY_BITS(a[1], 56);
        VERIFY_BITS(a[2], 56);
        VERIFY_BITS(a[3], 56);
        VERIFY_BITS(a[4], 52);

        /**  [... a b c] is a shorthand for ... + a<<104 + b<<52 + c<<0 mod n.
         *  px is a shorthand for sum(a[i]*a[x-i], i=0..x).
         *  Note that [x 0 0 0 0 0] = [x*R].
         */

        d  = (uint128_t)(a0*2) * a3
           + (uint128_t)(a1*2) * a2;
        VERIFY_BITS(d, 114);
        /* [d 0 0 0] = [p3 0 0 0] */
        c  = (uint128_t)a4 * a4;
        VERIFY_BITS(c, 112);
        /* [c 0 0 0 0 d 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */
        d += (c & M) * R; c >>= 52;
        VERIFY_BITS(d, 115);
        VERIFY_BITS(c, 60);
        /* [c 0 0 0 0 0 d 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */
        t3 = d & M; d >>= 52;
        VERIFY_BITS(t3, 52);
        VERIFY_BITS(d, 63);
        /* [c 0 0 0 0 d t3 0 0 0] = [p8 0 0 0 0 p3 0 0 0] */

        a4 *= 2;
        d += (uint128_t)a0 * a4
           + (uint128_t)(a1*2) * a3
           + (uint128_t)a2 * a2;
        VERIFY_BITS(d, 115);
        /* [c 0 0 0 0 d t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */
        d += c * R;
        VERIFY_BITS(d, 116);
        /* [d t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */
        t4 = d & M; d >>= 52;
        VERIFY_BITS(t4, 52);
        VERIFY_BITS(d, 64);
        /* [d t4 t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */
        tx = (t4 >> 48); t4 &= (M >> 4);
        VERIFY_BITS(tx, 4);
        VERIFY_BITS(t4, 48);
        /* [d t4+(tx<<48) t3 0 0 0] = [p8 0 0 0 p4 p3 0 0 0] */

        c  = (uint128_t)a0 * a0;
        VERIFY_BITS(c, 112);
        /* [d t4+(tx<<48) t3 0 0 c] = [p8 0 0 0 p4 p3 0 0 p0] */
        d += (uint128_t)a1 * a4
           + (uint128_t)(a2*2) * a3;
        VERIFY_BITS(d, 114);
        /* [d t4+(tx<<48) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        u0 = d & M; d >>= 52;
        VERIFY_BITS(u0, 52);
        VERIFY_BITS(d, 62);
        /* [d u0 t4+(tx<<48) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        /* [d 0 t4+(tx<<48)+(u0<<52) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        u0 = (u0 << 4) | tx;
        VERIFY_BITS(u0, 56);
        /* [d 0 t4+(u0<<48) t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        c += (uint128_t)u0 * (R >> 4);
        VERIFY_BITS(c, 113);
        /* [d 0 t4 t3 0 0 c] = [p8 0 0 p5 p4 p3 0 0 p0] */
        r[0] = c & M; c >>= 52;
        VERIFY_BITS(r[0], 52);
        VERIFY_BITS(c, 61);
        /* [d 0 t4 t3 0 c r0] = [p8 0 0 p5 p4 p3 0 0 p0] */

        a0 *= 2;
        c += (uint128_t)a0 * a1;
        VERIFY_BITS(c, 114);
        /* [d 0 t4 t3 0 c r0] = [p8 0 0 p5 p4 p3 0 p1 p0] */
        d += (uint128_t)a2 * a4
           + (uint128_t)a3 * a3;
        VERIFY_BITS(d, 114);
        /* [d 0 t4 t3 0 c r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */
        c += (d & M) * R; d >>= 52;
        VERIFY_BITS(c, 115);
        VERIFY_BITS(d, 62);
        /* [d 0 0 t4 t3 0 c r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */
        r[1] = c & M; c >>= 52;
        VERIFY_BITS(r[1], 52);
        VERIFY_BITS(c, 63);
        /* [d 0 0 t4 t3 c r1 r0] = [p8 0 p6 p5 p4 p3 0 p1 p0] */

        c += (uint128_t)a0 * a2
           + (uint128_t)a1 * a1;
        VERIFY_BITS(c, 114);
        /* [d 0 0 t4 t3 c r1 r0] = [p8 0 p6 p5 p4 p3 p2 p1 p0] */
        d += (uint128_t)a3 * a4;
        VERIFY_BITS(d, 114);
        /* [d 0 0 t4 t3 c r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        c += (d & M) * R; d >>= 52;
        VERIFY_BITS(c, 115);
        VERIFY_BITS(d, 62);
        /* [d 0 0 0 t4 t3 c r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        r[2] = c & M; c >>= 52;
        VERIFY_BITS(r[2], 52);
        VERIFY_BITS(c, 63);
        /* [d 0 0 0 t4 t3+c r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */

        c   += d * R + t3;
        VERIFY_BITS(c, 100);
        /* [t4 c r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        r[3] = c & M; c >>= 52;
        VERIFY_BITS(r[3], 52);
        VERIFY_BITS(c, 48);
        /* [t4+c r3 r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        c   += t4;
        VERIFY_BITS(c, 49);
        /* [c r3 r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        r[4] = c;
        VERIFY_BITS(r[4], 49);
        /* [r4 r3 r2 r1 r0] = [p8 p7 p6 p5 p4 p3 p2 p1 p0] */
        */
}
