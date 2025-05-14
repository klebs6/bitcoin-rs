// ---------------- [ File: bitcoinchain-params/src/create_select.rs ]
crate::ix!();

lazy_static!{
    /*
    static std::unique_ptr<const CChainParams> globalChainParams;
    */
}

/**
  | Return the currently selected parameters.
  | This won't change after app startup,
  | except for unit tests.
  |
  */
pub fn params() -> &'static ChainParams {
    
    todo!();
        /*
            assert(globalChainParams);
        return *globalChainParams;
        */
}

/**
  | Creates and returns a std::unique_ptr<CChainParams>
  | of the chosen chain.
  | 
  | 
  | -----------
  | @return
  | 
  | a CChainParams* of the chosen chain.
  | @throws a std::runtime_error if the
  | chain is not supported.
  |
  */
pub fn create_chain_params(
        args:  &ArgsManager,
        chain: &String) -> Box<ChainParams> {
    
    todo!();
        /*
            if (chain == CBaseChainParams::MAIN) {
            return std::unique_ptr<CChainParams>(new CMainParams());
        } else if (chain == CBaseChainParams::TESTNET) {
            return std::unique_ptr<CChainParams>(new CTestNetParams());
        } else if (chain == CBaseChainParams::SIGNET) {
            return std::unique_ptr<CChainParams>(new SigNetParams(args));
        } else if (chain == CBaseChainParams::REGTEST) {
            return std::unique_ptr<CChainParams>(new CRegTestParams(args));
        }
        throw std::runtime_error(strprintf("%s: Unknown chain %s.", __func__, chain));
        */
}

/**
  | Sets the params returned by Params()
  | to those for the given chain name. @throws
  | std::runtime_error when the chain
  | is not supported.
  |
  */
pub fn select_params(network: &String)  {
    
    todo!();
        /*
            SelectBaseParams(network);
        globalChainParams = CreateChainParams(gArgs, network);
        */
}
