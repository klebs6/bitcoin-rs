// ---------------- [ File: bitcoin-test/src/util_wallet.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/util/wallet.h]

lazy_static!{
    /*
    extern const std::string ADDRESS_BCRT1_UNSPENDABLE;
    */
}


/* ------------------ RPC-like  ------------------ */

/**
  | Import the address to the wallet
  |
  */
pub fn importaddress(
        wallet:  &mut Wallet,
        address: &String)  {
    
    todo!();
        /*
        
        */
}

//-------------------------------------------[.cpp/bitcoin/src/test/util/wallet.cpp]

pub const ADDRESS_BCRT1_UNSPENDABLE: &'static str = "bcrt1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3xueyj";

/**
  | Returns a new address from the wallet
  |
  */
#[cfg(ENABLE_WALLET)]
pub fn getnewaddress(w: &mut Wallet) -> String {
    
    todo!();
        /*
            constexpr auto output_type = OutputType::BECH32;
        TxDestination dest;
        bilingual_str error;
        if (!w.GetNewDestination(output_type, "", dest, error)) assert(false);

        return EncodeDestination(dest);
        */
}
