crate::ix!();

#[inline] pub fn ripemd160_rol(x: u32, i: i32) -> u32 {
    
    todo!();
        /*
            return (x << i) | (x >> (32 - i));
        */
}

#[inline] pub fn ripemd160_round(
        a: &mut u32,
        b: u32,
        c: &mut u32,
        d: u32,
        e: u32,
        f: u32,
        x: u32,
        k: u32,
        r: i32)  {
    
    todo!();
        /*
            a = rol(a + f + x + k, r) + e;
        c = rol(c, 10);
        */
}
