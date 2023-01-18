crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/init/bitcoin-gui.cpp]

pub const EXE_NAME: &'static str = "bitcoin-gui";

pub struct BitcoinGuiInit {
    node: NodeContext,
    ipc:  Box<dyn Ipc>,
}

impl Init for BitcoinGuiInit {
    fn init(&mut self) -> bool { todo!(); }
}

impl BitcoinGuiInit {

    pub fn new(arg0: *const u8) -> Self {
    
        todo!();
        /*


            : m_ipc(interfaces::MakeIpc(EXE_NAME, arg0, *this))
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

pub fn make_gui_init(
        argc: i32,
        argv: &[*mut u8]) -> Box<dyn Init> {
    
    todo!();
        /*
            return std::make_unique<init::BitcoinGuiInit>(argc > 0 ? argv[0] : "");
        */
}
