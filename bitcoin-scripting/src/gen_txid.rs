crate::ix!();

/**
  | A generic txid reference (txid or wtxid).
  |
  */
pub struct GenTxId {
    is_wtxid: bool,
    hash:     u256,
}

impl PartialEq<GenTxId> for GenTxId {
    
    #[inline] fn eq(&self, other: &GenTxId) -> bool {
        self.is_wtxid == other.is_wtxid 
            && self.hash == other.hash
    }
}

impl Eq for GenTxId {}

impl Ord for GenTxId {
    
    #[inline] fn cmp(&self, other: &GenTxId) -> Ordering {
        (&self.is_wtxid, &self.hash.blob).cmp(&(&other.is_wtxid, &other.hash.blob))
    }
}

impl PartialOrd<GenTxId> for GenTxId {
    #[inline] fn partial_cmp(&self, other: &GenTxId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//-------------------------------------------[.cpp/bitcoin/src/primitives/transaction.cpp]
impl GenTxId {

    pub fn new(
        is_wtxid: bool,
        hash:     &u256) -> Self {
    
        Self {
            is_wtxid: is_wtxid,
            hash:     hash.clone(),
        }
    }
    
    pub fn txid(hash: &u256) -> GenTxId {
        GenTxId::new(false, hash)
    }
    
    pub fn wtxid(hash: &u256) -> GenTxId {
        GenTxId::new(true, hash)
    }
    
    pub fn is_wtxid(&self) -> bool {
        self.is_wtxid
    }
    
    pub fn get_hash(&self) -> &u256 {
        &self.hash
    }
}
