crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/slice.h]

/**
  | Slice is a simple structure containing
  | a pointer into some external storage and
  | a size.  The user of a Slice must ensure that
  | the slice is not used after the corresponding
  | external storage has been deallocated.
  |
  | Multiple threads can invoke const methods on
  | a Slice without external synchronization, but
  | if any of the threads may call a non-const
  | method, all threads accessing the same Slice
  | must use external synchronization.
  */
pub struct Slice {
    data: *const u8,
    size: usize,
}

impl Default for Slice {
    
    /**
      | Create an empty slice.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        : data(""),
        : size(0),

        
        */
    }
}

impl Index<usize> for Slice {
    type Output = u8;
    
    /**
      | Return the ith byte in the referenced data.
      |
      | REQUIRES: n < size()
      */
    #[inline] fn index(&self, n: usize) -> &Self::Output {
        todo!();
        /*
            assert(n < size());
        return data_[n];
        */
    }
}

impl PartialEq<Slice> for Slice {
    
    fn eq(&self, other: &Slice) -> bool {
        todo!();
        /*
            return ((x.size() == y.size()) &&
              (memcmp(x.data(), y.data(), x.size()) == 0));
        */
    }
}

impl Eq for Slice {}

impl From<&String> for Slice {

    /**
      | Create a slice that refers to the contents
      | of "s"
      |
      */
    fn from(s: &String) -> Self {
    
        todo!();
        /*
        : data(s.data()),
        : size(s.size()),

        
        */
    }
}

impl From<*const u8> for Slice {

    /**
      | Create a slice that refers to s[0,strlen(s)-1]
      |
      */
    fn from(s: *const u8) -> Self {
    
        todo!();
        /*
        : data(s),
        : size(strlen(s)),

        
        */
    }
}

impl Slice {

    /**
      | Create a slice that refers to d[0,n-1].
      |
      */
    pub fn from_ptr_len(
        d: *const u8,
        n: usize) -> Self {
    
        todo!();
        /*
        : data(d),
        : size(n),

        
        */
    }

    /**
      | Return a pointer to the beginning of
      | the referenced data
      |
      */
    pub fn data(&self) -> *const u8 {
        
        todo!();
        /*
            return data_;
        */
    }

    /**
      | Return the length (in bytes) of the referenced
      | data
      |
      */
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return size_;
        */
    }

    /**
      | Return true iff the length of the referenced
      | data is zero
      |
      */
    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return size_ == 0;
        */
    }

    /**
      | Change this slice to refer to an empty
      | array
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            data_ = "";
        size_ = 0;
        */
    }

    /**
      | Drop the first "n" bytes from this slice.
      |
      */
    pub fn remove_prefix(&mut self, n: usize)  {
        
        todo!();
        /*
            assert(n <= size());
        data_ += n;
        size_ -= n;
        */
    }

    /**
      | Return a string that contains the copy
      | of the referenced data.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return std::string(data_, size_);
        */
    }

    /**
      | Return true iff "x" is a prefix of "*this"
      |
      */
    pub fn starts_with(&self, x: &Slice) -> bool {
        
        todo!();
        /*
            return ((size_ >= x.size_) && (memcmp(data_, x.data_, x.size_) == 0));
        */
    }
    
    /**
      | Three-way comparison.  Returns value:
      |   <  0 iff "*this" <  "b",
      |   == 0 iff "*this" == "b",
      |   >  0 iff "*this" >  "b"
      */
    #[inline] pub fn compare(&self, b: &Slice) -> i32 {
        
        todo!();
        /*
            const size_t min_len = (size_ < b.size_) ? size_ : b.size_;
      int r = memcmp(data_, b.data_, min_len);
      if (r == 0) {
        if (size_ < b.size_)
          r = -1;
        else if (size_ > b.size_)
          r = +1;
      }
      return r;
        */
    }
}
