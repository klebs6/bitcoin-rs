/**
  | अनित्य अनात्मन्  मोक्ष 
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/scriptpubkeyman.h]

/**
  | Default for -keypool
  |
  */
pub const DEFAULT_KEYPOOL_SIZE: u32 = 1000;

/** 
 | A key from a CWallet's keypool
 |
 | The wallet holds one (for pre HD-split wallets)
 | or several keypools. These are sets of keys
 | that have not yet been used to provide
 | addresses or receive change.
 |
 | The Bitcoin Core wallet was originally
 | a collection of unrelated private keys with
 | their associated addresses. If a non-HD wallet
 | generated a key/address, gave that address out
 | and then restored a backup from before that
 | key's generation, then any funds sent to that
 | address would be lost definitively.
 |
 | The keypool was implemented to avoid this
 | scenario (commit: 10384941). The wallet would
 | generate a set of keys (100 by default). When
 | a new public key was required, either to give
 | out as an address or to use in a change output,
 | it would be drawn from the keypool. The keypool
 | would then be topped up to maintain 100
 | keys. This ensured that as long as the wallet
 | hadn't used more than 100 keys since the
 | previous backup, all funds would be safe, since
 | a restored wallet would be able to scan for all
 | owned addresses.
 |
 | A keypool also allowed encrypted wallets to
 | give out addresses without having to be
 | decrypted to generate a new private key.
 |
 | With the introduction of HD wallets (commit:
 | f1902510), the keypool essentially became an
 | address look-ahead pool. Restoring old backups
 | can no longer definitively lose funds as long
 | as the addresses used were from the wallet's HD
 | seed (since all private keys can be rederived
 | from the seed).  However, if many addresses
 | were used since the backup, then the wallet may
 | not know how far ahead in the HD chain to look
 | for its addresses. The keypool is used to
 | implement a 'gap limit'. The keypool maintains
 | a set of keys (by default 1000) ahead of the
 | last used key and scans for the addresses of
 | those keys.  This avoids the risk of not seeing
 | transactions involving the wallet's addresses,
 | or of re-using the same address.  In the
 | unlikely case where none of the addresses in
 | the `gap limit` are used on-chain, the
 | look-ahead will not be incremented to keep
 | a constant size and addresses beyond this range
 | will not be detected by an old backup. For this
 | reason, it is not recommended to decrease
 | keypool size lower than default value.
 |
 | The HD-split wallet feature added a second
 | keypool (commit: 02592f4c). There is an
 | external keypool (for addresses to hand out)
 | and an internal keypool (for change addresses).
 |
 | Keypool keys are stored in the
 | wallet/keystore's keymap. The keypool data is
 | stored as sets of indexes in the wallet
 | (setInternalKeyPool, setExternalKeyPool and
 | set_pre_split_keypool), and a map from the key
 | to the index (m_pool_key_to_index). The
 | CKeyPool object is used to
 | serialize/deserialize the pool data to/from the
 | database.
 */
pub struct KeyPool {

    /**
      | The time at which the key was generated.
      | Set in AddKeypoolPubKeyWithDB
      |
      */
    n_time:      i64,

    /**
      | The public key
      |
      */
    vch_pub_key: PubKey,

    /**
      | Whether this keypool entry is in the
      | internal keypool (for change outputs)
      |
      */
    internal:    bool,

    /**
      | Whether this key was generated for a
      | keypool before the wallet was upgraded
      | to HD-split
      |
      */
    pre_split:   bool,
}

impl KeyPool {
    
    pub fn new(
        vch_pub_key_in: &PubKey,
        internal_in:    bool) -> Self {
    
        todo!();
        /*

        
        */
    }
    
    pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            int nVersion = s.GetVersion();
            if (!(s.GetType() & SER_GETHASH)) {
                s << nVersion;
            }
            s << nTime << vchPubKey << fInternal << m_pre_split;
        */
    }
    
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            int nVersion = s.GetVersion();
            if (!(s.GetType() & SER_GETHASH)) {
                s >> nVersion;
            }
            s >> nTime >> vchPubKey;
            try {
                s >> fInternal;
            } catch (std::ios_base::failure&) {
                /* flag as external address if we can't read the internal boolean
                   (this will be the case for any wallet before the HD chain split version) */
                fInternal = false;
            }
            try {
                s >> m_pre_split;
            } catch (std::ios_base::failure&) {
                /* flag as postsplit address if we can't read the m_pre_split boolean
                   (this will be the case for any wallet that upgrades to HD chain split) */
                m_pre_split = false;
            }
        */
    }
}
