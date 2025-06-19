// ---------------- [ File: bitcoin-aes/src/ctaes_mix_columns.rs ]
crate::ix!();

impl AESState {
    /// Apply the (inverse) MixColumns transform in‑place.
    ///
    /// * `inv == false` → forward transform  
    /// * `inv == true`  → inverse transform
    #[inline(always)]
    pub fn mix_columns(&mut self, inv: bool) {
        trace!(inv, "mix_columns – input = {:?}", self.slice());

        /* The MixColumns transform treats the bytes of the columns of the state as
         * coefficients of a 3rd degree polynomial over GF(2^8) and multiplies them
         * by the fixed polynomial a(x) = {03}x^3 + {01}x^2 + {01}x + {02}, modulo
         * x^4 + {01}.
         *
         * In the inverse transform, we multiply by the inverse of a(x),
         * a^-1(x) = {0b}x^3 + {0d}x^2 + {09}x + {0e}. This is equal to
         * a(x) * ({04}x^2 + {05}), so we can reuse the forward transform's code
         * (found in OpenSSL's bsaes-x86_64.pl, attributed to Jussi Kivilinna)
         *
         * In the bitsliced representation, a multiplication of every column by x
         * mod x^4 + 1 is simply a right rotation by 4 bits (one nibble).
         */

        let [s0, s1, s2, s3, s4, s5, s6, s7] = self.slice();

        /// Rotate the 16‑bit word right by *n* nibbles (= 4 × *n* bits).
        macro_rules! rot {
            ($v:expr, $n:expr) => {
                $v.rotate_right(($n * 4) as u32)
            };
        }

        // (x³ + x² + x) and (x³ + 1) partial products
        let s0_01  =  s0 ^ rot!(s0, 1);
        let s0_123 = rot!(s0_01, 1) ^ rot!(s0, 3);

        let s1_01  =  s1 ^ rot!(s1, 1);
        let s1_123 = rot!(s1_01, 1) ^ rot!(s1, 3);

        let s2_01  =  s2 ^ rot!(s2, 1);
        let s2_123 = rot!(s2_01, 1) ^ rot!(s2, 3);

        let s3_01  =  s3 ^ rot!(s3, 1);
        let s3_123 = rot!(s3_01, 1) ^ rot!(s3, 3);

        let s4_01  =  s4 ^ rot!(s4, 1);
        let s4_123 = rot!(s4_01, 1) ^ rot!(s4, 3);

        let s5_01  =  s5 ^ rot!(s5, 1);
        let s5_123 = rot!(s5_01, 1) ^ rot!(s5, 3);

        let s6_01  =  s6 ^ rot!(s6, 1);
        let s6_123 = rot!(s6_01, 1) ^ rot!(s6, 3);

        let s7_01  =  s7 ^ rot!(s7, 1);
        let s7_123 = rot!(s7_01, 1) ^ rot!(s7, 3);

        // s = (x³+x²+x)s + {02}(x³+1)s
        self.slice[0] = s7_01 ^ s0_123;
        self.slice[1] = s7_01 ^ s0_01 ^ s1_123;
        self.slice[2] = s1_01 ^ s2_123;
        self.slice[3] = s7_01 ^ s2_01 ^ s3_123;
        self.slice[4] = s7_01 ^ s3_01 ^ s4_123;
        self.slice[5] = s4_01 ^ s5_123;
        self.slice[6] = s5_01 ^ s6_123;
        self.slice[7] = s6_01 ^ s7_123;

        if inv {

            /* In the reverse direction, we further need to multiply by
             * {04}x^2 + {05}, which can be written as {04} * (x^2 + {01}) + {01}.
             *
             * First compute (x^2 + {01}) * s into the t?_02 variables: */

            /* multiply further by {04}(x²+1)+1 */
            trace!("mix_columns – after forward step = {:?}", self.slice);

            let t0_02 = self.slice[0] ^ rot!(self.slice[0], 2);
            let t1_02 = self.slice[1] ^ rot!(self.slice[1], 2);
            let t2_02 = self.slice[2] ^ rot!(self.slice[2], 2);
            let t3_02 = self.slice[3] ^ rot!(self.slice[3], 2);
            let t4_02 = self.slice[4] ^ rot!(self.slice[4], 2);
            let t5_02 = self.slice[5] ^ rot!(self.slice[5], 2);
            let t6_02 = self.slice[6] ^ rot!(self.slice[6], 2);
            let t7_02 = self.slice[7] ^ rot!(self.slice[7], 2);

            self.slice[0] ^= t6_02;
            self.slice[1] ^= t6_02 ^ t7_02;
            self.slice[2] ^= t0_02 ^ t7_02;
            self.slice[3] ^= t1_02 ^ t6_02;
            self.slice[4] ^= t2_02 ^ t6_02 ^ t7_02;
            self.slice[5] ^= t3_02 ^ t7_02;
            self.slice[6] ^= t4_02;
            self.slice[7] ^= t5_02;
        }

        trace!(inv, "mix_columns – output = {:?}", self.slice);
    }
}

#[cfg(test)]
mod mix_columns_tests {
    use super::*;

    #[traced_test]
    fn forward_then_inverse_is_identity() {
        let mut rng = rand::thread_rng();

        for _ in 0..1_000 {
            let sample: [u16; 8] = [
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
            ];

            let mut state = AESStateBuilder::default().slice(sample).build().unwrap();
            state.mix_columns(false); // forward
            state.mix_columns(true);  // inverse
            assert_eq!(state.slice(), &sample);
        }
    }
}
