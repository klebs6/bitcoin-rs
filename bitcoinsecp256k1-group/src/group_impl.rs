crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/group.h]

/**
  | A group element of the secp256k1 curve,
  | in affine coordinates.
  |
  */
pub struct Ge {
    x:        Fe,
    y:        Fe,

    /**
      | whether this represents the point at
      | infinity
      |
      */
    infinity: i32,
}

impl Ge {
    pub const fn new() -> Self {
        Self {
            x:        Fe::new(),
            y:        Fe::new(),
            infinity: 0,
        }
    }
}

macro_rules! GE_CONST {
    ($a:ident, 
     $b:ident, 
     $c:ident, 
     $d:ident, 
     $e:ident, 
     $f:ident, 
     $g:ident, 
     $h:ident, 
     $i:ident, 
     $j:ident, 
     $k:ident, 
     $l:ident, 
     $m:ident, 
     $n:ident, 
     $o:ident, 
     $p:ident) => {
        /*
                {FE_CONST((a),(b),(c),(d),(e),(f),(g),(h)), FE_CONST((i),(j),(k),(l),(m),(n),(o),(p)), 0}
        */
    }
}

macro_rules! GE_CONST_INFINITY {
    () => {
        /*
                {FE_CONST(0, 0, 0, 0, 0, 0, 0, 0), FE_CONST(0, 0, 0, 0, 0, 0, 0, 0), 1}
        */
    }
}

/**
  | A group element of the secp256k1 curve,
  | in jacobian coordinates.
  |
  */
pub struct Gej {

    /**
      | actual X: x/z^2
      |
      */
    x:        Fe,


    /**
      | actual Y: y/z^3
      |
      */
    y:        Fe,

    z:        Fe,

    /**
      | whether this represents the point at
      | infinity
      |
      */
    infinity: i32,
}

impl Gej {
    pub const fn new() -> Self {
        Self {
            x: Fe::new(),
            y: Fe::new(),
            z: Fe::new(),
            infinity: 0,
        }
    }
}

macro_rules! GEJ_CONST {
    ($a:ident, 
     $b:ident, 
     $c:ident, 
     $d:ident, 
     $e:ident, 
     $f:ident, 
     $g:ident, 
     $h:ident, 
     $i:ident, 
     $j:ident, 
     $k:ident, 
     $l:ident, 
     $m:ident, 
     $n:ident, 
     $o:ident, 
     $p:ident) => {
        /*
                {FE_CONST((a),(b),(c),(d),(e),(f),(g),(h)), FE_CONST((i),(j),(k),(l),(m),(n),(o),(p)), FE_CONST(0, 0, 0, 0, 0, 0, 0, 1), 0}
        */
    }
}

macro_rules! GEJ_CONST_INFINITY {
    () => {
        /*
                {FE_CONST(0, 0, 0, 0, 0, 0, 0, 0), FE_CONST(0, 0, 0, 0, 0, 0, 0, 0), FE_CONST(0, 0, 0, 0, 0, 0, 0, 0), 1}
        */
    }
}

pub struct GeStorage {
    x: FeStorage,
    y: FeStorage,
}

macro_rules! GE_STORAGE_CONST {
    ($a:ident, 
     $b:ident, 
     $c:ident, 
     $d:ident, 
     $e:ident, 
     $f:ident, 
     $g:ident, 
     $h:ident, 
     $i:ident, 
     $j:ident, 
     $k:ident, 
     $l:ident, 
     $m:ident, 
     $n:ident, 
     $o:ident, 
     $p:ident) => {
        /*
                {FE_STORAGE_CONST((a),(b),(c),(d),(e),(f),(g),(h)), FE_STORAGE_CONST((i),(j),(k),(l),(m),(n),(o),(p))}
        */
    }
}

macro_rules! ge_storage_const_get {
    ($t:ident) => {
        /*
                FE_STORAGE_CONST_GET(t.x), FE_STORAGE_CONST_GET(t.y)
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/group_impl.h]

/**
  | These exhaustive group test orders
  | and generators are chosen such that:
  | 
  | - The field size is equal to that of secp256k1,
  | so field code is the same.
  | 
  | - The curve equation is of the form y^2=x^3+B
  | for some constant B.
  | 
  | - The subgroup has a generator 2*P, where
  | P.x=1.
  | 
  | - The subgroup has size less than 1000
  | to permit exhaustive testing.
  | 
  | - The subgroup admits an endomorphism
  | of the form lambda*(x,y) == (beta*x,y).
  | 
  | These parameters are generated using
  | sage/gen_exhaustive_groups.sage.
  |
  */
lazy_static!{
    /*
    #if defined(EXHAUSTIVE_TEST_ORDER)
    #  if EXHAUSTIVE_TEST_ORDER == 13
    static const ge ge_const_g = GE_CONST(
        0xc3459c3d, 0x35326167, 0xcd86cce8, 0x07a2417f,
        0x5b8bd567, 0xde8538ee, 0x0d507b0c, 0xd128f5bb,
        0x8e467fec, 0xcd30000a, 0x6cc1184e, 0x25d382c2,
        0xa2f4494e, 0x2fbe9abc, 0x8b64abac, 0xd005fb24
    );
    static const fe fe_const_b = FE_CONST(
        0x3d3486b2, 0x159a9ca5, 0xc75638be, 0xb23a69bc,
        0x946a45ab, 0x24801247, 0xb4ed2b8e, 0x26b6a417
    );
    #  elif EXHAUSTIVE_TEST_ORDER == 199
    static const ge ge_const_g = GE_CONST(
        0x226e653f, 0xc8df7744, 0x9bacbf12, 0x7d1dcbf9,
        0x87f05b2a, 0xe7edbd28, 0x1f564575, 0xc48dcf18,
        0xa13872c2, 0xe933bb17, 0x5d9ffd5b, 0xb5b6e10c,
        0x57fe3c00, 0xbaaaa15a, 0xe003ec3e, 0x9c269bae
    );
    static const fe fe_const_b = FE_CONST(
        0x2cca28fa, 0xfc614b80, 0x2a3db42b, 0x00ba00b1,
        0xbea8d943, 0xdace9ab2, 0x9536daea, 0x0074defb
    );
    #  else
    #    error No known generator for the specified exhaustive test group order.
    #  endif
    #else
    /** Generator for secp256k1, value 'g' defined in
     *  "Standards for Efficient Cryptography" (SEC2) 2.7.1.
     */
    static const ge ge_const_g = GE_CONST(
        0x79BE667EUL, 0xF9DCBBACUL, 0x55A06295UL, 0xCE870B07UL,
        0x029BFCDBUL, 0x2DCE28D9UL, 0x59F2815BUL, 0x16F81798UL,
        0x483ADA77UL, 0x26A3C465UL, 0x5DA4FBFCUL, 0x0E1108A8UL,
        0xFD17B448UL, 0xA6855419UL, 0x9C47D08FUL, 0xFB10D4B8UL
    );

    static const fe fe_const_b = FE_CONST(0, 0, 0, 0, 0, 0, 0, 7);
    #endif
    */
}

pub fn ge_set_gej_zinv(
        r:  *mut Ge,
        a:  *const Gej,
        zi: *const Fe)  {
    
    todo!();
        /*
            fe zi2;
        fe zi3;
        fe_sqr(&zi2, zi);
        fe_mul(&zi3, &zi2, zi);
        fe_mul(&r->x, &a->x, &zi2);
        fe_mul(&r->y, &a->y, &zi3);
        r->infinity = a->infinity;
        */
}

/**
  | Set a group element equal to the point
  | with given X and Y coordinates
  |
  */
pub fn ge_set_xy(
        r: *mut Ge,
        x: *const Fe,
        y: *const Fe)  {
    
    todo!();
        /*
            r->infinity = 0;
        r->x = *x;
        r->y = *y;
        */
}

/**
  | Check whether a group element is the
  | point at infinity.
  |
  */
pub fn ge_is_infinity(a: *const Ge) -> i32 {
    
    todo!();
        /*
            return a->infinity;
        */
}

/**
  | Set r equal to the inverse of a (i.e.,
  | mirrored around the X axis)
  |
  */
pub fn ge_neg(
        r: *mut Ge,
        a: *const Ge)  {
    
    todo!();
        /*
            *r = *a;
        fe_normalize_weak(&r->y);
        fe_negate(&r->y, &r->y, 1);
        */
}

/**
  | Set a group element equal to another
  | which is given in jacobian coordinates.
  | Constant time.
  |
  */
pub fn ge_set_gej(
        r: *mut Ge,
        a: *mut Gej)  {
    
    todo!();
        /*
            fe z2, z3;
        r->infinity = a->infinity;
        fe_inv(&a->z, &a->z);
        fe_sqr(&z2, &a->z);
        fe_mul(&z3, &a->z, &z2);
        fe_mul(&a->x, &a->x, &z2);
        fe_mul(&a->y, &a->y, &z3);
        fe_set_int(&a->z, 1);
        r->x = a->x;
        r->y = a->y;
        */
}

/**
  | Set a group element equal to another
  | which is given in jacobian coordinates.
  |
  */
pub fn ge_set_gej_var(
        r: *mut Ge,
        a: *mut Gej)  {
    
    todo!();
        /*
            fe z2, z3;
        if (a->infinity) {
            ge_set_infinity(r);
            return;
        }
        fe_inv_var(&a->z, &a->z);
        fe_sqr(&z2, &a->z);
        fe_mul(&z3, &a->z, &z2);
        fe_mul(&a->x, &a->x, &z2);
        fe_mul(&a->y, &a->y, &z3);
        fe_set_int(&a->z, 1);
        ge_set_xy(r, &a->x, &a->y);
        */
}

/**
  | Set a batch of group elements equal to
  | the inputs given in jacobian coordinates
  |
  */
pub fn ge_set_all_gej_var(
        r:   *mut Ge,
        a:   *const Gej,
        len: usize)  {
    
    todo!();
        /*
            fe u;
        size_t i;
        size_t last_i = SIZE_MAX;

        for (i = 0; i < len; i++) {
            if (a[i].infinity) {
                ge_set_infinity(&r[i]);
            } else {
                /* Use destination's x coordinates as scratch space */
                if (last_i == SIZE_MAX) {
                    r[i].x = a[i].z;
                } else {
                    fe_mul(&r[i].x, &r[last_i].x, &a[i].z);
                }
                last_i = i;
            }
        }
        if (last_i == SIZE_MAX) {
            return;
        }
        fe_inv_var(&u, &r[last_i].x);

        i = last_i;
        while (i > 0) {
            i--;
            if (!a[i].infinity) {
                fe_mul(&r[last_i].x, &r[i].x, &u);
                fe_mul(&u, &u, &a[last_i].z);
                last_i = i;
            }
        }
        VERIFY_CHECK(!a[last_i].infinity);
        r[last_i].x = u;

        for (i = 0; i < len; i++) {
            if (!a[i].infinity) {
                ge_set_gej_zinv(&r[i], &a[i], &r[i].x);
            }
        }
        */
}

/**
  | Bring a batch inputs given in jacobian
  | coordinates (with known z-ratios)
  | to the same global z "denominator".
  | zr must contain the known z-ratios such
  | that mul(a[i].z, zr[i+1]) == a[i+1].z.
  | zr[0] is ignored. The x and y coordinates
  | of the result are stored in r, the common
  | z coordinate is stored in globalz.
  |
  */
pub fn ge_globalz_set_table_gej(
        len:     usize,
        r:       *mut Ge,
        globalz: *mut Fe,
        a:       *const Gej,
        zr:      *const Fe)  {
    
    todo!();
        /*
            size_t i = len - 1;
        fe zs;

        if (len > 0) {
            /* The z of the final point gives us the "global Z" for the table. */
            r[i].x = a[i].x;
            r[i].y = a[i].y;
            /* Ensure all y values are in weak normal form for fast negation of points */
            fe_normalize_weak(&r[i].y);
            *globalz = a[i].z;
            r[i].infinity = 0;
            zs = zr[i];

            /* Work our way backwards, using the z-ratios to scale the x/y values. */
            while (i > 0) {
                if (i != len - 1) {
                    fe_mul(&zs, &zs, &zr[i]);
                }
                i--;
                ge_set_gej_zinv(&r[i], &a[i], &zs);
            }
        }
        */
}

/**
  | Set a group element (jacobian) equal
  | to the point at infinity.
  |
  */
pub fn gej_set_infinity(r: *mut Gej)  {
    
    todo!();
        /*
            r->infinity = 1;
        fe_clear(&r->x);
        fe_clear(&r->y);
        fe_clear(&r->z);
        */
}

/**
  | Set a group element (affine) equal to
  | the point at infinity.
  |
  */
pub fn ge_set_infinity(r: *mut Ge)  {
    
    todo!();
        /*
            r->infinity = 1;
        fe_clear(&r->x);
        fe_clear(&r->y);
        */
}

/**
  | Clear a gej to prevent leaking
  | sensitive information.
  |
  */
pub fn gej_clear(r: *mut Gej)  {
    
    todo!();
        /*
            r->infinity = 0;
        fe_clear(&r->x);
        fe_clear(&r->y);
        fe_clear(&r->z);
        */
}

/**
  | Clear a ge to prevent leaking
  | sensitive information.
  |
  */
pub fn ge_clear(r: *mut Ge)  {
    
    todo!();
        /*
            r->infinity = 0;
        fe_clear(&r->x);
        fe_clear(&r->y);
        */
}

/**
  | Set a group element (affine) equal to
  | the point with the given X coordinate,
  | and given oddness for Y. Return value
  | indicates whether the result is valid.
  |
  */
pub fn ge_set_xo_var(
        r:   *mut Ge,
        x:   *const Fe,
        odd: i32) -> i32 {
    
    todo!();
        /*
            fe x2, x3;
        r->x = *x;
        fe_sqr(&x2, x);
        fe_mul(&x3, x, &x2);
        r->infinity = 0;
        fe_add(&x3, &fe_const_b);
        if (!fe_sqrt(&r->y, &x3)) {
            return 0;
        }
        fe_normalize_var(&r->y);
        if (fe_is_odd(&r->y) != odd) {
            fe_negate(&r->y, &r->y, 1);
        }
        return 1;
        */
}

/**
  | Set a group element (jacobian) equal
  | to another which is given in affine coordinates.
  |
  */
pub fn gej_set_ge(
        r: *mut Gej,
        a: *const Ge)  {
    
    todo!();
        /*
            r->infinity = a->infinity;
       r->x = a->x;
       r->y = a->y;
       fe_set_int(&r->z, 1);
        */
}

/**
  | Compare the X coordinate of a group element
  | (jacobian).
  |
  */
pub fn gej_eq_x_var(
        x: *const Fe,
        a: *const Gej) -> i32 {
    
    todo!();
        /*
            fe r, r2;
        VERIFY_CHECK(!a->infinity);
        fe_sqr(&r, &a->z); fe_mul(&r, &r, x);
        r2 = a->x; fe_normalize_weak(&r2);
        return fe_equal_var(&r, &r2);
        */
}

/**
  | Set r equal to the inverse of a (i.e.,
  | mirrored around the X axis)
  |
  */
pub fn gej_neg(
        r: *mut Gej,
        a: *const Gej)  {
    
    todo!();
        /*
            r->infinity = a->infinity;
        r->x = a->x;
        r->y = a->y;
        r->z = a->z;
        fe_normalize_weak(&r->y);
        fe_negate(&r->y, &r->y, 1);
        */
}

/**
  | Check whether a group element is the
  | point at infinity.
  |
  */
pub fn gej_is_infinity(a: *const Gej) -> i32 {
    
    todo!();
        /*
            return a->infinity;
        */
}

/**
  | Check whether a group element is valid
  | (i.e., on the curve).
  |
  */
pub fn ge_is_valid_var(a: *const Ge) -> i32 {
    
    todo!();
        /*
            fe y2, x3;
        if (a->infinity) {
            return 0;
        }
        /* y^2 = x^3 + 7 */
        fe_sqr(&y2, &a->y);
        fe_sqr(&x3, &a->x); fe_mul(&x3, &x3, &a->x);
        fe_add(&x3, &fe_const_b);
        fe_normalize_weak(&x3);
        return fe_equal_var(&y2, &x3);
        */
}

/**
  | Set r equal to the double of a. Constant
  | time.
  |
  */
#[inline] pub fn gej_double(
        r: *mut Gej,
        a: *const Gej)  {
    
    todo!();
        /*
            /* Operations: 3 mul, 4 sqr, 0 normalize, 12 mul_int/add/negate.
         *
         * Note that there is an implementation described at
         *     https://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l
         * which trades a multiply for a square, but in practice this is actually slower,
         * mainly because it requires more normalizations.
         */
        fe t1,t2,t3,t4;

        r->infinity = a->infinity;

        fe_mul(&r->z, &a->z, &a->y);
        fe_mul_int(&r->z, 2);       /* Z' = 2*Y*Z (2) */
        fe_sqr(&t1, &a->x);
        fe_mul_int(&t1, 3);         /* T1 = 3*X^2 (3) */
        fe_sqr(&t2, &t1);           /* T2 = 9*X^4 (1) */
        fe_sqr(&t3, &a->y);
        fe_mul_int(&t3, 2);         /* T3 = 2*Y^2 (2) */
        fe_sqr(&t4, &t3);
        fe_mul_int(&t4, 2);         /* T4 = 8*Y^4 (2) */
        fe_mul(&t3, &t3, &a->x);    /* T3 = 2*X*Y^2 (1) */
        r->x = t3;
        fe_mul_int(&r->x, 4);       /* X' = 8*X*Y^2 (4) */
        fe_negate(&r->x, &r->x, 4); /* X' = -8*X*Y^2 (5) */
        fe_add(&r->x, &t2);         /* X' = 9*X^4 - 8*X*Y^2 (6) */
        fe_negate(&t2, &t2, 1);     /* T2 = -9*X^4 (2) */
        fe_mul_int(&t3, 6);         /* T3 = 12*X*Y^2 (6) */
        fe_add(&t3, &t2);           /* T3 = 12*X*Y^2 - 9*X^4 (8) */
        fe_mul(&r->y, &t1, &t3);    /* Y' = 36*X^3*Y^2 - 27*X^6 (1) */
        fe_negate(&t2, &t4, 2);     /* T2 = -8*Y^4 (3) */
        fe_add(&r->y, &t2);         /* Y' = 36*X^3*Y^2 - 27*X^6 - 8*Y^4 (4) */
        */
}

/**
  | Set r equal to the double of a. If rzr is
  | not-NULL this sets *rzr such that r->z
  | == a->z * *rzr (where infinity means
  | an implicit z = 0).
  |
  */
pub fn gej_double_var(
        r:   *mut Gej,
        a:   *const Gej,
        rzr: *mut Fe)  {
    
    todo!();
        /*
            /** For secp256k1, 2Q is infinity if and only if Q is infinity. This is because if 2Q = infinity,
         *  Q must equal -Q, or that Q.y == -(Q.y), or Q.y is 0. For a point on y^2 = x^3 + 7 to have
         *  y=0, x^3 must be -7 mod p. However, -7 has no cube root mod p.
         *
         *  Having said this, if this function receives a point on a sextic twist, e.g. by
         *  a fault attack, it is possible for y to be 0. This happens for y^2 = x^3 + 6,
         *  since -6 does have a cube root mod p. For this point, this function will not set
         *  the infinity flag even though the point doubles to infinity, and the result
         *  point will be gibberish (z = 0 but infinity = 0).
         */
        if (a->infinity) {
            gej_set_infinity(r);
            if (rzr != NULL) {
                fe_set_int(rzr, 1);
            }
            return;
        }

        if (rzr != NULL) {
            *rzr = a->y;
            fe_normalize_weak(rzr);
            fe_mul_int(rzr, 2);
        }

        gej_double(r, a);
        */
}

/**
  | Set r equal to the sum of a and b. If rzr
  | is non-NULL this sets *rzr such that
  | r->z == a->z * *rzr (a cannot be infinity
  | in that case).
  |
  */
pub fn gej_add_var(
        r:   *mut Gej,
        a:   *const Gej,
        b:   *const Gej,
        rzr: *mut Fe)  {
    
    todo!();
        /*
            /* Operations: 12 mul, 4 sqr, 2 normalize, 12 mul_int/add/negate */
        fe z22, z12, u1, u2, s1, s2, h, i, i2, h2, h3, t;

        if (a->infinity) {
            VERIFY_CHECK(rzr == NULL);
            *r = *b;
            return;
        }

        if (b->infinity) {
            if (rzr != NULL) {
                fe_set_int(rzr, 1);
            }
            *r = *a;
            return;
        }

        r->infinity = 0;
        fe_sqr(&z22, &b->z);
        fe_sqr(&z12, &a->z);
        fe_mul(&u1, &a->x, &z22);
        fe_mul(&u2, &b->x, &z12);
        fe_mul(&s1, &a->y, &z22); fe_mul(&s1, &s1, &b->z);
        fe_mul(&s2, &b->y, &z12); fe_mul(&s2, &s2, &a->z);
        fe_negate(&h, &u1, 1); fe_add(&h, &u2);
        fe_negate(&i, &s1, 1); fe_add(&i, &s2);
        if (fe_normalizes_to_zero_var(&h)) {
            if (fe_normalizes_to_zero_var(&i)) {
                gej_double_var(r, a, rzr);
            } else {
                if (rzr != NULL) {
                    fe_set_int(rzr, 0);
                }
                gej_set_infinity(r);
            }
            return;
        }
        fe_sqr(&i2, &i);
        fe_sqr(&h2, &h);
        fe_mul(&h3, &h, &h2);
        fe_mul(&h, &h, &b->z);
        if (rzr != NULL) {
            *rzr = h;
        }
        fe_mul(&r->z, &a->z, &h);
        fe_mul(&t, &u1, &h2);
        r->x = t; fe_mul_int(&r->x, 2); fe_add(&r->x, &h3); fe_negate(&r->x, &r->x, 3); fe_add(&r->x, &i2);
        fe_negate(&r->y, &r->x, 5); fe_add(&r->y, &t); fe_mul(&r->y, &r->y, &i);
        fe_mul(&h3, &h3, &s1); fe_negate(&h3, &h3, 1);
        fe_add(&r->y, &h3);
        */
}

/**
  | Set r equal to the sum of a and b (with b
  | given in affine coordinates). This
  | is more efficient than gej_add_var.
  | It is identical to gej_add_ge
  | but without constant-time guarantee,
  | and b is allowed to be infinity. If rzr
  | is non-NULL this sets *rzr such that
  | r->z == a->z * *rzr (a cannot be infinity
  | in that case).
  |
  */
pub fn gej_add_ge_var(
        r:   *mut Gej,
        a:   *const Gej,
        b:   *const Ge,
        rzr: *mut Fe)  {
    
    todo!();
        /*
            /* 8 mul, 3 sqr, 4 normalize, 12 mul_int/add/negate */
        fe z12, u1, u2, s1, s2, h, i, i2, h2, h3, t;
        if (a->infinity) {
            VERIFY_CHECK(rzr == NULL);
            gej_set_ge(r, b);
            return;
        }
        if (b->infinity) {
            if (rzr != NULL) {
                fe_set_int(rzr, 1);
            }
            *r = *a;
            return;
        }
        r->infinity = 0;

        fe_sqr(&z12, &a->z);
        u1 = a->x; fe_normalize_weak(&u1);
        fe_mul(&u2, &b->x, &z12);
        s1 = a->y; fe_normalize_weak(&s1);
        fe_mul(&s2, &b->y, &z12); fe_mul(&s2, &s2, &a->z);
        fe_negate(&h, &u1, 1); fe_add(&h, &u2);
        fe_negate(&i, &s1, 1); fe_add(&i, &s2);
        if (fe_normalizes_to_zero_var(&h)) {
            if (fe_normalizes_to_zero_var(&i)) {
                gej_double_var(r, a, rzr);
            } else {
                if (rzr != NULL) {
                    fe_set_int(rzr, 0);
                }
                gej_set_infinity(r);
            }
            return;
        }
        fe_sqr(&i2, &i);
        fe_sqr(&h2, &h);
        fe_mul(&h3, &h, &h2);
        if (rzr != NULL) {
            *rzr = h;
        }
        fe_mul(&r->z, &a->z, &h);
        fe_mul(&t, &u1, &h2);
        r->x = t; fe_mul_int(&r->x, 2); fe_add(&r->x, &h3); fe_negate(&r->x, &r->x, 3); fe_add(&r->x, &i2);
        fe_negate(&r->y, &r->x, 5); fe_add(&r->y, &t); fe_mul(&r->y, &r->y, &i);
        fe_mul(&h3, &h3, &s1); fe_negate(&h3, &h3, 1);
        fe_add(&r->y, &h3);
        */
}

/**
  | Set r equal to the sum of a and b (with the
  | inverse of b's Z coordinate passed as
  | bzinv).
  |
  */
pub fn gej_add_zinv_var(
        r:     *mut Gej,
        a:     *const Gej,
        b:     *const Ge,
        bzinv: *const Fe)  {
    
    todo!();
        /*
            /* 9 mul, 3 sqr, 4 normalize, 12 mul_int/add/negate */
        fe az, z12, u1, u2, s1, s2, h, i, i2, h2, h3, t;

        if (b->infinity) {
            *r = *a;
            return;
        }
        if (a->infinity) {
            fe bzinv2, bzinv3;
            r->infinity = b->infinity;
            fe_sqr(&bzinv2, bzinv);
            fe_mul(&bzinv3, &bzinv2, bzinv);
            fe_mul(&r->x, &b->x, &bzinv2);
            fe_mul(&r->y, &b->y, &bzinv3);
            fe_set_int(&r->z, 1);
            return;
        }
        r->infinity = 0;

        /** We need to calculate (rx,ry,rz) = (ax,ay,az) + (bx,by,1/bzinv). Due to
         *  secp256k1's isomorphism we can multiply the Z coordinates on both sides
         *  by bzinv, and get: (rx,ry,rz*bzinv) = (ax,ay,az*bzinv) + (bx,by,1).
         *  This means that (rx,ry,rz) can be calculated as
         *  (ax,ay,az*bzinv) + (bx,by,1), when not applying the bzinv factor to rz.
         *  The variable az below holds the modified Z coordinate for a, which is used
         *  for the computation of rx and ry, but not for rz.
         */
        fe_mul(&az, &a->z, bzinv);

        fe_sqr(&z12, &az);
        u1 = a->x; fe_normalize_weak(&u1);
        fe_mul(&u2, &b->x, &z12);
        s1 = a->y; fe_normalize_weak(&s1);
        fe_mul(&s2, &b->y, &z12); fe_mul(&s2, &s2, &az);
        fe_negate(&h, &u1, 1); fe_add(&h, &u2);
        fe_negate(&i, &s1, 1); fe_add(&i, &s2);
        if (fe_normalizes_to_zero_var(&h)) {
            if (fe_normalizes_to_zero_var(&i)) {
                gej_double_var(r, a, NULL);
            } else {
                gej_set_infinity(r);
            }
            return;
        }
        fe_sqr(&i2, &i);
        fe_sqr(&h2, &h);
        fe_mul(&h3, &h, &h2);
        r->z = a->z; fe_mul(&r->z, &r->z, &h);
        fe_mul(&t, &u1, &h2);
        r->x = t; fe_mul_int(&r->x, 2); fe_add(&r->x, &h3); fe_negate(&r->x, &r->x, 3); fe_add(&r->x, &i2);
        fe_negate(&r->y, &r->x, 5); fe_add(&r->y, &t); fe_mul(&r->y, &r->y, &i);
        fe_mul(&h3, &h3, &s1); fe_negate(&h3, &h3, 1);
        fe_add(&r->y, &h3);
        */
}

/**
  | Set r equal to the sum of a and b (with b
  | given in affine coordinates, and not
  | infinity).
  |
  */
pub fn gej_add_ge(
        r: *mut Gej,
        a: *const Gej,
        b: *const Ge)  {
    
    todo!();
        /*
            /* Operations: 7 mul, 5 sqr, 4 normalize, 21 mul_int/add/negate/cmov */
        static const fe fe_1 = FE_CONST(0, 0, 0, 0, 0, 0, 0, 1);
        fe zz, u1, u2, s1, s2, t, tt, m, n, q, rr;
        fe m_alt, rr_alt;
        int infinity, degenerate;
        VERIFY_CHECK(!b->infinity);
        VERIFY_CHECK(a->infinity == 0 || a->infinity == 1);

        /** In:
         *    Eric Brier and Marc Joye, Weierstrass Elliptic Curves and Side-Channel Attacks.
         *    In D. Naccache and P. Paillier, Eds., Public Key Cryptography, vol. 2274 of Lecture Notes in Computer Science, pages 335-345. Springer-Verlag, 2002.
         *  we find as solution for a unified addition/doubling formula:
         *    lambda = ((x1 + x2)^2 - x1 * x2 + a) / (y1 + y2), with a = 0 for secp256k1's curve equation.
         *    x3 = lambda^2 - (x1 + x2)
         *    2*y3 = lambda * (x1 + x2 - 2 * x3) - (y1 + y2).
         *
         *  Substituting x_i = Xi / Zi^2 and yi = Yi / Zi^3, for i=1,2,3, gives:
         *    U1 = X1*Z2^2, U2 = X2*Z1^2
         *    S1 = Y1*Z2^3, S2 = Y2*Z1^3
         *    Z = Z1*Z2
         *    T = U1+U2
         *    M = S1+S2
         *    Q = T*M^2
         *    R = T^2-U1*U2
         *    X3 = 4*(R^2-Q)
         *    Y3 = 4*(R*(3*Q-2*R^2)-M^4)
         *    Z3 = 2*M*Z
         *  (Note that the paper uses xi = Xi / Zi and yi = Yi / Zi instead.)
         *
         *  This formula has the benefit of being the same for both addition
         *  of distinct points and doubling. However, it breaks down in the
         *  case that either point is infinity, or that y1 = -y2. We handle
         *  these cases in the following ways:
         *
         *    - If b is infinity we simply bail by means of a VERIFY_CHECK.
         *
         *    - If a is infinity, we detect this, and at the end of the
         *      computation replace the result (which will be meaningless,
         *      but we compute to be constant-time) with b.x : b.y : 1.
         *
         *    - If a = -b, we have y1 = -y2, which is a degenerate case.
         *      But here the answer is infinity, so we simply set the
         *      infinity flag of the result, overriding the computed values
         *      without even needing to cmov.
         *
         *    - If y1 = -y2 but x1 != x2, which does occur thanks to certain
         *      properties of our curve (specifically, 1 has nontrivial cube
         *      roots in our field, and the curve equation has no x coefficient)
         *      then the answer is not infinity but also not given by the above
         *      equation. In this case, we cmov in place an alternate expression
         *      for lambda. Specifically (y1 - y2)/(x1 - x2). Where both these
         *      expressions for lambda are defined, they are equal, and can be
         *      obtained from each other by multiplication by (y1 + y2)/(y1 + y2)
         *      then substitution of x^3 + 7 for y^2 (using the curve equation).
         *      For all pairs of nonzero points (a, b) at least one is defined,
         *      so this covers everything.
         */

        fe_sqr(&zz, &a->z);                       /* z = Z1^2 */
        u1 = a->x; fe_normalize_weak(&u1);        /* u1 = U1 = X1*Z2^2 (1) */
        fe_mul(&u2, &b->x, &zz);                  /* u2 = U2 = X2*Z1^2 (1) */
        s1 = a->y; fe_normalize_weak(&s1);        /* s1 = S1 = Y1*Z2^3 (1) */
        fe_mul(&s2, &b->y, &zz);                  /* s2 = Y2*Z1^2 (1) */
        fe_mul(&s2, &s2, &a->z);                  /* s2 = S2 = Y2*Z1^3 (1) */
        t = u1; fe_add(&t, &u2);                  /* t = T = U1+U2 (2) */
        m = s1; fe_add(&m, &s2);                  /* m = M = S1+S2 (2) */
        fe_sqr(&rr, &t);                          /* rr = T^2 (1) */
        fe_negate(&m_alt, &u2, 1);                /* Malt = -X2*Z1^2 */
        fe_mul(&tt, &u1, &m_alt);                 /* tt = -U1*U2 (2) */
        fe_add(&rr, &tt);                         /* rr = R = T^2-U1*U2 (3) */
        /** If lambda = R/M = 0/0 we have a problem (except in the "trivial"
         *  case that Z = z1z2 = 0, and this is special-cased later on). */
        degenerate = fe_normalizes_to_zero(&m) &
                     fe_normalizes_to_zero(&rr);
        /* This only occurs when y1 == -y2 and x1^3 == x2^3, but x1 != x2.
         * This means either x1 == beta*x2 or beta*x1 == x2, where beta is
         * a nontrivial cube root of one. In either case, an alternate
         * non-indeterminate expression for lambda is (y1 - y2)/(x1 - x2),
         * so we set R/M equal to this. */
        rr_alt = s1;
        fe_mul_int(&rr_alt, 2);       /* rr = Y1*Z2^3 - Y2*Z1^3 (2) */
        fe_add(&m_alt, &u1);          /* Malt = X1*Z2^2 - X2*Z1^2 */

        fe_cmov(&rr_alt, &rr, !degenerate);
        fe_cmov(&m_alt, &m, !degenerate);
        /* Now Ralt / Malt = lambda and is guaranteed not to be 0/0.
         * From here on out Ralt and Malt represent the numerator
         * and denominator of lambda; R and M represent the explicit
         * expressions x1^2 + x2^2 + x1x2 and y1 + y2. */
        fe_sqr(&n, &m_alt);                       /* n = Malt^2 (1) */
        fe_mul(&q, &n, &t);                       /* q = Q = T*Malt^2 (1) */
        /* These two lines use the observation that either M == Malt or M == 0,
         * so M^3 * Malt is either Malt^4 (which is computed by squaring), or
         * zero (which is "computed" by cmov). So the cost is one squaring
         * versus two multiplications. */
        fe_sqr(&n, &n);
        fe_cmov(&n, &m, degenerate);              /* n = M^3 * Malt (2) */
        fe_sqr(&t, &rr_alt);                      /* t = Ralt^2 (1) */
        fe_mul(&r->z, &a->z, &m_alt);             /* r->z = Malt*Z (1) */
        infinity = fe_normalizes_to_zero(&r->z) & ~a->infinity;
        fe_mul_int(&r->z, 2);                     /* r->z = Z3 = 2*Malt*Z (2) */
        fe_negate(&q, &q, 1);                     /* q = -Q (2) */
        fe_add(&t, &q);                           /* t = Ralt^2-Q (3) */
        fe_normalize_weak(&t);
        r->x = t;                                           /* r->x = Ralt^2-Q (1) */
        fe_mul_int(&t, 2);                        /* t = 2*x3 (2) */
        fe_add(&t, &q);                           /* t = 2*x3 - Q: (4) */
        fe_mul(&t, &t, &rr_alt);                  /* t = Ralt*(2*x3 - Q) (1) */
        fe_add(&t, &n);                           /* t = Ralt*(2*x3 - Q) + M^3*Malt (3) */
        fe_negate(&r->y, &t, 3);                  /* r->y = Ralt*(Q - 2x3) - M^3*Malt (4) */
        fe_normalize_weak(&r->y);
        fe_mul_int(&r->x, 4);                     /* r->x = X3 = 4*(Ralt^2-Q) */
        fe_mul_int(&r->y, 4);                     /* r->y = Y3 = 4*Ralt*(Q - 2x3) - 4*M^3*Malt (4) */

        /** In case a->infinity == 1, replace r with (b->x, b->y, 1). */
        fe_cmov(&r->x, &b->x, a->infinity);
        fe_cmov(&r->y, &b->y, a->infinity);
        fe_cmov(&r->z, &fe_1, a->infinity);
        r->infinity = infinity;
        */
}

/**
  | Rescale a jacobian point by b which must
  | be non-zero. Constant-time.
  |
  */
pub fn gej_rescale(
        r: *mut Gej,
        s: *const Fe)  {
    
    todo!();
        /*
            /* Operations: 4 mul, 1 sqr */
        fe zz;
        VERIFY_CHECK(!fe_is_zero(s));
        fe_sqr(&zz, s);
        fe_mul(&r->x, &r->x, &zz);                /* r->x *= s^2 */
        fe_mul(&r->y, &r->y, &zz);
        fe_mul(&r->y, &r->y, s);                  /* r->y *= s^3 */
        fe_mul(&r->z, &r->z, s);                  /* r->z *= s   */
        */
}

/**
  | Convert a group element to the storage
  | type.
  |
  */
pub fn ge_to_storage(
        r: *mut GeStorage,
        a: *const Ge)  {
    
    todo!();
        /*
            fe x, y;
        VERIFY_CHECK(!a->infinity);
        x = a->x;
        fe_normalize(&x);
        y = a->y;
        fe_normalize(&y);
        fe_to_storage(&r->x, &x);
        fe_to_storage(&r->y, &y);
        */
}

/**
  | Convert a group element back from the
  | storage type.
  |
  */
pub fn ge_from_storage(
        r: *mut Ge,
        a: *const GeStorage)  {
    
    todo!();
        /*
            fe_from_storage(&r->x, &a->x);
        fe_from_storage(&r->y, &a->y);
        r->infinity = 0;
        */
}

/**
  | If flag is true, set *r equal to *a; otherwise
  | leave it. Constant-time. Both *r and
  | *a must be initialized.
  |
  */
#[inline] pub fn ge_storage_cmov(
        r:    *mut GeStorage,
        a:    *const GeStorage,
        flag: i32)  {
    
    todo!();
        /*
            fe_storage_cmov(&r->x, &a->x, flag);
        fe_storage_cmov(&r->y, &a->y, flag);
        */
}

/**
  | Set r to be equal to lambda times a, where
  | lambda is chosen in a way such that this
  | is very fast.
  |
  */
pub fn ge_mul_lambda(
        r: *mut Ge,
        a: *const Ge)  {
    
    todo!();
        /*
            static const fe beta = FE_CONST(
            0x7ae96a2bul, 0x657c0710ul, 0x6e64479eul, 0xac3434e9ul,
            0x9cf04975ul, 0x12f58995ul, 0xc1396c28ul, 0x719501eeul
        );
        *r = *a;
        fe_mul(&r->x, &r->x, &beta);
        */
}

/**
  | Determine if a point (which is assumed
  | to be on the curve) is in the correct (sub)group
  | of the curve.
  | 
  | In normal mode, the used group is secp256k1,
  | which has cofactor=1 meaning that every
  | point on the curve is in the group, and
  | this function returns always true.
  | 
  | When compiling in exhaustive test mode,
  | a slightly different curve equation
  | is used, leading to a group with a (very)
  | small subgroup, and that subgroup is
  | what is used for all cryptographic operations.
  | In that mode, this function checks whether
  | a point that is on the curve is in fact
  | also in that subgroup.
  |
  */
pub fn ge_is_in_correct_subgroup(ge: *const Ge) -> i32 {
    
    todo!();
        /*
            #ifdef EXHAUSTIVE_TEST_ORDER
        gej out;
        int i;

        /* A very simple EC multiplication ladder that avoids a dependency on ecmult. */
        gej_set_infinity(&out);
        for (i = 0; i < 32; ++i) {
            gej_double_var(&out, &out, NULL);
            if ((((uint32_t)EXHAUSTIVE_TEST_ORDER) >> (31 - i)) & 1) {
                gej_add_ge_var(&out, &out, ge, NULL);
            }
        }
        return gej_is_infinity(&out);
    #else
        (c_void)ge;
        /* The real secp256k1 group has cofactor 1, so the subgroup is the entire curve. */
        return 1;
    #endif
        */
}
