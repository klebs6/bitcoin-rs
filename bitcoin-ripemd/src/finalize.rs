// ---------------- [ File: bitcoin-ripemd/src/finalize.rs ]
crate::ix!();

impl Ripemd160 {
    
    pub fn finalize(&mut self, hash: [u8; RIPEMD160_OUTPUT_SIZE])  {
        
        todo!();
        /*
            static const unsigned char pad[64] = {0x80};
        unsigned char sizedesc[8];
        WriteLE64(sizedesc, bytes << 3);
        Write(pad, 1 + ((119 - (bytes % 64)) % 64));
        Write(sizedesc, 8);
        WriteLE32(hash, s[0]);
        WriteLE32(hash + 4, s[1]);
        WriteLE32(hash + 8, s[2]);
        WriteLE32(hash + 12, s[3]);
        WriteLE32(hash + 16, s[4]);
        */
    }
}
