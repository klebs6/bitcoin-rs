crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/dummywallet.cpp]

pub struct DummyWalletInit {

}

impl WalletInitInterface for DummyWalletInit {

}

impl HasWalletSupport for DummyWalletInit {

    fn has_wallet_support(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
    
impl ParameterInteraction for DummyWalletInit {
    fn parameter_interaction(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
    
impl Construct for DummyWalletInit {
    fn construct(&self, node: &mut NodeContext)  {
        
        todo!();
        /*
            LogPrintf("No wallet support compiled in!\n");
        */
    }
}
    
impl AddWalletOptions for DummyWalletInit {
    fn add_wallet_options(&self, argsman: &mut ArgsManager)  {
        
        todo!();
        /*
            argsman.AddHiddenArgs({
            "-addresstype",
            "-avoidpartialspends",
            "-changetype",
            "-consolidatefeerate=<amt>",
            "-disablewallet",
            "-discardfee=<amt>",
            "-fallbackfee=<amt>",
            "-keypool=<n>",
            "-maxapsfee=<n>",
            "-maxtxfee=<amt>",
            "-mintxfee=<amt>",
            "-paytxfee=<amt>",
            "-signer=<cmd>",
            "-spendzeroconfchange",
            "-txconfirmtarget=<n>",
            "-wallet=<path>",
            "-walletbroadcast",
            "-walletdir=<dir>",
            "-walletnotify=<cmd>",
            "-walletrbf",
            "-dblogsize=<n>",
            "-flushwallet",
            "-privdb",
            "-walletrejectlongchains",
            "-unsafesqlitesync",
        });
        */
    }
}

pub const WALLET_INIT_INTERFACE: DummyWalletInit = DummyWalletInit { };

pub fn make_wallet(wallet: &Arc<Wallet>) -> Box<dyn WalletInterface> {
    
    todo!();
        /*
            throw std::logic_error("Wallet function called in non-wallet build.");
        */
}
