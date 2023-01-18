crate::ix!();

pub struct Num3072 {
    limbs: [Limb; num_3072::LIMBS],
}

pub mod num_3072 {

    use super::*;

    pub const BYTE_SIZE: usize = 384;

    #[cfg(HAVE___INT128)]      pub type DoubleLimb        = __int128;
    #[cfg(HAVE___INT128)]      pub type Limb              = u64;
    #[cfg(HAVE___INT128)]      pub const LIMBS:     usize = 48;
    #[cfg(HAVE___INT128)]      pub const LIMB_SIZE: usize = 64;

    #[cfg(not(HAVE___INT128))] pub type DoubleLimb        = u64;
    #[cfg(not(HAVE___INT128))] pub type Limb              = u32;
    #[cfg(not(HAVE___INT128))] pub const LIMBS:     usize = 96;
    #[cfg(not(HAVE___INT128))] pub const LIMB_SIZE: usize = 32;

    /**
      | Sanity check for Num3072 constants
      |
      */
    const_assert!{ LIMB_SIZE * LIMBS == 3072 }                   //Num3072 isn't 3072 bits

    const_assert!{ size_of::<DoubleLimb>() == size_of::<Limb>() * 2 } //"bad size for double_limb_t"

    const_assert!{ size_of::<Limb>() * 8 == LIMB_SIZE }             //"LIMB_SIZE is incorrect"

    /**
      | Hard coded values in MuHash3072 constructor
      | and Finalize
      |
      */
    const_assert!{ size_of::<Limb>() == 4 || size_of::<Limb>() == 8 } //"bad size for limb_t"
}

impl Default for Num3072 {
    
    fn default() -> Self {
        todo!();
        /*
            this->SetToOne(); }{
        */
    }
}

lazy_static!{
    /*
    SERIALIZE_METHODS(Num3072, obj)
        {
            for (auto& limb : obj.limbs) {
                READWRITE(limb);
            }
        }
    */
}

/**
  | in_out = in_out^(2^sq) * mul
  |
  */
#[inline] pub fn square_n_mul(
        in_out: &mut Num3072,
        sq:     i32,
        mul:    &Num3072)  {
    
    todo!();
        /*
            for (int j = 0; j < sq; ++j) in_out.Square();
        in_out.Multiply(mul);
        */
}

impl Num3072 {

    /**
      | Indicates whether d is larger than the
      | modulus.
      |
      */
    pub fn is_overflow(&self) -> bool {
        
        todo!();
        /*
            if (this->limbs[0] <= std::numeric_limits<limb_t>::max() - MAX_PRIME_DIFF) return false;
        for (int i = 1; i < LIMBS; ++i) {
            if (this->limbs[i] != std::numeric_limits<limb_t>::max()) return false;
        }
        return true;
        */
    }
    
    pub fn full_reduce(&mut self)  {
        
        todo!();
        /*
            limb_t c0 = MAX_PRIME_DIFF;
        limb_t c1 = 0;
        for (int i = 0; i < LIMBS; ++i) {
            addnextract2(c0, c1, this->limbs[i], this->limbs[i]);
        }
        */
    }
    
    pub fn get_inverse(&self) -> Num3072 {
        
        todo!();
        /*
            // For fast exponentiation a sliding window exponentiation with repunit
        // precomputation is utilized. See "Fast Point Decompression for Standard
        // Elliptic Curves" (Brumley, Järvinen, 2008).

        Num3072 p[12]; // p[i] = a^(2^(2^i)-1)
        Num3072 out;

        p[0] = *this;

        for (int i = 0; i < 11; ++i) {
            p[i + 1] = p[i];
            for (int j = 0; j < (1 << i); ++j) p[i + 1].Square();
            p[i + 1].Multiply(p[i]);
        }

        out = p[11];

        square_n_mul(out, 512, p[9]);
        square_n_mul(out, 256, p[8]);
        square_n_mul(out, 128, p[7]);
        square_n_mul(out, 64, p[6]);
        square_n_mul(out, 32, p[5]);
        square_n_mul(out, 8, p[3]);
        square_n_mul(out, 2, p[1]);
        square_n_mul(out, 1, p[0]);
        square_n_mul(out, 5, p[2]);
        square_n_mul(out, 3, p[0]);
        square_n_mul(out, 2, p[0]);
        square_n_mul(out, 4, p[0]);
        square_n_mul(out, 4, p[1]);
        square_n_mul(out, 3, p[0]);

        return out;
        */
    }
    
    pub fn multiply(&mut self, a: &Num3072)  {
        
        todo!();
        /*
            limb_t c0 = 0, c1 = 0, c2 = 0;
        Num3072 tmp;

        /* Compute limbs 0..N-2 of this*a into tmp, including one reduction. */
        for (int j = 0; j < LIMBS - 1; ++j) {
            limb_t d0 = 0, d1 = 0, d2 = 0;
            mul(d0, d1, this->limbs[1 + j], a.limbs[LIMBS + j - (1 + j)]);
            for (int i = 2 + j; i < LIMBS; ++i) muladd3(d0, d1, d2, this->limbs[i], a.limbs[LIMBS + j - i]);
            mulnadd3(c0, c1, c2, d0, d1, d2, MAX_PRIME_DIFF);
            for (int i = 0; i < j + 1; ++i) muladd3(c0, c1, c2, this->limbs[i], a.limbs[j - i]);
            extract3(c0, c1, c2, tmp.limbs[j]);
        }

        /* Compute limb N-1 of a*b into tmp. */
        assert(c2 == 0);
        for (int i = 0; i < LIMBS; ++i) muladd3(c0, c1, c2, this->limbs[i], a.limbs[LIMBS - 1 - i]);
        extract3(c0, c1, c2, tmp.limbs[LIMBS - 1]);

        /* Perform a second reduction. */
        muln2(c0, c1, MAX_PRIME_DIFF);
        for (int j = 0; j < LIMBS; ++j) {
            addnextract2(c0, c1, tmp.limbs[j], this->limbs[j]);
        }

        assert(c1 == 0);
        assert(c0 == 0 || c0 == 1);

        /* Perform up to two more reductions if the internal state has already
         * overflown the MAX of Num3072 or if it is larger than the modulus or
         * if both are the case.
         * */
        if (this->IsOverflow()) this->FullReduce();
        if (c0) this->FullReduce();
        */
    }
    
    pub fn square(&mut self)  {
        
        todo!();
        /*
            limb_t c0 = 0, c1 = 0, c2 = 0;
        Num3072 tmp;

        /* Compute limbs 0..N-2 of this*this into tmp, including one reduction. */
        for (int j = 0; j < LIMBS - 1; ++j) {
            limb_t d0 = 0, d1 = 0, d2 = 0;
            for (int i = 0; i < (LIMBS - 1 - j) / 2; ++i) muldbladd3(d0, d1, d2, this->limbs[i + j + 1], this->limbs[LIMBS - 1 - i]);
            if ((j + 1) & 1) muladd3(d0, d1, d2, this->limbs[(LIMBS - 1 - j) / 2 + j + 1], this->limbs[LIMBS - 1 - (LIMBS - 1 - j) / 2]);
            mulnadd3(c0, c1, c2, d0, d1, d2, MAX_PRIME_DIFF);
            for (int i = 0; i < (j + 1) / 2; ++i) muldbladd3(c0, c1, c2, this->limbs[i], this->limbs[j - i]);
            if ((j + 1) & 1) muladd3(c0, c1, c2, this->limbs[(j + 1) / 2], this->limbs[j - (j + 1) / 2]);
            extract3(c0, c1, c2, tmp.limbs[j]);
        }

        assert(c2 == 0);
        for (int i = 0; i < LIMBS / 2; ++i) muldbladd3(c0, c1, c2, this->limbs[i], this->limbs[LIMBS - 1 - i]);
        extract3(c0, c1, c2, tmp.limbs[LIMBS - 1]);

        /* Perform a second reduction. */
        muln2(c0, c1, MAX_PRIME_DIFF);
        for (int j = 0; j < LIMBS; ++j) {
            addnextract2(c0, c1, tmp.limbs[j], this->limbs[j]);
        }

        assert(c1 == 0);
        assert(c0 == 0 || c0 == 1);

        /* Perform up to two more reductions if the internal state has already
         * overflown the MAX of Num3072 or if it is larger than the modulus or
         * if both are the case.
         * */
        if (this->IsOverflow()) this->FullReduce();
        if (c0) this->FullReduce();
        */
    }
    
    pub fn set_to_one(&mut self)  {
        
        todo!();
        /*
            this->limbs[0] = 1;
        for (int i = 1; i < LIMBS; ++i) this->limbs[i] = 0;
        */
    }
    
    pub fn divide(&mut self, a: &Num3072)  {
        
        todo!();
        /*
            if (this->IsOverflow()) this->FullReduce();

        Num3072 inv{};
        if (a.IsOverflow()) {
            Num3072 b = a;
            b.FullReduce();
            inv = b.GetInverse();
        } else {
            inv = a.GetInverse();
        }

        this->Multiply(inv);
        if (this->IsOverflow()) this->FullReduce();
        */
    }
    
    pub fn new(data: &[u8; num_3072::BYTE_SIZE]) -> Self {
    
        todo!();
        /*


            for (int i = 0; i < LIMBS; ++i) {
            if (sizeof(limb_t) == 4) {
                this->limbs[i] = ReadLE32(data + 4 * i);
            } else if (sizeof(limb_t) == 8) {
                this->limbs[i] = ReadLE64(data + 8 * i);
            }
        }
        */
    }
    
    pub fn to_bytes(&mut self, out: &mut [u8; num_3072::BYTE_SIZE])  {
        
        todo!();
        /*
            for (int i = 0; i < LIMBS; ++i) {
            if (sizeof(limb_t) == 4) {
                WriteLE32(out + i * 4, this->limbs[i]);
            } else if (sizeof(limb_t) == 8) {
                WriteLE64(out + i * 8, this->limbs[i]);
            }
        }
        */
    }
}
