// ---------------- [ File: bitcoin-hash/src/hash_256.rs ]
crate::ix!();

/**
  | A hasher class for Bitcoin's 256-bit
  | hash (double SHA-256).
  |
  */
#[derive(Default)]
pub struct Hash256 {
    sha: Sha256,
}

pub const HASH256_OUTPUT_SIZE: usize = SHA256_OUTPUT_SIZE;

impl Hash256 {

    pub fn finalize(&mut self, output: &[u8])  {
        
        todo!();
        /*
            assert(output.size() == OUTPUT_SIZE);
            unsigned char buf[CSHA256::OUTPUT_SIZE];
            sha.Finalize(buf);
            sha.Reset().Write(buf, CSHA256::OUTPUT_SIZE).Finalize(output.data());
        */
    }
    
    pub fn write(&mut self, input: &[u8]) -> &mut Hash256 {
        
        todo!();
        /*
            sha.Write(input.data(), input.size());
            return *this;
        */
    }
    
    pub fn reset(&mut self) -> &mut Hash256 {
        
        todo!();
        /*
            sha.Reset();
            return *this;
        */
    }
}
