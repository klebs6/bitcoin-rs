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

impl<T> ShlAssign<T> for Sha512 {
    fn shl_assign(&mut self, mut rhs: T) {
        self.feed_data_in(&mut rhs);
    }
}

pub const SHA512_OUTPUT_SIZE: usize = 64;

/**
  | Initialize SHA-512 state.
  |
  */
#[inline] pub fn sha512_initialize(s: *mut u64) {
    unsafe {
        let s = core::slice::from_raw_parts_mut(s, 8);
        s[0] = 0x6a09e667f3bcc908u64;
        s[1] = 0xbb67ae8584caa73bu64;
        s[2] = 0x3c6ef372fe94f82bu64;
        s[3] = 0xa54ff53a5f1d36f1u64;
        s[4] = 0x510e527fade682d1u64;
        s[5] = 0x9b05688c2b3e6c1fu64;
        s[6] = 0x1f83d9abfb41bd6bu64;
        s[7] = 0x5be0cd19137e2179u64;
    }
}

impl Sha512 {

    pub fn size(&self) -> u64 {
        self.bytes
    }

    pub fn new() -> Self {
        let mut h = Sha512::default();
        sha512_initialize(h.s.as_mut_ptr());
        h
    }

    pub fn write(&mut self, data: *const u8, mut len: usize) -> &mut Sha512 {
        // Mirrors the C++ flow closely.
        let mut data_ptr = data;
        let mut bufsize = (self.bytes % 128) as usize;

        if bufsize != 0 && bufsize + len >= 128 {
            // Fill the buffer, and process it.
            let need = 128 - bufsize;
            unsafe { ptr::copy_nonoverlapping(data_ptr, self.buf[bufsize..].as_mut_ptr(), need); }
            self.bytes = self.bytes.wrapping_add(need as u64);
            data_ptr = unsafe { data_ptr.add(need) };
            len -= need;
            sha512_transform(self.s.as_mut_ptr(), self.buf.as_ptr());
            bufsize = 0;
        }

        while len >= 128 {
            // Process full chunks directly from the source.
            sha512_transform(self.s.as_mut_ptr(), data_ptr);
            data_ptr = unsafe { data_ptr.add(128) };
            len -= 128;
            self.bytes = self.bytes.wrapping_add(128);
        }

        if len > 0 {
            unsafe { ptr::copy_nonoverlapping(data_ptr, self.buf[bufsize..].as_mut_ptr(), len); }
            self.bytes = self.bytes.wrapping_add(len as u64);
        }
        self
    }
    
    pub fn finalize(&mut self, hash: &mut [u8; SHA512_OUTPUT_SIZE]) {
        // static const unsigned char pad[128] = {0x80};
        // sizedesc holds 128-bit length; we only fill the low 64 as in C++.
        let mut pad = [0u8; 128];
        pad[0] = 0x80;

        let mut sizedesc = [0u8; 16];
        let bitlen = self.bytes << 3;
        // WriteBE64(sizedesc + 8, bytes << 3);
        sizedesc[8..16].copy_from_slice(&bitlen.to_be_bytes());

        // Write(pad, 1 + ((239 - (bytes % 128)) % 128));
        let padlen = 1 + ((239u64.wrapping_sub(self.bytes % 128)) % 128) as usize;
        self.write(pad.as_ptr(), padlen);
        self.write(sizedesc.as_ptr(), 16);

        // Emit state in BE
        write_be64_into(hash,  0, self.s[0]);
        write_be64_into(hash,  8, self.s[1]);
        write_be64_into(hash, 16, self.s[2]);
        write_be64_into(hash, 24, self.s[3]);
        write_be64_into(hash, 32, self.s[4]);
        write_be64_into(hash, 40, self.s[5]);
        write_be64_into(hash, 48, self.s[6]);
        write_be64_into(hash, 56, self.s[7]);
    }

    pub fn reset(&mut self) -> &mut Sha512 {
        self.bytes = 0;
        sha512_initialize(self.s.as_mut_ptr());
        self
    }

    /**
      | Helper to easily feed data into a Sha512.
      |
      | WARNING: As in the C++ version, this writes the raw
      | in‑memory bytes of `T`—**not** a serialized form.
      */
    #[inline] pub fn feed_data_in<T>(&mut self, rhs: &mut T) {
        let p = (rhs as *mut T) as *const u8;
        let n = core::mem::size_of::<T>();
        unsafe { self.write(p, n); }
    }

    pub fn finalize_to_array(mut self) -> [u8; 64] {
        let mut out = [0u8; 64];
        self.finalize(&mut out);
        out
    }
}
