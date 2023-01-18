crate::ix!();

/**
  | Decode a base64ed PSBT into a PartiallySignedTransaction
  |
  */
pub fn decode_base64psbt(
        psbt:      &mut PartiallySignedTransaction,
        base64_tx: &str,
        error:     &mut String) -> bool {
    
    let mut invalid: bool = false;

    let tx_data: String = decode_base64(base64_tx,Some(&mut invalid));

    if invalid {
        *error = "invalid base64".to_string();
        return false;
    }

    decode_rawpsbt(psbt,&tx_data,error)
}

/**
  | Decode a raw (binary blob) PSBT into
  | a PartiallySignedTransaction
  |
  */
pub fn decode_rawpsbt(
        psbt:  &mut PartiallySignedTransaction,
        tx_data:   &str,
        mut error: &str) -> bool {
    
    let mut ss_data: DataStream 
    = DataStream::new_with_slice(
        tx_data.as_bytes(), 
        SER_NETWORK as i32, 
        PROTOCOL_VERSION
    );

    let mut try_block = || -> TryBlockResult::<_,&'static str> {

        ss_data.stream(&psbt);

        if !ss_data.empty() {
            error = "extra data after PSBT";
            return TryBlockResult::Return(false);
        }

        TryBlockResult::Success
    };

    match try_block() {
        TryBlockResult::Return(v)  => return v,
        TryBlockResult::Err(e)  => {
            error = e;
            return false;
        }
        TryBlockResult::Success => { }
        TryBlockResult::Break   => { }
    }

    true
}
