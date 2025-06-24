crate::ix!();

impl Num3072 {

    /// Return the modular inverse using sliding‑window exponentiation.
    ///
    /// For fast exponentiation a sliding window exponentiation with repunit precomputation is
    /// utilized. 
    ///
    /// See "Fast Point Decompression for Standard Elliptic Curves" (Brumley, Järvinen, 2008).
    ///
    pub fn get_inverse(&self) -> Num3072 {
        trace!("Num3072::get_inverse");

        // p[i] = a^(2^(2^i) − 1)
        let mut p = [Num3072::default(); 12];
        p[0] = *self;

        for i in 0..11 {
            p[i + 1] = p[i];
            for _ in 0..(1 << i) {
                p[i + 1].square();
            }
            let base = p[i];           // copy, avoids overlapping borrows
            p[i + 1].multiply(&base);
        }

        let mut out = p[11];
        square_n_mul(&mut out, 512, &p[9]);
        square_n_mul(&mut out, 256, &p[8]);
        square_n_mul(&mut out, 128, &p[7]);
        square_n_mul(&mut out,  64, &p[6]);
        square_n_mul(&mut out,  32, &p[5]);
        square_n_mul(&mut out,   8, &p[3]);
        square_n_mul(&mut out,   2, &p[1]);
        square_n_mul(&mut out,   1, &p[0]);
        square_n_mul(&mut out,   5, &p[2]);
        square_n_mul(&mut out,   3, &p[0]);
        square_n_mul(&mut out,   2, &p[0]);
        square_n_mul(&mut out,   4, &p[0]);
        square_n_mul(&mut out,   4, &p[1]);
        square_n_mul(&mut out,   3, &p[0]);
        out
    }
}
