crate::ix!();

/**
  | Undo information for a CTransaction
  |
  */
pub struct TxUndo {

    /**
      | undo information for all txins
      |
      */
    pub vprevout: Vec<Coin>,
}

pub fn tx_to_univ(
        tx:              &Transaction,
        hash_block:      &u256,
        entry:           &mut UniValue,
        include_hex:     Option<bool>,
        serialize_flags: Option<i32>,
        txundo:          *const TxUndo,
        verbosity:       Option<TxVerbosity>)  {

    let include_hex:            bool = include_hex.unwrap_or(true);
    let serialize_flags:         i32 = serialize_flags.unwrap_or(0);
    let verbosity:       TxVerbosity = verbosity.unwrap_or(TxVerbosity::SHOW_DETAILS);

    todo!();
        /*
        
        */
}
