crate::ix!();

/**
  | Fast randomness source. This is seeded
  | once with secure random data, but is
  | completely deterministic and does
  | not gather more entropy after that.
  | 
  | This class is not thread-safe.
  |
  */
pub struct FastRandomContext {
    pub requires_seed: bool,
    pub rng:           ChaCha20,
    pub bytebuf:       [u8; 64],
    pub bytebuf_size:  i32,
    pub bitbuf:        u64,
    pub bitbuf_size:   i32,
}

impl Default for FastRandomContext {

    fn default() -> Self {
        let deterministic = false;

        Self::new(deterministic)
    }
}

/**
  | Compatibility with the C++11
  | 
  | UniformRandomBitGenerator concept
  |
  */
pub type FastRandomContextResultType = u64;

pub trait RandRange {

    fn randrange(&mut self, range: u64) -> u64;
}

impl RandRange for FastRandomContext {

    /**
      | Generate a random integer in the range
      | [0..range).
      | 
      | Precondition: range > 0.
      |
      */
    fn randrange(&mut self, mut range: u64) -> u64 {

        assert!(range != 0);

        range -= 1;

        let bits: u64 = count_bits(range);

        while true {

            let ret: u64 = self.randbits(bits.try_into().unwrap());

            if ret <= range {
                return ret;
            }
        }

        unreachable!();
    }
}

impl From<&u256> for FastRandomContext {

    /**
      | Initialize with explicit seed (only
      | for testing)
      |
      */
    fn from(seed: &u256) -> Self {
    
        let mut rng = ChaCha20::default();
        rng.set_key(seed.blob.begin(), 32);

        Self {
            requires_seed: false,
            rng:           rng,
            bytebuf:       [0; 64],
            bytebuf_size:  0,
            bitbuf:        0,
            bitbuf_size:   0,
        }
    }
}

impl RngCore for FastRandomContext {

    fn next_u32(&mut self) -> u32 {
        self.randbits(32).try_into().unwrap()
    }

    fn next_u64(&mut self) -> u64 {

        if self.bytebuf_size < 8 {
            self.fill_byte_buffer();
        }

        let offset: isize = self.bytebuf_size.try_into().unwrap();

        let ret: u64 = readle64(
            unsafe {
                self.bytebuf.as_mut_ptr().add(64).offset(-offset)
            }
        );

        self.bytebuf_size -= 8;

        ret
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {

        if self.requires_seed {
            self.random_seed();
        }

        self.rng.keystream(dest.as_mut_ptr(), size_of_val(&dest));
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl From<&mut FastRandomContext> for FastRandomContext {

    /**
      | Move a FastRandomContext. If the original
      | one is used again, it will be reseeded.
      |
      */
    fn from(other: &mut FastRandomContext) -> Self {

        let mut x: Self = unsafe { std::mem::zeroed() };

        x.requires_seed = other.requires_seed;
        x.rng           = other.rng.clone();
        x.bytebuf       = other.bytebuf;

        x.bytebuf_size  = other.bytebuf_size;
        x.bitbuf        = other.bitbuf;
        x.bitbuf_size   = other.bitbuf_size;

        other.requires_seed = true;
        other.bytebuf_size  = 0;
        other.bitbuf_size   = 0;

        x
    }
}

impl FastRandomContext {

    pub fn new(deterministic: bool) -> Self {
    
        let mut x = Self {
            requires_seed: !deterministic,
            rng:           ChaCha20::default(),
            bytebuf:       [0; 64],
            bytebuf_size:  0,
            bitbuf:        0,
            bitbuf_size:   0,
        };

        if !deterministic {
            return x;
        }

        let mut seed: u256 = u256::default();

        x.rng.set_key(seed.blob.begin(), 32);

        x
    }
    
    pub fn fill_byte_buffer(&mut self)  {

        //I call this unsafe because we are
        //essentially borrowing self as mut twice
        //
        //the below trick, converting to ptr and
        //back is a hack to silence the borrow
        //czecher
        unsafe {
            let ptr = self.bytebuf.as_mut_slice().as_mut_ptr();
            let len = self.bytebuf.len();

            let slice = unsafe { std::slice::from_raw_parts_mut(ptr,len) };

            self.fill_bytes(slice);

            self.bytebuf_size = size_of_val(&self.bytebuf).try_into().unwrap();
        }
        
    }

    pub fn fill_bit_buffer(&mut self)  {
        
        self.bitbuf = self.rand64();
        self.bitbuf_size = 64;
    }
    
    /**
      | Generate a random 64-bit integer.
      |
      */
    pub fn rand64(&mut self) -> u64 {
        self.next_u64()
    }

    /**
      | Generate a random (bits)-bit integer.
      |
      */
    pub fn randbits(&mut self, bits: i32) -> u64 {
        
        if bits == 0 {
            0

        } else if bits > 32 {
            self.rand64() >> (64 - bits)

        } else {

            if self.bitbuf_size < bits {
                self.fill_bit_buffer();
            }

            let ret: u64 = self.bitbuf & (!0 >> (64 - bits));
            self.bitbuf >>= bits;
            self.bitbuf_size -= bits;
            ret
        }
    }

    /**
      | Generate a random 32-bit integer.
      |
      */
    pub fn rand32(&mut self) -> u32 {
        self.next_u32()
    }

    /**
      | Generate a random boolean.
      |
      */
    pub fn randbool(&mut self) -> bool {
        self.randbits(1) != 0
    }
    
    pub fn min() -> u64 {
        0
    }
    
    pub fn max() -> u64 {
        u64::MAX
    }
    
    #[inline] pub fn invoke(&mut self) -> u64 {
        self.rand64()
    }
    
    pub fn random_seed(&mut self)  {
        
        let seed: u256 = get_rand_hash();
        self.rng.set_key(seed.blob.begin(), 32);
        self.requires_seed = false;
    }
    
    /**
      | generate a random uint256.
      |
      */
    pub fn rand256(&mut self) -> u256 {
        
        if self.bytebuf_size < 32 {
            self.fill_byte_buffer();
        }

        let mut ret = u256::default();

        let offset: isize = self.bytebuf_size.try_into().unwrap();

        unsafe {
            let dst = ret.blob.begin() as *mut c_void;
            let src = self.bytebuf.as_mut_ptr().add(64).offset(-offset);

            libc::memcpy(
                dst, 
                src as *const c_void, 
                32
            );
        }

        self.bytebuf_size -= 32;

        ret
    }
    
    /**
      | Generate random bytes.
      |
      */
    pub fn randbytes(&mut self, len: usize) -> Vec<u8> {
        
        if self.requires_seed {
            self.random_seed();
        }

        let mut ret = Vec::<u8>::with_capacity(len);

        if len > 0 {
            self.rng.keystream(ret.as_mut_ptr(), len);
        }

        ret
    }
    
}
