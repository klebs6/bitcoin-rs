// ---------------- [ File: bitcoin-ripemd/src/write.rs ]
crate::ix!();

impl Ripemd160 {

    pub fn write(&mut self, 
        data: *const u8,
        len:  usize) -> &mut Ripemd160 {
        
        todo!();
        /*
            const unsigned char* end = data + len;
        size_t bufsize = bytes % 64;
        if (bufsize && bufsize + len >= 64) {
            // Fill the buffer, and process it.
            memcpy(buf + bufsize, data, 64 - bufsize);
            bytes += 64 - bufsize;
            data += 64 - bufsize;
            ripemd160::Transform(s, buf);
            bufsize = 0;
        }
        while (end - data >= 64) {
            // Process full chunks directly from the source.
            ripemd160::Transform(s, data);
            bytes += 64;
            data += 64;
        }
        if (end > data) {
            // Fill the buffer with what remains.
            memcpy(buf + bufsize, data, end - data);
            bytes += end - data;
        }
        return *this;
        */
    }
}
