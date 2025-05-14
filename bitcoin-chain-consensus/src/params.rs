// ---------------- [ File: bitcoin-chain-consensus/src/params.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/consensus/params.h]

/**
  | Parameters that influence chain consensus.
  |
  */
#[derive(Default)]
pub struct ChainConsensusParams {

    pub hash_genesis_block:                 u256,
    pub n_subsidy_halving_interval:         i32,

    /**
      | Block hash that is excepted from BIP16
      | enforcement
      |
      */
    pub bip16exception:                     u256,

    /**
      | Block height and hash at which BIP34
      | becomes active
      |
      */
    pub bip34height:                        i32,

    pub bip34hash:                          u256,

    /**
      | Block height at which BIP65 becomes
      | active
      |
      */
    pub bip65height:                        i32,

    /**
      | Block height at which BIP66 becomes
      | active
      |
      */
    pub bip66height:                        i32,

    /**
      | Block height at which CSV (BIP68, BIP112
      | and BIP113) becomes active
      |
      */
    pub csv_height:                         i32,

    /**
      | Block height at which Segwit (BIP141,
      | BIP143 and BIP147) becomes active.
      | 
      | -----------
      | @note
      | 
      | segwit v0 script rules are enforced
      | on all blocks except the
      | 
      | BIP 16 exception blocks.
      |
      */
    pub segwit_height:                      i32,

    /**
      | Don't warn about unknown BIP 9 activations
      | below this height.
      | 
      | This prevents us from warning about
      | the CSV and segwit activations.
      |
      */
    pub min_bip9warning_height:             i32,

    /**
      | Minimum blocks including miner confirmation
      | of the total of 2016 blocks in a retargeting
      | period, (nPowTargetTimespan / nPowTargetSpacing)
      | which is also used for BIP9 deployments.
      | 
      | Examples: 1916 for 95%, 1512 for testchains.
      |
      */
    pub n_rule_change_activation_threshold: u32,

    pub n_miner_confirmation_window:        u32,
    pub deployments:                        Deployments,

    /**
      | Proof of work parameters
      |
      */
    pub pow_limit:                          u256,

    pub pow_allow_min_difficulty_blocks:    bool,
    pub pow_no_retargeting:                 bool,
    pub n_pow_target_spacing:               i64,
    pub n_pow_target_timespan:              i64,

    /**
      | The best chain should have at least this
      | much work
      |
      */
    pub n_minimum_chain_work:               u256,

    /**
      | By default assume that the signatures
      | in ancestors of this block are valid
      |
      */
    pub default_assume_valid:               u256,

    /**
      | If true, witness commitments contain
      | a payload equal to a Bitcoin Script solution
      | to the signet challenge. See BIP325.
      |
      */
    pub signet_blocks:                      bool, // default = { false }

    pub signet_challenge:                   Vec<u8>,
}

impl ChainConsensusParams {

    pub fn difficulty_adjustment_interval(&self) -> i64 {
        self.n_pow_target_timespan / self.n_pow_target_spacing
    }
    
    pub fn deployment_height(&self, dep: ConsensusBuriedDeployment) -> i32 {

        match dep {
            ConsensusBuriedDeployment::DEPLOYMENT_HEIGHTINCB => return self.bip34height,
            ConsensusBuriedDeployment::DEPLOYMENT_CLTV       => return self.bip65height,
            ConsensusBuriedDeployment::DEPLOYMENT_DERSIG     => return self.bip66height,
            ConsensusBuriedDeployment::DEPLOYMENT_CSV        => return self.csv_height,
            ConsensusBuriedDeployment::DEPLOYMENT_SEGWIT     => return self.segwit_height,
        }

        i32::MAX
    }
}
