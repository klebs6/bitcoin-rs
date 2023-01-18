crate::ix!();

/**
  | @see ChainState::FlushStateToDisk
  |
  */
pub enum FlushStateMode {
    NONE,
    IF_NEEDED,
    PERIODIC,
    ALWAYS
}

pub enum DisconnectResult {

    /**
      | All good.
      |
      */
    DISCONNECT_OK,      

    /**
      | Rolled back, but UTXO set was inconsistent
      | with block.
      |
      */
    DISCONNECT_UNCLEAN, 

    /**
      | Something else went wrong.
      |
      */
    DISCONNECT_FAILED,   
}
