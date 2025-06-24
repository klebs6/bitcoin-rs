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
