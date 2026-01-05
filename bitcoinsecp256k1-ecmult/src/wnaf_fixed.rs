// ---------------- [ File: bitcoinsecp256k1-ecmult/src/wnaf_fixed.rs ]
crate::ix!();

/// Convert a number to WNAF notation.
/// 
/// The number becomes represented by sum(2^{wi} * wnaf[i], i=0..WNAF_SIZE(w)+1) - return_val.
/// 
/// It has the following guarantees:
/// 
/// - each wnaf[i] is either 0 or an odd integer between -(1 << w) and (1 << w)
/// 
/// - the number of words set is always WNAF_SIZE(w)
/// 
/// - the returned skew is 0 or 1
///
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
