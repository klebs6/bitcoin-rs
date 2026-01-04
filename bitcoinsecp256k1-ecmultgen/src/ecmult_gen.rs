// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_gen.h]

/**
  | Set ECMULT_GEN_PREC_BITS to 2, 4 or
  | 8.
  |
  */
const_assert!{
    ECMULT_GEN_PREC_BITS == 2 
        || ECMULT_GEN_PREC_BITS == 4 
        || ECMULT_GEN_PREC_BITS == 8
}

#[cfg(feature="secp256k1-use-basic-config")] 
pub const ECMULT_GEN_PREC_BITS: usize = 4;

pub const ECMULT_GEN_PREC_B: usize = ECMULT_GEN_PREC_BITS;
pub const ECMULT_GEN_PREC_G: usize = 1 << ECMULT_GEN_PREC_B;
pub const ECMULT_GEN_PREC_N: usize = 256 / ECMULT_GEN_PREC_B;

pub type EcMultGenContextPrec = [[*mut GeStorage; ECMULT_GEN_PREC_N]; ECMULT_GEN_PREC_G];

/** 
  | For accelerating the computation of a*G:
  |
  | To harden against timing attacks, use the
  | following mechanism:
  |
  | - Break up the multiplicand into groups of
  | PREC_B bits, called n_0, n_1, n_2, ...,
  | n_(PREC_N-1).
  |
  | - Compute sum(n_i * (PREC_G)^i * G + U_i,
  | i=0 ... PREC_N-1), where:
  |
  |   - U_i = U * 2^i, for i=0 ... PREC_N-2
  |
  |   - U_i = U * (1-2^(PREC_N-1)), for i=PREC_N-1
  |   where U is a point with no known
  |   corresponding scalar. Note that sum(U_i,
  |   i=0 ... PREC_N-1) = 0.
  |
  | For each i, and each of the PREC_G possible
  | values of n_i, (n_i * (PREC_G)^i * G + U_i)
  | is precomputed (call it prec(i, n_i)). The
  | formula now becomes sum(prec(i, n_i), i=0
  | ... PREC_N-1).
  |
  | None of the resulting prec group elements
  | have a known scalar, and neither do any of
  | the intermediate sums while computing a*G.
  */
pub struct EcMultGenContext {

    /**
      | prec[j][i] = (PREC_G)^j * i * G + U_i
      |
      */
    prec:    EcMultGenContextPrec,

    blind:   Scalar,
    initial: Gej,
}

impl EcMultGenContext {

    pub const fn new() -> Self {
        Self {
            prec:    [[null_mut(); ECMULT_GEN_PREC_N]; ECMULT_GEN_PREC_G],
            blind:   Scalar::new(),
            initial: Gej::new(),
        }
    }
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_gen_impl.h]

#[cfg(not(USE_ECMULT_STATIC_PRECOMPUTATION))]
pub const ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE: usize = round_to_align!(
    //sizeof(*((ecmult_gen_context*) NULL)->prec)
    size_of::<EcMultGenContextPrec>()
);

#[cfg(USE_ECMULT_STATIC_PRECOMPUTATION)]
pub const ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE: usize = 0;

pub fn ecmult_gen_context_init(ctx: *mut EcMultGenContext)  {
    
    todo!();
        /*
            ctx->prec = NULL;
        */
}

pub fn ecmult_gen_context_build(
        ctx:      *mut EcMultGenContext,
        prealloc: *mut *mut c_void)  {
    
    todo!();
        /*
            #ifndef USE_ECMULT_STATIC_PRECOMPUTATION
        ge prec[ECMULT_GEN_PREC_N * ECMULT_GEN_PREC_G];
        gej gj;
        gej nums_gej;
        int i, j;
        size_t const prealloc_size = ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;
        c_void* const base = *prealloc;
    #endif

        if (ctx->prec != NULL) {
            return;
        }
    #ifndef USE_ECMULT_STATIC_PRECOMPUTATION
        ctx->prec = (ge_storage (*)[ECMULT_GEN_PREC_N][ECMULT_GEN_PREC_G])manual_alloc(prealloc, prealloc_size, base, prealloc_size);

        /* get the generator */
        gej_set_ge(&gj, &ge_const_g);

        /* Construct a group element with no known corresponding scalar (nothing up my sleeve). */
        {
            static const unsigned char nums_b32[33] = "The scalar for this x is unknown";
            fe nums_x;
            ge nums_ge;
            int r;
            r = fe_set_b32(&nums_x, nums_b32);
            (c_void)r;
            VERIFY_CHECK(r);
            r = ge_set_xo_var(&nums_ge, &nums_x, 0);
            (c_void)r;
            VERIFY_CHECK(r);
            gej_set_ge(&nums_gej, &nums_ge);
            /* Add G to make the bits in x uniformly distributed. */
            gej_add_ge_var(&nums_gej, &nums_gej, &ge_const_g, NULL);
        }

        /* compute prec. */
        {
            gej precj[ECMULT_GEN_PREC_N * ECMULT_GEN_PREC_G]; /* Jacobian versions of prec. */
            gej gbase;
            gej numsbase;
            gbase = gj; /* PREC_G^j * G */
            numsbase = nums_gej; /* 2^j * nums. */
            for (j = 0; j < ECMULT_GEN_PREC_N; j++) {
                /* Set precj[j*PREC_G .. j*PREC_G+(PREC_G-1)] to (numsbase, numsbase + gbase, ..., numsbase + (PREC_G-1)*gbase). */
                precj[j*ECMULT_GEN_PREC_G] = numsbase;
                for (i = 1; i < ECMULT_GEN_PREC_G; i++) {
                    gej_add_var(&precj[j*ECMULT_GEN_PREC_G + i], &precj[j*ECMULT_GEN_PREC_G + i - 1], &gbase, NULL);
                }
                /* Multiply gbase by PREC_G. */
                for (i = 0; i < ECMULT_GEN_PREC_B; i++) {
                    gej_double_var(&gbase, &gbase, NULL);
                }
                /* Multiply numbase by 2. */
                gej_double_var(&numsbase, &numsbase, NULL);
                if (j == ECMULT_GEN_PREC_N - 2) {
                    /* In the last iteration, numsbase is (1 - 2^j) * nums instead. */
                    gej_neg(&numsbase, &numsbase);
                    gej_add_var(&numsbase, &numsbase, &nums_gej, NULL);
                }
            }
            ge_set_all_gej_var(prec, precj, ECMULT_GEN_PREC_N * ECMULT_GEN_PREC_G);
        }
        for (j = 0; j < ECMULT_GEN_PREC_N; j++) {
            for (i = 0; i < ECMULT_GEN_PREC_G; i++) {
                ge_to_storage(&(*ctx->prec)[j][i], &prec[j*ECMULT_GEN_PREC_G + i]);
            }
        }
    #else
        (c_void)prealloc;
        ctx->prec = (ge_storage (*)[ECMULT_GEN_PREC_N][ECMULT_GEN_PREC_G])ecmult_static_context;
    #endif
        ecmult_gen_blind(ctx, NULL);
        */
}

pub fn ecmult_gen_context_is_built(ctx: *const EcMultGenContext) -> i32 {
    
    todo!();
        /*
            return ctx->prec != NULL;
        */
}

pub fn ecmult_gen_context_finalize_memcpy(
        dst: *mut EcMultGenContext,
        src: *const EcMultGenContext)  {
    
    todo!();
        /*
            #ifndef USE_ECMULT_STATIC_PRECOMPUTATION
        if (src->prec != NULL) {
            /* We cast to c_void* first to suppress a -Wcast-align warning. */
            dst->prec = (ge_storage (*)[ECMULT_GEN_PREC_N][ECMULT_GEN_PREC_G])(c_void*)((unsigned char*)dst + ((unsigned char*)src->prec - (unsigned char*)src));
        }
    #else
        (c_void)dst, (c_void)src;
    #endif
        */
}

pub fn ecmult_gen_context_clear(ctx: *mut EcMultGenContext)  {
    
    todo!();
        /*
        scalar_clear(&ctx->blind);
        gej_clear(&ctx->initial);
        ctx->prec = NULL;
        */
}

/**
  | Multiply with the generator: R = a*G
  |
  */
pub fn ecmult_gen(
        ctx: *const EcMultGenContext,
        r:   *mut Gej,
        gn:  *const Scalar)  {
    
    todo!();
        /*
        ge add;
        ge_storage adds;
        scalar gnb;
        int bits;
        int i, j;
        memset(&adds, 0, sizeof(adds));
        *r = ctx->initial;
        /* Blind scalar/point multiplication by computing (n-b)G + bG instead of nG. */
        scalar_add(&gnb, gn, &ctx->blind);
        add.infinity = 0;
        for (j = 0; j < ECMULT_GEN_PREC_N; j++) {
            bits = scalar_get_bits(&gnb, j * ECMULT_GEN_PREC_B, ECMULT_GEN_PREC_B);
            for (i = 0; i < ECMULT_GEN_PREC_G; i++) {
                /** This uses a conditional move to avoid any secret data in array indexes.
                 *   _Any_ use of secret indexes has been demonstrated to result in timing
                 *   sidechannels, even when the cache-line access patterns are uniform.
                 *  See also:
                 *   "A word of warning", CHES 2013 Rump Session, by Daniel J. Bernstein and Peter Schwabe
                 *    (https://cryptojedi.org/peter/data/chesrump-20130822.pdf) and
                 *   "Cache Attacks and Countermeasures: the Case of AES", RSA 2006,
                 *    by Dag Arne Osvik, Adi Shamir, and Eran Tromer
                 *    (https://www.tau.ac.il/~tromer/papers/cache.pdf)
                 */
                ge_storage_cmov(&adds, &(*ctx->prec)[j][i], i == bits);
            }
            ge_from_storage(&add, &adds);
            gej_add_ge(r, r, &add);
        }
        bits = 0;
        ge_clear(&add);
        scalar_clear(&gnb);
        */
}

/**
  | Setup blinding values for ecmult_gen.
  |
  */
pub fn ecmult_gen_blind(
        ctx:    *mut EcMultGenContext,
        seed32: *const u8)  {
    
    todo!();
        /*
        scalar b;
        gej gb;
        fe s;
        unsigned char nonce32[32];
        rfc6979_hmac_sha256 rng;
        int overflow;
        unsigned char keydata[64] = {0};
        if (seed32 == NULL) {
            /* When seed is NULL, reset the initial point and blinding value. */
            gej_set_ge(&ctx->initial, &ge_const_g);
            gej_neg(&ctx->initial, &ctx->initial);
            scalar_set_int(&ctx->blind, 1);
        }
        /* The prior blinding value (if not reset) is chained forward by including it in the hash. */
        scalar_get_b32(nonce32, &ctx->blind);
        /** Using a CSPRNG allows a failure free interface, avoids needing large amounts of random data,
         *   and guards against weak or adversarial seeds.  This is a simpler and safer interface than
         *   asking the caller for blinding values directly and expecting them to retry on failure.
         */
        memcpy(keydata, nonce32, 32);
        if (seed32 != NULL) {
            memcpy(keydata + 32, seed32, 32);
        }
        rfc6979_hmac_sha256_initialize(&rng, keydata, seed32 ? 64 : 32);
        memset(keydata, 0, sizeof(keydata));
        /* Accept unobservably small non-uniformity. */
        rfc6979_hmac_sha256_generate(&rng, nonce32, 32);
        overflow = !fe_set_b32(&s, nonce32);
        overflow |= fe_is_zero(&s);
        fe_cmov(&s, &fe_one, overflow);
        /* Randomize the projection to defend against multiplier sidechannels. */
        gej_rescale(&ctx->initial, &s);
        fe_clear(&s);
        rfc6979_hmac_sha256_generate(&rng, nonce32, 32);
        scalar_set_b32(&b, nonce32, NULL);
        /* A blinding value of 0 works, but would undermine the projection hardening. */
        scalar_cmov(&b, &scalar_one, scalar_is_zero(&b));
        rfc6979_hmac_sha256_finalize(&rng);
        memset(nonce32, 0, 32);
        ecmult_gen(ctx, &gb, &b);
        scalar_negate(&b, &b);
        ctx->blind = b;
        ctx->initial = gb;
        scalar_clear(&b);
        gej_clear(&gb);
        */
}
