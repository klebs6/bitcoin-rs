// ---------------- [ File: bitcoin-sha256/src/sha256.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha256.h]

/**
  | A hasher class for SHA-256.
  |
  | Translated with guidance from G.W.
  | Bickerstaff, 12/2021
  */
#[derive(Serialize,Deserialize)]
pub struct Sha256 {
    s:     [u32; 8],

    #[serde(with = "BigArray")]
    buf:   [u8; 64],
    bytes: u64,
}

impl Sha256 {

    /// Initialize SHA‑256 state words to the IV defined in FIPS 180‑4.
    pub fn initialize(&mut self) {
        // Safety: caller guarantees `s` points to at least eight `u32`s.
        unsafe {
            const IV: [u32; 8] = [
                0x6a09e667,
                0xbb67ae85,
                0x3c6ef372,
                0xa54ff53a,
                0x510e527f,
                0x9b05688c,
                0x1f83d9ab,
                0x5be0cd19,
            ];
            for (i, &val) in IV.iter().enumerate() {
                self.s[i] = val;
            }
        }
        trace!(target: "sha256", "sha256 state initialised");

    }
}

/**
  | One round of SHA-256.
  |
  */
#[inline] pub fn sha256_round(
        a: u32,
        b: u32,
        c: u32,
        d: &mut u32,
        e: u32,
        f: u32,
        g: u32,
        h: &mut u32,
        k: u32)  {

    #[inline] fn my_sigma0(x: u32) -> u32 {
        
        todo!();
            /*
                return (x >> 2 | x << 30) ^ (x >> 13 | x << 19) ^ (x >> 22 | x << 10);
            */
    }


    #[inline] fn my_sigma1(x: u32) -> u32 {
        
        todo!();
            /*
                return (x >> 6 | x << 26) ^ (x >> 11 | x << 21) ^ (x >> 25 | x << 7);
            */
    }

    todo!();
        /*
            uint32_t t1 = h + MySigma1(e) + Ch(e, f, g) + k;
        uint32_t t2 = MySigma0(a) + Maj(a, b, c);
        d += t1;
        h = t1 + t2;
        */
}

pub const SHA256_OUTPUT_SIZE: usize = 32;

#[inline] pub fn sha256_ch(
        x: u32,
        y: u32,
        z: u32) -> u32 {
    
    todo!();
        /*
            return z ^ (x & (y ^ z));
        */
}

#[inline] pub fn sha256_maj(
        x: u32,
        y: u32,
        z: u32) -> u32 {
    
    todo!();
        /*
            return (x & y) | (z & (x | y));
        */
}

#[inline] pub fn sha256_sigma0(x: u32) -> u32 {
    
    todo!();
        /*
            return (x >> 7 | x << 25) ^ (x >> 18 | x << 14) ^ (x >> 3);
        */
}

#[inline] pub fn sha256_sigma1(x: u32) -> u32 {
    
    todo!();
        /*
            return (x >> 17 | x << 15) ^ (x >> 19 | x << 13) ^ (x >> 10);
        */
}
