// ---------------- [ File: bitcoin-autofile/src/autofile.rs ]
crate::ix!();

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
