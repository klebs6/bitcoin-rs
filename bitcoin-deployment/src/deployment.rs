crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/deploymentinfo.h]

pub struct VBDeploymentInfo {

    /**
      | Deployment name
      |
      */
    name:      *const u8,

    /**
      | Whether GBT clients can safely ignore
      | this rule in simplified usage
      |
      */
    gbt_force: bool,
}

lazy_static!{
    /*
    extern const VBDeploymentInfo VersionBitsDeploymentInfo[consensus::MAX_VERSION_BITS_DEPLOYMENTS];
    */
}

#[inline] pub fn deployment_name_with_deployment_pos(pos: ConsensusDeploymentPos) -> String {
    
    todo!();
        /*
            assert(consensus::ValidDeployment(pos));
        return VersionBitsDeploymentInfo[pos].name;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/deploymentinfo.cpp]

lazy_static!{
    /*
    const struct VBDeploymentInfo VersionBitsDeploymentInfo[consensus::MAX_VERSION_BITS_DEPLOYMENTS] = {
        {
            /*.name =*/ "testdummy",
            /*.gbt_force =*/ true,
        },
        {
            /*.name =*/ "taproot",
            /*.gbt_force =*/ true,
        },
    };
    */
}

pub fn deployment_name(dep: ConsensusBuriedDeployment) -> String {
    
    todo!();
        /*

        assert(ValidDeployment(dep));

        // no default case, so the compiler can
        // warn about missing cases
        switch (dep) {
            case consensus::DEPLOYMENT_HEIGHTINCB:
                return "bip34";
            case consensus::DEPLOYMENT_CLTV:
                return "bip65";
            case consensus::DEPLOYMENT_DERSIG:
                return "bip66";
            case consensus::DEPLOYMENT_CSV:
                return "csv";
            case consensus::DEPLOYMENT_SEGWIT:
                return "segwit";
        } 
        return "";
        */
}

//-------------------------------------------[.cpp/bitcoin/src/deploymentstatus.h]

/**
  | Global cache for versionbits deployment
  | status
  |
  */
lazy_static!{
    /*
    extern VersionBitsCache g_versionbitscache;
    */
}

/**
  | Determine if a deployment is active
  | for the next block
  |
  */
#[inline] pub fn next_deployment_active_after_with_buried_deployment(
        pindex_prev: *const BlockIndex,
        params:      &ChainConsensusParams,
        dep:         ConsensusBuriedDeployment) -> bool {
    
    todo!();
        /*
            assert(consensus::ValidDeployment(dep));
        return (pindexPrev == nullptr ? 0 : pindexPrev->nHeight + 1) >= params.DeploymentHeight(dep);
        */
}

#[inline] pub fn next_deployment_active_after_with_deployment_pos(
        pindex_prev: *const BlockIndex,
        params:      &ChainConsensusParams,
        dep:         ConsensusDeploymentPos) -> bool {
    
    todo!();
        /*
        assert(consensus::ValidDeployment(dep));
        return ThresholdState::ACTIVE == g_versionbitscache.State(pindexPrev, params, dep);
        */
}

/**
  | Determine if a deployment is enabled
  | (can ever be active)
  |
  */
#[inline] pub fn deployment_enabled_with_buried_deployment(
        params: &ChainConsensusParams,
        dep:    ConsensusBuriedDeployment) -> bool {
    
    todo!();
        /*
        assert(consensus::ValidDeployment(dep));
        return params.DeploymentHeight(dep) != std::numeric_limits<int>::max();
        */
}

#[inline] pub fn deployment_enabled_with_deployment_pos(
        params: &ChainConsensusParams,
        dep:    ConsensusDeploymentPos) -> bool {
    
    todo!();
        /*
        assert(consensus::ValidDeployment(dep));
        return params.vDeployments[dep].nStartTime != consensus::BIP9Deployment::NEVER_ACTIVE;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/deploymentstatus.cpp]

lazy_static!{
    /*
    VersionBitsCache g_versionbitscache;
    */
}

/**
  | Basic sanity checking for BuriedDeployment/DeploymentPos
  | enums and
  | 
  | ValidDeployment check
  |
  */
const_assert!{
    valid_deployment_with_deployment_pos(ConsensusDeploymentPos::DEPLOYMENT_TESTDUMMY)
}

/**
  | sanity check of DeploymentPos failed
  | (MAX value considered valid)
  |
  */
const_assert!{
    !valid_deployment_with_deployment_pos(ConsensusDeploymentPos::MAX_VERSION_BITS_DEPLOYMENTS)
}

/**
  | sanity check of BuriedDeployment failed
  | (overlaps with DeploymentPos)
  |
  */
lazy_static!{//TODO: how can we check this as const?
    /*
    const_assert!{
        !valid_deployment_with_buried_deployment(ConsensusDeploymentPos::DEPLOYMENT_TESTDUMMY as consensus::BuriedDeployment)
    }
    */
}

/**
  | ValidDeployment only checks upper
  | bounds for ensuring validity.
  | 
  | This checks that the lowest possible
  | value or the type is also a (specific)
  | valid deployment so that lower bounds
  | don't need to be checked.
  |
  */
pub const fn is_minimum_deployment_heightincb_for_buried_deployment() -> bool {
    true//TODO uncomment the block and use to check
        /*
           todo!();
           is_minimum::<consensus::BuriedDeployment, ConsensusDeploymentPos::DEPLOYMENT_HEIGHTINCB>()
           using U = typename std::underlying_type<T>::type;
           return x == std::numeric_limits<U>::min();
           */
}

pub const fn is_minimum_deployment_testdummy_for_deployment_pos() -> bool {
    true//TODO uncomment the block and use to check
        /*
           todo!();
           is_minimum::<ConsensusDeploymentPos, ConsensusDeploymentPos::DEPLOYMENT_TESTDUMMY>()
           using U = typename std::underlying_type<T>::type;
           return x == std::numeric_limits<U>::min();
           */
}

const_assert!{
    is_minimum_deployment_heightincb_for_buried_deployment()
} //"heightincb is not minimum value for ConsensusBuriedDeployment"

const_assert!{
    is_minimum_deployment_testdummy_for_deployment_pos()
} //"testdummy is not minimum value for DeploymentPos"
