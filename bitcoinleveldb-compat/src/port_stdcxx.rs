crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/port/port_stdcxx.h]

pub mod port {

    use super::*;

    /**
       | The following boolean constant must be true on
       | a little-endian machine and false otherwise.
       |
       | or some other expression
       |
      */
    pub const LITTLE_ENDIAN: bool = !crate::port::LEVELDB_IS_BIG_ENDIAN;

    /**
       Thinly wraps std::mutex.
      */
    #[LOCKABLE]
    pub struct Mutex {
        mu: parking_lot::RawMutex,
    }

    impl Mutex {

        #[EXCLUSIVE_LOCK_FUNCTION()]
        pub fn lock(&mut self)  {
            
            todo!();
            /*
                mu_.lock();
            */
        }

        #[UNLOCK_FUNCTION()]
        pub fn unlock(&mut self)  {
            
            todo!();
            /*
                mu_.unlock();
            */
        }

        #[ASSERT_EXCLUSIVE_LOCK()]
        pub fn assert_held(&mut self)  {
            
            todo!();
            /*
            
            */
        }
    }
      
    /**
      | Thinly wraps std::condition_variable.
      |
      */
    pub struct CondVar {
        cv: std::sync::Condvar,
        mu: *const Mutex,
    }

    impl CondVar {

        pub fn new(mu: *mut Mutex) -> Self {
        
            todo!();
            /*
            : mu(mu),

                assert(mu != nullptr);
            */
        }
        
        pub fn wait(&mut self)  {
            
            todo!();
            /*
                std::unique_lock<std::mutex> lock(mu_->mu_, std::adopt_lock);
            cv_.wait(lock);
            lock.release();
            */
        }
        
        pub fn signal(&mut self)  {
            
            todo!();
            /*
                cv_.notify_one();
            */
        }
        
        pub fn signal_all(&mut self)  {
            
            todo!();
            /*
                cv_.notify_all();
            */
        }
    }

    /**
      | Store the snappy compression of
      | "input[0,input_length-1]" in *output.
      |
      | Returns false if snappy is not supported by
      | this port.
      */
    #[inline] pub fn snappy_compress(
            input:  *const u8,
            length: usize,
            output: *mut String) -> bool {
        
        todo!();
            /*
                #if HAVE_SNAPPY
          output->resize(snappy::MaxCompressedLength(length));
          size_t outlen;
          snappy::RawCompress(input, length, &(*output)[0], &outlen);
          output->resize(outlen);
          return true;
        #else
          // Silence compiler warnings about unused arguments.
          (c_void)input;
          (c_void)length;
          (c_void)output;
        #endif  // HAVE_SNAPPY

          return false;
            */
    }

    /**
      | If input[0,input_length-1] looks like a valid
      | snappy compressed buffer, store the size of the
      | uncompressed data in *result and return true.
      | Else return false.
      */
    #[inline] pub fn snappy_get_uncompressed_length(
            input:  *const u8,
            length: usize,
            result: *mut usize) -> bool {
        
        todo!();
            /*
                #if HAVE_SNAPPY
          return snappy::GetUncompressedLength(input, length, result);
        #else
          // Silence compiler warnings about unused arguments.
          (c_void)input;
          (c_void)length;
          (c_void)result;
          return false;
        #endif  // HAVE_SNAPPY
            */
    }

    /**
      | Attempt to snappy uncompress
      | input[0,input_length-1] into *output.
      |
      | Returns true if successful, false if the input
      | is invalid lightweight compressed data.
      |
      | REQUIRES: at least the first "n" bytes of
      | output[] must be writable where "n" is the
      | result of a successful call to
      | Snappy_GetUncompressedLength.
      */
    #[inline] pub fn snappy_uncompress(
            input:  *const u8,
            length: usize,
            output: *mut u8) -> bool {
        
        todo!();
            /*
                #if HAVE_SNAPPY
          return snappy::RawUncompress(input, length, output);
        #else
          // Silence compiler warnings about unused arguments.
          (c_void)input;
          (c_void)length;
          (c_void)output;
          return false;
        #endif  // HAVE_SNAPPY
            */
    }

    /**
      | If heap profiling is not supported, returns
      | false.
      |
      | Else repeatedly calls (*func)(arg, data, n) and
      | then returns true.
      |
      | The concatenation of all "data[0,n-1]"
      | fragments is the heap profile.
      */
    #[inline] pub fn get_heap_profile(
            func: fn(
                    _0: *mut c_void,
                    _1: *const u8,
                    _2: i32
            ) -> c_void,
            arg:  *mut c_void) -> bool {
        
        todo!();
            /*
                // Silence compiler warnings about unused arguments.
          (c_void)func;
          (c_void)arg;
          return false;
            */
    }

    /**
      | Extend the CRC to include the first n bytes of
      | buf.
      |
      | Returns zero if the CRC cannot be extended
      | using acceleration, else returns the newly
      | extended CRC value (which may also be zero).
      */
    #[inline] pub fn acceleratedcrc32c(
            crc:  u32,
            buf:  *const u8,
            size: usize) -> u32 {
        
        todo!();
            /*
                #if HAVE_CRC32C
          return ::crc32c::Extend(crc, reinterpret_cast<const uint8_t*>(buf), size);
        #else
          // Silence compiler warnings about unused arguments.
          (c_void)crc;
          (c_void)buf;
          (c_void)size;
          return 0;
        #endif  // HAVE_CRC32C
            */
    }

    //-------------------------------------------[.cpp/bitcoin/src/leveldb/port/port_example.h]

    /*
       | This file contains the specification, but not
       | the implementations, of the
       | types/operations/etc. that should be defined by
       | a platform specific port_<platform>.h file.
       | Use this file as a reference for how to port
       | this package to a new platform.
       |
       | TODO(jorlow): Many of these belong more in the
       |               environment class rather than
       |               here. We should try moving them
       |               and see if it affects perf.
       */

    // ------------------ Miscellaneous -------------------
}
