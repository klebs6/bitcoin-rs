// ---------------- [ File: bitcoin-fees/src/fees.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/policy/fees.h]

/**
  | Enumeration of reason for returned
  | fee estimate
  |
  */
pub enum FeeReason {
    NONE,
    HALF_ESTIMATE,
    FULL_ESTIMATE,
    DOUBLE_ESTIMATE,
    CONSERVATIVE,
    MEMPOOL_MIN,
    PAYTXFEE,
    FALLBACK,
    REQUIRED,
}

pub struct FeeCalculation
{
    est:             EstimationResult,
    reason:          FeeReason, //= FeeReason::NONE;
    desired_target:  i32,       // default = 0
    returned_target: i32,       // default = 0
}

///------------------------
//-------------------------------------------[.cpp/bitcoin/src/policy/fees.cpp]

///------------------------

//-------------------------------------------[.cpp/bitcoin/src/util/fees.h]
//-------------------------------------------[.cpp/bitcoin/src/util/fees.cpp]

pub fn string_for_fee_reason(reason: FeeReason) -> String {
    
    todo!();
        /*
            static const std::map<FeeReason, std::string> fee_reason_strings = {
            {FeeReason::NONE, "None"},
            {FeeReason::HALF_ESTIMATE, "Half Target 60% Threshold"},
            {FeeReason::FULL_ESTIMATE, "Target 85% Threshold"},
            {FeeReason::DOUBLE_ESTIMATE, "Double Target 95% Threshold"},
            {FeeReason::CONSERVATIVE, "Conservative Double Target longer horizon"},
            {FeeReason::MEMPOOL_MIN, "Mempool Min Fee"},
            {FeeReason::PAYTXFEE, "PayTxFee set"},
            {FeeReason::FALLBACK, "Fallback fee"},
            {FeeReason::REQUIRED, "Minimum Required Fee"},
        };
        auto reason_string = fee_reason_strings.find(reason);

        if (reason_string == fee_reason_strings.end()) return "Unknown";

        return reason_string->second;
        */
}

pub fn fee_mode_map() -> &'static Vec<(String,FeeEstimateMode)> {
    
    todo!();
        /*
            static const std::vector<std::pair<std::string, FeeEstimateMode>> FEE_MODES = {
            {"unset", FeeEstimateMode::UNSET},
            {"economical", FeeEstimateMode::ECONOMICAL},
            {"conservative", FeeEstimateMode::CONSERVATIVE},
        };
        return FEE_MODES;
        */
}

pub fn fee_modes(delimiter: &String) -> String {
    
    todo!();
        /*
            return Join(FeeModeMap(), delimiter, [&](const std::pair<std::string, FeeEstimateMode>& i) { return i.first; });
        */
}

