crate::ix!();

///------------------------------
pub struct AmountCompression {

}

impl AmountCompression {

    pub fn ser<Stream: Write + VarIntWriter>(&mut self, 
        stream: &mut Stream,
        val:    u64)  {
    
        stream.write_varint(compress_amount(val));
    }
    
    pub fn unser<Stream: Read + VarIntReader>(&mut self, 
        stream: &mut Stream,
        val:    &mut u64)  {
    
        let mut v = stream.read_varint().unwrap();

        *val = decompress_amount(v);
    }
}
