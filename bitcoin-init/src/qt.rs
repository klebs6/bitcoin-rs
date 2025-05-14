// ---------------- [ File: bitcoin-init/src/qt.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/init/bitcoin-qt.cpp]

pub struct BitcoinQtInit {
    node: NodeContext,
}

impl Init for BitcoinQtInit {
    fn init(&mut self) -> bool { todo!(); }
}

impl Default for BitcoinQtInit {
    
    fn default() -> Self {
        todo!();
        /*


            m_node.args = &gArgs;
            m_node.init = this;
        */
    }
}

impl BitcoinQtInit {
    
    pub fn make_node(&mut self) -> Box<dyn NodeInterface> {
        
        todo!();
        /*
            return interfaces::MakeNode(m_node);
        */
    }
    
    pub fn make_chain(&mut self) -> Box<dyn ChainInterface> {
        
        todo!();
        /*
            return interfaces::MakeChain(m_node);
        */
    }
    
    pub fn make_wallet_client(&mut self, chain: &mut dyn ChainInterface) -> Box<dyn WalletClient> {
        
        todo!();
        /*
            return MakeWalletClient(chain, *Assert(m_node.args));
        */
    }
    
    pub fn make_echo(&mut self) -> Box<dyn Echo> {
        
        todo!();
        /*
            return interfaces::MakeEcho();
        */
    }
}

pub fn make_gui_init(
        argc: i32,
        argv: &[*mut u8]) -> Box<dyn Init> {
    
    todo!();
        /*
            return std::make_unique<init::BitcoinQtInit>();
        */
}
