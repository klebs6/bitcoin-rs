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
#[derive(Serialize,Deserialize,Default)]
pub struct MuHash3072 {

    numerator:   Num3072,
    denominator: Num3072,
}

impl MuHash3072 {

    pub fn to_num3072(in_: &[u8]) -> Num3072 {
        trace!("MuHash3072::to_num3072");
        let hash = Sha256::digest(in_);
        let key = hash.as_slice();
        let nonce = [0u8; 8];            // Bitcoin uses an all‑zero 64‑bit nonce
        let mut stream = [0u8; num_3072::BYTE_SIZE];

        // ChaCha20Legacy == 20‑round, 8‑byte nonce variant
        let mut cipher = ChaCha20Legacy::new_from_slices(key, &nonce).unwrap();
        cipher.apply_keystream(&mut stream);
        Num3072::new(&stream)
    }
    
    /**
      | A singleton with variable sized data
      | in it.
      |
      */
    pub fn new(in_: &[u8]) -> Self {
        trace!("MuHash3072::new");
        MuHash3072 {
            numerator:   Self::to_num3072(in_),
            denominator: Num3072::default(),
        }
    }
    
    /**
      | Finalize into a 32-byte hash. Does not
      | change this object's value.
      |
      */
    pub fn finalize(&mut self, out: &mut u256) {

        trace!("MuHash3072::finalize");

        self.numerator.divide(&self.denominator);

        // Needed to keep the MuHash object valid
        self.denominator.set_to_one();

        let mut data = [0u8; num_3072::BYTE_SIZE];
        self.numerator.to_bytes(&mut data);
        let digest = Sha256::digest(&data);
        *out = u256::from_le_bytes(digest.as_slice().try_into().unwrap());
    }
    
    /**
      | Insert a single piece of data into the
      | set.
      |
      */
    pub fn insert(&mut self, in_: &[u8]) -> &mut Self {
        trace!("MuHash3072::insert");
        self.numerator.multiply(&Self::to_num3072(in_));
        self
    }
    
    /**
      | Remove a single piece of data from the
      | set.
      |
      */
    pub fn remove(&mut self, in_: &[u8]) -> &mut Self {
        trace!("MuHash3072::remove");
        self.denominator.multiply(&Self::to_num3072(in_));
        self
    }
}

#[cfg(test)]
mod set_semantics_validation {
    use super::*;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use bitcoin_u256::u256;
    use tracing::info;

    /// Order‑independence: inserting (A,B) equals inserting (B,A).
    #[traced_test]
    fn insertion_is_commutative() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = ChaCha20Rng::from_seed([4u8; 32]);

        for round in 0..1_024 {
            let mut a = vec![0u8; (rng.next_u32() % 80 + 1) as usize];
            rng.fill_bytes(&mut a);
            let mut b = vec![0u8; (rng.next_u32() % 80 + 1) as usize];
            rng.fill_bytes(&mut b);

            // A then B
            let mut h_ab = MuHash3072::new(&a);
            h_ab.insert(&b);
            let mut out_ab = u256::default();
            h_ab.finalize(&mut out_ab);

            // B then A
            let mut h_ba = MuHash3072::new(&b);
            h_ba.insert(&a);
            let mut out_ba = u256::default();
            h_ba.finalize(&mut out_ba);

            assert_eq!(out_ab, out_ba, "Round {round} failed");
        }
        info!("insertion_is_commutative passed 1 024 rounds");
        Ok(())
    }

    /// Inserting then removing an element is a no‑op on the final hash.
    #[traced_test]
    fn insert_then_remove_noop() -> Result<(), Box<dyn std::error::Error>> {
        let payload = b"stateless‑validation‑vector";

        let mut h1 = MuHash3072::new(payload);
        let mut out_ref = u256::default();
        h1.finalize(&mut out_ref);

        // Insert + remove same payload
        let mut h2 = MuHash3072::new(payload);
        h2.insert(b"ephemeral");
        h2.remove(b"ephemeral");
        let mut out_test = u256::default();
        h2.finalize(&mut out_test);

        assert_eq!(out_ref, out_test);
        Ok(())
    }
}
