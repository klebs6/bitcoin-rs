crate::ix!();

///-------------------------


//-------------------------------------------[.cpp/bitcoin/src/validationinterface.h]

/**
  | Add wallet name to persistent configuration
  | so it will be loaded on startup.
  |
  */
pub fn add_wallet_setting<'a>(
        chain:       &'a mut dyn ChainInterface,
        wallet_name: &str) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Remove wallet name from persistent
  | configuration so it will not be loaded
  | on startup.
  |
  */
pub fn remove_wallet_setting<'a>(
        chain:       &'a mut dyn ChainInterface,
        wallet_name: &str) -> bool {
    
    todo!();
        /*
        
        */
}

