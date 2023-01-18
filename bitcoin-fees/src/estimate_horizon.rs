crate::ix!();

/**
  | Identifier for each of the 3 different
  | TxConfirmStats which will track history
  | over different time horizons.
  |
  */
pub enum FeeEstimateHorizon {
    SHORT_HALFLIFE,
    MED_HALFLIFE,
    LONG_HALFLIFE,
}

lazy_static!{
    /*
    static constexpr auto ALL_FEE_ESTIMATE_HORIZONS = std::array{
        FeeEstimateHorizon::SHORT_HALFLIFE,
        FeeEstimateHorizon::MED_HALFLIFE,
        FeeEstimateHorizon::LONG_HALFLIFE,
    };
    */
}

pub fn string_for_fee_estimate_horizon(horizon: FeeEstimateHorizon) -> String {
    
    todo!();
        /*
        switch (horizon) {
        case FeeEstimateHorizon::SHORT_HALFLIFE: return "short";
        case FeeEstimateHorizon::MED_HALFLIFE: return "medium";
        case FeeEstimateHorizon::LONG_HALFLIFE: return "long";
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
}

