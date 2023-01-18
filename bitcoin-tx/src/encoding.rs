crate::ix!();

pub fn decode_hex_tx(
        tx:             &mut MutableTransaction,
        hex_tx:         &String,
        try_no_witness: Option<bool>,
        try_witness:    Option<bool>) -> bool {

    let try_no_witness: bool = try_no_witness.unwrap_or(false);
    let try_witness:    bool = try_witness.unwrap_or(true);

    todo!();
        /*
        
        */
}

pub fn encode_hex_tx(
        tx:              &Transaction,
        serialize_flags: Option<i32>) -> String {
    let serialize_flags: i32 = serialize_flags.unwrap_or(0);

    todo!();
        /*
        
        */
}


pub fn encode_destination(dest: &TxDestination) -> String {
    
    todo!();
        /*
            return std::visit(DestinationEncoder(Params()), dest);
        */
}

/**
  | Verbose level for block's transaction
  |
  */
pub enum TxVerbosity {

    /**
      Only TXID for each block's transaction
      */
    SHOW_TXID,                

    /**
      Include TXID, inputs, outputs, and other
      common block's transaction information
      */
    SHOW_DETAILS,             

    /**
      The same as previous option with information
      about prevouts if available
      */
    SHOW_DETAILS_AND_PREVOUT,  
}
