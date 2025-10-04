crate::ix!();

/**
  | One round of SHA-512.
  |
  */
#[inline] pub fn sha512_round(
        a: u64,
        b: u64,
        c: u64,
        d: &mut u64,
        e: u64,
        f: u64,
        g: u64,
        h: &mut u64,
        k: u64,
        w: u64)  {

    #[inline] fn my_sigma0(x: u64) -> u64 {
        
        todo!();
            /*
                return (x >> 28 | x << 36) ^ (x >> 34 | x << 30) ^ (x >> 39 | x << 25);
            */
    }

    #[inline] fn my_sigma1(x: u64) -> u64 {
        
        todo!();
            /*
                return (x >> 14 | x << 50) ^ (x >> 18 | x << 46) ^ (x >> 41 | x << 23);
            */
    }

    
    todo!();
        /*
            uint64_t t1 = h + MySigma1(e) + Ch(e, f, g) + k + w;
        uint64_t t2 = MySigma0(a) + Maj(a, b, c);
        d += t1;
        h = t1 + t2;
        */
}
