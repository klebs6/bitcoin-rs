// ---------------- [ File: bitcoin-muhash/src/divide.rs ]
crate::ix!();

impl Num3072 {

    /// Return *true* iff the value is exactly one.
    #[inline]
    pub fn is_one(&self) -> bool {
        trace!("Num3072::is_one");
        self.limbs()[0] == 1 && self.limbs()[1..].iter().all(|&x| x == 0)
    }

    pub fn divide(&mut self, a: &Num3072) {
        trace!("Num3072::divide");
        if self.is_overflow() {
            self.full_reduce();
        }

        /* Fast‑path: dividing by 1 is a no‑op and avoids an expensive
         * 3 000‑square sliding‑window modular‑inverse calculation.     */
        if a.is_one() {
            return;
        }

        let inv = if a.is_overflow() {
            let mut b = *a;
            b.full_reduce();
            b.get_inverse()
        } else {
            a.get_inverse()
        };

        self.multiply(&inv);
        if self.is_overflow() {
            self.full_reduce();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::muhash::MuHash3072;

    fn num(b: &[u8]) -> Num3072 { MuHash3072::to_num3072(b) }

    #[test]
    fn is_one_detects_one() {
        let one = Num3072::default();
        assert!(one.is_one());
        let mut n = one;
        n.limbs_mut()[0] = n.limbs()[0].wrapping_add(1);
        assert!(!n.is_one());
    }

    #[test]
    fn divide_by_one_is_noop_but_reduces_self_if_overflow() {
        // Create an overflow value then divide by 1 → should reduce
        let mut x = Num3072::default();
        let lims = x.limbs().len();
        x.limbs_mut()[0] = Limb::MAX - crate::MAX_PRIME_DIFF + 5;
        for i in 1..lims { x.limbs_mut()[i] = Limb::MAX; }
        assert!(x.is_overflow());

        let one = Num3072::default();
        let mut y = x;
        y.divide(&one);

        assert!(!y.is_overflow(), "divide() must reduce overflow even for divisor 1");
    }

    #[test]
    fn divide_by_self_yields_one() {
        let mut x = num(b"alice");
        let orig = x;
        x.divide(&orig);
        assert!(x.is_one());
    }

    #[test]
    fn divide_correctness_inverse_multiplication() {
        let a = num(b"alice");
        let b = num(b"bob");
        let mut x = a;
        x.multiply(&b);

        let mut y = x;     // y = a*b
        y.divide(&b);      // y = a

        assert_eq!(y.limbs(), a.limbs());
    }

    #[test]
    fn divide_handles_overflow_divisor() {
        // Craft an "overflow" divisor and check path that reduces it internally.
        let mut d = Num3072::default();
        let lims = d.limbs().len();
        d.limbs_mut()[0] = Limb::MAX - crate::MAX_PRIME_DIFF + 3;
        for i in 1..lims { d.limbs_mut()[i] = Limb::MAX; }
        assert!(d.is_overflow());

        let mut x = Num3072::default();
        x.limbs_mut()[0] = 7; // x ≠ 0

        let mut y = x;
        y.divide(&d); // should compute inverse(d) after reducing it, then multiply

        // Multiply back to recover x: (x * inv(d)) * d = x
        let mut back = y;
        back.multiply(&d);
        back.full_reduce();
        x.full_reduce();
        assert_eq!(back.limbs(), x.limbs());
    }
}
