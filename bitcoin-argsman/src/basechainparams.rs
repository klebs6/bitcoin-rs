// ---------------- [ File: bitcoin-argsman/src/basechainparams.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/chainparamsbase.h]

/**
  | BaseChainParams defines the base
  | parameters (shared between bitcoin-cli
  | and bitcoind) of a given instance of
  | the Bitcoin system.
  |
  */
#[derive(Default)]
pub struct BaseChainParams {
    pub rpc_port:                  u16,
    pub onion_service_target_port: u16,
    pub str_data_dir:              String,
}

pub mod base_chain_params {

    /**
      | Chain name strings
      |
      */
    pub const MAIN:    &'static str = "main";
    pub const TESTNET: &'static str = "test";
    pub const SIGNET:  &'static str = "signet";
    pub const REGTEST: &'static str = "regtest";
}

impl BaseChainParams {

    pub fn data_dir(&self) -> &String {
        
        &self.str_data_dir
    }
    
    pub fn rpc_port(&self) -> u16 {
        
       self.rpc_port
    }
    
    pub fn onion_service_target_port(&self) -> u16 {
        
        self.onion_service_target_port
    }
    
    pub fn new(
        data_dir:                  &str,
        rpc_port:                  u16,
        onion_service_target_port: u16) -> Self {
        Self {
            rpc_port:                  rpc_port,
            onion_service_target_port: onion_service_target_port,
            str_data_dir:              data_dir.to_string()
        }
    }
}

//-------------------------------------------[.cpp/bitcoin/src/chainparamsbase.cpp]

impl ArgsManagerInner {

    /**
      | Set the arguments for chainparams
      |
      */
    pub fn setup_chain_params_base_options(&mut self)  {

        self.add_arg(&base_options::ARG_SET_CHAIN);
        self.add_arg(&base_options::ARG_REGTEST);
        self.add_arg(&base_options::ARG_TESTACTIVATIONHEIGHT);
        self.add_arg(&base_options::ARG_TESTNET);
        self.add_arg(&base_options::ARG_SET_VBPARAMS);
        self.add_arg(&base_options::ARG_SIGNET);
        self.add_arg(&base_options::ARG_SIGNET_CHALLENGE);
        self.add_arg(&base_options::ARG_SIGNET_SEEDNODE);
    }
}

lazy_static!{
    static ref global_chain_base_params: Arc<Mutex<BaseChainParams>> = Arc::new(Mutex::new(BaseChainParams::default()));
}

/**
  | Return the currently selected parameters.
  | This won't change after app startup,
  | except for unit tests.
  |
  */
pub fn base_params() -> MutexGuard<'static, BaseChainParams> {
    
    global_chain_base_params.lock()
}

/**
  | Creates and returns
  | a std::unique_ptr<CBaseChainParams> of the
  | chosen chain.
  | 
  | -----------
  | @return
  | 
  | a CBaseChainParams* of the chosen chain.
  | @throws a std::runtime_error if the
  | chain is not supported.
  |
  -------------------------
  | Port numbers for incoming Tor connections
  | (8334, 18334, 38334, 18445) have been
  | chosen arbitrarily to keep ranges of
  | used ports tight.
  |
  */
pub fn create_base_chain_params(chain: &str) -> Result<Box<BaseChainParams>, StdException> {

    match chain {
        base_chain_params::MAIN => {
            Ok(Box::new(BaseChainParams::new("",8332,8334)))
        },
        base_chain_params::TESTNET => {
            Ok(Box::new(BaseChainParams::new("testnet3",18332,18334)))
        },
        base_chain_params::SIGNET => {
            Ok(Box::new(BaseChainParams::new("signet",38332,38334)))
        },
        base_chain_params::REGTEST => {
            Ok(Box::new(BaseChainParams::new("regtest",18443,18445)))
        },
        _ => {
            let msg = format!("{}: Unknown chain {}.",func![],chain);
            Err(runtime_error(&msg))
        }
    }
}

/**
  | Sets the params returned by Params()
  | to those for the given network.
  |
  */
pub fn select_base_params(chain: &str)  {
    
    *base_params() = *create_base_chain_params(chain).unwrap();

    let guard = G_ARGS.lock();

    let mut inner = (*guard).cs_args.lock();

    inner.select_config_network(chain);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    static M: OnceLock<Mutex<()>> = OnceLock::new();
    fn lock() -> std::sync::MutexGuard<'static,()> { M.get_or_init(|| Mutex::new(())).lock().unwrap() }

    #[test]
    fn create_base_params_returns_expected_ports_and_dirs() {
        let _g = lock();
        let main = create_base_chain_params(base_chain_params::MAIN).unwrap();
        assert_eq!(main.rpc_port(), 8332);
        assert_eq!(main.onion_service_target_port(), 8334);
        assert_eq!(main.data_dir(), "");

        let test = create_base_chain_params(base_chain_params::TESTNET).unwrap();
        assert_eq!(test.rpc_port(), 18332);
        assert_eq!(test.data_dir(), "testnet3");
    }

    #[test]
    fn select_base_params_sets_globals_and_network() {
        let _g = lock();
        select_base_params(base_chain_params::REGTEST);
        assert_eq!(base_params().data_dir(), "regtest");

        // Also ensure ArgsManagerInner.network is set by select_base_params
        let am = G_ARGS.lock();
        let inner = am.cs_args.lock();
        assert_eq!(inner.network.as_deref(), Some(base_chain_params::REGTEST));
    }
}
