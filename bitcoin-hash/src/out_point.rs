// ---------------- [ File: bitcoin-hash/src/out_point.rs ]
crate::ix!();

/**
  | An outpoint - a combination of a transaction
  | hash and an index n into its vout
  |
  */
#[derive(Clone,Serialize,Deserialize)]
pub struct OutPoint {
    pub hash: u256,
    pub n:    u32,
}

impl RecursiveDynamicUsage for OutPoint {

    fn recursive_dynamic_usage(&self) -> usize {
        
        todo!();
            /*
                return 0;
            */
    }
}

impl Hash for OutPoint {

     fn hash<H: Hasher>(&self, state: &mut H) {
         todo!();

     }
}

pub const OUT_POINT_NULL_INDEX: u32 = u32::MAX;

impl Default for OutPoint {
    
    fn default() -> Self {
        todo!();
        /*
        : n(OUT_POINT_NULL_INDEX),
        */
    }
}

impl Ord for OutPoint {
    
    #[inline] fn cmp(&self, other: &OutPoint) -> Ordering {
        todo!();
        /*
            int cmp = a.hash.Compare(b.hash);
            return cmp < 0 || (cmp == 0 && a.n < b.n);
        */
    }
}

impl PartialOrd<OutPoint> for OutPoint {
    #[inline] fn partial_cmp(&self, other: &OutPoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl PartialEq<OutPoint> for OutPoint {
    
    #[inline] fn eq(&self, other: &OutPoint) -> bool {
        todo!();
        /*
            return (a.hash == b.hash && a.n == b.n);
        */
    }
}

impl Eq for OutPoint {}

impl OutPoint {
    
    pub fn new(
        hash_in: &u256,
        n_in:    u32) -> Self {
    
        todo!();
        /*
        : hash(hashIn),
        : n(nIn),

        
        */
    }
    
    pub fn set_null(&mut self)  {
        
        todo!();
        /*
            hash.SetNull(); n = OUT_POINT_NULL_INDEX;
        */
    }
    
    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            return (hash.IsNull() && n == OUT_POINT_NULL_INDEX);
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return strprintf("OutPoint(%s, %u)", hash.ToString().substr(0,10), n);
        */
    }
}
