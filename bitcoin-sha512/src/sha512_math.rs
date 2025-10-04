crate::ix!();

#[inline] pub fn sha512_ch(
        x: u64,
        y: u64,
        z: u64) -> u64 {
    
    todo!();
        /*
            return z ^ (x & (y ^ z));
        */
}

#[inline] pub fn sha512_maj(
        x: u64,
        y: u64,
        z: u64) -> u64 {
    
    todo!();
        /*
            return (x & y) | (z & (x | y));
        */
}

#[inline] pub fn sha512_sigma0(x: u64) -> u64 {
    
    todo!();
        /*
            return (x >> 1 | x << 63) ^ (x >> 8 | x << 56) ^ (x >> 7);
        */
}

#[inline] pub fn sha512_sigma1(x: u64) -> u64 {
    
    todo!();
        /*
            return (x >> 19 | x << 45) ^ (x >> 61 | x << 3) ^ (x >> 6);
        */
}
