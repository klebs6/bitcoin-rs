crate::ix!();

pub struct BitStreamWriter<OStream> {

    ostream: Rc<RefCell<OStream>>,

    /**
      | Buffered byte waiting to be written
      | to the output stream. The byte is written
      | buffer when m_offset reaches 8 or Flush()
      | is called.
      |
      */
    buffer:  u8, // default = { 0 }

    /**
      | Number of high order bits in m_buffer
      | already written by previous
      | 
      | Write() calls and not yet flushed to
      | the stream. The next bit to be written
      | to is at this offset from the most significant
      | bit position.
      |
      */
    offset:  i32, // default = { 0 }
}

impl<OStream> Drop for BitStreamWriter<OStream> {
    fn drop(&mut self) {
        todo!();
        /*
            Flush();
        */
    }
}

impl<OStream> BitStreamWriter<OStream> {
    
    pub fn new(ostream: &mut OStream) -> Self {
    
        todo!();
        /*
        : ostream(ostream),

        
        */
    }

    /**
      | Write the nbits least significant bits
      | of a 64-bit int to the output stream.
      | Data is buffered until it completes
      | an octet.
      |
      */
    pub fn write(&mut self, 
        data:  u64,
        nbits: i32)  {
        
        todo!();
        /*
            if (nbits < 0 || nbits > 64) {
                throw std::out_of_range("nbits must be between 0 and 64");
            }

            while (nbits > 0) {
                int bits = std::min(8 - m_offset, nbits);
                m_buffer |= (data << (64 - nbits)) >> (64 - 8 + m_offset);
                m_offset += bits;
                nbits -= bits;

                if (m_offset == 8) {
                    Flush();
                }
            }
        */
    }

    /**
      | Flush any unwritten bits to the output
      | stream, padding with 0's to the next
      | byte boundary.
      |
      */
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            if (m_offset == 0) {
                return;
            }

            m_ostream << m_buffer;
            m_buffer = 0;
            m_offset = 0;
        */
    }
}
