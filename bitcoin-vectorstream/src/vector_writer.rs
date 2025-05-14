// ---------------- [ File: bitcoin-vectorstream/src/vector_writer.rs ]
crate::ix!();

/**
  | Minimal stream for overwriting and/or
  | appending to an existing byte vector
  | 
  | The referenced vector will grow as necessary
  |
  */
pub struct VectorWriter {
    n_type:    i32,
    n_version: i32,
    vch_data:  Rc<RefCell<Vec<u8>>>,
    n_pos:     usize,
}

impl<T> Shl<&T> for VectorWriter {
    type Output = VectorWriter;
    
    #[inline] fn shl(self, rhs: &T) -> Self::Output {
        todo!();
        /*
            // Serialize to this stream
            ::Serialize(*this, obj);
            return (*this);
        */
    }
}

impl VectorWriter {

    /**
      | @param[in] nTypeIn
      | 
      | Serialization Type
      | ----------
      | @param[in] nVersionIn
      | 
      | Serialization Version (including
      | any flags)
      | ----------
      | @param[in] vchDataIn
      | 
      | Referenced byte vector to overwrite/append
      | ----------
      | @param[in] nPosIn
      | 
      | Starting position. Vector index where
      | writes should start. The vector will
      | initially grow as necessary to max(nPosIn,
      | vec.size()). So to append, use vec.size().
      |
      */
    pub fn new(
        n_type_in:    i32,
        n_version_in: i32,
        vch_data_in:  &mut Vec<u8>,
        n_pos_in:     usize) -> Self {
    
        todo!();
        /*
        : n_type(nTypeIn),
        : n_version(nVersionIn),
        : vch_data(vchDataIn),
        : n_pos(nPosIn),

            if(nPos > vchData.size())
                vchData.resize(nPos);
        */
    }

    /**
      | (other params same as above)
      | 
      | -----------
      | @param[in] args
      | 
      | A list of items to serialize starting
      | at nPosIn.
      |
      */
    pub fn new_with_args<Args>(
        n_type_in:    i32,
        n_version_in: i32,
        vch_data_in:  &mut Vec<u8>,
        n_pos_in:     usize,
        args:         Args) -> Self {
    
        todo!();
        /*


            : CVectorWriter(nTypeIn, nVersionIn, vchDataIn, nPosIn)
            ::SerializeMany(*this, std::forward<Args>(args)...);
        */
    }
    
    pub fn write(&mut self, 
        pch:    *const u8,
        n_size: usize)  {
        
        todo!();
        /*
            assert(nPos <= vchData.size());
            size_t nOverwrite = std::min(nSize, vchData.size() - nPos);
            if (nOverwrite) {
                memcpy(vchData.data() + nPos, reinterpret_cast<const unsigned char*>(pch), nOverwrite);
            }
            if (nOverwrite < nSize) {
                vchData.insert(vchData.end(), reinterpret_cast<const unsigned char*>(pch) + nOverwrite, reinterpret_cast<const unsigned char*>(pch) + nSize);
            }
            nPos += nSize;
        */
    }
    
    pub fn get_version(&self) -> i32 {
        
        todo!();
        /*
            return nVersion;
        */
    }
    
    pub fn get_type(&self) -> i32 {
        
        todo!();
        /*
            return nType;
        */
    }
}
