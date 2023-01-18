crate::ix!();

/**
  | Template for capturing information
  | about block/transaction validation.
  | This is instantiated by TxValidationState
  | and BlockValidationState for validation
  | information on transactions and blocks
  | respectively.
  |
  */
#[derive(Default)]
pub struct ValidationState<R> {
    mode:          validation_state::ModeState, //{ModeState::M_VALID};
    result:        R,
    reject_reason: String,
    debug_message: String,
}

pub mod validation_state {

    pub enum ModeState {

        /**
          | everything ok
          |
          */
        M_VALID,   

        /**
          | network rule violation (DoS value may
          | be set)
          |
          */
        M_INVALID, 

        /**
          | run-time error
          |
          */
        M_ERROR,   
    }

    impl Default for ModeState {
        fn default() -> Self {
            ModeState::M_VALID
        }
    }
}

impl<R> ValidationState<R> {

    pub fn invalid(&mut self, 
        result:        R,
        reject_reason: Option<&str>,
        debug_message: Option<&str>) -> bool {

        let reject_reason: &str = reject_reason.unwrap_or("");
        let debug_message: &str = debug_message.unwrap_or("");

        todo!();
        /*
            m_result = result;
            m_reject_reason = reject_reason;
            m_debug_message = debug_message;
            if (m_mode != ModeState::M_ERROR) m_mode = ModeState::M_INVALID;
            return false;
        */
    }
    
    pub fn error(&mut self, reject_reason: &String) -> bool {
        
        todo!();
        /*
            if (m_mode == ModeState::M_VALID)
                m_reject_reason = reject_reason;
            m_mode = ModeState::M_ERROR;
            return false;
        */
    }
    
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return m_mode == ModeState::M_VALID;
        */
    }
    
    pub fn is_invalid(&self) -> bool {
        
        todo!();
        /*
            return m_mode == ModeState::M_INVALID;
        */
    }
    
    pub fn is_error(&self) -> bool {
        
        todo!();
        /*
            return m_mode == ModeState::M_ERROR;
        */
    }
    
    pub fn get_result(&self) -> R {
        
        todo!();
        /*
            return m_result;
        */
    }
    
    pub fn get_reject_reason(&self) -> String {
        
        todo!();
        /*
            return m_reject_reason;
        */
    }
    
    pub fn get_debug_message(&self) -> String {
        
        todo!();
        /*
            return m_debug_message;
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            if (IsValid()) {
                return "Valid";
            }

            if (!m_debug_message.empty()) {
                return m_reject_reason + ", " + m_debug_message;
            }

            return m_reject_reason;
        */
    }
}
