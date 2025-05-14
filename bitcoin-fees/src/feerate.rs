// ---------------- [ File: bitcoin-fees/src/feerate.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/policy/feerate.h]
//-------------------------------------------[.cpp/bitcoin/src/policy/feerate.cpp]

/**
  | Fee rate in satoshis per kilobyte: CAmount
  | / kB
  |
  */
pub struct FeeRate {

    /**
      | unit is satoshis-per-1,000-bytes
      |
      */
    n_satoshis_perk: Amount,
}

impl Default for FeeRate {
    
    /**
      | Fee rate of 0 satoshis per kB
      |
      */
    fn default() -> Self {
        Self {
            n_satoshis_perk: 0,
        }
    }
}

impl Ord for FeeRate {
    
    #[inline] fn cmp(&self, other: &FeeRate) -> Ordering {
        todo!();
        /*
            return a.nSatoshisPerK < b.nSatoshisPerK;
        */
    }
}

impl PartialOrd<FeeRate> for FeeRate {
    #[inline] fn partial_cmp(&self, other: &FeeRate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<FeeRate> for FeeRate {
    
    #[inline] fn eq(&self, other: &FeeRate) -> bool {
        todo!();
        /*
            return a.nSatoshisPerK == b.nSatoshisPerK;
        */
    }
}

impl Eq for FeeRate {}

impl AddAssign<&FeeRate> for FeeRate {

    #[inline]fn add_assign(&mut self, other: &FeeRate) {
        todo!();
        /*
            nSatoshisPerK += a.nSatoshisPerK; return *this;
        */
    }
}

impl FeeRate {

    pub fn new<I>(n_satoshis_perk: I) -> Self {
    
        todo!();
        /*
            : nSatoshisPerK(_nSatoshisPerK) 
            // We've previously had bugs creep in from silent double->int conversion...
            const_assert(std::is_integral<I>::value, "CFeeRate should be used without floats");
        */
    }

    /**
      | Return the fee in satoshis for a size
      | of 1000 bytes
      |
      */
    pub fn get_fee_perk(&self) -> Amount {
        
        todo!();
        /*
            return GetFee(1000);
        */
    }
    
    /**
      | Constructor for a fee rate in satoshis
      | per kvB (sat/kvB).
      | 
      | Passing a num_bytes value of COIN (1e8)
      | returns a fee rate in satoshis per vB
      | (sat/vB), e.g. (nFeePaid * 1e8 / 1e3)
      | == (nFeePaid / 1e5), where 1e5 is the
      | ratio to convert from BTC/kvB to sat/vB.
      |
      */
    pub fn new_with_fee_paid(
        n_fee_paid: &Amount,
        num_bytes:  u32) -> Self {
    
        todo!();
        /*
           const int64_t nSize{num_bytes};

        if (nSize > 0) {
            nSatoshisPerK = nFeePaid * 1000 / nSize;
        } else {
            nSatoshisPerK = 0;
        }
        */
    }
    
    /**
      | Return the fee in satoshis for the given
      | size in bytes.
      |
      */
    pub fn get_fee(&self, num_bytes: u32) -> Amount {
        
        todo!();
        /*
        const int64_t nSize{num_bytes};

        CAmount nFee = nSatoshisPerK * nSize / 1000;

        if (nFee == 0 && nSize != 0) {
            if (nSatoshisPerK > 0) nFee = CAmount(1);
            if (nSatoshisPerK < 0) nFee = CAmount(-1);
        }

        return nFee;
        */
    }
    
    pub fn to_string(&self, fee_estimate_mode: Option<&FeeEstimateMode>) -> String {

        let fee_estimate_mode: &FeeEstimateMode = fee_estimate_mode.unwrap_or(&FeeEstimateMode::BTC_KVB);
        
        todo!();
        /*
            switch (fee_estimate_mode) {
        case FeeEstimateMode::SAT_VB: return strprintf("%d.%03d %s/vB", nSatoshisPerK / 1000, nSatoshisPerK % 1000, CURRENCY_ATOM);
        default:                      return strprintf("%d.%08d %s/kvB", nSatoshisPerK / COIN, nSatoshisPerK % COIN, CURRENCY_UNIT);
        }
        */
    }
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CFeeRate, obj) { READWRITE(obj.nSatoshisPerK); }
    */
}
