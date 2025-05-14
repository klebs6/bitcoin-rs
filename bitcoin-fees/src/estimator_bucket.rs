// ---------------- [ File: bitcoin-fees/src/estimator_bucket.rs ]
crate::ix!();

/**
  | Used to return detailed information
  | about a fee estimate calculation
  |
  */
#[derive(Default)]
pub struct EstimationResult
{
    pass:  EstimatorBucket,
    fail:  EstimatorBucket,
    decay: f64,
    scale: u32,
}

/**
  | Used to return detailed information
  | about a feerate bucket
  |
  */
pub struct EstimatorBucket
{
    start:           f64,
    end:             f64,
    within_target:   f64,
    total_confirmed: f64,
    in_mempool:      f64,
    left_mempool:    f64,
}

impl Default for EstimatorBucket {

    fn default() -> Self {
        Self {
            start:          -1.0,
            end:            -1.0,
            within_target:   0.0,
            total_confirmed: 0.0,
            in_mempool:      0.0,
            left_mempool:    0.0,
        }
    }
}
