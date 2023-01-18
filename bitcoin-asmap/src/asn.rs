crate::ix!();

lazy_static!{
    static ref ASN_BIT_SIZES: Vec<u8> = vec!{15, 16, 17, 18, 19, 20, 21, 22, 23, 24};
}

pub fn decodeasn<'a, I>(
        bitpos: &mut I,
        endpos: &I) -> u32
where I: Iterator<Item = &'a bool> {
    
    todo!();
        /*
            return DecodeBits(bitpos, endpos, 1, ASN_BIT_SIZES);
        */
}
