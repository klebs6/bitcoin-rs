crate::ix!();

pub const fn valid_deployment_with_deployment_pos(dep: ConsensusDeploymentPos) -> bool {
    (dep as u16) < (ConsensusDeploymentPos::MAX_VERSION_BITS_DEPLOYMENTS as u16)
}

/**
  | Struct for each individual consensus
  | rule change using BIP9.
  |
  */
#[derive(Default)]
pub struct BIP9Deployment {

    /**
      | Bit position to select the particular
      | bit in nVersion.
      |
      */
    pub bit:                   i32,

    /**
      | Start MedianTime for version bits miner
      | confirmation. Can be a date in the past
      |
      */
    pub n_start_time:          i64,

    /**
      | Timeout/expiry MedianTime for the
      | deployment attempt.
      |
      */
    pub n_timeout:             i64,

    /**
      | If lock in occurs, delay activation
      | until at least this block height. Note
      | that activation will only occur on a
      | retarget boundary.
      |
      */
    pub min_activation_height: i32, // default = { 0 }
}

/**
  | Constant for nTimeout very far in the
  | future.
  |
  */
pub const BIP9_DEPLOYMENT_NO_TIMEOUT: i64 = i64::MAX;

/**
  | Special value for nStartTime indicating
  | that the deployment is always active.
  | 
  | This is useful for testing, as it means
  | tests don't need to deal with the activation
  | process (which takes at least 3 BIP9
  | intervals). Only tests that specifically
  | test the behaviour during activation
  | cannot use this.
  |
  */
pub const BIP9_DEPLOYMENT_ALWAYS_ACTIVE: i64 = -1;

/**
  | Special value for nStartTime indicating
  | that the deployment is never active.
  | 
  | This is useful for integrating the code
  | changes for a new feature prior to deploying
  | it on some or all networks.
  |
  */
pub const BIP9_DEPLOYMENT_NEVER_ACTIVE: i64 = -2;

pub type Deployments = [BIP9Deployment; ConsensusDeploymentPos::MAX_VERSION_BITS_DEPLOYMENTS as usize];
