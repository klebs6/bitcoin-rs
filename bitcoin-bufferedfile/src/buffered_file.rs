crate::ix!();

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
