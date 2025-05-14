// ---------------- [ File: bitcoin-bitstream/src/data_stream.rs ]
crate::ix!();

/**
  | Double ended buffer combining vector
  | and stream-like interfaces. >> and
  | << read and write unformatted data using
  | the above serialization templates.
  | 
  | Fills with data in linear time; some
  | stringstream implementations take
  | N^2 time.
  |
  */
pub struct DataStream {
    vch:        SerializeData,
    n_read_pos: u32, // default = { 0 }
    n_type:     i32,
    n_version:  i32,
}

impl Index<usize> for DataStream {
    type Output = SerializeData;
    
    #[inline] fn index(&self, pos: usize) -> &Self::Output {
        todo!();
        /*
            return vch[pos + nReadPos];
        */
    }
}

impl IndexMut<usize> for DataStream {
    
    #[inline] fn index_mut(&mut self, pos: usize) -> &mut Self::Output {
        todo!();
        /*
            return vch[pos + nReadPos];
        */
    }
}

impl StreamInto for DataStream {
    
    #[inline] fn stream_into<Item>(&self, rhs: &mut Item) {
        todo!();
        /*
            // Serialize to this stream
            ::Serialize(*this, obj);
            return (*this);
        */
    }
}

impl StreamItems for DataStream {
    
    #[inline] fn stream<Item>(&mut self, x: Item) {
        todo!();
        /*
            // Unserialize from this stream
            ::Unserialize(*this, obj);
            return (*this);
        */
    }
}

impl DataStream {
    
    pub fn new(
        n_type_in:    i32,
        n_version_in: i32) -> Self {
    
        todo!();
        /*


            : nType{nTypeIn},
              nVersion{nVersionIn}
        */
    }
    
    pub fn new_with_slice(
        sp:           &[u8],
        n_type_in:    i32,
        n_version_in: i32) -> Self {
    
        todo!();
        /*
            : vch(sp.data(), sp.data() + sp.size()),
              nType{nTypeIn},
              nVersion{nVersionIn}
        */
    }
    
    pub fn new_with_args<Args>(
        n_type_in:    i32,
        n_version_in: i32,
        args:         Args) -> Self {
    
        todo!();
        /*


            : nType{nTypeIn},
              nVersion{nVersionIn}

            ::SerializeMany(*this, std::forward<Args>(args)...);
        */
    }
    
    pub fn str_(&self) -> String {
        
        todo!();
        /*
            return (std::string(begin(), end()));
        */
    }

    /* ----------------- Vector subset  ----------------- */

    pub fn begin(&self) -> Box<dyn Iterator<Item = u8>> {
        
        todo!();
        /*
            return vch.begin() + nReadPos;
        */
    }
    
    pub fn begin_mut(&mut self) -> Box<dyn Iterator<Item = u8>> {
        
        todo!();
        /*
            return vch.begin() + nReadPos;
        */
    }
    
    pub fn end(&self) -> Box<dyn Iterator<Item = u8>> {
        
        todo!();
        /*
            return vch.end();
        */
    }
    
    pub fn end_mut(&mut self) -> Box<dyn Iterator<Item = u8>> {
        
        todo!();
        /*
            return vch.end();
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return vch.size() - nReadPos;
        */
    }
    
    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return vch.size() == nReadPos;
        */
    }
    
    pub fn resize(&mut self, 
        n: usize,
        c: Option<u8>)  {
        let c: u8 = c.unwrap_or(0);

        todo!();
        /*
            vch.resize(n + nReadPos, c);
        */
    }
    
    pub fn reserve(&mut self, n: usize)  {
        
        todo!();
        /*
            vch.reserve(n + nReadPos);
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            vch.clear(); nReadPos = 0;
        */
    }
    
    pub fn insert_item(&mut self, 
        it: Box<dyn Iterator<Item = u8>>,
        x:  u8) -> Box<dyn Iterator<Item = u8>> {
        
        todo!();
        /*
            return vch.insert(it, x);
        */
    }
    
    pub fn insert_multi(&mut self, 
        it: Box<dyn Iterator<Item = u8>>,
        n:  usize,
        x:  u8)  {
        
        todo!();
        /*
            vch.insert(it, n, x);
        */
    }

    pub fn as_slice(&self) -> &[u8] {
        let data: *const u8 = self.data();
        let size:     usize = self.size();

        unsafe {
            std::slice::from_raw_parts(data,size)
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let data: *mut u8 = self.data_mut();
        let size:   usize = self.size();

        unsafe {
            std::slice::from_raw_parts_mut(data,size)
        }
    }
    
    pub fn data_mut(&mut self) -> *mut u8 {
        
        todo!();
        /*
            return vch.data() + nReadPos;
        */
    }
    
    pub fn data(&self) -> *const u8 {
        
        todo!();
        /*
            return vch.data() + nReadPos;
        */
    }
    
    pub fn insert_with_iterator_range(&mut self, 
        it:    Box<dyn Iterator<Item = u8>>,
        first: Box<dyn Iterator<Item = u8>>,
        last:  Box<dyn Iterator<Item = u8>>)  {
        
        todo!();
        /*
            if (last == first) return;
            assert(last - first > 0);
            if (it == vch.begin() + nReadPos && (unsigned int)(last - first) <= nReadPos)
            {
                // special case for inserting at the front when there's room
                nReadPos -= (last - first);
                memcpy(&vch[nReadPos], &first[0], last - first);
            }
            else
                vch.insert(it, first, last);
        */
    }
    
    pub fn insert_with_pointer_range(&mut self, 
        it:    Box<dyn Iterator<Item = u8>>,
        first: *const u8,
        last:  *const u8)  {
        
        todo!();
        /*
            if (last == first) return;
            assert(last - first > 0);
            if (it == vch.begin() + nReadPos && (unsigned int)(last - first) <= nReadPos)
            {
                // special case for inserting at the front when there's room
                nReadPos -= (last - first);
                memcpy(&vch[nReadPos], &first[0], last - first);
            }
            else
                vch.insert(it, first, last);
        */
    }
    
    pub fn erase(&mut self, 
        it: Box<dyn Iterator<Item=u8>>) -> Box<dyn Iterator<Item=u8>> {
        
        todo!();
        /*
            if (it == vch.begin() + nReadPos)
            {
                // special case for erasing from the front
                if (++nReadPos >= vch.size())
                {
                    // whenever we reach the end, we take the opportunity to clear the buffer
                    nReadPos = 0;
                    return vch.erase(vch.begin(), vch.end());
                }
                return vch.begin() + nReadPos;
            }
            else
                return vch.erase(it);
        */
    }
    
    pub fn erase_range(&mut self, 
        first: Box<dyn Iterator<Item=u8>>,
        last:  Box<dyn Iterator<Item=u8>>) -> Box<dyn Iterator<Item=u8>> {
        
        todo!();
        /*
            if (first == vch.begin() + nReadPos)
            {
                // special case for erasing from the front
                if (last == vch.end())
                {
                    nReadPos = 0;
                    return vch.erase(vch.begin(), vch.end());
                }
                else
                {
                    nReadPos = (last - vch.begin());
                    return last;
                }
            }
            else
                return vch.erase(first, last);
        */
    }
    
    #[inline] pub fn compact(&mut self)  {
        
        todo!();
        /*
            vch.erase(vch.begin(), vch.begin() + nReadPos);
            nReadPos = 0;
        */
    }
    
    pub fn rewind(&mut self, n: Option<usize>) -> bool {

        todo!();
        /*
            // Total rewind if no size is passed
            if (!n) {
                nReadPos = 0;
                return true;
            }
            // Rewind by n characters if the buffer hasn't been compacted yet
            if (*n > nReadPos)
                return false;
            nReadPos -= *n;
            return true;
        */
    }

    /* ----------------- Stream subset  ----------------- */

    pub fn eof(&self) -> bool {
        
        todo!();
        /*
            return size() == 0;
        */
    }
    
    pub fn rdbuf(&mut self) -> *mut DataStream {
        
        todo!();
        /*
            return this;
        */
    }
    
    pub fn in_avail(&self) -> i32 {
        
        todo!();
        /*
            return size();
        */
    }
    
    pub fn set_type(&mut self, n: i32)  {
        
        todo!();
        /*
            nType = n;
        */
    }
    
    pub fn get_type(&self) -> i32 {
        
        todo!();
        /*
            return nType;
        */
    }
    
    pub fn set_version(&mut self, n: i32)  {
        
        todo!();
        /*
            nVersion = n;
        */
    }
    
    pub fn get_version(&self) -> i32 {
        
        todo!();
        /*
            return nVersion;
        */
    }
    
    pub fn read(&mut self, 
        pch:    *mut u8,
        n_size: usize)  {
        
        todo!();
        /*
            if (nSize == 0) return;

            // Read from the beginning of the buffer
            unsigned int nReadPosNext = nReadPos + nSize;
            if (nReadPosNext > vch.size()) {
                throw std::ios_base::failure("DataStream::read(): end of data");
            }
            memcpy(pch, &vch[nReadPos], nSize);
            if (nReadPosNext == vch.size())
            {
                nReadPos = 0;
                vch.clear();
                return;
            }
            nReadPos = nReadPosNext;
        */
    }
    
    pub fn ignore(&mut self, n_size: i32)  {
        
        todo!();
        /*
            // Ignore from the beginning of the buffer
            if (nSize < 0) {
                throw std::ios_base::failure("DataStream::ignore(): nSize negative");
            }
            unsigned int nReadPosNext = nReadPos + nSize;
            if (nReadPosNext >= vch.size())
            {
                if (nReadPosNext > vch.size())
                    throw std::ios_base::failure("DataStream::ignore(): end of data");
                nReadPos = 0;
                vch.clear();
                return;
            }
            nReadPos = nReadPosNext;
        */
    }
    
    pub fn write(&mut self, 
        pch:    *const u8,
        n_size: usize)  {
        
        todo!();
        /*
            // Write to the end of the buffer
            vch.insert(vch.end(), pch, pch + nSize);
        */
    }
    
    pub fn serialize<Stream>(&self, s: &mut Stream) {
    
        todo!();
        /*
            // Special case: stream << stream concatenates like stream += stream
            if (!vch.empty())
                s.write((char*)vch.data(), vch.size() * sizeof(value_type));
        */
    }

    /**
      | XOR the contents of this stream with
      | a certain key.
      | 
      | -----------
      | @param[in] key
      | 
      | The key used to XOR the data in this stream.
      |
      */
    pub fn xor(&mut self, key: &Vec<u8>)  {
        
        todo!();
        /*
            if (key.size() == 0) {
                return;
            }

            for (usize i = 0, j = 0; i != size(); i++) {
                vch[i] ^= key[j++];

                // This potentially acts on very many bytes of data, so it's
                // important that we calculate `j`, i.e. the `key` index in this
                // way instead of doing a %, which would effectively be a division
                // for each byte Xor'd -- much slower than need be.
                if (j == key.size())
                    j = 0;
            }
        */
    }
}
