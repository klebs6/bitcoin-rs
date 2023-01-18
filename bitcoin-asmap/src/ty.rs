crate::ix!();

lazy_static!{
    static ref TYPE_BIT_SIZES: Vec<u8> = vec!{0, 0, 1};
}

pub fn decode_type<'a, I>(
        bitpos: &mut I,
        endpos: &I) -> Instruction 
where I: Iterator<Item = &'a bool> {
    
    Instruction::new(decode_bits(bitpos,endpos,0,&TYPE_BIT_SIZES))
}

