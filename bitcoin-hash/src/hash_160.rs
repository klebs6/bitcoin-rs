crate::ix!();

/**
  | A hasher class for Bitcoin's 160-bit
  | hash (SHA-256 + RIPEMD-160).
  |
  */
pub struct Hash160 {
    sha: Sha256,
}

pub const HASH160_OUTPUT_SIZE: usize = RIPEMD160_OUTPUT_SIZE;

impl Hash160 {

    pub fn finalize(&mut self, output: &[u8])  {
        
        todo!();
        /*
            assert(output.size() == OUTPUT_SIZE);
            unsigned char buf[CSHA256::OUTPUT_SIZE];
            sha.Finalize(buf);
            CRIPEMD160().Write(buf, CSHA256::OUTPUT_SIZE).Finalize(output.data());
        */
    }
    
    pub fn write(&mut self, input: &[u8]) -> &mut Hash160 {
        
        todo!();
        /*
            sha.Write(input.data(), input.size());
            return *this;
        */
    }
    
    pub fn reset(&mut self) -> &mut Hash160 {
        
        todo!();
        /*
            sha.Reset();
            return *this;
        */
    }
}

/**
  | Compute the 160-bit hash an object.
  |
  */
#[inline] pub fn hash160<T1>(in1: &T1) -> u160 {

    todo!();
        /*
            u160 result;
        Hash160().Write(MakeUCharSpan(in1)).Finalize(result);
        return result;
        */
}
