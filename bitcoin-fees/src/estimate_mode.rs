crate::ix!();

/**
  | Used to determine type of fee estimation
  | requested
  |
  */
pub enum FeeEstimateMode {

    /**
      | Use default settings based on other
      | criteria
      |
      */
    UNSET,        

    /**
      | Force estimateSmartFee to use non-conservative
      | estimates
      |
      */
    ECONOMICAL,   

    /**
      | Force estimateSmartFee to use conservative
      | estimates
      |
      */
    CONSERVATIVE, 

    /**
      | Use BTC/kvB fee rate unit
      |
      */
    BTC_KVB,      

    /**
      | Use sat/vB fee rate unit
      |
      */
    SAT_VB,       
}

pub fn invalid_estimate_mode_error_message() -> String {
    
    todo!();
        /*
            return "Invalid estimate_mode parameter, must be one of: \"" + FeeModes("\", \"") + "\"";
        */
}

pub fn fee_mode_from_string(
    mode_string:       &String,
    fee_estimate_mode: &mut FeeEstimateMode) -> bool {
    
    todo!();
        /*
            auto searchkey = ToUpper(mode_string);
        for (const auto& pair : FeeModeMap()) {
            if (ToUpper(pair.first) == searchkey) {
                fee_estimate_mode = pair.second;
                return true;
            }
        }
        return false;
        */
}
