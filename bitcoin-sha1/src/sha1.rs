// ---------------- [ File: bitcoin-sha1/src/sha1.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha1.h]
//-------------------------------------------[.cpp/bitcoin/src/crypto/sha1.cpp]

/**
  | A hasher class for Sha1.
  |
  */
#[derive(Debug, Builder, Getters)]
#[getset(get = "pub")]
pub struct Sha1 {
    s:     [u32; 5],
    buf:   [u8; 64],
    bytes: u64,
}

impl Default for Sha1 {
    fn default() -> Self {
        Self {
            s:     [0; 5],
            buf:   [0; 64],
            bytes: 0,
        }
    }
}

/// Size of a SHA‑1 digest in bytes.
pub const SHA1_OUTPUT_SIZE: usize = 20;

/**
  | Initialize SHA-1 state.
  |
  */
#[inline(always)]
pub fn sha1_initialize(s: *mut u32) {
    unsafe {
        *s.add(0) = 0x6745_2301;
        *s.add(1) = 0xEFCD_AB89;
        *s.add(2) = 0x98BA_DCFE;
        *s.add(3) = 0x1032_5476;
        *s.add(4) = 0xC3D2_E1F0;
    }
}

impl Sha1 {

    pub fn new() -> Self {
        let mut h = Sha1::default();
        unsafe { sha1_initialize(h.s.as_mut_ptr()) };
        info!("Sha1::new ‑ state initialised");
        h
    }

    pub fn write(&mut self, data: *const u8, len: usize) -> &mut Self {
        unsafe {
            let mut cur = data;
            let end = data.add(len);
            let mut buf_len = (self.bytes % 64) as usize;

            // Finish filling the internal buffer if necessary.
            if buf_len > 0 && buf_len + len >= 64 {
                std::ptr::copy_nonoverlapping(
                    cur,
                    self.buf.as_mut_ptr().add(buf_len),
                    64 - buf_len,
                );
                self.bytes += (64 - buf_len) as u64;
                cur = cur.add(64 - buf_len);
                sha1_transform(self.s.as_mut_ptr(), self.buf.as_ptr());
                buf_len = 0;
            }

            // Process full 64‑byte chunks straight from the input.
            while end.offset_from(cur) as usize >= 64 {
                sha1_transform(self.s.as_mut_ptr(), cur);
                self.bytes += 64;
                cur = cur.add(64);
            }

            // Buffer remaining data.
            if end > cur {
                let remaining = end.offset_from(cur) as usize;
                std::ptr::copy_nonoverlapping(
                    cur,
                    self.buf.as_mut_ptr().add(buf_len),
                    remaining,
                );
                self.bytes += remaining as u64;
            }
        }
        self
    }
    
    pub fn finalize(&mut self, hash_out: &mut [u8; SHA1_OUTPUT_SIZE]) {
        // Build padding (1000 0000 …).
        let mut pad = [0u8; 64];
        pad[0] = 0x80;

        let mut sizedesc = [0u8; 8];
        unsafe { write_be64(sizedesc.as_mut_ptr(), self.bytes << 3) };

        // Pad so that final length (including 8‑byte length) ≡ 0 mod 64.
        let pad_len = 1 + ((119 - (self.bytes % 64)) % 64) as usize;
        self.write(pad.as_ptr(), pad_len);
        self.write(sizedesc.as_ptr(), 8);

        // Produce the final digest in big‑endian.
        for (i, chunk) in hash_out.chunks_exact_mut(4).enumerate() {
            chunk.copy_from_slice(&self.s[i].to_be_bytes());
        }

        debug!("Sha1::finalize ‑ digest computed");
    }
    
    pub fn reset(&mut self) -> &mut Self {
        self.bytes = 0;
        unsafe { sha1_initialize(self.s.as_mut_ptr()) };
        info!("Sha1::reset ‑ state cleared");
        self
    }
}
