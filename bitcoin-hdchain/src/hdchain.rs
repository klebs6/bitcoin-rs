crate::ix!();

/**
  | simple HD chain data model
  |
  */
pub struct HDChain {

    n_external_chain_counter: u32,
    n_internal_chain_counter: u32,

    /**
      | seed hash160
      |
      */
    seed_id:                  KeyID,

    n_version:                i32,
}

pub mod hd_chain {
    pub const VERSION_HD_BASE:        i32 = 1;
    pub const VERSION_HD_CHAIN_SPLIT: i32 = 2;
    pub const CURRENT_VERSION:        i32 = VERSION_HD_CHAIN_SPLIT;
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CHDChain, obj)
        {
            READWRITE(obj.nVersion, obj.nExternalChainCounter, obj.seed_id);
            if (obj.nVersion >= VERSION_HD_CHAIN_SPLIT) {
                READWRITE(obj.nInternalChainCounter);
            }
        }
    */
}

impl Default for HDChain {
    
    fn default() -> Self {
        todo!();
        /*


            SetNull();
        */
    }
}

impl PartialEq<HDChain> for HDChain {
    
    #[inline] fn eq(&self, other: &HDChain) -> bool {
        todo!();
        /*
            return seed_id == chain.seed_id;
        */
    }
}

impl Eq for HDChain {}

impl HDChain {

    pub fn set_null(&mut self)  {
        
        todo!();
        /*
            nVersion = CHDChain::CURRENT_VERSION;
            nExternalChainCounter = 0;
            nInternalChainCounter = 0;
            seed_id.SetNull();
        */
    }
}
