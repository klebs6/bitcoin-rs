crate::ix!();

pub struct CoinEntry {
    outpoint: Arc<Mutex<OutPoint>>,
    key:      u8,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CoinEntry, obj) { 
        READWRITE(obj.key, obj.outpoint->hash, VARINT(obj.outpoint->n)); 
    }
    */
}

impl CoinEntry {
    
    pub fn new(ptr: Arc<OutPoint>) -> Self {
    
        todo!();
        /*
           : outpoint(const_cast<OutPoint*>(ptr)), key(DB_COIN)
           */
    }
}
