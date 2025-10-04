// ---------------- [ File: bitcoinchain-params/src/create_select.rs ]
crate::ix!();

/// Global accessor for the currently selected active chain parameters (`Params()` in C++).
///
/// Until the full network‑selection logic is ported, we expose a **static
/// main‑network** instance with `n_default_port = 8333`.  
/// This satisfies callers such as `get_listen_port()` without blocking
/// incremental translation work.
pub fn params() -> &'static ChainParams {
    static GLOBAL_CHAIN_PARAMS: Lazy<ChainParams> = Lazy::new(|| {
        let mut p = ChainParams::default();
        p.n_default_port = 8333;
        debug!("create_select::params – using MAIN defaults");
        p
    });
    &GLOBAL_CHAIN_PARAMS
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
