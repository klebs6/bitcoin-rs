// ---------------- [ File: bitcoin-chain-consensus/src/consensus_deployment.rs ]
crate::ix!();

/**
  | A buried deployment is one where the
  | height of the activation has been hardcoded
  | into the client implementation long
  | after the consensus change has activated.
  | See BIP 90.
  |
  */
#[repr(i16)]
#[derive(PartialEq,Eq,Ord,PartialOrd,Clone)]
pub enum ConsensusBuriedDeployment {

    /**
      | buried deployments get negative values
      | to avoid overlap with ConsensusDeploymentPos
      |
      */
    DEPLOYMENT_HEIGHTINCB = i16::MIN,
    DEPLOYMENT_CLTV,
    DEPLOYMENT_DERSIG,
    DEPLOYMENT_CSV,
    DEPLOYMENT_SEGWIT,
}

pub fn valid_deployment_with_buried_deployment(dep: ConsensusBuriedDeployment) -> bool {
    
    dep <= ConsensusBuriedDeployment::DEPLOYMENT_SEGWIT
}

#[repr(u16)]
pub enum ConsensusDeploymentPos {

    DEPLOYMENT_TESTDUMMY = 0,

    /**
      | Deployment of Schnorr/Taproot (BIPs
      | 340-342)
      |
      */
    DEPLOYMENT_TAPROOT = 1, 

    /**
       | @note
       | 
       | Also add new deployments to
       | 
       | VersionBitsDeploymentInfo in deploymentinfo.cpp
       |
       */
    MAX_VERSION_BITS_DEPLOYMENTS = 2,
}
