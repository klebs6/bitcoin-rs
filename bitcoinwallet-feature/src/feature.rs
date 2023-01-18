crate::ix!();

/**
  | (client) version numbers for particular
  | wallet features
  |
  */
bitflags!{

    pub struct WalletFeature: u32 {

        /*
          | the earliest version new wallets supports
          | (only useful for getwalletinfo's clientversion
          | output)
          |
          */
        const FEATURE_BASE              = 10500; 

        /*
          | wallet encryption
          |
          */
        const FEATURE_WALLETCRYPT       = 40000; 

        /*
          | compressed public keys
          |
          */
        const FEATURE_COMPRPUBKEY       = 60000; 

        /*
          | Hierarchical key derivation after
          | BIP32 (HD Wallet)
          |
          */
        const FEATURE_HD                = 130000; 

        /*
          | Wallet with HD chain split (change outputs
          | will use m/0'/1'/k)
          |
          */
        const FEATURE_HD_SPLIT          = 139900; 

        /*
          | Wallet without a default key written
          |
          */
        const FEATURE_NO_DEFAULT_KEY    = 159900; 

        /*
          | Upgraded to HD SPLIT and can have a pre-split
          | keypool
          |
          */
        const FEATURE_PRE_SPLIT_KEYPOOL = 169900; 

        const FEATURE_LATEST            = Self::FEATURE_PRE_SPLIT_KEYPOOL.bits;
    }
}

pub fn is_feature_supported(
        wallet_version:  i32,
        feature_version: i32) -> bool {
    
    todo!();
        /*
            return wallet_version >= feature_version;
        */
}

pub fn get_closest_wallet_feature(version: i32) -> WalletFeature {
    
    todo!();
        /*
            static constexpr std::array wallet_features{FEATURE_LATEST, FEATURE_PRE_SPLIT_KEYPOOL, FEATURE_NO_DEFAULT_KEY, FEATURE_HD_SPLIT, FEATURE_HD, FEATURE_COMPRPUBKEY, FEATURE_WALLETCRYPT, FEATURE_BASE};
        for (const WalletFeature& wf : wallet_features) {
            if (version >= wf) return wf;
        }
        return static_cast<WalletFeature>(0);
        */
}
