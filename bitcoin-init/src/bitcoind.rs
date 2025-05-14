// ---------------- [ File: bitcoin-init/src/bitcoind.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/init/bitcoind.cpp]

pub struct BitcoindInit {
    node: Rc<RefCell<NodeContext>>,
}

impl Init for BitcoindInit {
    fn init(&mut self) -> bool { todo!(); }
}

impl BitcoindInit {

    pub fn new(node: &mut NodeContext) -> Self {
    
        todo!();
        /*
        : node(node),

            m_node.args = &gArgs;
            m_node.init = this;
        */
    }
    
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
