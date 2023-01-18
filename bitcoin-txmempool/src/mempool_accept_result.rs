crate::ix!();

/**
  | Validation result for a single transaction
  | mempool acceptance.
  |
  */
pub struct MempoolAcceptResult {

    pub result_type: MempoolAcceptResultType,
    pub state:       TxValidationState,

    /*
      | The following fields are only present
      | when m_result_type = ResultType::VALID
      |
      */

    /**
      | Mempool transactions replaced by the
      | tx per BIP 125 rules.
      |
      */
    pub replaced_transactions: Option<LinkedList<TransactionRef>>,

    /**
      | Raw base fees in satoshis.
      |
      */
    pub base_fees:             Option<Amount>,
}

/**
  | Used to indicate the results of mempool
  | validation.
  |
  */
#[derive(PartialEq,Eq,Clone,Debug)]
pub enum MempoolAcceptResultType {

    /**
      | Fully validated, valid.
      |
      */
    VALID, 

    /**
      | Invalid.
      |
      */
    INVALID, 
}

impl MempoolAcceptResult {

    
    pub fn failure(state: TxValidationState) -> MempoolAcceptResult {
        
        todo!();
        /*
            return MempoolAcceptResult(state);
        */
    }
    
    pub fn success(
        replaced_txns: LinkedList<TransactionRef>,
        fees:          Amount) -> MempoolAcceptResult {
        
        todo!();
        /*
            return MempoolAcceptResult(std::move(replaced_txns), fees);
        */
    }

    /*
      | Private constructors. Use static methods
      | 
      | MempoolAcceptResult::Success, etc.
      | to construct.
      |
      */

    /**
      | Constructor for failure case
      |
      */
    pub fn new_failure_case(state: TxValidationState) -> Self {
    
        todo!();
        /*
           : m_result_type(ResultType::INVALID), m_state(state) 
           Assume(!state.IsValid()); // Can be invalid or error
           */
    }

    /**
      | Constructor for success case
      |
      */
    pub fn new_success_case(
        replaced_txns: LinkedList<TransactionRef>,
        fees:          Amount) -> Self {
    
        todo!();
        /*
           : m_result_type(ResultType::VALID),
           m_replaced_transactions(std::move(replaced_txns)), m_base_fees(fees)
           */
    }
}

/**
  | Validation result for package mempool
  | acceptance.
  |
  */
pub struct PackageMempoolAcceptResult {

    state:      PackageValidationState,

    /**
      | Map from wtxid to finished MempoolAcceptResults.
      | The client is responsible for keeping
      | track of the transaction objects themselves.
      | If a result is not present, it means validation
      | was unfinished for that transaction.
      | If there was a package-wide error (see
      | result in m_state), m_tx_results will
      | be empty.
      |
      */
    tx_results: HashMap<u256,MempoolAcceptResult>,
}

impl PackageMempoolAcceptResult {
    
    pub fn new_with_state_and_results(
        state:   PackageValidationState,
        results: HashMap<u256,MempoolAcceptResult>) -> Self {
    
        todo!();
        /*
           : m_state{state}, m_tx_results(std::move(results))
        */
    }

    /**
      | Constructor to create a PackageMempoolAcceptResult
      | from a single MempoolAcceptResult
      |
      */
    pub fn new_with_wtxid_and_result(
        wtxid:  &u256,
        result: &MempoolAcceptResult) -> Self {
    
        todo!();
        /*
            : m_tx_results{ {wtxid, result} }
        */
    }
}

