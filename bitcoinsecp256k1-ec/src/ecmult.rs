crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult.h]

pub struct EcMultContext {

    /*
      | For accelerating the computation of
      | a*P + b*G:
      |
      */

    //NOTE: these vectors were slices before we
    //wanted a const constructor.
    //
    //perhaps this will cause problems...
    //perhaps not

    /**
      | odd multiples of the generator
      |
      */
    pre_g:     Vec<*mut GeStorage>,

    /**
      | odd multiples of 2^128*generator
      |
      */
    pre_g_128: Vec<*mut GeStorage>,
}

impl EcMultContext {

    pub const fn new() -> Self {
        Self {
            pre_g:     vec![],
            pre_g_128: vec![],
        }
    }
}

pub type EcMultMultiCallback = fn(
    sc:   *mut Scalar,
    pt:   *mut Ge,
    idx:  usize,
    data: *mut c_void
) -> i32;

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_impl.h]

/**
  | We need to lower these values for exhaustive
  | tests because the tables cannot have
  | infinities in them (this breaks the
  | affine-isomorphism stuff which tracks
  | z-ratios)
  |
  */
#[cfg(EXHAUSTIVE_TEST_ORDER)]
lazy_static!{
    /*
    #  if EXHAUSTIVE_TEST_ORDER > 128
    #    define WINDOW_A 5
    #    define WINDOW_G 8
    #  elif EXHAUSTIVE_TEST_ORDER > 8
    #    define WINDOW_A 4
    #    define WINDOW_G 4
    #  else
    #    define WINDOW_A 2
    #    define WINDOW_G 2
    #  endif
    */
}

/**
  | optimal for 128-bit and 256-bit exponents.
  |
  */
#[cfg(not(EXHAUSTIVE_TEST_ORDER))]
pub const WINDOW_A: usize = 5;

/** 
 | Larger values for ECMULT_WINDOW_SIZE result in
 | possibly better performance at the cost of an
 | exponentially larger precomputed table. The
 | exact table size is
 |
 |      (1 << (WINDOW_G - 2))
 |      * sizeof(ge_storage)  bytes,
 |
 |  where sizeof(ge_storage) is
 |  typically 64 bytes but can be larger due to
 |  platform-specific padding and alignment.
 |
 |  Two tables of this size are used (due to the
 |  endomorphism optimization).
 */
#[cfg(not(EXHAUSTIVE_TEST_ORDER))]
pub const WINDOW_G: usize = ECMULT_WINDOW_SIZE;

/** 
 | Noone will ever need more than a window size of
 | 24. The code might be correct for larger values
 | of ECMULT_WINDOW_SIZE but this is not not
 | tested.
 |
 | The following limitations are known, and there are probably more:
 |
 | If WINDOW_G > 27 and size_t has 32 bits, then
 | the code is incorrect because the size of the
 | memory object that we allocate (in bytes) will
 | not fit in a size_t.
 |
 | If WINDOW_G > 31 and int has 32 bits, then the
 | code is incorrect because certain expressions
 | will overflow.
 error msg: "Set ECMULT_WINDOW_SIZE to an integer in range [2..24]"
 */
const_assert!{
    2 <= ECMULT_WINDOW_SIZE &&
    25 > ECMULT_WINDOW_SIZE
}

pub const WNAF_BITS: usize = 128;

macro_rules! WNAF_SIZE_BITS {
    ($bits:ident, $w:ident) => {
        /*
                (((bits) + (w) - 1) / (w))
        */
    }
}

macro_rules! WNAF_SIZE {
    ($w:ident) => {
        /*
                WNAF_SIZE_BITS(WNAF_BITS, w)
        */
    }
}

/**
  | The number of entries a table with precomputed
  | multiples needs to have.
  |
  */
macro_rules! ECMULT_TABLE_SIZE {
    ($w:ident) => {
        /*
                (1 << ((w)-2))
        */
    }
}

/**
  | The number of objects allocated on the
  | scratch space for ecmult_multi algorithms
  |
  */
pub const PIPPENGER_SCRATCH_OBJECTS:   usize = 6;
pub const STRAUSS_SCRATCH_OBJECTS:     usize = 6;
pub const PIPPENGER_MAX_BUCKET_WINDOW: usize = 12;

/**
  | Minimum number of points for which pippenger_wnaf
  | is faster than strauss wnaf
  |
  */
pub const ECMULT_PIPPENGER_THRESHOLD:  usize = 88;
pub const ECMULT_MAX_POINTS_PER_BATCH: usize = 5000000;

/**
  | Fill a table 'prej' with precomputed
  | odd multiples of a. Prej will contain
  | the values [1*a,3*a,...,(2*n-1)*a],
  | so it space for n values. zr[0] will contain
  | prej[0].z / a.z. The other zr[i] values
  | = prej[i].z / prej[i-1].z.
  | 
  | Prej's Z values are undefined, except
  | for the last value.
  |
  */
pub fn ecmult_odd_multiples_table(
        n:    i32,
        prej: *mut Gej,
        zr:   *mut Fe,
        a:    *const Gej)  {
    
    todo!();
        /*
        gej d;
        ge a_ge, d_ge;
        int i;

        VERIFY_CHECK(!a->infinity);

        gej_double_var(&d, a, NULL);

        /*
         * Perform the additions on an isomorphism where 'd' is affine: drop the z coordinate
         * of 'd', and scale the 1P starting value's x/y coordinates without changing its z.
         */
        d_ge.x = d.x;
        d_ge.y = d.y;
        d_ge.infinity = 0;

        ge_set_gej_zinv(&a_ge, a, &d.z);
        prej[0].x = a_ge.x;
        prej[0].y = a_ge.y;
        prej[0].z = a->z;
        prej[0].infinity = 0;

        zr[0] = d.z;
        for (i = 1; i < n; i++) {
            gej_add_ge_var(&prej[i], &prej[i-1], &d_ge, &zr[i]);
        }

        /*
         * Each point in 'prej' has a z coordinate too small by a factor of 'd.z'. Only
         * the final point's z coordinate is actually used though, so just update that.
         */
        fe_mul(&prej[n-1].z, &prej[n-1].z, &d.z);
        */
}

/**
  | Fill a table 'pre' with precomputed
  | odd multiples of a.
  | 
  | There are two versions of this function:
  | 
  | - ecmult_odd_multiples_table_globalz_windowa
  | which brings its resulting point set
  | to a single constant Z denominator,
  | stores the X and Y coordinates as ge_storage
  | points in pre, and stores the global
  | Z in rz.
  | 
  | It only operates on tables sized for
  | WINDOW_A wnaf multiples.
  | 
  | - ecmult_odd_multiples_table_storage_var,
  | which converts its resulting point
  | set to actually affine points, and stores
  | those in pre.
  | 
  | It operates on tables of any size.
  | 
  | To compute a*P + b*G, we compute a table
  | for P using the first function, and for
  | G using the second (which requires an
  | inverse, but it only needs to happen
  | once).
  |
  */
pub fn ecmult_odd_multiples_table_globalz_windowa(
        pre:     *mut Ge,
        globalz: *mut Fe,
        a:       *const Gej)  {
    
    todo!();
        /*
        gej prej[ECMULT_TABLE_SIZE(WINDOW_A)];
        fe zr[ECMULT_TABLE_SIZE(WINDOW_A)];

        /* Compute the odd multiples in Jacobian form. */
        ecmult_odd_multiples_table(ECMULT_TABLE_SIZE(WINDOW_A), prej, zr, a);
        /* Bring them to the same Z denominator. */
        ge_globalz_set_table_gej(ECMULT_TABLE_SIZE(WINDOW_A), pre, globalz, prej, zr);
        */
}

pub fn ecmult_odd_multiples_table_storage_var(
        n:   i32,
        pre: *mut GeStorage,
        a:   *const Gej)  {
    
    todo!();
        /*
        gej d;
        ge d_ge, p_ge;
        gej pj;
        fe zi;
        fe zr;
        fe dx_over_dz_squared;
        int i;

        VERIFY_CHECK(!a->infinity);

        gej_double_var(&d, a, NULL);

        /* First, we perform all the additions in an isomorphic curve obtained by multiplying
         * all `z` coordinates by 1/`d.z`. In these coordinates `d` is affine so we can use
         * `gej_add_ge_var` to perform the additions. For each addition, we store
         * the resulting y-coordinate and the z-ratio, since we only have enough memory to
         * store two field elements. These are sufficient to efficiently undo the isomorphism
         * and recompute all the `x`s.
         */
        d_ge.x = d.x;
        d_ge.y = d.y;
        d_ge.infinity = 0;

        ge_set_gej_zinv(&p_ge, a, &d.z);
        pj.x = p_ge.x;
        pj.y = p_ge.y;
        pj.z = a->z;
        pj.infinity = 0;

        for (i = 0; i < (n - 1); i++) {
            fe_normalize_var(&pj.y);
            fe_to_storage(&pre[i].y, &pj.y);
            gej_add_ge_var(&pj, &pj, &d_ge, &zr);
            fe_normalize_var(&zr);
            fe_to_storage(&pre[i].x, &zr);
        }

        /* Invert d.z in the same batch, preserving pj.z so we can extract 1/d.z */
        fe_mul(&zi, &pj.z, &d.z);
        fe_inv_var(&zi, &zi);

        /* Directly set `pre[n - 1]` to `pj`, saving the inverted z-coordinate so
         * that we can combine it with the saved z-ratios to compute the other zs
         * without any more inversions. */
        ge_set_gej_zinv(&p_ge, &pj, &zi);
        ge_to_storage(&pre[n - 1], &p_ge);

        /* Compute the actual x-coordinate of D, which will be needed below. */
        fe_mul(&d.z, &zi, &pj.z);  /* d.z = 1/d.z */
        fe_sqr(&dx_over_dz_squared, &d.z);
        fe_mul(&dx_over_dz_squared, &dx_over_dz_squared, &d.x);

        /* Going into the second loop, we have set `pre[n-1]` to its final affine
         * form, but still need to set `pre[i]` for `i` in 0 through `n-2`. We
         * have `zi = (p.z * d.z)^-1`, where
         *
         *     `p.z` is the z-coordinate of the point on the isomorphic curve
         *           which was ultimately assigned to `pre[n-1]`.
         *     `d.z` is the multiplier that must be applied to all z-coordinates
         *           to move from our isomorphic curve back to secp256k1; so the
         *           product `p.z * d.z` is the z-coordinate of the secp256k1
         *           point assigned to `pre[n-1]`.
         *
         * All subsequent inverse-z-coordinates can be obtained by multiplying this
         * factor by successive z-ratios, which is much more efficient than directly
         * computing each one.
         *
         * Importantly, these inverse-zs will be coordinates of points on secp256k1,
         * while our other stored values come from computations on the isomorphic
         * curve. So in the below loop, we will take care not to actually use `zi`
         * or any derived values until we're back on secp256k1.
         */
        i = n - 1;
        while (i > 0) {
            fe zi2, zi3;
            const fe *rzr;
            i--;

            ge_from_storage(&p_ge, &pre[i]);

            /* For each remaining point, we extract the z-ratio from the stored
             * x-coordinate, compute its z^-1 from that, and compute the full
             * point from that. */
            rzr = &p_ge.x;
            fe_mul(&zi, &zi, rzr);
            fe_sqr(&zi2, &zi);
            fe_mul(&zi3, &zi2, &zi);
            /* To compute the actual x-coordinate, we use the stored z ratio and
             * y-coordinate, which we obtained from `gej_add_ge_var`
             * in the loop above, as well as the inverse of the square of its
             * z-coordinate. We store the latter in the `zi2` variable, which is
             * computed iteratively starting from the overall Z inverse then
             * multiplying by each z-ratio in turn.
             *
             * Denoting the z-ratio as `rzr`, we observe that it is equal to `h`
             * from the inside of the above `gej_add_ge_var` call. This satisfies
             *
             *    rzr = d_x * z^2 - x * d_z^2
             *
             * where (`d_x`, `d_z`) are Jacobian coordinates of `D` and `(x, z)`
             * are Jacobian coordinates of our desired point -- except both are on
             * the isomorphic curve that we were using when we called `gej_add_ge_var`.
             * To get back to secp256k1, we must multiply both `z`s by `d_z`, or
             * equivalently divide both `x`s by `d_z^2`. Our equation then becomes
             *
             *    rzr = d_x * z^2 / d_z^2 - x
             *
             * (The left-hand-side, being a ratio of z-coordinates, is unaffected
             * by the isomorphism.)
             *
             * Rearranging to solve for `x`, we have
             *
             *     x = d_x * z^2 / d_z^2 - rzr
             *
             * But what we actually want is the affine coordinate `X = x/z^2`,
             * which will satisfy
             *
             *     X = d_x / d_z^2 - rzr / z^2
             *       = dx_over_dz_squared - rzr * zi2
             */
            fe_mul(&p_ge.x, rzr, &zi2);
            fe_negate(&p_ge.x, &p_ge.x, 1);
            fe_add(&p_ge.x, &dx_over_dz_squared);
            /* y is stored_y/z^3, as we expect */
            fe_mul(&p_ge.y, &p_ge.y, &zi3);
            /* Store */
            ge_to_storage(&pre[i], &p_ge);
        }
        */
}

/**
  | The following two macro retrieves a
  | particular odd multiple from a table
  | of precomputed multiples.
  |
  */
macro_rules! ecmult_table_get_ge {
    ($r:ident, 
     $pre:ident, 
     $n:ident, 
     $w:ident) => {
        /*
                do { 
            VERIFY_CHECK(((n) & 1) == 1); 
            VERIFY_CHECK((n) >= -((1 << ((w)-1)) - 1)); 
            VERIFY_CHECK((n) <=  ((1 << ((w)-1)) - 1)); 
            if ((n) > 0) { 
                *(r) = (pre)[((n)-1)/2]; 
            } else { 
                *(r) = (pre)[(-(n)-1)/2]; 
                fe_negate(&((r)->y), &((r)->y), 1); 
            } 
        } while(0)
        */
    }
}

macro_rules! ecmult_table_get_ge_storage {
    ($r:ident, 
     $pre:ident, 
     $n:ident, 
     $w:ident) => {
        /*
                do { 
            VERIFY_CHECK(((n) & 1) == 1); 
            VERIFY_CHECK((n) >= -((1 << ((w)-1)) - 1)); 
            VERIFY_CHECK((n) <=  ((1 << ((w)-1)) - 1)); 
            if ((n) > 0) { 
                ge_from_storage((r), &(pre)[((n)-1)/2]); 
            } else { 
                ge_from_storage((r), &(pre)[(-(n)-1)/2]); 
                fe_negate(&((r)->y), &((r)->y), 1); 
            } 
        } while(0)
        */
    }
}

lazy_static!{
    /*
    static const size_t ECMULT_CONTEXT_PREALLOCATED_SIZE =
        ROUND_TO_ALIGN(sizeof((*((ecmult_context*) NULL)->pre_g)[0]) * ECMULT_TABLE_SIZE(WINDOW_G))
        + ROUND_TO_ALIGN(sizeof((*((ecmult_context*) NULL)->pre_g_128)[0]) * ECMULT_TABLE_SIZE(WINDOW_G))
        ;
    */
}

pub fn ecmult_context_init(ctx: *mut EcMultContext)  {
    
    todo!();
        /*
            ctx->pre_g = NULL;
        ctx->pre_g_128 = NULL;
        */
}

pub fn ecmult_context_build(
        ctx:      *mut EcMultContext,
        prealloc: *mut *mut c_void)  {
    
    todo!();
        /*
            gej gj;
        c_void* const base = *prealloc;
        size_t const prealloc_size = ECMULT_CONTEXT_PREALLOCATED_SIZE;

        if (ctx->pre_g != NULL) {
            return;
        }

        /* get the generator */
        gej_set_ge(&gj, &ge_const_g);

        {
            size_t size = sizeof((*ctx->pre_g)[0]) * ((size_t)ECMULT_TABLE_SIZE(WINDOW_G));
            /* check for overflow */
            VERIFY_CHECK(size / sizeof((*ctx->pre_g)[0]) == ((size_t)ECMULT_TABLE_SIZE(WINDOW_G)));
            ctx->pre_g = (ge_storage (*)[])manual_alloc(prealloc, sizeof((*ctx->pre_g)[0]) * ECMULT_TABLE_SIZE(WINDOW_G), base, prealloc_size);
        }

        /* precompute the tables with odd multiples */
        ecmult_odd_multiples_table_storage_var(ECMULT_TABLE_SIZE(WINDOW_G), *ctx->pre_g, &gj);

        {
            gej g_128j;
            int i;

            size_t size = sizeof((*ctx->pre_g_128)[0]) * ((size_t) ECMULT_TABLE_SIZE(WINDOW_G));
            /* check for overflow */
            VERIFY_CHECK(size / sizeof((*ctx->pre_g_128)[0]) == ((size_t)ECMULT_TABLE_SIZE(WINDOW_G)));
            ctx->pre_g_128 = (ge_storage (*)[])manual_alloc(prealloc, sizeof((*ctx->pre_g_128)[0]) * ECMULT_TABLE_SIZE(WINDOW_G), base, prealloc_size);

            /* calculate 2^128*generator */
            g_128j = gj;
            for (i = 0; i < 128; i++) {
                gej_double_var(&g_128j, &g_128j, NULL);
            }
            ecmult_odd_multiples_table_storage_var(ECMULT_TABLE_SIZE(WINDOW_G), *ctx->pre_g_128, &g_128j);
        }
        */
}

pub fn ecmult_context_finalize_memcpy(
        dst: *mut EcMultContext,
        src: *const EcMultContext)  {
    
    todo!();
        /*
            if (src->pre_g != NULL) {
            /* We cast to c_void* first to suppress a -Wcast-align warning. */
            dst->pre_g = (ge_storage (*)[])(c_void*)((unsigned char*)dst + ((unsigned char*)(src->pre_g) - (unsigned char*)src));
        }
        if (src->pre_g_128 != NULL) {
            dst->pre_g_128 = (ge_storage (*)[])(c_void*)((unsigned char*)dst + ((unsigned char*)(src->pre_g_128) - (unsigned char*)src));
        }
        */
}

pub fn ecmult_context_is_built(ctx: *const EcMultContext) -> i32 {
    
    todo!();
        /*
            return ctx->pre_g != NULL;
        */
}

pub fn ecmult_context_clear(ctx: *mut EcMultContext)  {
    
    todo!();
        /*
            ecmult_context_init(ctx);
        */
}

/**
  | Convert a number to WNAF notation. The
  | number becomes represented by sum(2^i
  | * wnaf[i], i=0..bits), with the following
  | guarantees:
  | 
  | - each wnaf[i] is either 0, or an odd integer
  | between -(1<<(w-1) - 1) and (1<<(w-1)
  | - 1)
  | 
  | - two non-zero entries in wnaf are separated
  | by at least w-1 zeroes.
  | 
  | - the number of set values in wnaf is returned.
  | This number is at most 256, and at most
  | one more than the number of bits in the
  | (absolute value) of the input.
  |
  */
pub fn ecmult_wnaf(
        wnaf: *mut i32,
        len:  i32,
        a:    *const Scalar,
        w:    i32) -> i32 {
    
    todo!();
        /*
            scalar s;
        int last_set_bit = -1;
        int bit = 0;
        int sign = 1;
        int carry = 0;

        VERIFY_CHECK(wnaf != NULL);
        VERIFY_CHECK(0 <= len && len <= 256);
        VERIFY_CHECK(a != NULL);
        VERIFY_CHECK(2 <= w && w <= 31);

        memset(wnaf, 0, len * sizeof(wnaf[0]));

        s = *a;
        if (scalar_get_bits(&s, 255, 1)) {
            scalar_negate(&s, &s);
            sign = -1;
        }

        while (bit < len) {
            int now;
            int word;
            if (scalar_get_bits(&s, bit, 1) == (unsigned int)carry) {
                bit++;
                continue;
            }

            now = w;
            if (now > len - bit) {
                now = len - bit;
            }

            word = scalar_get_bits_var(&s, bit, now) + carry;

            carry = (word >> (w-1)) & 1;
            word -= carry << w;

            wnaf[bit] = sign * word;
            last_set_bit = bit;

            bit += now;
        }
    #ifdef VERIFY
        CHECK(carry == 0);
        while (bit < 256) {
            CHECK(scalar_get_bits(&s, bit++, 1) == 0);
        }
    #endif
        return last_set_bit + 1;
        */
}

pub struct StraussPointState {
    na_1:        Scalar,
    na_lam:      Scalar,
    wnaf_na_1:   [i32; 129],
    wnaf_na_lam: [i32; 129],
    bits_na_1:   i32,
    bits_na_lam: i32,
    input_pos:   usize,
}

pub struct StraussState {
    prej:      *mut Gej,
    zr:        *mut Fe,
    pre_a:     *mut Ge,
    pre_a_lam: *mut Ge,
    ps:        *mut StraussPointState,
}

pub fn ecmult_strauss_wnaf(
        ctx:   *const EcMultContext,
        state: *const StraussState,
        r:     *mut Gej,
        num:   usize,
        a:     *const Gej,
        na:    *const Scalar,
        ng:    *const Scalar)  {
    
    todo!();
        /*
            ge tmpa;
        fe Z;
        /* Splitted G factors. */
        scalar ng_1, ng_128;
        int wnaf_ng_1[129];
        int bits_ng_1 = 0;
        int wnaf_ng_128[129];
        int bits_ng_128 = 0;
        int i;
        int bits = 0;
        size_t np;
        size_t no = 0;

        for (np = 0; np < num; ++np) {
            if (scalar_is_zero(&na[np]) || gej_is_infinity(&a[np])) {
                continue;
            }
            state->ps[no].input_pos = np;
            /* split na into na_1 and na_lam (where na = na_1 + na_lam*lambda, and na_1 and na_lam are ~128 bit) */
            scalar_split_lambda(&state->ps[no].na_1, &state->ps[no].na_lam, &na[np]);

            /* build wnaf representation for na_1 and na_lam. */
            state->ps[no].bits_na_1   = ecmult_wnaf(state->ps[no].wnaf_na_1,   129, &state->ps[no].na_1,   WINDOW_A);
            state->ps[no].bits_na_lam = ecmult_wnaf(state->ps[no].wnaf_na_lam, 129, &state->ps[no].na_lam, WINDOW_A);
            VERIFY_CHECK(state->ps[no].bits_na_1 <= 129);
            VERIFY_CHECK(state->ps[no].bits_na_lam <= 129);
            if (state->ps[no].bits_na_1 > bits) {
                bits = state->ps[no].bits_na_1;
            }
            if (state->ps[no].bits_na_lam > bits) {
                bits = state->ps[no].bits_na_lam;
            }
            ++no;
        }

        /* Calculate odd multiples of a.
         * All multiples are brought to the same Z 'denominator', which is stored
         * in Z. Due to secp256k1' isomorphism we can do all operations pretending
         * that the Z coordinate was 1, use affine addition formulae, and correct
         * the Z coordinate of the result once at the end.
         * The exception is the precomputed G table points, which are actually
         * affine. Compared to the base used for other points, they have a Z ratio
         * of 1/Z, so we can use gej_add_zinv_var, which uses the same
         * isomorphism to efficiently add with a known Z inverse.
         */
        if (no > 0) {
            /* Compute the odd multiples in Jacobian form. */
            ecmult_odd_multiples_table(ECMULT_TABLE_SIZE(WINDOW_A), state->prej, state->zr, &a[state->ps[0].input_pos]);
            for (np = 1; np < no; ++np) {
                gej tmp = a[state->ps[np].input_pos];
    #ifdef VERIFY
                fe_normalize_var(&(state->prej[(np - 1) * ECMULT_TABLE_SIZE(WINDOW_A) + ECMULT_TABLE_SIZE(WINDOW_A) - 1].z));
    #endif
                gej_rescale(&tmp, &(state->prej[(np - 1) * ECMULT_TABLE_SIZE(WINDOW_A) + ECMULT_TABLE_SIZE(WINDOW_A) - 1].z));
                ecmult_odd_multiples_table(ECMULT_TABLE_SIZE(WINDOW_A), state->prej + np * ECMULT_TABLE_SIZE(WINDOW_A), state->zr + np * ECMULT_TABLE_SIZE(WINDOW_A), &tmp);
                fe_mul(state->zr + np * ECMULT_TABLE_SIZE(WINDOW_A), state->zr + np * ECMULT_TABLE_SIZE(WINDOW_A), &(a[state->ps[np].input_pos].z));
            }
            /* Bring them to the same Z denominator. */
            ge_globalz_set_table_gej(ECMULT_TABLE_SIZE(WINDOW_A) * no, state->pre_a, &Z, state->prej, state->zr);
        } else {
            fe_set_int(&Z, 1);
        }

        for (np = 0; np < no; ++np) {
            for (i = 0; i < ECMULT_TABLE_SIZE(WINDOW_A); i++) {
                ge_mul_lambda(&state->pre_a_lam[np * ECMULT_TABLE_SIZE(WINDOW_A) + i], &state->pre_a[np * ECMULT_TABLE_SIZE(WINDOW_A) + i]);
            }
        }

        if (ng) {
            /* split ng into ng_1 and ng_128 (where gn = gn_1 + gn_128*2^128, and gn_1 and gn_128 are ~128 bit) */
            scalar_split_128(&ng_1, &ng_128, ng);

            /* Build wnaf representation for ng_1 and ng_128 */
            bits_ng_1   = ecmult_wnaf(wnaf_ng_1,   129, &ng_1,   WINDOW_G);
            bits_ng_128 = ecmult_wnaf(wnaf_ng_128, 129, &ng_128, WINDOW_G);
            if (bits_ng_1 > bits) {
                bits = bits_ng_1;
            }
            if (bits_ng_128 > bits) {
                bits = bits_ng_128;
            }
        }

        gej_set_infinity(r);

        for (i = bits - 1; i >= 0; i--) {
            int n;
            gej_double_var(r, r, NULL);
            for (np = 0; np < no; ++np) {
                if (i < state->ps[np].bits_na_1 && (n = state->ps[np].wnaf_na_1[i])) {
                    ECMULT_TABLE_GET_GE(&tmpa, state->pre_a + np * ECMULT_TABLE_SIZE(WINDOW_A), n, WINDOW_A);
                    gej_add_ge_var(r, r, &tmpa, NULL);
                }
                if (i < state->ps[np].bits_na_lam && (n = state->ps[np].wnaf_na_lam[i])) {
                    ECMULT_TABLE_GET_GE(&tmpa, state->pre_a_lam + np * ECMULT_TABLE_SIZE(WINDOW_A), n, WINDOW_A);
                    gej_add_ge_var(r, r, &tmpa, NULL);
                }
            }
            if (i < bits_ng_1 && (n = wnaf_ng_1[i])) {
                ECMULT_TABLE_GET_GE_STORAGE(&tmpa, *ctx->pre_g, n, WINDOW_G);
                gej_add_zinv_var(r, r, &tmpa, &Z);
            }
            if (i < bits_ng_128 && (n = wnaf_ng_128[i])) {
                ECMULT_TABLE_GET_GE_STORAGE(&tmpa, *ctx->pre_g_128, n, WINDOW_G);
                gej_add_zinv_var(r, r, &tmpa, &Z);
            }
        }

        if (!r->infinity) {
            fe_mul(&r->z, &r->z, &Z);
        }
        */
}

/**
  | Double multiply: R = na*A + ng*G
  |
  */
pub fn ecmult(
        ctx: *const EcMultContext,
        r:   *mut Gej,
        a:   *const Gej,
        na:  *const Scalar,
        ng:  *const Scalar)  {
    
    todo!();
        /*
            gej prej[ECMULT_TABLE_SIZE(WINDOW_A)];
        fe zr[ECMULT_TABLE_SIZE(WINDOW_A)];
        ge pre_a[ECMULT_TABLE_SIZE(WINDOW_A)];
        struct strauss_point_state ps[1];
        ge pre_a_lam[ECMULT_TABLE_SIZE(WINDOW_A)];
        struct strauss_state state;

        state.prej = prej;
        state.zr = zr;
        state.pre_a = pre_a;
        state.pre_a_lam = pre_a_lam;
        state.ps = ps;
        ecmult_strauss_wnaf(ctx, &state, r, 1, a, na, ng);
        */
}

pub fn strauss_scratch_size(n_points: usize) -> usize {
    
    todo!();
        /*
            static const size_t point_size = (2 * sizeof(ge) + sizeof(gej) + sizeof(fe)) * ECMULT_TABLE_SIZE(WINDOW_A) + sizeof(struct strauss_point_state) + sizeof(gej) + sizeof(scalar);
        return n_points*point_size;
        */
}

pub fn ecmult_strauss_batch(
        error_callback: *const Callback,
        ctx:            *const EcMultContext,
        scratch:        *mut Scratch,
        r:              *mut Gej,
        inp_g_sc:       *const Scalar,
        cb:             EcMultMultiCallback,
        cbdata:         *mut c_void,
        n_points:       usize,
        cb_offset:      usize) -> i32 {
    
    todo!();
        /*
            gej* points;
        scalar* scalars;
        struct strauss_state state;
        size_t i;
        const size_t scratch_checkpoint = scratch_checkpoint(error_callback, scratch);

        gej_set_infinity(r);
        if (inp_g_sc == NULL && n_points == 0) {
            return 1;
        }

        points = (gej*)scratch_alloc(error_callback, scratch, n_points * sizeof(gej));
        scalars = (scalar*)scratch_alloc(error_callback, scratch, n_points * sizeof(scalar));
        state.prej = (gej*)scratch_alloc(error_callback, scratch, n_points * ECMULT_TABLE_SIZE(WINDOW_A) * sizeof(gej));
        state.zr = (fe*)scratch_alloc(error_callback, scratch, n_points * ECMULT_TABLE_SIZE(WINDOW_A) * sizeof(fe));
        state.pre_a = (ge*)scratch_alloc(error_callback, scratch, n_points * ECMULT_TABLE_SIZE(WINDOW_A) * sizeof(ge));
        state.pre_a_lam = (ge*)scratch_alloc(error_callback, scratch, n_points * ECMULT_TABLE_SIZE(WINDOW_A) * sizeof(ge));
        state.ps = (struct strauss_point_state*)scratch_alloc(error_callback, scratch, n_points * sizeof(struct strauss_point_state));

        if (points == NULL || scalars == NULL || state.prej == NULL || state.zr == NULL || state.pre_a == NULL || state.pre_a_lam == NULL || state.ps == NULL) {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        for (i = 0; i < n_points; i++) {
            ge point;
            if (!cb(&scalars[i], &point, i+cb_offset, cbdata)) {
                scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
                return 0;
            }
            gej_set_ge(&points[i], &point);
        }
        ecmult_strauss_wnaf(ctx, &state, r, n_points, points, scalars, inp_g_sc);
        scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
        return 1;
        */
}

/**
  | Wrapper for ecmult_multi_func
  | interface
  |
  */
pub fn ecmult_strauss_batch_single(
        error_callback: *const Callback,
        actx:           *const EcMultContext,
        scratch:        *mut Scratch,
        r:              *mut Gej,
        inp_g_sc:       *const Scalar,
        cb:             EcMultMultiCallback,
        cbdata:         *mut c_void,
        n:              usize) -> i32 {
    
    todo!();
        /*
            return ecmult_strauss_batch(error_callback, actx, scratch, r, inp_g_sc, cb, cbdata, n, 0);
        */
}

pub fn strauss_max_points(
        error_callback: *const Callback,
        scratch:        *mut Scratch) -> usize {
    
    todo!();
        /*
            return scratch_max_allocation(error_callback, scratch, STRAUSS_SCRATCH_OBJECTS) / strauss_scratch_size(1);
        */
}

/**
  | Convert a number to WNAF notation.
  | 
  | The number becomes represented by sum(2^{wi}
  | * wnaf[i], i=0..WNAF_SIZE(w)+1) -
  | return_val.
  | 
  | It has the following guarantees:
  | 
  | - each wnaf[i] is either 0 or an odd integer
  | between -(1 << w) and (1 << w)
  | 
  | - the number of words set is always WNAF_SIZE(w)
  | 
  | - the returned skew is 0 or 1
  |
  */
pub fn wnaf_fixed(
        wnaf: *mut i32,
        s:    *const Scalar,
        w:    i32) -> i32 {
    
    todo!();
        /*
            int skew = 0;
        int pos;
        int max_pos;
        int last_w;
        const scalar *work = s;

        if (scalar_is_zero(s)) {
            for (pos = 0; pos < WNAF_SIZE(w); pos++) {
                wnaf[pos] = 0;
            }
            return 0;
        }

        if (scalar_is_even(s)) {
            skew = 1;
        }

        wnaf[0] = scalar_get_bits_var(work, 0, w) + skew;
        /* Compute last window size. Relevant when window size doesn't divide the
         * number of bits in the scalar */
        last_w = WNAF_BITS - (WNAF_SIZE(w) - 1) * w;

        /* Store the position of the first nonzero word in max_pos to allow
         * skipping leading zeros when calculating the wnaf. */
        for (pos = WNAF_SIZE(w) - 1; pos > 0; pos--) {
            int val = scalar_get_bits_var(work, pos * w, pos == WNAF_SIZE(w)-1 ? last_w : w);
            if(val != 0) {
                break;
            }
            wnaf[pos] = 0;
        }
        max_pos = pos;
        pos = 1;

        while (pos <= max_pos) {
            int val = scalar_get_bits_var(work, pos * w, pos == WNAF_SIZE(w)-1 ? last_w : w);
            if ((val & 1) == 0) {
                wnaf[pos - 1] -= (1 << w);
                wnaf[pos] = (val + 1);
            } else {
                wnaf[pos] = val;
            }
            /* Set a coefficient to zero if it is 1 or -1 and the proceeding digit
             * is strictly negative or strictly positive respectively. Only change
             * coefficients at previous positions because above code assumes that
             * wnaf[pos - 1] is odd.
             */
            if (pos >= 2 && ((wnaf[pos - 1] == 1 && wnaf[pos - 2] < 0) || (wnaf[pos - 1] == -1 && wnaf[pos - 2] > 0))) {
                if (wnaf[pos - 1] == 1) {
                    wnaf[pos - 2] += 1 << w;
                } else {
                    wnaf[pos - 2] -= 1 << w;
                }
                wnaf[pos - 1] = 0;
            }
            ++pos;
        }

        return skew;
        */
}

pub struct PippengerPointState {
    skew_na:   i32,
    input_pos: usize,
}

pub struct PippengerState {
    wnaf_na: *mut i32,
    ps:      *mut PippengerPointState,
}

/**
 | pippenger_wnaf computes the result of
 | a multi-point multiplication as follows: 
 |
 | The scalars are brought into wnaf with n_wnaf
 | elements each. 
 |
 | Then for every i < n_wnaf, first each point is
 | added to a "bucket" corresponding to the
 | point's wnaf[i]. 
 |
 | Second, the buckets are added together such
 | that r += 1*bucket[0] + 3*bucket[1]
 | + 5*bucket[2] + ...
 */
pub fn ecmult_pippenger_wnaf(
        buckets:       *mut Gej,
        bucket_window: i32,
        state:         *mut PippengerState,
        r:             *mut Gej,
        sc:            *const Scalar,
        pt:            *const Ge,
        num:           usize) -> i32 {
    
    todo!();
        /*
            size_t n_wnaf = WNAF_SIZE(bucket_window+1);
        size_t np;
        size_t no = 0;
        int i;
        int j;

        for (np = 0; np < num; ++np) {
            if (scalar_is_zero(&sc[np]) || ge_is_infinity(&pt[np])) {
                continue;
            }
            state->ps[no].input_pos = np;
            state->ps[no].skew_na = wnaf_fixed(&state->wnaf_na[no*n_wnaf], &sc[np], bucket_window+1);
            no++;
        }
        gej_set_infinity(r);

        if (no == 0) {
            return 1;
        }

        for (i = n_wnaf - 1; i >= 0; i--) {
            gej running_sum;

            for(j = 0; j < ECMULT_TABLE_SIZE(bucket_window+2); j++) {
                gej_set_infinity(&buckets[j]);
            }

            for (np = 0; np < no; ++np) {
                int n = state->wnaf_na[np*n_wnaf + i];
                struct pippenger_point_state point_state = state->ps[np];
                ge tmp;
                int idx;

                if (i == 0) {
                    /* correct for wnaf skew */
                    int skew = point_state.skew_na;
                    if (skew) {
                        ge_neg(&tmp, &pt[point_state.input_pos]);
                        gej_add_ge_var(&buckets[0], &buckets[0], &tmp, NULL);
                    }
                }
                if (n > 0) {
                    idx = (n - 1)/2;
                    gej_add_ge_var(&buckets[idx], &buckets[idx], &pt[point_state.input_pos], NULL);
                } else if (n < 0) {
                    idx = -(n + 1)/2;
                    ge_neg(&tmp, &pt[point_state.input_pos]);
                    gej_add_ge_var(&buckets[idx], &buckets[idx], &tmp, NULL);
                }
            }

            for(j = 0; j < bucket_window; j++) {
                gej_double_var(r, r, NULL);
            }

            gej_set_infinity(&running_sum);
            /* Accumulate the sum: bucket[0] + 3*bucket[1] + 5*bucket[2] + 7*bucket[3] + ...
             *                   = bucket[0] +   bucket[1] +   bucket[2] +   bucket[3] + ...
             *                   +         2 *  (bucket[1] + 2*bucket[2] + 3*bucket[3] + ...)
             * using an intermediate running sum:
             * running_sum = bucket[0] +   bucket[1] +   bucket[2] + ...
             *
             * The doubling is done implicitly by deferring the final window doubling (of 'r').
             */
            for(j = ECMULT_TABLE_SIZE(bucket_window+2) - 1; j > 0; j--) {
                gej_add_var(&running_sum, &running_sum, &buckets[j], NULL);
                gej_add_var(r, r, &running_sum, NULL);
            }

            gej_add_var(&running_sum, &running_sum, &buckets[0], NULL);
            gej_double_var(r, r, NULL);
            gej_add_var(r, r, &running_sum, NULL);
        }
        return 1;
        */
}

/**
  | Returns optimal bucket_window (number
  | of bits of a scalar represented by a set
  | of buckets) for a given number of points.
  |
  */
pub fn pippenger_bucket_window(n: usize) -> i32 {
    
    todo!();
        /*
            if (n <= 1) {
            return 1;
        } else if (n <= 4) {
            return 2;
        } else if (n <= 20) {
            return 3;
        } else if (n <= 57) {
            return 4;
        } else if (n <= 136) {
            return 5;
        } else if (n <= 235) {
            return 6;
        } else if (n <= 1260) {
            return 7;
        } else if (n <= 4420) {
            return 9;
        } else if (n <= 7880) {
            return 10;
        } else if (n <= 16050) {
            return 11;
        } else {
            return PIPPENGER_MAX_BUCKET_WINDOW;
        }
        */
}

/**
  | Returns the maximum optimal number
  | of points for a bucket_window.
  |
  */
pub fn pippenger_bucket_window_inv(bucket_window: i32) -> usize {
    
    todo!();
        /*
            switch(bucket_window) {
            case 1: return 1;
            case 2: return 4;
            case 3: return 20;
            case 4: return 57;
            case 5: return 136;
            case 6: return 235;
            case 7: return 1260;
            case 8: return 1260;
            case 9: return 4420;
            case 10: return 7880;
            case 11: return 16050;
            case PIPPENGER_MAX_BUCKET_WINDOW: return SIZE_MAX;
        }
        return 0;
        */
}

#[inline] pub fn ecmult_endo_split(
        s1: *mut Scalar,
        s2: *mut Scalar,
        p1: *mut Ge,
        p2: *mut Ge)  {
    
    todo!();
        /*
            scalar tmp = *s1;
        scalar_split_lambda(s1, s2, &tmp);
        ge_mul_lambda(p2, p1);

        if (scalar_is_high(s1)) {
            scalar_negate(s1, s1);
            ge_neg(p1, p1);
        }
        if (scalar_is_high(s2)) {
            scalar_negate(s2, s2);
            ge_neg(p2, p2);
        }
        */
}

/**
  | Returns the scratch size required for
  | a given number of points (excluding
  | base point G) without considering alignment.
  |
  */
pub fn pippenger_scratch_size(
        n_points:      usize,
        bucket_window: i32) -> usize {
    
    todo!();
        /*
            size_t entries = 2*n_points + 2;
        size_t entry_size = sizeof(ge) + sizeof(scalar) + sizeof(struct pippenger_point_state) + (WNAF_SIZE(bucket_window+1)+1)*sizeof(int);
        return (sizeof(gej) << bucket_window) + sizeof(struct pippenger_state) + entries * entry_size;
        */
}

pub fn ecmult_pippenger_batch(
        error_callback: *const Callback,
        ctx:            *const EcMultContext,
        scratch:        *mut Scratch,
        r:              *mut Gej,
        inp_g_sc:       *const Scalar,
        cb:             EcMultMultiCallback,
        cbdata:         *mut c_void,
        n_points:       usize,
        cb_offset:      usize) -> i32 {
    
    todo!();
        /*
            const size_t scratch_checkpoint = scratch_checkpoint(error_callback, scratch);
        /* Use 2(n+1) with the endomorphism, when calculating batch
         * sizes. The reason for +1 is that we add the G scalar to the list of
         * other scalars. */
        size_t entries = 2*n_points + 2;
        ge *points;
        scalar *scalars;
        gej *buckets;
        struct pippenger_state *state_space;
        size_t idx = 0;
        size_t point_idx = 0;
        int i, j;
        int bucket_window;

        (c_void)ctx;
        gej_set_infinity(r);
        if (inp_g_sc == NULL && n_points == 0) {
            return 1;
        }

        bucket_window = pippenger_bucket_window(n_points);
        points = (ge *) scratch_alloc(error_callback, scratch, entries * sizeof(*points));
        scalars = (scalar *) scratch_alloc(error_callback, scratch, entries * sizeof(*scalars));
        state_space = (struct pippenger_state *) scratch_alloc(error_callback, scratch, sizeof(*state_space));
        if (points == NULL || scalars == NULL || state_space == NULL) {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        state_space->ps = (struct pippenger_point_state *) scratch_alloc(error_callback, scratch, entries * sizeof(*state_space->ps));
        state_space->wnaf_na = (int *) scratch_alloc(error_callback, scratch, entries*(WNAF_SIZE(bucket_window+1)) * sizeof(int));
        buckets = (gej *) scratch_alloc(error_callback, scratch, (1<<bucket_window) * sizeof(*buckets));
        if (state_space->ps == NULL || state_space->wnaf_na == NULL || buckets == NULL) {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        if (inp_g_sc != NULL) {
            scalars[0] = *inp_g_sc;
            points[0] = ge_const_g;
            idx++;
            ecmult_endo_split(&scalars[0], &scalars[1], &points[0], &points[1]);
            idx++;
        }

        while (point_idx < n_points) {
            if (!cb(&scalars[idx], &points[idx], point_idx + cb_offset, cbdata)) {
                scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
                return 0;
            }
            idx++;
            ecmult_endo_split(&scalars[idx - 1], &scalars[idx], &points[idx - 1], &points[idx]);
            idx++;
            point_idx++;
        }

        ecmult_pippenger_wnaf(buckets, bucket_window, state_space, r, scalars, points, idx);

        /* Clear data */
        for(i = 0; (size_t)i < idx; i++) {
            scalar_clear(&scalars[i]);
            state_space->ps[i].skew_na = 0;
            for(j = 0; j < WNAF_SIZE(bucket_window+1); j++) {
                state_space->wnaf_na[i * WNAF_SIZE(bucket_window+1) + j] = 0;
            }
        }
        for(i = 0; i < 1<<bucket_window; i++) {
            gej_clear(&buckets[i]);
        }
        scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
        return 1;
        */
}

/**
  | Wrapper for ecmult_multi_func
  | interface
  |
  */
pub fn ecmult_pippenger_batch_single(
        error_callback: *const Callback,
        actx:           *const EcMultContext,
        scratch:        *mut Scratch,
        r:              *mut Gej,
        inp_g_sc:       *const Scalar,
        cb:             EcMultMultiCallback,
        cbdata:         *mut c_void,
        n:              usize) -> i32 {
    
    todo!();
        /*
            return ecmult_pippenger_batch(error_callback, actx, scratch, r, inp_g_sc, cb, cbdata, n, 0);
        */
}

/**
  | Returns the maximum number of points
  | in addition to G that can be used with
  | a given scratch space. The function
  | ensures that fewer points may also be
  | used.
  |
  */
pub fn pippenger_max_points(
        error_callback: *const Callback,
        scratch:        *mut Scratch) -> usize {
    
    todo!();
        /*
            size_t max_alloc = scratch_max_allocation(error_callback, scratch, PIPPENGER_SCRATCH_OBJECTS);
        int bucket_window;
        size_t res = 0;

        for (bucket_window = 1; bucket_window <= PIPPENGER_MAX_BUCKET_WINDOW; bucket_window++) {
            size_t n_points;
            size_t max_points = pippenger_bucket_window_inv(bucket_window);
            size_t space_for_points;
            size_t space_overhead;
            size_t entry_size = sizeof(ge) + sizeof(scalar) + sizeof(struct pippenger_point_state) + (WNAF_SIZE(bucket_window+1)+1)*sizeof(int);

            entry_size = 2*entry_size;
            space_overhead = (sizeof(gej) << bucket_window) + entry_size + sizeof(struct pippenger_state);
            if (space_overhead > max_alloc) {
                break;
            }
            space_for_points = max_alloc - space_overhead;

            n_points = space_for_points/entry_size;
            n_points = n_points > max_points ? max_points : n_points;
            if (n_points > res) {
                res = n_points;
            }
            if (n_points < max_points) {
                /* A larger bucket_window may support even more points. But if we
                 * would choose that then the caller couldn't safely use any number
                 * smaller than what this function returns */
                break;
            }
        }
        return res;
        */
}

/**
  | Computes ecmult_multi by simply multiplying
  | and adding each point. Does not require
  | a scratch space
  |
  */
pub fn ecmult_multi_simple_var(
        ctx:      *const EcMultContext,
        r:        *mut Gej,
        inp_g_sc: *const Scalar,
        cb:       EcMultMultiCallback,
        cbdata:   *mut c_void,
        n_points: usize) -> i32 {
    
    todo!();
        /*
            size_t point_idx;
        scalar szero;
        gej tmpj;

        scalar_set_int(&szero, 0);
        gej_set_infinity(r);
        gej_set_infinity(&tmpj);
        /* r = inp_g_sc*G */
        ecmult(ctx, r, &tmpj, &szero, inp_g_sc);
        for (point_idx = 0; point_idx < n_points; point_idx++) {
            ge point;
            gej pointj;
            scalar scalar;
            if (!cb(&scalar, &point, point_idx, cbdata)) {
                return 0;
            }
            /* r += scalar*point */
            gej_set_ge(&pointj, &point);
            ecmult(ctx, &tmpj, &pointj, &scalar, NULL);
            gej_add_var(r, r, &tmpj, NULL);
        }
        return 1;
        */
}

/**
  | Compute the number of batches and the
  | batch size given the maximum batch size
  | and the total number of points
  |
  */
pub fn ecmult_multi_batch_size_helper(
        n_batches:          *mut usize,
        n_batch_points:     *mut usize,
        max_n_batch_points: usize,
        n:                  usize) -> i32 {
    
    todo!();
        /*
            if (max_n_batch_points == 0) {
            return 0;
        }
        if (max_n_batch_points > ECMULT_MAX_POINTS_PER_BATCH) {
            max_n_batch_points = ECMULT_MAX_POINTS_PER_BATCH;
        }
        if (n == 0) {
            *n_batches = 0;
            *n_batch_points = 0;
            return 1;
        }
        /* Compute ceil(n/max_n_batch_points) and ceil(n/n_batches) */
        *n_batches = 1 + (n - 1) / max_n_batch_points;
        *n_batch_points = 1 + (n - 1) / *n_batches;
        return 1;
        */
}

pub type EcMultMultiFunc = fn(
        error_callback: *const Callback,
        _1:             *const EcMultContext,
        _2:             *mut Scratch,
        _3:             *mut Gej,
        _4:             *const Scalar,
        cb:             EcMultMultiCallback,
        _6:             *mut c_void,
        _7:             usize
) -> i32;

/**
 | Multi-multiply: 
 | R = inp_g_sc * G + sum_i ni * Ai.
 |
 | Chooses the right algorithm for a given number
 | of points and scratch space size. 
 |
 | Resets and overwrites the given scratch
 | space. If the points do not fit in the scratch
 | space the algorithm is repeatedly run with
 | batches of points. 
 |
 | If no scratch space is given then a simple
 | algorithm is used that simply multiplies the
 | points with the corresponding scalars and adds
 | them up.
 |
 | Returns: 
 |
 | 1 on success (including when inp_g_sc is NULL
 | and n is 0)
 |
 | 0 if there is not enough scratch space for
 | a single point or callback returns 0
 */
pub fn ecmult_multi_var(
        error_callback: *const Callback,
        ctx:            *const EcMultContext,
        scratch:        *mut Scratch,
        r:              *mut Gej,
        inp_g_sc:       *const Scalar,
        cb:             EcMultMultiCallback,
        cbdata:         *mut c_void,
        n:              usize) -> i32 {
    
    todo!();
        /*
            size_t i;

        int (*f)(const callback* error_callback, const ecmult_context*, scratch*, gej*, const scalar*, ecmult_multi_callback cb, c_void*, size_t, size_t);
        size_t n_batches;
        size_t n_batch_points;

        gej_set_infinity(r);
        if (inp_g_sc == NULL && n == 0) {
            return 1;
        } else if (n == 0) {
            scalar szero;
            scalar_set_int(&szero, 0);
            ecmult(ctx, r, r, &szero, inp_g_sc);
            return 1;
        }
        if (scratch == NULL) {
            return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
        }

        /* Compute the batch sizes for Pippenger's algorithm given a scratch space. If it's greater than
         * a threshold use Pippenger's algorithm. Otherwise use Strauss' algorithm.
         * As a first step check if there's enough space for Pippenger's algo (which requires less space
         * than Strauss' algo) and if not, use the simple algorithm. */
        if (!ecmult_multi_batch_size_helper(&n_batches, &n_batch_points, pippenger_max_points(error_callback, scratch), n)) {
            return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
        }
        if (n_batch_points >= ECMULT_PIPPENGER_THRESHOLD) {
            f = ecmult_pippenger_batch;
        } else {
            if (!ecmult_multi_batch_size_helper(&n_batches, &n_batch_points, strauss_max_points(error_callback, scratch), n)) {
                return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
            }
            f = ecmult_strauss_batch;
        }
        for(i = 0; i < n_batches; i++) {
            size_t nbp = n < n_batch_points ? n : n_batch_points;
            size_t offset = n_batch_points*i;
            gej tmp;
            if (!f(error_callback, ctx, scratch, &tmp, i == 0 ? inp_g_sc : NULL, cb, cbdata, nbp, offset)) {
                return 0;
            }
            gej_add_var(r, r, &tmp, NULL);
            n -= nbp;
        }
        return 1;
        */
}
