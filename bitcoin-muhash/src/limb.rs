crate::ix!();

pub type Limb       = num_3072::Limb;
pub type DoubleLimb = num_3072::DoubleLimb;

pub const LIMB_SIZE: usize = num_3072::LIMB_SIZE;

/**
  | 2^3072 - 1103717, the largest 3072-bit
  | safe prime number, is used as the modulus.
  |
  */
pub const MAX_PRIME_DIFF: Limb = 1103717;

/**
  | Extract the lowest limb of [c0,c1,c2]
  | into n, and left shift the number by 1
  | limb.
  |
  */
#[inline] pub fn extract3(
        c0: &mut Limb,
        c1: &mut Limb,
        c2: &mut Limb,
        n:  &mut Limb)  {
    
    todo!();
        /*
            n = c0;
        c0 = c1;
        c1 = c2;
        c2 = 0;
        */
}

/**
  | [c0,c1] = a * b
  |
  */
#[inline] pub fn mul(
        c0: &mut Limb,
        c1: &mut Limb,
        a:  &Limb,
        b:  &Limb)  {
    
    todo!();
        /*
            double_limb_t t = (double_limb_t)a * b;
        c1 = t >> LIMB_SIZE;
        c0 = t;
        */
}

/**
  | [c0,c1,c2] += n * [d0,d1,d2]. c2 is 0
  | initially
  |
  */
#[inline] pub fn mulnadd3(
        c0: &mut Limb,
        c1: &mut Limb,
        c2: &mut Limb,
        d0: &mut Limb,
        d1: &mut Limb,
        d2: &mut Limb,
        n:  &Limb)  {
    
    todo!();
        /*
            double_limb_t t = (double_limb_t)d0 * n + c0;
        c0 = t;
        t >>= LIMB_SIZE;
        t += (double_limb_t)d1 * n + c1;
        c1 = t;
        t >>= LIMB_SIZE;
        c2 = t + d2 * n;
        */
}

/**
  | [c0,c1] *= n
  |
  */
#[inline] pub fn muln2(
        c0: &mut Limb,
        c1: &mut Limb,
        n:  &Limb)  {
    
    todo!();
        /*
            double_limb_t t = (double_limb_t)c0 * n;
        c0 = t;
        t >>= LIMB_SIZE;
        t += (double_limb_t)c1 * n;
        c1 = t;
        */
}

/**
  | [c0,c1,c2] += a * b
  |
  */
#[inline] pub fn muladd3(
        c0: &mut Limb,
        c1: &mut Limb,
        c2: &mut Limb,
        a:  &Limb,
        b:  &Limb)  {
    
    todo!();
        /*
            double_limb_t t = (double_limb_t)a * b;
        limb_t th = t >> LIMB_SIZE;
        limb_t tl = t;

        c0 += tl;
        th += (c0 < tl) ? 1 : 0;
        c1 += th;
        c2 += (c1 < th) ? 1 : 0;
        */
}

/**
  | [c0,c1,c2] += 2 * a * b
  |
  */
#[inline] pub fn muldbladd3(
        c0: &mut Limb,
        c1: &mut Limb,
        c2: &mut Limb,
        a:  &Limb,
        b:  &Limb)  {
    
    todo!();
        /*
            double_limb_t t = (double_limb_t)a * b;
        limb_t th = t >> LIMB_SIZE;
        limb_t tl = t;

        c0 += tl;
        limb_t tt = th + ((c0 < tl) ? 1 : 0);
        c1 += tt;
        c2 += (c1 < tt) ? 1 : 0;
        c0 += tl;
        th += (c0 < tl) ? 1 : 0;
        c1 += th;
        c2 += (c1 < th) ? 1 : 0;
        */
}

/**
  | Add limb a to [c0,c1]: [c0,c1] += a. Then
  | extract the lowest limb of [c0,c1] into
  | n, and left shift the number by 1 limb.
  |
  */
#[inline] pub fn addnextract2(
        c0: &mut Limb,
        c1: &mut Limb,
        a:  &Limb,
        n:  &mut Limb)  {
    
    todo!();
        /*
            limb_t c2 = 0;

        // add
        c0 += a;
        if (c0 < a) {
            c1 += 1;

            // Handle case when c1 has overflown
            if (c1 == 0)
                c2 = 1;
        }

        // extract
        n = c0;
        c0 = c1;
        c1 = c2;
        */
}
