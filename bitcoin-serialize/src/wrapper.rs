// ---------------- [ File: bitcoin-serialize/src/wrapper.rs ]
crate::ix!();

/**
  | Simple wrapper class to serialize objects
  | using a formatter; used by Using().
  |
  */
pub struct Wrapper<'a, T> {
    object: &'a T,
}

impl<'a, T> Wrapper<'a, T> {

    pub fn new(obj: &'a T) -> Self {
    
        todo!();
        /*
        : object(obj),
        */
    }
    
    pub fn serialize<Formatter,Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            Formatter().Ser(s, m_object);
        */
    }
    
    pub fn unserialize<Formatter,Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            Formatter().Unser(s, m_object);
        */
    }
}
