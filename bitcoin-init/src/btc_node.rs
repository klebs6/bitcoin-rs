// ---------------- [ File: bitcoin-init/src/btc_node.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/init/bitcoin-node.cpp]

pub const EXE_NAME: &'static str = "bitcoin-node";

pub struct BitcoinNodeInit {
    node: NodeContext,
    ipc:  Box<dyn Ipc>,
}

impl Init for BitcoinNodeInit {
    fn init(&mut self) -> bool { todo!(); }
}

impl BitcoinNodeInit {

    pub fn new(
        node: &mut NodeContext,
        arg0: *const u8) -> Self {
    
        todo!();
        /*


            : m_node(node),
                  m_ipc(interfaces::MakeIpc(EXE_NAME, arg0, *this))

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
    
    pub fn ipc(&mut self) -> *mut dyn Ipc {
        
        todo!();
        /*
            return m_ipc.get();
        */
    }
}
