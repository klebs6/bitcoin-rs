crate::ix!();

/**
  | WalletContext struct containing references to
  | state shared between CWallet instances, like
  | the reference to the chain interface, and the
  | list of opened wallets.
  |
  | Future shared state can be added here as an
  | alternative to adding global variables.
  |
  | The struct isn't intended to have any member
  | functions. It should just be a collection of
  | state pointers that doesn't pull in
  | dependencies or implement behavior.
  */
pub struct WalletContext {

    chain:           Rc<RefCell<dyn ChainInterface>>, // default = { nullptr }

    /**
      | Currently a raw pointer because the
      | memory is not managed by this struct
      |
      */
    args:            Rc<RefCell<ArgsManager>>, // default = { nullptr }

    wallets_mutex:   std::sync::Mutex<WalletContextInner>,
}

pub struct WalletContextInner {
    wallets:         Vec<Arc<dyn WalletInterface>>,
    wallet_load_fns: LinkedList<LoadWalletFn>,
}

pub trait GetWalletContext {

    /**
      | Return pointer to internal context,
      | useful for testing.
      |
      */
    fn context(&mut self) -> *mut WalletContext {
        
        todo!();
        /*
            return nullptr;
        */
    }
}
