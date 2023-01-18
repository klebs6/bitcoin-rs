crate::ix!();

pub struct FeeFilterRounder {
    feeset:        HashSet<f64>,
    insecure_rand: FastRandomContext,
}

pub mod fee_filter_rounder {

    pub const MAX_FILTER_FEERATE: f64 = 1e7;

    /**
      | FEE_FILTER_SPACING is just used to
      | provide some quantization of fee filter
      | results.
      | 
      | Historically it reused FEE_SPACING,
      | but it is completely unrelated, and
      | was made a separate constant so the two
      | concepts are not tied together
      |
      */
    pub const FEE_FILTER_SPACING: f64 = 1.1;
}

impl FeeFilterRounder {

    /**
      | Create new FeeFilterRounder
      |
      */
    pub fn new(min_incremental_fee: &FeeRate) -> Self {
    
        todo!();
        /*
        CAmount minFeeLimit = std::max(CAmount(1), minIncrementalFee.GetFeePerK() / 2);
        feeset.insert(0);
        for (double bucketBoundary = minFeeLimit; bucketBoundary <= MAX_FILTER_FEERATE; bucketBoundary *= FEE_FILTER_SPACING) {
            feeset.insert(bucketBoundary);
        }
        */
    }
    
    /**
      | Quantize a minimum fee for privacy purpose
      | before broadcast. Not thread-safe
      | due to use of FastRandomContext
      |
      */
    pub fn round(&mut self, current_min_fee: Amount) -> Amount {
        
        todo!();
        /*
        std::set<double>::iterator it = feeset.lower_bound(currentMinFee);
        if ((it != feeset.begin() && insecure_rand.rand32() % 3 != 0) || it == feeset.end()) {
            it--;
        }
        return static_cast<CAmount>(*it);
        */
    }
}
