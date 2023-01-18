crate::ix!();

/**
  | wallet flags in the upper section (> 1 <<
  | 31) will lead to not opening the wallet if
  | flag is unknown unknown wallet flags in the
  | lower section <= (1 << 31) will be
  | tolerated
  */
bitflags!{

    pub struct WalletFlags: u64 {

        /*
          | will categorize coins as clean (not
          | reused) and dirty (reused), and handle
          | them with privacy considerations in
          | mind
          |
          */
        const WALLET_FLAG_AVOID_REUSE = 1 << 0; 

        /*
          | Indicates that the metadata has already
          | been upgraded to contain key origins
          |
          */
        const WALLET_FLAG_KEY_ORIGIN_METADATA = 1 << 1; 

        /*
          | Indicates that the descriptor cache
          | has been upgraded to cache last hardened
          | xpubs
          |
          */
        const WALLET_FLAG_LAST_HARDENED_XPUB_CACHED = 1 << 2; 

        /*
          | will enforce the rule that the wallet
          | can't contain any private keys (only
          | watch-only/pubkeys)
          |
          */
        const WALLET_FLAG_DISABLE_PRIVATE_KEYS = 1 << 32; 

        /*
          | Flag set when a wallet contains no HD seed
          | and no private keys, scripts, addresses,
          | and other watch only things, and is
          | therefore "blank."
          |
          | The only function this flag serves is to
          | distinguish a blank wallet from a newly
          | created wallet when the wallet database is
          | loaded, to avoid initialization that
          | should only happen on first run.
          |
          | This flag is also a mandatory flag to
          | prevent previous versions of bitcoin from
          | opening the wallet, thinking it was newly
          | created, and then improperly
          | reinitializing it.
          */
        const WALLET_FLAG_BLANK_WALLET = 1 << 33;

        /*
          | Indicate that this wallet supports
          | DescriptorScriptPubKeyMan
          |
          */
        const WALLET_FLAG_DESCRIPTORS = 1 << 34;

        /*
          | Indicates that the wallet needs an external
          | signer
          |
          */
        const WALLET_FLAG_EXTERNAL_SIGNER = 1 << 35;
    }
}
