crate::ix!();

lazy_static!{
    pub static ref MIN_TRANSACTION_OUTPUT_WEIGHT: usize = WITNESS_SCALE_FACTOR * get_serialize_size(&DEFAULT_TX_OUT, Some(PROTOCOL_VERSION));
    pub static ref MAX_OUTPUTS_PER_BLOCK:         usize = MAX_BLOCK_WEIGHT / *MIN_TRANSACTION_OUTPUT_WEIGHT;
}
