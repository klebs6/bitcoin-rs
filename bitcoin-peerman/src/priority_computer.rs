crate::ix!();

impl Into<GenTxId> for Announcement {

    fn into(self) -> GenTxId {
        
        match self.is_wtxid() != 0 {
            true  => GenTxId::wtxid(&self.txhash),
            false => GenTxId::txid(&self.txhash),
        }
    }
}

/**
  | Type alias for priorities.
  |
  */
pub type Priority = u64;

/**
  | A functor with embedded salt that computes
  | priority of an announcement.
  | 
  | Higher priorities are selected first.
  |
  */
#[derive(Debug,Clone)]
pub struct PriorityComputer {
    pub k0: u64,
    pub k1: u64,
}

impl PriorityComputer {
    
    pub fn new(deterministic: bool) -> Self {
    
        todo!();
        /*
            :
            m_k0{deterministic ? 0 : GetRand(0xFFFFFFFFFFFFFFFF)},
            m_k1{deterministic ? 0 : GetRand(0xFFFFFFFFFFFFFFFF)}
        */
    }
    
    pub fn invoke(&self, 
        txhash:    &u256,
        peer:      NodeId,
        preferred: bool) -> Priority {

        let mut hasher 
        = SipHasher::new_with_keys(self.k0,self.k1);

        hasher.write(txhash.as_slice());
        hasher.write_i64(peer);
        
        let low_bits: u64 = hasher.finish() >> 1;

        let preferred = match preferred {
            true  => 1,
            false => 0,
        };

        low_bits | preferred << 63
    }
    
    pub fn invoke_announcement(&self, ann: &Announcement) -> Priority {
        
        self.invoke(&ann.txhash,ann.peer,ann.preferred() != 0)
    }
}
