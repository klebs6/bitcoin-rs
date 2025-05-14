// ---------------- [ File: bitcoin-muhash/src/muhash.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/muhash.h]
//-------------------------------------------[.cpp/bitcoin/src/crypto/muhash.cpp]

/**
  | A class representing MuHash sets
  | 
  | MuHash is a hashing algorithm that supports
  | adding set elements in any order but
  | also deleting in any order. As a result,
  | it can maintain a running sum for a set
  | of data as a whole, and add/remove when
  | data is added to or removed from it. A
  | downside of MuHash is that computing
  | an inverse is relatively expensive.
  | This is solved by representing the running
  | value as a fraction, and multiplying
  | added elements into the numerator and
  | removed elements into the denominator.
  | Only when the final hash is desired,
  | a single modular inverse and multiplication
  | is needed to combine the two. The combination
  | is also run on serialization to allow
  | for space-efficient storage on disk.
  | 
  | As the update operations are also associative,
  | H(a)+H(b)+H(c)+H(d) can in fact be
  | computed as (H(a)+H(b)) + (H(c)+H(d)).
  | This implies that all of this is perfectly
  | parallellizable: each thread can process
  | an arbitrary subset of the update operations,
  | allowing them to be efficiently combined
  | later.
  | 
  | MuHash does not support checking if
  | an element is already part of the set.
  | That is why this class does not enforce
  | the use of a set as the data it represents
  | because there is no efficient way to
  | do so.
  | 
  | It is possible to add elements more than
  | once and also to remove elements that
  | have not been added before. However,
  | this implementation is intended to
  | represent a set of elements.
  | 
  | See also https://cseweb.ucsd.edu/~mihir/papers/inchash.pdf
  | and https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2017-May/014337.html.
  |
  */
#[derive(Default)]
pub struct MuHash3072 {

    numerator:   Num3072,
    denominator: Num3072,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(MuHash3072, obj)
        {
            READWRITE(obj.m_numerator);
            READWRITE(obj.m_denominator);
        }
    */
}

impl MuHash3072 {

    pub fn to_num3072(&mut self, in_: &[u8]) -> Num3072 {
        
        todo!();
        /*
            unsigned char tmp[num_3072::BYTE_SIZE];

        uint256 hashed_in = (CHashWriter(SER_DISK, 0) << in).GetSHA256();
        ChaCha20(hashed_in.data(), hashed_in.size()).Keystream(tmp, Num3072::BYTE_SIZE);
        Num3072 out{tmp};

        return out;
        */
    }
    
    /**
      | A singleton with variable sized data
      | in it.
      |
      */
    pub fn new(in_: &[u8]) -> Self {
    
        todo!();
        /*
            m_numerator = ToNum3072(in);
        */
    }
    
    /**
      | Finalize into a 32-byte hash. Does not
      | change this object's value.
      |
      */
    pub fn finalize(&mut self, out: &mut u256)  {
        
        todo!();
        /*
            m_numerator.Divide(m_denominator);
        m_denominator.SetToOne();  // Needed to keep the MuHash object valid

        unsigned char data[Num3072::BYTE_SIZE];
        m_numerator.ToBytes(data);

        out = (CHashWriter(SER_DISK, 0) << data).GetSHA256();
        */
    }
    
    /**
      | Insert a single piece of data into the
      | set.
      |
      */
    pub fn insert(&mut self, in_: &[u8]) -> &mut MuHash3072 {
        
        todo!();
        /*
            m_numerator.Multiply(ToNum3072(in));
        return *this;
        */
    }
    
    /**
      | Remove a single piece of data from the
      | set.
      |
      */
    pub fn remove(&mut self, in_: &[u8]) -> &mut MuHash3072 {
        
        todo!();
        /*
            m_denominator.Multiply(ToNum3072(in));
        return *this;
        */
    }
}
