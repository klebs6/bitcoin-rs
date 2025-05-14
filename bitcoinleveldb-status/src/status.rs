// ---------------- [ File: bitcoinleveldb-status/src/status.rs ]
/*!
  | A Status encapsulates the result of an
  | operation.  It may indicate success, or it may
  | indicate an error with an associated error
  | message.
  |
  | Multiple threads can invoke const methods on
  | a Status without external synchronization, but
  | if any of the threads may call a non-const
  | method, all threads accessing the same Status
  | must use external synchronization.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/status.cc]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/status.h]

pub struct Status {

    /**
      | OK status has a null state_.  Otherwise,
      | state_ is a new[] array of the following
      | form:
      |
      |    state_[0..3] == length of message
      |    state_[4]    == code
      |    state_[5..]  == message
      */
    state: *const u8,
}

pub enum StatusCode {
    Ok              = 0,
    NotFound        = 1,
    Corruption      = 2,
    NotSupported    = 3,
    InvalidArgument = 4,
    IOError         = 5
}

impl Default for Status {
    
    /**
      | Create a success status.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        : state(nullptr),
        */
    }
}

impl Drop for Status {

    fn drop(&mut self) {
        todo!();
        /*
            delete[] state_;
        */
    }
}
    
impl Status {

    pub fn new_from_other(rhs: Status) -> Self {
    
        todo!();
        /*
        : state(rhs.state_),

            rhs.state_ = nullptr;
        */
    }

    pub fn new_from_other_copy(rhs: &Status) -> Self {
    
        todo!();
        /*
            state_ = (rhs.state_ == nullptr)
            ? nullptr : CopyState(rhs.state_);
        */
    }
    
    /**
      | Return a success status.
      |
      */
    pub fn ok() -> crate::Status {
        
        todo!();
        /*
            return Status();
        */
    }

    /**
      | Return error status of an appropriate
      | type.
      |
      */
    pub fn not_found(
        msg:  &Slice,
        msg2: Option<&Slice>) -> crate::Status {

        let df           = Slice::default();
        let msg2: &Slice = msg2.unwrap_or(&df);

        todo!();
        /*
            return Status(kNotFound, msg, msg2);
        */
    }
    
    pub fn corruption(
        msg:  &Slice,
        msg2: Option<&Slice>) -> crate::Status {

        let df           = Slice::default();
        let msg2: &Slice = msg2.unwrap_or(&df);

        todo!();
        /*
            return Status(kCorruption, msg, msg2);
        */
    }
    
    pub fn not_supported(
        msg:  &Slice,
        msg2: Option<&Slice>) -> crate::Status {

        let df = Slice::default();
        let msg2: &Slice = msg2.unwrap_or(&df);

        todo!();
        /*
            return Status(kNotSupported, msg, msg2);
        */
    }
    
    pub fn invalid_argument(
        msg:  &Slice,
        msg2: Option<&Slice>) -> crate::Status {

        let df           = Slice::default();
        let msg2: &Slice = msg2.unwrap_or(&df);

        todo!();
        /*
            return Status(kInvalidArgument, msg, msg2);
        */
    }
    
    pub fn io_error(
        msg:  &Slice,
        msg2: Option<&Slice>) -> crate::Status {

        let df           = Slice::default();
        let msg2: &Slice = msg2.unwrap_or(&df);

        todo!();
        /*
            return Status(kIOError, msg, msg2);
        */
    }

    /**
      | Returns true iff the status indicates
      | success.
      |
      */
    pub fn is_ok(&self) -> bool {
        
        todo!();
        /*
            return (state_ == nullptr);
        */
    }

    /**
      | Returns true iff the status indicates
      | a NotFound error.
      |
      */
    pub fn is_not_found(&self) -> bool {
        
        todo!();
        /*
            return code() == kNotFound;
        */
    }

    /**
      | Returns true iff the status indicates
      | a Corruption error.
      |
      */
    pub fn is_corruption(&self) -> bool {
        
        todo!();
        /*
            return code() == kCorruption;
        */
    }

    /**
      | Returns true iff the status indicates
      | an
      | 
      | IOError.
      |
      */
    pub fn is_io_error(&self) -> bool {
        
        todo!();
        /*
            return code() == kIOError;
        */
    }

    /**
      | Returns true iff the status indicates
      | a NotSupportedError.
      |
      */
    pub fn is_not_supported_error(&self) -> bool {
        
        todo!();
        /*
            return code() == kNotSupported;
        */
    }

    /**
      | Returns true iff the status indicates
      | an
      | 
      | InvalidArgument.
      |
      */
    pub fn is_invalid_argument(&self) -> bool {
        
        todo!();
        /*
            return code() == kInvalidArgument;
        */
    }

    pub fn code(&self) -> StatusCode {
        
        todo!();
        /*
            return (state_ == nullptr) ? kOk : static_cast<StatusCode>(state_[4]);
        */
    }
    
    #[inline] pub fn assign_from_other_copy(&mut self, rhs: &Status) -> &mut Status {
        
        todo!();
        /*
            // The following condition catches both aliasing (when this == &rhs),
      // and the common case where both rhs and *this are ok.
      if (state_ != rhs.state_) {
        delete[] state_;
        state_ = (rhs.state_ == nullptr) ? nullptr : CopyState(rhs.state_);
      }
      return *this;
        */
    }
    
    #[inline] pub fn assign_from_other_move(&mut self, rhs: Status) -> &mut Status {
        
        todo!();
        /*
            std::swap(state_, rhs.state_);
      return *this;
        */
    }
    
    pub fn copy_state(&mut self, state: *const u8) -> *const u8 {
        
        todo!();
        /*
            uint32_t size;
      memcpy(&size, state, sizeof(size));
      char* result = new char[size + 5];
      memcpy(result, state, size + 5);
      return result;
        */
    }
    
    pub fn new(
        code: StatusCode,
        msg:  &Slice,
        msg2: &Slice) -> Self {
    
        todo!();
        /*


            assert(code != kOk);
      const uint32_t len1 = static_cast<uint32_t>(msg.size());
      const uint32_t len2 = static_cast<uint32_t>(msg2.size());
      const uint32_t size = len1 + (len2 ? (2 + len2) : 0);
      char* result = new char[size + 5];
      memcpy(result, &size, sizeof(size));
      result[4] = static_cast<char>(code);
      memcpy(result + 5, msg.data(), len1);
      if (len2) {
        result[5 + len1] = ':';
        result[6 + len1] = ' ';
        memcpy(result + 7 + len1, msg2.data(), len2);
      }
      state_ = result;
        */
    }
    
    /**
      | Return a string representation of this status
      | suitable for printing.
      |
      | Returns the string "OK" for success.
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            if (state_ == nullptr) {
        return "OK";
      } else {
        char tmp[30];
        const char* type;
        switch (code()) {
          case kOk:
            type = "OK";
            break;
          case kNotFound:
            type = "NotFound: ";
            break;
          case kCorruption:
            type = "Corruption: ";
            break;
          case kNotSupported:
            type = "Not implemented: ";
            break;
          case kInvalidArgument:
            type = "Invalid argument: ";
            break;
          case kIOError:
            type = "IO error: ";
            break;
          default:
            snprintf(tmp, sizeof(tmp),
                     "Unknown code(%d): ", static_cast<int>(code()));
            type = tmp;
            break;
        }
        std::string result(type);
        uint32_t length;
        memcpy(&length, state_, sizeof(length));
        result.append(state_ + 5, length);
        return result;
      }
        */
    }
}
