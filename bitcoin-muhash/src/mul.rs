crate::ix!();

/**
  | [c0,c1] = a * b
  |
  */
#[inline]
pub fn mul(c0: &mut Limb, c1: &mut Limb, a: &Limb, b: &Limb) {
    trace!("mul");
    let t: DoubleLimb = (*a as DoubleLimb) * (*b as DoubleLimb);
    *c1 = (t >> LIMB_SIZE) as Limb;
    *c0 = t as Limb;
}

/**
  | [c0,c1,c2] += n * [d0,d1,d2]. c2 is 0
  | initially
  |
  */
#[inline]
pub fn mulnadd3(
    c0: &mut Limb,
    c1: &mut Limb,
    c2: &mut Limb,
    d0: &mut Limb,
    d1: &mut Limb,
    d2: &mut Limb,
    n: &Limb,
) {
    trace!("mulnadd3");
    let mut t: DoubleLimb = (*d0 as DoubleLimb) * (*n as DoubleLimb) + *c0 as DoubleLimb;
    *c0 = t as Limb;
    t >>= LIMB_SIZE;
    t += (*d1 as DoubleLimb) * (*n as DoubleLimb) + *c1 as DoubleLimb;
    *c1 = t as Limb;
    t >>= LIMB_SIZE;
    *c2 = (t + (*d2 as DoubleLimb) * (*n as DoubleLimb)) as Limb;
}

/**
  | [c0,c1] *= n
  |
  */
#[inline]
pub fn muln2(c0: &mut Limb, c1: &mut Limb, n: &Limb) {
    trace!("muln2");
    let mut t: DoubleLimb = (*c0 as DoubleLimb) * (*n as DoubleLimb);
    *c0 = t as Limb;
    t >>= LIMB_SIZE;
    t += (*c1 as DoubleLimb) * (*n as DoubleLimb);
    *c1 = t as Limb;
}

/**
  | [c0,c1,c2] += a * b
  |
  */
#[inline]
pub fn muladd3(
    c0: &mut Limb,
    c1: &mut Limb,
    c2: &mut Limb,
    a: &Limb,
    b: &Limb,
) {
    trace!("muladd3");
    let t: DoubleLimb = (*a as DoubleLimb) * (*b as DoubleLimb);
    let th: Limb = (t >> LIMB_SIZE) as Limb;
    let tl: Limb = t as Limb;

    let (new_c0, carry0) = c0.overflowing_add(tl);
    *c0 = new_c0;
    let mut th = th + if carry0 { 1 } else { 0 };

    let (new_c1, carry1) = c1.overflowing_add(th);
    *c1 = new_c1;
    *c2 += if carry1 { 1 } else { 0 };
}

/**
  | [c0,c1,c2] += 2 * a * b
  |
  */
#[inline]
pub fn muldbladd3(
    c0: &mut Limb,
    c1: &mut Limb,
    c2: &mut Limb,
    a: &Limb,
    b: &Limb,
) {
    trace!("muldbladd3");
    // First add
    muladd3(c0, c1, c2, a, b);
    // Second add
    muladd3(c0, c1, c2, a, b);
}
