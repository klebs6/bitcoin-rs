crate::ix!();

/**
  | Describes a place in the block chain
  | to another node such that if the other
  | node doesn't have the same branch, it
  | can find a recent common trunk.
  | 
  | The further back it is, the further before
  | the fork it may be.
  |
  */
#[derive(Default)]
pub struct BlockLocator {
    pub have: Vec<u256>,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CBlockLocator, obj)
        {
            int nVersion = s.GetVersion();
            if (!(s.GetType() & SER_GETHASH))
                READWRITE(nVersion);
            READWRITE(obj.vHave);
        }
    */
}

impl RecursiveDynamicUsage for BlockLocator {

    fn recursive_dynamic_usage(&self) -> usize {
        
        todo!();
            /*
                return memusage::DynamicUsage(locator.vHave);
            */
    }
}

impl BlockLocator {

    pub fn new(have_in: &Vec<u256>) -> Self {
    
        Self {
            have: have_in.to_vec(),
        }
    }
    
    pub fn set_null(&mut self)  {
        
        self.have.clear()
    }
    
    pub fn is_null(&self) -> bool {
        
        self.have.is_empty()
    }
}
