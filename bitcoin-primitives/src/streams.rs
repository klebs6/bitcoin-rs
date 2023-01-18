crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/streams.h]

/**
  | Dummy data type to identify deserializing
  | constructors.
  | 
  | By convention, a constructor of a type
  | T with signature
  | 
  | template <typename Stream> T::T(deserialize_type,
  | Stream& s)
  | 
  | is a deserializing constructor, which
  | builds the type by deserializing it
  | from s. If T contains const fields, this
  | is likely the only way to do so.
  |
  */
pub struct DeserializeType {}

lazy_static!{
    /*
    constexpr deserialize_type deserialize {}
    */
}

pub struct OverrideStream<Stream> {
    stream:    *mut Stream,
    n_type:    i32,
    n_version: i32,
}

impl<Stream> StreamInto for OverrideStream<Stream> {
    
    #[inline] fn stream_into<Item>(&self, rhs: &mut Item) {
        todo!();
        /*
            // Serialize to this stream
            ::Serialize(*this, obj);
            return (*this);
        */
    }
}

impl<Stream> StreamItems for OverrideStream<Stream> {
    
    #[inline] fn stream<Item>(&mut self, x: Item) {
        todo!();
        /*
            // Unserialize from this stream
            ::Unserialize(*this, obj);
            return (*this);
        */
    }
}

//------------------------------
pub trait GetType {
    fn get_type(&self) -> i32;
}

pub trait GetVersion {
    fn get_version(&self) -> i32;
}

impl<Stream> GetType for OverrideStream<Stream> {

    fn get_type(&self) -> i32 {
        
        todo!();
        /*
            return nType;
        */
    }
}

impl<Stream> GetVersion for OverrideStream<Stream> {

    fn get_version(&self) -> i32 {
        
        todo!();
        /*
            return nVersion;
        */
    }
}

impl<Stream> OverrideStream<Stream> {
    
    pub fn new(
        stream:    *mut Stream,
        n_type:    i32,
        n_version: i32) -> Self {
    
        todo!();
        /*
        : stream(stream_),
        : n_type(nType_),
        : n_version(nVersion_),

        
        */
    }
    
    pub fn write(&mut self, 
        pch:    *const u8,
        n_size: usize)  {
        
        todo!();
        /*
            stream->write(pch, nSize);
        */
    }
    
    pub fn read(&mut self, 
        pch:    *mut u8,
        n_size: usize)  {
        
        todo!();
        /*
            stream->read(pch, nSize);
        */
    }
    
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return stream->size();
        */
    }
    
    pub fn ignore(&mut self, size: usize)  {
        
        todo!();
        /*
            return stream->ignore(size);
        */
    }
}

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

///-----------------------
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

///--------------------
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

/**
  | Non-refcounted RAII wrapper for FILE*
  | 
  | Will automatically close the file when
  | it goes out of scope if not null.
  | 
  | If you're returning the file pointer,
  | return file.release().
  | 
  | If you need to close the file early, use
  | file.fclose() instead of fclose(file).
  |
  */
#[no_copy]
pub struct AutoFile {
    n_type:    i32,
    n_version: i32,
    file:      *mut libc::FILE,
}

impl Drop for AutoFile {

    fn drop(&mut self) {
        todo!();
        /*
            fclose();
        */
    }
}

impl<T> Shl<&T> for AutoFile {

    type Output = AutoFile;
    
    #[inline] fn shl(self, rhs: &T) -> Self::Output {
        todo!();
        /*
            // Serialize to this stream
            if (!file)
                throw std::ios_base::failure("AutoFile::operator<<: file handle is nullptr");
            ::Serialize(*this, obj);
            return (*this);
        */
    }
}

impl<T> Shr<T> for AutoFile {

    type Output = AutoFile;

    #[inline] fn shr(self, rhs: T) -> Self::Output {
        todo!();
        /*
            // Unserialize from this stream
            if (!file)
                throw std::ios_base::failure("AutoFile::operator>>: file handle is nullptr");
            ::Unserialize(*this, obj);
            return (*this);
        */
    }
}

impl AutoFile {
    
    pub fn new(
        filenew:      *mut libc::FILE,
        n_type_in:    i32,
        n_version_in: i32) -> Self {
    
        todo!();
        /*
        : n_type(nTypeIn),
        : n_version(nVersionIn),

            file = filenew;
        */
    }
    
    pub fn fclose(&mut self)  {
        
        todo!();
        /*
            if (file) {
                ::fclose(file);
                file = nullptr;
            }
        */
    }

    /**
      | Get wrapped FILE* with transfer of ownership.
      | 
      | -----------
      | @note
      | 
      | This will invalidate the AutoFile
      | object, and makes it the responsibility
      | of the caller of this function to clean
      | up the returned FILE*.
      |
      */
    pub fn release(&mut self) -> *mut libc::FILE {
        
        todo!();
        /*
            FILE* ret = file; file = nullptr; return ret;
        */
    }

    /**
      | Get wrapped FILE* without transfer
      | of ownership.
      | 
      | -----------
      | @note
      | 
      | Ownership of the FILE* will remain with
      | this class. Use this only if the scope
      | of the
      | 
      | AutoFile outlives use of the passed
      | pointer.
      |
      */
    pub fn get(&self) -> *mut libc::FILE {
        
        todo!();
        /*
            return file;
        */
    }

    /**
      | Return true if the wrapped FILE* is nullptr,
      | false otherwise.
      |
      */
    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            return (file == nullptr);
        */
    }

    /* ----------------- Stream subset  ----------------- */

    
    pub fn get_type(&self) -> i32 {
        
        todo!();
        /*
            return nType;
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
            if (!file)
                throw std::ios_base::failure("AutoFile::read: file handle is nullptr");
            if (fread(pch, 1, nSize, file) != nSize)
                throw std::ios_base::failure(feof(file) ? "AutoFile::read: end of file" : "AutoFile::read: fread failed");
        */
    }
    
    pub fn ignore(&mut self, n_size: usize)  {
        
        todo!();
        /*
            if (!file)
                throw std::ios_base::failure("AutoFile::ignore: file handle is nullptr");
            unsigned char data[4096];
            while (nSize > 0) {
                size_t nNow = std::min<size_t>(nSize, sizeof(data));
                if (fread(data, 1, nNow, file) != nNow)
                    throw std::ios_base::failure(feof(file) ? "AutoFile::ignore: end of file" : "AutoFile::read: fread failed");
                nSize -= nNow;
            }
        */
    }
    
    pub fn write(&mut self, 
        pch:    *const u8,
        n_size: usize)  {
        
        todo!();
        /*
            if (!file)
                throw std::ios_base::failure("AutoFile::write: file handle is nullptr");
            if (fwrite(pch, 1, nSize, file) != nSize)
                throw std::ios_base::failure("AutoFile::write: write failed");
        */
    }
}

/**
  | Non-refcounted RAII wrapper around
  | a FILE* that implements a ring buffer
  | to deserialize from. It guarantees
  | the ability to rewind a given number
  | of bytes.
  | 
  | Will automatically close the file when
  | it goes out of scope if not null.
  | 
  | If you need to close the file early, use
  | file.fclose() instead of fclose(file).
  |
  */
#[no_copy]
pub struct BufferedFile {
    n_type:       i32,
    n_version:    i32,

    /**
      | source file
      |
      */
    src:          *mut libc::FILE,

    /**
      | how many bytes have been read from source
      |
      */
    n_src_pos:    u64,

    /**
      | how many bytes have been read from this
      |
      */
    n_read_pos:   u64,

    /**
      | up to which position we're allowed to
      | read
      |
      */
    n_read_limit: u64,

    /**
      | how many bytes we guarantee to rewind
      |
      */
    n_rewind:     u64,

    /**
      | the buffer
      |
      */
    vch_buf:      Vec<u8>,
}

impl Drop for BufferedFile {
    fn drop(&mut self) {
        todo!();
        /*
            fclose();
        */
    }
}

impl<T> Shr<T> for BufferedFile {
    type Output = BufferedFile;

    #[inline] fn shr(self, rhs: T) -> Self::Output {
        todo!();
        /*
            // Unserialize from this stream
            ::Unserialize(*this, obj);
            return (*this);
        */
    }
}

impl BufferedFile {

    /**
      | read data from the source to fill the
      | buffer
      |
      */
    pub fn fill(&mut self) -> bool {
        
        todo!();
        /*
            unsigned int pos = nSrcPos % vchBuf.size();
            unsigned int readNow = vchBuf.size() - pos;
            unsigned int nAvail = vchBuf.size() - (nSrcPos - nReadPos) - nRewind;
            if (nAvail < readNow)
                readNow = nAvail;
            if (readNow == 0)
                return false;
            size_t nBytes = fread((c_void*)&vchBuf[pos], 1, readNow, src);
            if (nBytes == 0) {
                throw std::ios_base::failure(feof(src) ? "BufferedFile::Fill: end of file" : "BufferedFile::Fill: fread failed");
            }
            nSrcPos += nBytes;
            return true;
        */
    }
    
    pub fn new(
        file_in:      *mut libc::FILE,
        n_buf_size:   u64,
        n_rewind_in:  u64,
        n_type_in:    i32,
        n_version_in: i32) -> Self {
    
        todo!();
        /*


            : nType(nTypeIn), nVersion(nVersionIn), nSrcPos(0), nReadPos(0), nReadLimit(std::numeric_limits<uint64_t>::max()), nRewind(nRewindIn), vchBuf(nBufSize, 0)

            if (nRewindIn >= nBufSize)
                throw std::ios_base::failure("Rewind limit must be less than buffer size");
            src = fileIn;
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
    
    pub fn fclose(&mut self)  {
        
        todo!();
        /*
            if (src) {
                ::fclose(src);
                src = nullptr;
            }
        */
    }

    /**
      | check whether we're at the end of the
      | source file
      |
      */
    pub fn eof(&self) -> bool {
        
        todo!();
        /*
            return nReadPos == nSrcPos && feof(src);
        */
    }

    /**
      | read a number of bytes
      |
      */
    pub fn read(&mut self, 
        pch:    *mut u8,
        n_size: usize)  {
        
        todo!();
        /*
            if (nSize + nReadPos > nReadLimit)
                throw std::ios_base::failure("Read attempted past buffer limit");
            while (nSize > 0) {
                if (nReadPos == nSrcPos)
                    Fill();
                unsigned int pos = nReadPos % vchBuf.size();
                size_t nNow = nSize;
                if (nNow + pos > vchBuf.size())
                    nNow = vchBuf.size() - pos;
                if (nNow + nReadPos > nSrcPos)
                    nNow = nSrcPos - nReadPos;
                memcpy(pch, &vchBuf[pos], nNow);
                nReadPos += nNow;
                pch += nNow;
                nSize -= nNow;
            }
        */
    }

    /**
      | return the current reading position
      |
      */
    pub fn get_pos(&self) -> u64 {
        
        todo!();
        /*
            return nReadPos;
        */
    }

    /**
      | rewind to a given reading position
      |
      */
    pub fn set_pos(&mut self, n_pos: u64) -> bool {
        
        todo!();
        /*
            size_t bufsize = vchBuf.size();
            if (nPos + bufsize < nSrcPos) {
                // rewinding too far, rewind as far as possible
                nReadPos = nSrcPos - bufsize;
                return false;
            }
            if (nPos > nSrcPos) {
                // can't go this far forward, go as far as possible
                nReadPos = nSrcPos;
                return false;
            }
            nReadPos = nPos;
            return true;
        */
    }

    /**
      | prevent reading beyond a certain position
      | no argument removes the limit
      |
      */
    pub fn set_limit(&mut self, n_pos: Option<u64>) -> bool {
        let n_pos = n_pos.unwrap_or(u64::MAX);
        
        todo!();
        /*
            if (nPos < nReadPos)
                return false;
            nReadLimit = nPos;
            return true;
        */
    }

    /**
      | search for a given byte in the stream,
      | and remain positioned on it
      |
      */
    pub fn find_byte(&mut self, ch: u8)  {
        
        todo!();
        /*
            while (true) {
                if (nReadPos == nSrcPos)
                    Fill();
                if (vchBuf[nReadPos % vchBuf.size()] == ch)
                    break;
                nReadPos++;
            }
        */
    }
}

pub trait StreamItems {
    fn stream<Item>(&mut self, x: Item);
}

pub trait StreamInto {
    fn stream_into<Item>(&self, x: &mut Item);
}
