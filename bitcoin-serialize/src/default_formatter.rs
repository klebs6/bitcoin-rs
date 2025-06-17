crate::ix!();

/**
  | Default formatter. Serializes objects
  | as themselves.
  | 
  | The vector/prevector serialization
  | code passes this to VectorFormatter
  | to enable reusing that logic. It shouldn't
  | be needed elsewhere.
  |
  */
pub struct DefaultFormatter<'a,T> { 
    item: &'a mut T,
}

impl<'a,T> DefaultFormatter<'a,T> {
    
    pub fn ser<Stream>(
        s: &mut Stream,
        t: &T)  {
    
        todo!();
        /*
            Serialize(s, t);
        */
    }
    
    pub fn unser<Stream>(
        s: &mut Stream,
        t: &mut T)  {
    
        todo!();
        /*
            Unserialize(s, t);
        */
    }
}
