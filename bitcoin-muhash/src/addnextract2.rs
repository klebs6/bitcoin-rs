// ---------------- [ File: bitcoin-muhash/src/addnextract2.rs ]
crate::ix!();

/**
  | Add limb a to [c0,c1]: [c0,c1] += a. Then
  | extract the lowest limb of [c0,c1] into
  | n, and left shift the number by 1 limb.
  |
  */
#[inline]
pub fn addnextract2(c0: &mut Limb, c1: &mut Limb, a: &Limb, n: &mut Limb) {

    trace!("addnextract2");

    let mut c2: Limb = 0;

    // add
    let (new_c0, carry0) = c0.overflowing_add(*a);

    *c0 = new_c0;

    if carry0 {

        let (new_c1, carry1) = c1.overflowing_add(1);

        *c1 = new_c1;

        // Handle case when c1 has overflown
        if carry1 {
            c2 = 1;
        }
    }

    // extract
    *n = *c0;
    *c0 = *c1;
    *c1 = c2;
}
