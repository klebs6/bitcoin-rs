crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha256.cpp]
impl Write for Sha256 {

    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_ptr(buf.as_ptr(), buf.len());
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

impl Sha256 {

    pub fn new() -> Self { Self::default() }

    pub fn write_from_iterator(&mut self, 
        data: Box<dyn Iterator<Item=u8>>,
        len:  usize) 
    {
        todo!();
    }

    pub fn write_ptr(&mut self, 
        data: *const u8,
        len:  usize) {
        
        todo!();
        /*
            const unsigned char* end = data + len;
        size_t bufsize = bytes % 64;
        if (bufsize && bufsize + len >= 64) {
            // Fill the buffer, and process it.
            memcpy(buf + bufsize, data, 64 - bufsize);
            bytes += 64 - bufsize;
            data += 64 - bufsize;
            Transform(s, buf, 1);
            bufsize = 0;
        }
        if (end - data >= 64) {
            size_t blocks = (end - data) / 64;
            Transform(s, data, blocks);
            data += 64 * blocks;
            bytes += 64 * blocks;
        }
        if (end > data) {
            // Fill the buffer with what remains.
            memcpy(buf + bufsize, data, end - data);
            bytes += end - data;
        }
        return *this;
        */
    }

    pub fn finalize(&mut self, hash: &mut [u8; SHA256_OUTPUT_SIZE])  {
        
        todo!();
        /*
            static const unsigned char pad[64] = {0x80};
        unsigned char sizedesc[8];
        WriteBE64(sizedesc, bytes << 3);
        Write(pad, 1 + ((119 - (bytes % 64)) % 64));
        Write(sizedesc, 8);
        WriteBE32(hash, s[0]);
        WriteBE32(hash + 4, s[1]);
        WriteBE32(hash + 8, s[2]);
        WriteBE32(hash + 12, s[3]);
        WriteBE32(hash + 16, s[4]);
        WriteBE32(hash + 20, s[5]);
        WriteBE32(hash + 24, s[6]);
        WriteBE32(hash + 28, s[7]);
        */
    }
    
    pub fn reset(&mut self) -> &mut Sha256 {
        
        todo!();
        /*
            bytes = 0;
        sha256::Initialize(s);
        return *this;
        */
    }
}
