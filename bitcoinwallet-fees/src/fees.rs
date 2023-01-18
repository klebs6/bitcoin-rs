crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/fees.h]
//-------------------------------------------[.cpp/bitcoin/src/wallet/fees.cpp]

/**
  | Return the minimum required absolute
  | fee for this size based on the required
  | fee rate
  |
  */
pub fn get_required_fee(
    wallet:     &Wallet,
    n_tx_bytes: u32) -> Amount {
    
    get_required_fee_rate(wallet).get_fee(n_tx_bytes)
}

/**
  | Estimate the minimum fee considering
  | user set parameters and the required
  | fee
  |
  */
pub fn get_minimum_fee(
        wallet:       &Wallet,
        n_tx_bytes:   u32,
        coin_control: &CoinControl,
        fee_calc:     *mut FeeCalculation) -> Amount {
    
    todo!();
        /*
            return GetMinimumFeeRate(wallet, coin_control, feeCalc).GetFee(nTxBytes);
        */
}

/**
  | Return the minimum required feerate
  | taking into account the minimum relay
  | feerate and user set minimum transaction
  | feerate
  |
  */
pub fn get_required_fee_rate(wallet: &Wallet) -> FeeRate {
    
    todo!();
        /*
            return std::max(wallet.m_min_fee, wallet.chain().relayMinFee());
        */
}

/**
  | Estimate the minimum fee rate considering
  | user set parameters and the required
  | fee
  |
  */
pub fn get_minimum_fee_rate(
        wallet:       &Wallet,
        coin_control: &CoinControl,
        fee_calc:     *mut FeeCalculation) -> FeeRate {
    
    todo!();
        /*
            /* User control of how to calculate fee uses the following parameter precedence:
           1. coin_control.m_feerate
           2. coin_control.m_confirm_target
           3. m_pay_tx_fee (user-set member variable of wallet)
           4. m_confirm_target (user-set member variable of wallet)
           The first parameter that is set is used.
        */
        CFeeRate feerate_needed;
        if (coin_control.m_feerate) { // 1.
            feerate_needed = *(coin_control.m_feerate);
            if (feeCalc) feeCalc->reason = FeeReason::PAYTXFEE;
            // Allow to override automatic min/max check over coin control instance
            if (coin_control.fOverrideFeeRate) return feerate_needed;
        }
        else if (!coin_control.m_confirm_target && wallet.m_pay_tx_fee != CFeeRate(0)) { // 3. TODO: remove magic value of 0 for wallet member m_pay_tx_fee
            feerate_needed = wallet.m_pay_tx_fee;
            if (feeCalc) feeCalc->reason = FeeReason::PAYTXFEE;
        }
        else { // 2. or 4.
            // We will use smart fee estimation
            unsigned int target = coin_control.m_confirm_target ? *coin_control.m_confirm_target : wallet.m_confirm_target;
            // By default estimates are economical iff we are signaling opt-in-RBF
            bool conservative_estimate = !coin_control.m_signal_bip125_rbf.value_or(wallet.m_signal_rbf);
            // Allow to override the default fee estimate mode over the CoinControl instance
            if (coin_control.m_fee_mode == FeeEstimateMode::CONSERVATIVE) conservative_estimate = true;
            else if (coin_control.m_fee_mode == FeeEstimateMode::ECONOMICAL) conservative_estimate = false;

            feerate_needed = wallet.chain().estimateSmartFee(target, conservative_estimate, feeCalc);
            if (feerate_needed == CFeeRate(0)) {
                // if we don't have enough data for estimateSmartFee, then use fallback fee
                feerate_needed = wallet.m_fallback_fee;
                if (feeCalc) feeCalc->reason = FeeReason::FALLBACK;

                // directly return if fallback fee is disabled (feerate 0 == disabled)
                if (wallet.m_fallback_fee == CFeeRate(0)) return feerate_needed;
            }
            // Obey mempool min fee when using smart fee estimation
            CFeeRate min_mempool_feerate = wallet.chain().mempoolMinFee();
            if (feerate_needed < min_mempool_feerate) {
                feerate_needed = min_mempool_feerate;
                if (feeCalc) feeCalc->reason = FeeReason::MEMPOOL_MIN;
            }
        }

        // prevent user from paying a fee below the required fee rate
        CFeeRate required_feerate = GetRequiredFeeRate(wallet);
        if (required_feerate > feerate_needed) {
            feerate_needed = required_feerate;
            if (feeCalc) feeCalc->reason = FeeReason::REQUIRED;
        }
        return feerate_needed;
        */
}

/**
  | Return the maximum feerate for discarding
  | change.
  |
  */
pub fn get_discard_rate(wallet: &Wallet) -> FeeRate {
    
    todo!();
        /*
            unsigned int highest_target = wallet.chain().estimateMaxBlocks();
        CFeeRate discard_rate = wallet.chain().estimateSmartFee(highest_target, false /* conservative */);
        // Don't let discard_rate be greater than longest possible fee estimate if we get a valid fee estimate
        discard_rate = (discard_rate == CFeeRate(0)) ? wallet.m_discard_rate : std::min(discard_rate, wallet.m_discard_rate);
        // Discard rate must be at least dustRelayFee
        discard_rate = std::max(discard_rate, wallet.chain().relayDustFee());
        return discard_rate;
        */
}
