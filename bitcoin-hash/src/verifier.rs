crate::ix!();

/**
  | Reads data from an underlying stream,
  | while hashing the read data.
  |
  */
pub struct HashVerifier<Source> {
    base: HashWriter,
    source: *mut Source,
}

impl<Source,T> Shr<T> for HashVerifier<Source> {

    type Output = HashVerifier<Source>;
    
    #[inline] fn shr(self, rhs: T) -> Self::Output {
        todo!();
        /*
            // Unserialize from this stream
            ::Unserialize(*this, obj);
            return (*this);
        */
    }
}

impl<Source> HashVerifier<Source> {
    
    pub fn new(source: *mut Source) -> Self {
    
        todo!();
        /*


            : HashWriter(source_->GetType(), source_->GetVersion()), source(source_)
        */
    }
    
    pub fn read(&mut self, 
        pch:    *mut u8,
        n_size: usize)  {
        
        todo!();
        /*
            source->read(pch, nSize);
            this->write(pch, nSize);
        */
    }
    
    pub fn ignore(&mut self, n_size: usize)  {
        
        todo!();
        /*
            char data[1024];
            while (nSize > 0) {
                size_t now = std::min<size_t>(nSize, 1024);
                read(data, now);
                nSize -= now;
            }
        */
    }
}
