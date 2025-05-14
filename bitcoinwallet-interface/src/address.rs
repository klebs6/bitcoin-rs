// ---------------- [ File: bitcoinwallet-interface/src/address.rs ]
crate::ix!();

/**
  | Information about one wallet address.
  |
  */
pub struct WalletAddress {
    dest:    TxDestination,
    is_mine: IsMineType,
    name:    String,
    purpose: String,
}

impl WalletAddress {

    pub fn new(
        dest:    TxDestination,
        is_mine: IsMineType,
        name:    String,
        purpose: String) -> Self {
    
        todo!();
        /*
        : dest(std::move(dest)),
        : is_mine(is_mine),
        : name(std::move(name)),
        : purpose(std::move(purpose)),

        
        */
    }
}
