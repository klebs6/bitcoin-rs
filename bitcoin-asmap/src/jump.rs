crate::ix!();

lazy_static!{
    static ref JUMP_BIT_SIZES: Vec<u8> 
    = vec!{
         5,  6,  7,  8, 
         9, 10, 11, 12, 
        13, 14, 15, 16, 
        17, 18, 19, 20, 
        21, 22, 23, 24, 
        25, 26, 27, 28, 
        29, 30
    };
}

pub fn decode_jump<'a, I>(
        bitpos: &mut I,
        endpos: &I) -> u32
where I: Iterator<Item = &'a bool> {
    
    todo!();
        /*
            return DecodeBits(bitpos, endpos, 17, JUMP_BIT_SIZES);
        */
}

