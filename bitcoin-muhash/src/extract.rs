crate::ix!();

/**
  | Extract the lowest limb of [c0,c1,c2]
  | into n, and left shift the number by 1
  | limb.
  |
  */
#[inline]
pub fn extract3(c0: &mut Limb, c1: &mut Limb, c2: &mut Limb, n: &mut Limb) {
    trace!("extract3");
    *n = *c0;
    *c0 = *c1;
    *c1 = *c2;
    *c2 = 0;
}
