crate::ix!();

/**
  | Serialization wrapper class for integers
  | in VarInt format.
  |
  */
pub struct VarIntFormatter<const Mode: VarIntMode> {

}

impl<const Mode: VarIntMode> VarIntFormatter<Mode> {
    
    pub fn ser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: I)  {
    
        todo!();
        /*
            WriteVarInt<Stream,Mode,typename std::remove_cv<I>::type>(s, v);
        */
    }
    
    
    pub fn unser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: &mut I)  {
    
        todo!();
        /*
            v = ReadVarInt<Stream,Mode,typename std::remove_cv<I>::type>(s);
        */
    }
}
