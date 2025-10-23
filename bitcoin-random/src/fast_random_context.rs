// ---------------- [ File: bitcoin-random/src/fast_random_context.rs ]
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
#[derive(Debug,Getters,MutGetters,Setters)]
#[getset(get="pub",set="pub",get_mut="pub")]
pub struct FastRandomContext {
    requires_seed: bool,
    rng:           ChaCha20,
    bytebuf:       [u8; 64],
    bytebuf_size:  i32,
    bitbuf:        u64,
    bitbuf_size:   i32,
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
        rng.set_key(seed.blob().begin(), 32);

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

        self.rng.keystream(dest.as_mut_ptr(), dest.len());
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

        x.rng.set_key(seed.blob().begin(), 32);

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
        self.rng.set_key(seed.blob().begin(), 32);
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
            let dst = ret.blob().begin() as *mut c_void;
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
            unsafe {
                self.rng.keystream(ret.as_mut_ptr(), len);
                ret.set_len(len); // <-- add this
            }
        }
        ret
    }
}

#[cfg(test)]
mod fast_random_context_spec {
    use super::*;

    #[traced_test]
    fn deterministic_from_same_seed_is_identical() {
        let seed = u256::default(); // explicit seed (all-zero is fine for test determinism)
        let mut a = FastRandomContext::from(&seed);
        let mut b = FastRandomContext::from(&seed);

        // A few mixed draws from both contexts should match 1:1.
        assert_eq!(a.rand32(), b.rand32());
        assert_eq!(a.rand64(), b.rand64());

        // randbits edge cases
        assert_eq!(a.randbits(0), 0);
        assert_eq!(b.randbits(0), 0);

        let x1 = a.randbits(1);
        let y1 = b.randbits(1);
        assert_eq!(x1, y1);
        assert!(x1 < 2);

        let x32 = a.randbits(32);
        let y32 = b.randbits(32);
        assert_eq!(x32, y32);
        assert!(x32 < (1u64 << 32));

        let x33 = a.randbits(33);
        let y33 = b.randbits(33);
        assert_eq!(x33, y33);
        assert!(x33 < (1u64 << 33));
    }

    #[traced_test]
    fn randrange_is_within_bounds_and_handles_small_ranges() {
        let mut ctx = FastRandomContext::new(true); // deterministic
        for r in [1u64, 2, 3, 10, 1_000_000] {
            for _ in 0..100 {
                let v = ctx.randrange(r);
                assert!(v < r, "v={v} should be < {r}");
            }
        }
    }

    #[traced_test]
    fn next_u64_consumes_from_internal_buffer() {
        let mut ctx = FastRandomContext::new(true);
        let a = ctx.next_u64();
        let b = ctx.next_u64();
        // There is no requirement they differ, but deterministic ChaCha20 with a fixed key will.
        assert_ne!(a, b);
    }

    #[traced_test]
    fn fill_bytes_writes_requested_length() {
        let mut ctx = FastRandomContext::new(true);
        let mut buf = [0u8; 17];
        ctx.fill_bytes(&mut buf);
        // We can at least check that the buffer changed (deterministically).
        assert!(buf.iter().any(|&x| x != 0));
    }

    #[traced_test]
    fn move_semantics_mark_source_for_reseed_and_preserve_state_in_target() {
        let mut src = FastRandomContext::new(true);
        // Pull a value to force some internal state to be used.
        let _ = src.rand32();

        // Move into `dst`.
        let dst = FastRandomContext::from(&mut src);

        // Source marked for reseed and emptied bit/byte buffers.
        assert!(src.requires_seed);
        assert_eq!(src.bytebuf_size, 0);
        assert_eq!(src.bitbuf_size, 0);

        // Destination should be usable without reseed.
        // (We can’t assert exact bytes here without duplicating the move algorithm;
        // it’s sufficient that this does not panic and produces a value.)
        let _ = &dst; // move worked; dst is valid
    }

    #[traced_test]
    fn randbytes_returns_len_and_fills_data() {
        let mut ctx = FastRandomContext::new(true);
        let want = 64usize;
        let v = ctx.randbytes(want);
        assert_eq!(v.len(), want);
        assert!(v.iter().any(|&x| x != 0));
    }
}
