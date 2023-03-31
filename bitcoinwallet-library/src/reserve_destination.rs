crate::ix!();

/**
  | A wrapper to reserve an address from
  | a wallet
  | 
  | ReserveDestination is used to reserve
  | an address.
  | 
  | It is currently only used inside of CreateTransaction.
  | 
  | Instantiating a ReserveDestination
  | does not reserve an address. To do so,
  | 
  | GetReservedDestination() needs to
  | be called on the object. Once an address
  | has been reserved, call KeepDestination()
  | on the ReserveDestination object to
  | make sure it is not returned. Call ReturnDestination()
  | to return the address so it can be re-used
  | (for example, if the address was used
  | in a new transaction and that transaction
  | was not completed and needed to be aborted).
  | 
  | If an address is reserved and KeepDestination()
  | is not called, then the address will
  | be returned when the ReserveDestination
  | goes out of scope.
  |
  */
pub struct ReserveDestination {

    /**
      | The wallet to reserve from
      |
      */
    pwallet:  *const Wallet,

    /**
      | The ScriptPubKeyMan to reserve from.
      | Based on type when GetReservedDestination
      | is called
      |
      */
    spk_man:  *mut ScriptPubKeyMan, // default = { nullptr }

    ty:       OutputType,

    /**
      | The index of the address's key in the
      | keypool
      |
      */
    n_index:  i64, // default = { -1 }

    /**
      | The destination
      |
      */
    address:  TxDestination,

    /**
      | Whether this is from the internal (change
      | output) keypool
      |
      */
    internal: bool, // default = { false }

}

impl Drop for ReserveDestination {

    /**
      | Destructor. If a key has been reserved
      | and not KeepKey'ed, it will be returned
      | to the keypool
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            ReturnDestination();
        */
    }
}

impl ReserveDestination {

    /**
      | Construct a ReserveDestination object.
      | This does NOT reserve an address yet
      |
      */
    pub fn new(
        pwallet: *mut Wallet,
        ty:      OutputType) -> Self {
    
        todo!();
        /*
        : pwallet(pwallet),
        : ty(type),

        
        */
    }

    /**
      | Reserve an address
      |
      */
    pub fn get_reserved_destination(&mut self, 
        pubkey:   &mut TxDestination,
        internal: bool,
        error:    &mut BilingualStr) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Return reserved address
      |
      */
    pub fn return_destination(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Keep the address. Do not return it's
      | key to the keypool when this object goes
      | out of scope
      |
      */
    pub fn keep_destination(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
