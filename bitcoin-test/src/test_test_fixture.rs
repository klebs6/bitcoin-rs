crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/wallet/test/wallet_test_fixture.h]
//-------------------------------------------[.cpp/bitcoin/src/wallet/test/wallet_test_fixture.cpp]

/**
  | Testing setup and teardown for wallet.
  |
  */
pub struct WalletTestingSetup {
    base: TestingSetup,

    /**
      | defualt = interfaces::MakeWalletClient(*m_node.chain,
      | *Assert(m_node.args));
      |
      */
    wallet_client:               Box<interfaces::WalletClient>,

    wallet:                      Wallet,
    chain_notifications_handler: Box<dyn Handler>,
}

impl WalletTestingSetup {
    
    pub fn new(chain_name: &String) -> Self {
        let chain_name: &String =
                 chain_name.unwrap_or(CBaseChainParams_MAIN);
        todo!();
        /*

        
        */
    }
    
    pub fn new(chain_name: &String) -> Self {
    
        todo!();
        /*


            : TestingSetup(chainName),
          m_wallet(m_node.chain.get(), "", CreateMockWalletDatabase())

        m_wallet.LoadWallet();
        m_chain_notifications_handler = m_node.chain->handleNotifications({ &m_wallet, [](CWallet*) {} });
        m_wallet_client->registerRpcs();
        */
    }
}
