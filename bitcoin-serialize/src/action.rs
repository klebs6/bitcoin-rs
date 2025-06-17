// ---------------- [ File: bitcoin-serialize/src/action.rs ]
crate::ix!();

/**
  | Support for SERIALIZE_METHODS and
  | READWRITE macro.
  |
  */
pub struct SerActionSerialize { }

impl SerActionSerialize {
    
    pub fn for_read(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

pub struct SerActionUnserialize { }

impl SerActionUnserialize {
    
    pub fn for_read(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
