crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/chainparams.h]
//-------------------------------------------[.cpp/bitcoin/src/chainparams.cpp]

/**
  | Holds various statistics on transactions
  | within a chain. Used to estimate verification
  | progress during chain sync.
  | 
  | See also: CChainParams::TxData,
  | 
  | GuessVerificationProgress.
  |
  */
#[derive(Default)]
pub struct ChainTxData {

    /**
      | UNIX timestamp of last known number
      | of transactions
      |
      */
    n_time:     i64,

    /**
      | total number of transactions between
      | genesis and that timestamp
      |
      */
    n_tx_count: i64,

    /**
      | estimated number of transactions per
      | second after that timestamp
      |
      */
    d_tx_rate:  f64,
}

/**
  | ChainParams defines various tweakable
  | parameters of a given instance of the
  | Bitcoin system.
  |
  */
#[derive(Default)]
pub struct ChainParams {
    consensus:                  ChainConsensusParams,
    pch_message_start:          MessageHeaderMessageStartChars,
    n_default_port:             u16,
    n_prune_after_height:       u64,
    assumed_blockchain_size:    u64,
    assumed_chain_state_size:   u64,
    seeds:                      Vec<String>,
    base_58prefixes:            [Vec<u8>; chain_params::Base58Type::MAX_BASE58_TYPES as usize],
    bech32_hrp:                 String,
    str_networkid:              String,
    genesis:                    Block,
    fixed_seeds:                Vec<u8>,
    default_consistency_checks: bool,
    require_standard:           bool,
    is_test_chain:              bool,
    is_mockable_chain:          bool,
    checkpoint_data:            CheckpointData,
    assumeutxo_data:            MapAssumeUtxo,
    chain_tx_data:              ChainTxData,
}

pub mod chain_params {

    #[repr(usize)]
    pub enum Base58Type {
        PUBKEY_ADDRESS,
        SCRIPT_ADDRESS,
        SECRET_KEY,
        EXT_PUBLIC_KEY,
        EXT_SECRET_KEY,
        MAX_BASE58_TYPES
    }
}

impl ChainParams {

    pub fn get_consensus(&self) -> Arc<ChainConsensusParams> {
        
        todo!();
        /*
            return consensus;
        */
    }
    
    pub fn message_start(&self) -> &MessageHeaderMessageStartChars {
        
        todo!();
        /*
            return pchMessageStart;
        */
    }
    
    pub fn get_default_port(&self) -> u16 {
        
        todo!();
        /*
            return nDefaultPort;
        */
    }
    
    pub fn get_default_port_with_network(&self, net: Network) -> u16 {
        
        todo!();
        /*
            return net == NET_I2P ? I2P_SAM31_PORT : GetDefaultPort();
        */
    }
    
    pub fn get_default_port_from_addr(&self, addr: &String) -> u16 {
        
        todo!();
        /*
            CNetAddr a;
            return a.SetSpecial(addr) ? GetDefaultPort(a.GetNetwork()) : GetDefaultPort();
        */
    }
    
    pub fn genesis_block(&self) -> &Block {
        
        todo!();
        /*
            return genesis;
        */
    }

    /**
      | Default value for -checkmempool and
      | -checkblockindex argument
      |
      */
    pub fn default_consistency_checks(&self) -> bool {
        
        todo!();
        /*
            return fDefaultConsistencyChecks;
        */
    }

    /**
      | Policy: Filter transactions that do
      | not match well-defined patterns
      |
      */
    pub fn require_standard(&self) -> bool {
        
        todo!();
        /*
            return fRequireStandard;
        */
    }

    /**
      | If this chain is exclusively used for
      | testing
      |
      */
    pub fn is_test_chain(&self) -> bool {
        
        todo!();
        /*
            return m_is_test_chain;
        */
    }

    /**
      | If this chain allows time to be mocked
      |
      */
    pub fn is_mockable_chain(&self) -> bool {
        
        todo!();
        /*
            return m_is_mockable_chain;
        */
    }
    
    pub fn prune_after_height(&self) -> u64 {
        
        todo!();
        /*
            return nPruneAfterHeight;
        */
    }

    /**
      | Minimum free space (in GB) needed for
      | data directory
      |
      */
    pub fn assumed_blockchain_size(&self) -> u64 {
        
        todo!();
        /*
            return m_assumed_blockchain_size;
        */
    }

    /**
      | Minimum free space (in GB) needed for
      | data directory when pruned; Does not
      | include prune target
      |
      */
    pub fn assumed_chain_state_size(&self) -> u64 {
        
        todo!();
        /*
            return m_assumed_chain_state_size;
        */
    }

    /**
      | Whether it is possible to mine blocks
      | on demand (no retargeting)
      |
      */
    pub fn mine_blocks_on_demand(&self) -> bool {
        
        todo!();
        /*
            return consensus.fPowNoRetargeting;
        */
    }

    /**
      | Return the network string
      |
      */
    pub fn network_id_string(&self) -> String {
        
        todo!();
        /*
            return strNetworkID;
        */
    }

    /**
      | Return the list of hostnames to look
      | up for DNS seeds
      |
      */
    pub fn dns_seeds(&self) -> &Vec<String> {
        
        todo!();
        /*
            return vSeeds;
        */
    }
    
    pub fn base_58prefix(&self, ty: chain_params::Base58Type) -> &Vec<u8> {
        
        todo!();
        /*
            return base58Prefixes[type];
        */
    }
    
    pub fn bech32hrp(&self) -> &String {
        
        todo!();
        /*
            return bech32_hrp;
        */
    }
    
    pub fn fixed_seeds(&self) -> &Vec<u8> {
        
        todo!();
        /*
            return vFixedSeeds;
        */
    }
    
    pub fn checkpoints(&self) -> &CheckpointData {
        
        todo!();
        /*
            return checkpointData;
        */
    }

    /**
      | Get allowed assumeutxo configuration.
      | @see ChainstateManager
      |
      */
    pub fn assumeutxo(&self) -> &MapAssumeUtxo {
        
        todo!();
        /*
            return m_assumeutxo_data;
        */
    }
    
    pub fn tx_data(&self) -> &ChainTxData {
        
        todo!();
        /*
            return chainTxData;
        */
    }
}
