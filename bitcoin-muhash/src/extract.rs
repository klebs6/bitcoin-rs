// ---------------- [ File: bitcoin-muhash/src/extract.rs ]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract3_basic() {
        let mut c0: Limb = 11;
        let mut c1: Limb = 22;
        let mut c2: Limb = 33;
        let mut n: Limb = 0;

        extract3(&mut c0, &mut c1, &mut c2, &mut n);

        assert_eq!(n, 11);
        assert_eq!(c0, 22);
        assert_eq!(c1, 33);
        assert_eq!(c2, 0);
    }

    #[test]
    fn extract3_with_zeroes_and_nonzero_c2() {
        let mut c0: Limb = 0;
        let mut c1: Limb = 0;
        let mut c2: Limb = 123;
        let mut n: Limb = 0;

        extract3(&mut c0, &mut c1, &mut c2, &mut n);

        assert_eq!(n, 0);
        assert_eq!(c0, 0);
        assert_eq!(c1, 123);
        assert_eq!(c2, 0);
    }
}
