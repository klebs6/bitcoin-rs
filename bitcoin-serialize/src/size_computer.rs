crate::ix!();

/**
  | ::GetSerializeSize implementations
  | 
  | Computing the serialized size of objects
  | is done through a special stream object
  | of type CSizeComputer, which only records
  | the number of bytes written to it.
  | 
  | If your Serialize or SerializationOp
  | method has non-trivial overhead for
  | serialization, it may be worthwhile
  | to implement a specialized version
  | for
  | 
  | CSizeComputer, which uses the s.seek()
  | method to record bytes that would be
  | written instead.
  |
  */
pub struct SizeComputer {
    n_size:    usize,
    n_version: i32,
}

impl<T> Shl<&T> for SizeComputer {
    type Output = SizeComputer;
    
    #[inline] fn shl(self, rhs: &T) -> Self::Output {
        todo!();
        /*
            ::Serialize(*this, obj);
            return (*this);
        */
    }
}

impl SizeComputer {
    
    pub const fn new(n_version_in: i32) -> Self {
        Self {
            n_size:    0,
            n_version: n_version_in,
        }
    }
    
    pub fn write(&mut self, 
        psz:    *const u8,
        n_size: usize)  {
        
        todo!();
        /*
            this->nSize += _nSize;
        */
    }

    /**
      | Pretend _nSize bytes are written, without
      | specifying them.
      |
      */
    pub fn seek(&mut self, n_size: usize)  {
        
        todo!();
        /*
            this->nSize += _nSize;
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return nSize;
        */
    }
    
    pub fn get_version(&self) -> i32 {
        
        todo!();
        /*
            return nVersion;
        */
    }
}
