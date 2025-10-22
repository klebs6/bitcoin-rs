// ---------------- [ File: bitcoin-tx-confirm-stats/src/estimation_result.rs ]
crate::ix!();

/**
  | Used to return detailed information
  | about a fee estimate calculation
  |
  */
#[derive(Getters,Setters,MutGetters,Default)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct FeeRateEstimationResult
{
    pass:  FeeRateEstimatorBucket,
    fail:  FeeRateEstimatorBucket,
    decay: f64,
    scale: u32,
}

#[cfg(test)]
mod estimation_result_spec {
    use super::*;

    #[traced_test]
    fn default_values_and_getters_setters_work() {
        let mut r = FeeRateEstimationResult::default();

        // Defaults
        assert_eq!(r.pass().start(), &-1.0);
        assert_eq!(r.fail().end(),   &-1.0);
        assert_eq!(r.decay(), &0.0);
        assert_eq!(r.scale(), &0);

        // Setters
        let mut p = FeeRateEstimatorBucket::default();
        p.set_start(1.0);
        p.set_end(2.0);
        p.set_within_target(3.0);
        p.set_total_confirmed(4.0);
        p.set_in_mempool(5.0);
        p.set_left_mempool(6.0);

        let mut f = FeeRateEstimatorBucket::default();
        f.set_start(7.0);
        f.set_end(8.0);

        r.set_pass(p);
        r.set_fail(f);
        r.set_decay(0.999);
        r.set_scale(2);

        assert_eq!(r.pass().start(), &1.0);
        assert_eq!(r.pass().end(),   &2.0);
        assert_eq!(r.pass().within_target(), &3.0);
        assert_eq!(r.pass().total_confirmed(), &4.0);
        assert_eq!(r.pass().in_mempool(), &5.0);
        assert_eq!(r.pass().left_mempool(), &6.0);

        assert_eq!(r.fail().start(), &7.0);
        assert_eq!(r.fail().end(),   &8.0);
        assert_eq!(r.decay(), &0.999);
        assert_eq!(r.scale(), &2);
    }
}
