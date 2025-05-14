// ---------------- [ File: bitcoin-fees/src/encoded_double_formatter.rs ]
crate::ix!();

pub struct EncodedDoubleFormatter {

}

impl EncodedDoubleFormatter {
    
    pub fn ser<Stream>(&mut self, 
        s: &mut Stream,
        v: f64)  {
    
        todo!();
        /*
            s << EncodeDouble(v);
        */
    }
    
    pub fn unser<Stream>(&mut self, 
        s: &mut Stream,
        v: &mut f64)  {
    
        todo!();
        /*
            uint64_t encoded;
            s >> encoded;
            v = DecodeDouble(encoded);
        */
    }
}
