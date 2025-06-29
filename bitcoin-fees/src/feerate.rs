// ---------------- [ File: bitcoin-fees/src/feerate.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/policy/feerate.h]
//-------------------------------------------[.cpp/bitcoin/src/policy/feerate.cpp]

/// Fee rate in satoshis per kilobyte (¹/₁₀₀₀ of a megabyte).
///
/// Internally this is stored exactly as integer satoshis per **1000 bytes**
/// ( **k** = 1000, matching the historical behaviour in Bitcoin Core).
#[derive(Serialize,Deserialize,Getters, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[getset(get = "pub")]
pub struct FeeRate {
    /// Satoshis per **k** B.
    ///
    /// This is *never* exposed mutably; callers go through the typed API
    /// so we can guarantee all invariants.
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
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.n_satoshis_perk.cmp(&other.n_satoshis_perk)
    }
}

impl PartialOrd for FeeRate {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AddAssign<&FeeRate> for FeeRate {
    #[inline]
    fn add_assign(&mut self, other: &FeeRate) {
        self.n_satoshis_perk += other.n_satoshis_perk;
    }
}

impl fmt::Display for FeeRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string(None))
    }
}

impl FeeRate {

    /// Construct a fee‑rate from an explicit **sat/kvB** integer.
    #[inline]
    pub fn new<I>(n_satoshis_perk: I) -> Self
    where
        I: Into<Amount> + Copy,
    {
        // NOTE: in the C++, we've previously had bugs creep in from silent double->int conversion...
        // Please keep this in mind.
        //
        let value = n_satoshis_perk.into();
        trace!(value, "FeeRate::new");
        Self {
            n_satoshis_perk: value,
        }
    }

    /// Fee for **1000 bytes** (one *kvB*).
    ///
    /// Return the fee in satoshis for a size of 1000 bytes
    ///
    #[inline]
    pub fn get_fee_perk(&self) -> Amount {
        self.get_fee(1000)
    }
    
    /// Construct from a *paid* fee and the serialized *size* in bytes.
    ///
    /// Constructor for a fee rate in satoshis per kvB (sat/kvB).
    ///
    /// When `num_bytes == COIN (1 × 10⁸)`, the resulting instance represents
    /// sat/vB instead of sat/kvB (this is identical to Core’s dual‑unit ctor).
    ///
    /// Passing a num_bytes value of COIN (1e8) returns a fee rate in satoshis per vB (sat/vB),
    /// e.g. (nFeePaid * 1e8 / 1e3) == (nFeePaid / 1e5), where 1e5 is the ratio to convert from
    /// BTC/kvB to sat/vB.
    ///
    pub fn new_with_fee_paid(n_fee_paid: &Amount, num_bytes: u32) -> Self {
        trace!(
            n_fee_paid = *n_fee_paid,
            num_bytes,
            "FeeRate::new_with_fee_paid"
        );

        let n_satoshis_perk = if num_bytes != 0 {
            // Cast to i128 to avoid intermediate overflow then back.
            ((*n_fee_paid as i128) * 1000 / num_bytes as i128) as Amount
        } else {
            0
        };

        Self { n_satoshis_perk }
    }
    
    /// Fee for an arbitrary serialization size (in *bytes*).
    ///
    /// The algorithm intentionally **truncates toward 0** to replicate the
    /// original behaviour; if that would yield `0` while the rate is non‑zero,
    /// the result is bumped to ±1 sat so that the direction of the fee is
    /// retained and never hidden.
    pub fn get_fee(&self, num_bytes: u32) -> Amount {
        trace!(rate = self.n_satoshis_perk, num_bytes, "FeeRate::get_fee");

        if num_bytes == 0 {
            return 0;
        }

        let n_size = num_bytes as i64;
        let mut n_fee: Amount =
            ((self.n_satoshis_perk as i128) * n_size as i128 / 1000) as Amount;

        if n_fee == 0 {
            if self.n_satoshis_perk > 0 {
                n_fee = 1;
            } else if self.n_satoshis_perk < 0 {
                n_fee = -1;
            }
        }

        n_fee
    }

    /// Human‑readable representation.
    ///
    /// Defaults to `FeeEstimateMode::BTC_KVB` when no mode is supplied.
    pub fn to_string(&self, fee_estimate_mode: Option<&FeeEstimateMode>) -> String {
        let mode = fee_estimate_mode.unwrap_or(&FeeEstimateMode::BTC_KVB);

        match mode {
            FeeEstimateMode::SAT_VB => {
                let whole = self.n_satoshis_perk / 1000;
                let frac = (self.n_satoshis_perk.abs() % 1000) as u32;
                format!("{whole}.{frac:03} {CURRENCY_ATOM}/vB")
            }
            _ => {
                let whole = self.n_satoshis_perk / COIN;
                let frac = (self.n_satoshis_perk.abs() % COIN) as u32;
                format!("{whole}.{frac:08} {CURRENCY_UNIT}/kvB")
            }
        }
    }
}

#[cfg(test)]
mod fee_rate_validation_tests {
    use super::*;

    #[traced_test]
    fn zero_fee_rate_returns_zero() {
        let fee_rate = FeeRate::new(0);
        assert_eq!(fee_rate.get_fee(0), 0);
        assert_eq!(fee_rate.get_fee(100_000), 0);
    }

    #[traced_test]
    fn positive_fee_rate_behaviour() {
        let fee_rate = FeeRate::new(1_000); // 1000 sat/kvB == 1 sat/byte
        assert_eq!(fee_rate.get_fee(0), 0);
        assert_eq!(fee_rate.get_fee(1), 1);
        assert_eq!(fee_rate.get_fee(121), 121);
        assert_eq!(fee_rate.get_fee(999), 999);
        assert_eq!(fee_rate.get_fee(1_000), 1_000);
        assert_eq!(fee_rate.get_fee(9_000), 9_000);
    }

    #[traced_test]
    fn negative_fee_rate_behaviour() {
        let fee_rate = FeeRate::new(-1_000);
        assert_eq!(fee_rate.get_fee(0), 0);
        assert_eq!(fee_rate.get_fee(1), -1);
        assert_eq!(fee_rate.get_fee(121), -121);
        assert_eq!(fee_rate.get_fee(999), -999);
    }

    #[traced_test]
    fn tiny_fee_rate_rounds_up_or_down() {
        let pos = FeeRate::new(123);
        assert_eq!(pos.get_fee(8), 1); // special‑case bump
        assert_eq!(pos.get_fee(9), 1);
        assert_eq!(pos.get_fee(121), 14);
        assert_eq!(pos.get_fee(122), 15);

        let neg = FeeRate::new(-123);
        assert_eq!(neg.get_fee(8), -1);
        assert_eq!(neg.get_fee(9), -1);
    }

    #[traced_test]
    fn add_assign_and_ordering() {
        let mut a = FeeRate::new(1);
        let b = FeeRate::new(2);

        assert!(a < b);
        assert!(b > a);
        assert!(a == a);
        assert!(a <= b && a <= a);
        assert!(b >= a && b >= b);

        a += &a; // a == 2 sat/kvB now
        assert_eq!(a, b);
    }

    #[traced_test]
    fn to_string_formats_correctly() {
        let fee_rate = FeeRate::new(1);
        assert_eq!(fee_rate.to_string(None), "0.00000001 BTC/kvB");
        assert_eq!(
            fee_rate.to_string(Some(&FeeEstimateMode::BTC_KVB)),
            "0.00000001 BTC/kvB"
        );
        assert_eq!(
            fee_rate.to_string(Some(&FeeEstimateMode::SAT_VB)),
            "0.001 sat/vB"
        );
    }

    #[traced_test]
    fn ctor_from_fee_and_size() {
        let fee_rate = FeeRate::new_with_fee_paid(&1_000, 2_000); // 0.5 sat/B ⇒ 500 sat/kvB
        assert_eq!(fee_rate.get_fee_perk(), 500);
    }
}
