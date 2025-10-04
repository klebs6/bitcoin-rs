// ---------------- [ File: bitcoin-sha512/src/sha512.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha512.h]
//-------------------------------------------[.cpp/bitcoin/src/crypto/sha512.cpp]

/**
  | A hasher class for SHA-512.
  |
  */
#[derive(Clone)]
pub struct Sha512 {
    s:     [u64; 8],
    buf:   [u8; 128],
    bytes: u64,
}

impl Default for Sha512 {

    fn default() -> Self {
        Self {
            s:     [0; 8],
            buf:   [0; 128],
            bytes: 0,
        }
    }
}

pub const SHA512_OUTPUT_SIZE: usize = 64;

/**
  | Initialize SHA-256 state.
  |
  */
#[inline] pub fn sha512_initialize(s: *mut u64)  {
    
    todo!();
        /*
            s[0] = 0x6a09e667f3bcc908ull;
        s[1] = 0xbb67ae8584caa73bull;
        s[2] = 0x3c6ef372fe94f82bull;
        s[3] = 0xa54ff53a5f1d36f1ull;
        s[4] = 0x510e527fade682d1ull;
        s[5] = 0x9b05688c2b3e6c1full;
        s[6] = 0x1f83d9abfb41bd6bull;
        s[7] = 0x5be0cd19137e2179ull;
        */
}

impl<T> ShlAssign<T> for Sha512 {
    fn shl_assign(&mut self, mut rhs: T) {
        self.feed_data_in(&mut rhs);
    }
}

impl Sha512 {

    pub fn size(&self) -> u64 {
        
        todo!();
        /*
            return bytes;
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*
        : bytes(0),

            sha512::Initialize(s);
        */
    }
    
    pub fn write(&mut self, 
        data: *const u8,
        len:  usize) -> &mut Sha512 {
        
        todo!();
        /*
            const unsigned char* end = data + len;
        size_t bufsize = bytes % 128;
        if (bufsize && bufsize + len >= 128) {
            // Fill the buffer, and process it.
            memcpy(buf + bufsize, data, 128 - bufsize);
            bytes += 128 - bufsize;
            data += 128 - bufsize;
            sha512::Transform(s, buf);
            bufsize = 0;
        }
        while (end - data >= 128) {
            // Process full chunks directly from the source.
            sha512::Transform(s, data);
            data += 128;
            bytes += 128;
        }
        if (end > data) {
            // Fill the buffer with what remains.
            memcpy(buf + bufsize, data, end - data);
            bytes += end - data;
        }
        return *this;
        */
    }
    
    pub fn finalize(&mut self, hash: [u8; SHA512_OUTPUT_SIZE])  {
        
        todo!();
        /*
            static const unsigned char pad[128] = {0x80};
        unsigned char sizedesc[16] = {0x00};
        WriteBE64(sizedesc + 8, bytes << 3);
        Write(pad, 1 + ((239 - (bytes % 128)) % 128));
        Write(sizedesc, 16);
        WriteBE64(hash, s[0]);
        WriteBE64(hash + 8, s[1]);
        WriteBE64(hash + 16, s[2]);
        WriteBE64(hash + 24, s[3]);
        WriteBE64(hash + 32, s[4]);
        WriteBE64(hash + 40, s[5]);
        WriteBE64(hash + 48, s[6]);
        WriteBE64(hash + 56, s[7]);
        */
    }
    
    pub fn reset(&mut self) -> &mut Sha512 {
        
        todo!();
        /*
            bytes = 0;
        sha512::Initialize(s);
        return *this;
        */
    }
    
    /**
      | Helper to easily feed data into a Sha512.
      | 
      | -----------
      | @note
      | 
      | this does not serialize the passed object
      | (like stream.h's << operators do).
      | 
      | Its raw memory representation is used
      | directly.
      |
      */
    #[inline] pub fn feed_data_in<T>(&mut self, rhs: &mut T) {
        todo!();
        /*
            const_assert(!std::is_same<typename std::decay<T>::type, char*>::value, "Calling operator<<(Sha512, char*) is probably not what you want");
        const_assert(!std::is_same<typename std::decay<T>::type, unsigned char*>::value, "Calling operator<<(Sha512, unsigned char*) is probably not what you want");
        const_assert(!std::is_same<typename std::decay<T>::type, const char*>::value, "Calling operator<<(Sha512, const char*) is probably not what you want");
        const_assert(!std::is_same<typename std::decay<T>::type, const unsigned char*>::value, "Calling operator<<(Sha512, const unsigned char*) is probably not what you want");
        hasher.Write((const unsigned char*)&data, sizeof(data));
        return hasher;
        */
    }
}
