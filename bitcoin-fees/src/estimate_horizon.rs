// ---------------- [ File: bitcoin-fees/src/estimate_horizon.rs ]
crate::ix!();

/**
  | Identifier for each of the 3 different
  | TxConfirmStats which will track history
  | over different time horizons.
  |
  */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeeEstimateHorizon {
    SHORT_HALFLIFE,
    MED_HALFLIFE,
    LONG_HALFLIFE,
}

pub const ALL_FEE_ESTIMATE_HORIZONS: [FeeEstimateHorizon; 3] = [
    FeeEstimateHorizon::SHORT_HALFLIFE,
    FeeEstimateHorizon::MED_HALFLIFE,
    FeeEstimateHorizon::LONG_HALFLIFE,
];

pub fn string_for_fee_estimate_horizon(horizon: FeeEstimateHorizon) -> String {
    // switch (…) { … } + assert(false) in Core
    match horizon {
        FeeEstimateHorizon::SHORT_HALFLIFE => "short".to_string(),
        FeeEstimateHorizon::MED_HALFLIFE   => "medium".to_string(),
        FeeEstimateHorizon::LONG_HALFLIFE  => "long".to_string(),
    }
}
