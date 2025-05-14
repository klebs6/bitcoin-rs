// ---------------- [ File: bitcoin-psbt/src/count.rs ]
crate::ix!();

/**
  | Counts the unsigned inputs of a PSBT.
  |
  */
pub fn count_psbt_unsigned_inputs(psbt: &PartiallySignedTransaction) -> usize {
    
    let mut count: usize = 0;

    for input in psbt.inputs.iter() {

        if !psbt_input_signed(input) {
            count += 1;
        }
    }

    count
}
