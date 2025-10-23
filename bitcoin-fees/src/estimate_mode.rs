// ---------------- [ File: bitcoin-fees/src/estimate_mode.rs ]
crate::ix!();

/**
  | Used to determine type of fee estimation
  | requested
  |
  */
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
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

pub fn fee_mode_map() -> &'static Vec<(String, FeeEstimateMode)> {
    use once_cell::sync::Lazy;
    static MODES: Lazy<Vec<(String, FeeEstimateMode)>> = Lazy::new(|| {
        vec![
            ("unset".to_string(),       FeeEstimateMode::UNSET),
            ("economical".to_string(),  FeeEstimateMode::ECONOMICAL),
            ("conservative".to_string(),FeeEstimateMode::CONSERVATIVE),
        ]
    });
    &MODES
}

pub fn fee_modes(delimiter: &str) -> String {
    fee_mode_map()
        .iter()
        .map(|(s, _)| s.as_str())
        .collect::<Vec<_>>()
        .join(delimiter)
}

pub fn invalid_estimate_mode_error_message() -> String {
    // "Invalid estimate_mode parameter, must be one of: \"" + FeeModes("\", \"") + "\""
    format!(
        "Invalid estimate_mode parameter, must be one of: \"{}\"",
        fee_modes(&"\", \"".to_string())
    )
}

pub fn fee_mode_from_string(
    mode_string:       &str,
    fee_estimate_mode: &mut FeeEstimateMode,
) -> bool {
    // auto searchkey = ToUpper(mode_string); match against FeeModeMap
    let key = mode_string.to_ascii_lowercase();
    for (name, mode) in fee_mode_map().iter() {
        if name.eq_ignore_ascii_case(&key) {
            *fee_estimate_mode = *mode;
            return true;
        }
    }
    false
}
