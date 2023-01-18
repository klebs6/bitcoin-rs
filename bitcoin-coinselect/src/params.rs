crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/coinselection.h]

/**
  | target minimum change amount
  |
  */
pub const MIN_CHANGE: Amount = COIN / 100;

/**
  | final minimum change amount after paying
  | for fees
  |
  */
pub const MIN_FINAL_CHANGE: Amount = MIN_CHANGE / 2;

/**
  | Parameters for one iteration of Coin
  | Selection.
  |
  */
pub struct CoinSelectionParams {

    /**
      | Size of a change output in bytes, determined
      | by the output type.
      |
      */
    change_output_size:   usize, // default = 0

    /**
      | Size of the input to spend a change output
      | in virtual bytes.
      |
      */
    change_spend_size:    usize, // default = 0

    /**
      | Cost of creating the change output.
      |
      */
    change_fee:           Amount, // default = { 0 }

    /**
      | Cost of creating the change output +
      | cost of spending the change output in
      | the future.
      |
      */
    cost_of_change:       Amount, // default = { 0 }

    /**
      | The targeted feerate of the transaction
      | being built.
      |
      */
    effective_feerate:    FeeRate,

    /**
      | The feerate estimate used to estimate
      | an upper bound on what should be sufficient
      | to spend the change output sometime
      | in the future.
      |
      */
    long_term_feerate:    FeeRate,

    /**
      | If the cost to spend a change output at
      | the discard feerate exceeds its value,
      | drop it to fees.
      |
      */
    discard_feerate:      FeeRate,

    /**
      | Size of the transaction before coin
      | selection, consisting of the header
      | and recipient output(s), excluding
      | the inputs and change output(s).
      |
      */
    tx_noinputs_size:     usize, // default = 0

    /**
      | Indicate that we are subtracting the
      | fee from outputs
      |
      */
    subtract_fee_outputs: bool, // default = false

    /**
      | When true, always spend all (up to OUTPUT_GROUP_MAX_ENTRIES)
      | or none of the outputs associated with
      | the same address. This helps reduce
      | privacy leaks resulting from address
      | reuse. Dust outputs are not eligible
      | to be added to output groups and thus
      | not considered.
      |
      */
    avoid_partial_spends: bool, // default = false
}

impl CoinSelectionParams {

    pub fn new(
        change_output_size: usize,
        change_spend_size:  usize,
        effective_feerate:  FeeRate,
        long_term_feerate:  FeeRate,
        discard_feerate:    FeeRate,
        tx_noinputs_size:   usize,
        avoid_partial:      bool) -> Self {
    
        todo!();
        /*
            :
            change_output_size(change_output_size),
            change_spend_size(change_spend_size),
            m_effective_feerate(effective_feerate),
            m_long_term_feerate(long_term_feerate),
            m_discard_feerate(discard_feerate),
            tx_noinputs_size(tx_noinputs_size),
            m_avoid_partial_spends(avoid_partial)
        */
    }
}
