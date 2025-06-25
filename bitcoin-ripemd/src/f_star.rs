// ---------------- [ File: bitcoin-ripemd/src/f_star.rs ]
crate::ix!();

/* ----- Internal RIPEMD-160 implementation.   ----- */
#[inline] pub fn ripemd160_f1(
        x: u32,
        y: u32,
        z: u32) -> u32 {
    
    todo!();
        /*
            return x ^ y ^ z;
        */
}

#[inline] pub fn ripemd160_f2(
        x: u32,
        y: u32,
        z: u32) -> u32 {
    
    todo!();
        /*
            return (x & y) | (~x & z);
        */
}

#[inline] pub fn ripemd160_f3(
        x: u32,
        y: u32,
        z: u32) -> u32 {
    
    todo!();
        /*
            return (x | ~y) ^ z;
        */
}

#[inline] pub fn ripemd160_f4(
        x: u32,
        y: u32,
        z: u32) -> u32 {
    
    todo!();
        /*
            return (x & z) | (y & ~z);
        */
}

#[inline] pub fn ripemd160_f5(
        x: u32,
        y: u32,
        z: u32) -> u32 {
    
    todo!();
        /*
            return x ^ (y | ~z);
        */
}
