// ---------------- [ File: bitcoin-bitstream/src/bitstream_reader.rs ]
crate::ix!();

pub struct BitStreamReader<IStream> {

    istream: Rc<RefCell<IStream>>,

    /**
      | Buffered byte read in from the input
      | stream. A new byte is read into the buffer
      | when m_offset reaches 8.
      |
      */
    buffer:  u8, // default = { 0 }


    /**
      | Number of high order bits in m_buffer
      | already returned by previous
      | 
      | Read() calls. The next bit to be returned
      | is at this offset from the most significant
      | bit position.
      |
      */
    offset:  i32, // default = { 8 }
}

impl<IStream> BitStreamReader<IStream> {
    
    pub fn new(istream: &mut IStream) -> Self {
    
        todo!();
        /*
        : istream(istream),

        
        */
    }

    /**
      | Read the specified number of bits from
      | the stream. The data is returned in the
      | nbits least significant bits of a 64-bit
      | uint.
      |
      */
    pub fn read(&mut self, nbits: i32) -> u64 {
        
        todo!();
        /*
            if (nbits < 0 || nbits > 64) {
                throw std::out_of_range("nbits must be between 0 and 64");
            }

            uint64_t data = 0;
            while (nbits > 0) {
                if (m_offset == 8) {
                    m_istream >> m_buffer;
                    m_offset = 0;
                }

                int bits = std::min(8 - m_offset, nbits);
                data <<= bits;
                data |= static_cast<uint8_t>(m_buffer << m_offset) >> (8 - bits);
                m_offset += bits;
                nbits -= bits;
            }
            return data;
        */
    }
}
