crate::ix!();

lazy_static!{
    static ref MATCH_BIT_SIZES: Vec<u8> = vec!{1, 2, 3, 4, 5, 6, 7, 8};
}

pub fn decode_match<'a, I>(
        bitpos: &mut I,
        endpos: &I) -> u32 
where I: Iterator<Item = &'a bool> {
    
    todo!();
        /*
            return DecodeBits(bitpos, endpos, 2, MATCH_BIT_SIZES);
        */
}

