crate::ix!();

/**
  | Minimal stream for reading from an existing
  | vector by reference
  |
  */
pub struct VectorReader {
    ty:      i32,
    version: i32,
    data:    Arc<Vec<u8>>,
    pos:     usize, // default = 0
}

impl<T> Shr<T> for VectorReader {
    type Output = VectorReader;
    
    #[inline] fn shr(self, rhs: T) -> Self::Output {
        todo!();
        /*
            // Unserialize from this stream
            ::Unserialize(*this, obj);
            return (*this);
        */
    }
}

impl VectorReader {

    /**
      | @param[in] type
      | 
      | Serialization Type
      | ----------
      | @param[in] version
      | 
      | Serialization Version (including
      | any flags)
      | ----------
      | @param[in] data
      | 
      | Referenced byte vector to overwrite/append
      | ----------
      | @param[in] pos
      | 
      | Starting position. Vector index where
      | reads should start.
      |
      */
    pub fn new(
        ty:      i32,
        version: i32,
        data:    &Vec<u8>,
        pos:     usize) -> Self {
    
        todo!();
        /*


            : m_type(type), m_version(version), m_data(data), m_pos(pos)

            if (m_pos > m_data.size()) {
                throw std::ios_base::failure("VectorReader(...): end of data (m_pos > m_data.size())");
            }
        */
    }

    /**
      | (other params same as above)
      | 
      | -----------
      | @param[in] args
      | 
      | A list of items to deserialize starting
      | at pos.
      |
      */
    pub fn new_with_args<Args>(
        ty:      i32,
        version: i32,
        data:    &Vec<u8>,
        pos:     usize,
        args:    Args) -> Self {
    
        todo!();
        /*


            : VectorReader(type, version, data, pos)

            ::UnserializeMany(*this, std::forward<Args>(args)...);
        */
    }
    
    pub fn get_version(&self) -> i32 {
        
        todo!();
        /*
            return m_version;
        */
    }
    
    pub fn get_type(&self) -> i32 {
        
        todo!();
        /*
            return m_type;
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return m_data.size() - m_pos;
        */
    }
    
    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return m_data.size() == m_pos;
        */
    }
    
    pub fn read(&mut self, 
        dst: *mut u8,
        n:   usize)  {
        
        todo!();
        /*
            if (n == 0) {
                return;
            }

            // Read from the beginning of the buffer
            size_t pos_next = m_pos + n;
            if (pos_next > m_data.size()) {
                throw std::ios_base::failure("VectorReader::read(): end of data");
            }
            memcpy(dst, m_data.data() + m_pos, n);
            m_pos = pos_next;
        */
    }
}
