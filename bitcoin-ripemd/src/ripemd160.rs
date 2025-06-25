// ---------------- [ File: bitcoin-ripemd/src/ripemd160.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ripemd160.h]

/**
  | A hasher class for RIPEMD-160.
  |
  */
pub struct Ripemd160 {
    s:     [u32; 5],
    buf:   [u8; 64],
    bytes: u64,
}

pub const RIPEMD160_OUTPUT_SIZE: usize = 20;

impl Default for Ripemd160 {

    fn default() -> Self {
    
        todo!();
        /*
        : bytes(0),

            ripemd160::Initialize(s);
        */
    }
}

impl Ripemd160 {

    pub fn new() -> Self { Self::default() }
    
    pub fn reset(&mut self) -> &mut Ripemd160 {
        
        todo!();
        /*
            bytes = 0;
        ripemd160::Initialize(s);
        return *this;
        */
    }

    /// Feed an arbitrary byte‑slice into the running RIPEMD‑160
    /// computation (thin convenience wrapper around `write`).
    #[inline]
    pub fn update(&mut self, data: &[u8]) -> &mut Self {
        // SAFETY: pointer/length pair is exactly what the low‑level
        // API expects, and both remain valid for the duration of
        // this call.
        self.write(data.as_ptr(), data.len())
    }
}

//-------------------------------------------[.cpp/bitcoin/src/crypto/ripemd160.cpp]

/**
  | Initialize RIPEMD-160 state.
  |
  */
#[inline] pub fn ripemd160_initialize(s: *mut u32)  {
    
    todo!();
        /*
            s[0] = 0x67452301ul;
        s[1] = 0xEFCDAB89ul;
        s[2] = 0x98BADCFEul;
        s[3] = 0x10325476ul;
        s[4] = 0xC3D2E1F0ul;
        */
}
