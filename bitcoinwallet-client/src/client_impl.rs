// ---------------- [ File: bitcoinwallet-client/src/client_impl.rs ]
crate::ix!();

///-----------------------
pub struct WalletClientImpl {
    context:          WalletContext,
    wallet_filenames: Vec<String>,
    rpc_handlers:     Vec<Box<dyn Handler>>,
    rpc_commands:     LinkedList<RPCCommand>,
}

impl WalletClient for WalletClientImpl { }

impl ChainClient for WalletClientImpl {}

impl GetWalletContext for WalletClientImpl {}

impl Drop for WalletClientImpl {
    fn drop(&mut self) {
        todo!();
        /*
            UnloadWallets(m_context);
        */
    }
}

/* -------------- ChainClient methods  -------------- */

impl RegisterRpcs for WalletClientImpl {

    fn register_rpcs(&mut self)  {
        
        todo!();
        /*
            for (const CRPCCommand& command : GetWalletRPCCommands()) {
                m_rpc_commands.emplace_back(command.category, command.name, [this, &command](const JSONRPCRequest& request, UniValue& result, bool last_handler) {
                    JSONRPCRequest wallet_request = request;
                    wallet_request.context = &m_context;
                    return command.actor(wallet_request, result, last_handler);
                }, command.argNames, command.unique_id);
                m_rpc_handlers.emplace_back(m_context.chain->handleRpc(m_rpc_commands.back()));
            }
        */
    }
}
    
impl Verify for WalletClientImpl {

    fn verify(&mut self) -> bool {
        
        todo!();
        /*
            return VerifyWallets(m_context);
        */
    }
}
    
impl Load for WalletClientImpl {

    fn load(&mut self) -> bool {
        
        todo!();
        /*
            return LoadWallets(m_context);
        */
    }
}
    
impl Start for WalletClientImpl {

    fn start(&mut self, scheduler: &mut Scheduler)  {
        
        todo!();
        /*
            return StartWallets(m_context, scheduler);
        */
    }
}
    
impl Flush for WalletClientImpl {

    fn flush(&mut self)  {
        
        todo!();
        /*
            return FlushWallets(m_context);
        */
    }
}
    
impl Stop for WalletClientImpl {

    fn stop(&mut self)  {
        
        todo!();
        /*
            return StopWallets(m_context);
        */
    }
}
    
impl SetMockTime for WalletClientImpl {

    fn set_mock_time(&mut self, time: i64)  {
        
        todo!();
        /*
            return SetMockTime(time);
        */
    }
}

impl CreateWallet for WalletClientImpl {

    /* ------------- WalletClient methods  ------------- */
    fn create_wallet(&mut self, 
        name:                  &String,
        passphrase:            &SecureString,
        wallet_creation_flags: u64,
        error:                 &mut BilingualStr,
        warnings:              &mut Vec<BilingualStr>) -> Box<dyn WalletInterface> {
        
        todo!();
        /*
            std::shared_ptr<CWallet> wallet;
            DatabaseOptions options;
            DatabaseStatus status;
            options.require_create = true;
            options.create_flags = wallet_creation_flags;
            options.create_passphrase = passphrase;
            return MakeWallet(m_context, CreateWallet(m_context, name, true /* load_on_start */, options, status, error, warnings));
        */
    }
}
    
impl WalletClientLoadWallet for WalletClientImpl {

    fn load_wallet(&mut self, 
        name:     &String,
        error:    &mut BilingualStr,
        warnings: &mut Vec<BilingualStr>) -> Box<dyn WalletInterface> {
        
        todo!();
        /*
            DatabaseOptions options;
            DatabaseStatus status;
            options.require_existing = true;
            return MakeWallet(m_context, LoadWallet(m_context, name, true /* load_on_start */, options, status, error, warnings));
        */
    }
}
    
impl GetWalletDir for WalletClientImpl {

    fn get_wallet_dir(&mut self) -> String {
        
        todo!();
        /*
            return fs::PathToString(GetWalletDir());
        */
    }
}
    
impl ListWalletDir for WalletClientImpl {

    fn list_wallet_dir(&mut self) -> Vec<String> {
        
        todo!();
        /*
            std::vector<std::string> paths;
            for (auto& path : ListDatabases(GetWalletDir())) {
                paths.push_back(fs::PathToString(path));
            }
            return paths;
        */
    }
}
    
impl GetWallets for WalletClientImpl {

    fn get_wallets(&mut self) -> Vec<Box<dyn WalletInterface>> {
        
        todo!();
        /*
            std::vector<std::unique_ptr<Wallet>> wallets;
            for (const auto& wallet : GetWallets(m_context)) {
                wallets.emplace_back(MakeWallet(m_context, wallet));
            }
            return wallets;
        */
    }
}
    
impl HandleLoadWallet for WalletClientImpl {

    fn handle_load_wallet(&mut self, fn_: LoadWalletFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return HandleLoadWallet(m_context, std::move(fn));
        */
    }
}
    
impl WalletClientImpl {

    pub fn new(
        chain: Amo<Box<dyn ChainInterface>>,
        args:  &mut ArgsManager) -> Self {
    
        todo!();
        /*


            m_context.chain = &chain;
            m_context.args = &args;
        */
    }

    pub fn context(&mut self) -> *mut WalletContext {
        
        todo!();
        /*
            return &m_context;
        */
    }
}

/**
  | Return implementation of ChainClient interface
  | for a wallet client. This function will be
  | undefined in builds where ENABLE_WALLET is
  | false.
  */
#[cfg(not(feature = "wallet"))]
pub fn make_wallet_client(
    chain: Amo<Box<dyn ChainInterface>>,
    args:  &mut ArgsManager) -> Box<dyn WalletClient> {
    
    todo!();
        /*
            throw std::logic_error("Wallet function called in non-wallet build.");
        */
}

#[cfg(feature = "wallet")]
pub fn make_wallet_client(
    chain: Amo<Box<dyn ChainInterface>>,
    args:  &mut ArgsManager) -> Box<WalletClient> {

    todo!();
        /*
            return std::make_unique<WalletClientImpl>(chain, args);
        */
}
