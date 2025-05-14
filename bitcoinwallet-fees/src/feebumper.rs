// ---------------- [ File: bitcoinwallet-fees/src/feebumper.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/feebumper.h]

pub enum FeeBumperResult
{
    OK,
    INVALID_ADDRESS_OR_KEY,
    INVALID_REQUEST,
    INVALID_PARAMETER,
    WALLET_ERROR,
    MISC_ERROR,
}

/**
  | Return whether transaction can be bumped.
  |
  */
pub fn feebumper_transaction_can_be_bumped(
        wallet: &Wallet,
        txid:   &u256) -> bool {
    
    todo!();
        /*
            LOCK(wallet.cs_wallet);
        const CWalletTx* wtx = wallet.GetWalletTx(txid);
        if (wtx == nullptr) return false;

        std::vector<bilingual_str> errors_dummy;
        feebumper::Result res = PreconditionChecks(wallet, *wtx, errors_dummy);
        return res == feebumper::Result::OK;
        */
}

/**
  | Create bumpfee transaction based on
  | feerate estimates.
  |
  */
pub fn feebumper_create_rate_bump_transaction(
        wallet:       &Wallet,
        txid:         &u256,
        coin_control: &CoinControl,
        errors:       &mut Vec<BilingualStr>,
        old_fee:      &mut Amount,
        new_fee:      &mut Amount,
        mtx:          &mut MutableTransaction) -> FeeBumperResult {
    
    todo!();
        /*
            // We are going to modify coin control later, copy to re-use
        CCoinControl new_coin_control(coin_control);

        LOCK(wallet.cs_wallet);
        errors.clear();
        auto it = wallet.mapWallet.find(txid);
        if (it == wallet.mapWallet.end()) {
            errors.push_back(Untranslated("Invalid or non-wallet transaction id"));
            return Result::INVALID_ADDRESS_OR_KEY;
        }
        const CWalletTx& wtx = it->second;

        Result result = PreconditionChecks(wallet, wtx, errors);
        if (result != Result::OK) {
            return result;
        }

        // Fill in recipients(and preserve a single change key if there is one)
        std::vector<CRecipient> recipients;
        for (const auto& output : wtx.tx->vout) {
            if (!OutputIsChange(wallet, output)) {
                CRecipient recipient = {output.scriptPubKey, output.nValue, false};
                recipients.push_back(recipient);
            } else {
                TxDestination change_dest;
                ExtractDestination(output.scriptPubKey, change_dest);
                new_coin_control.destChange = change_dest;
            }
        }

        isminefilter filter = wallet.GetLegacyScriptPubKeyMan() && wallet.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS) ? ISMINE_WATCH_ONLY : ISMINE_SPENDABLE;
        old_fee = CachedTxGetDebit(wallet, wtx, filter) - wtx.tx->GetValueOut();

        if (coin_control.m_feerate) {
            // The user provided a feeRate argument.
            // We calculate this here to avoid compiler warning on the cs_wallet lock
            const int64_t maxTxSize{CalculateMaximumSignedTxSize(*wtx.tx, &wallet).vsize};
            Result res = CheckFeeRate(wallet, wtx, *new_coin_control.m_feerate, maxTxSize, errors);
            if (res != Result::OK) {
                return res;
            }
        } else {
            // The user did not provide a feeRate argument
            new_coin_control.m_feerate = EstimateFeeRate(wallet, wtx, old_fee, new_coin_control);
        }

        // Fill in required inputs we are double-spending(all of them)
        // N.B.: bip125 doesn't require all the inputs in the replaced transaction to be
        // used in the replacement transaction, but it's very important for wallets to make
        // sure that happens. If not, it would be possible to bump a transaction A twice to
        // A2 and A3 where A2 and A3 don't conflict (or alternatively bump A to A2 and A2
        // to A3 where A and A3 don't conflict). If both later get confirmed then the sender
        // has accidentally double paid.
        for (const auto& inputs : wtx.tx->vin) {
            new_coin_control.Select(OutPoint(inputs.prevout));
        }
        new_coin_control.fAllowOtherInputs = true;

        // We cannot source new unconfirmed inputs(bip125 rule 2)
        new_coin_control.m_min_depth = 1;

        CTransactionRef tx_new;
        CAmount fee_ret;
        int change_pos_in_out = -1; // No requested location for change
        bilingual_str fail_reason;
        FeeCalculation fee_calc_out;
        if (!CreateTransaction(wallet, recipients, tx_new, fee_ret, change_pos_in_out, fail_reason, new_coin_control, fee_calc_out, false)) {
            errors.push_back(Untranslated("Unable to create transaction.") + Untranslated(" ") + fail_reason);
            return Result::WALLET_ERROR;
        }

        // Write back new fee if successful
        new_fee = fee_ret;

        // Write back transaction
        mtx = CMutableTransaction(*tx_new);
        // Mark new tx not replaceable, if requested.
        if (!coin_control.m_signal_bip125_rbf.value_or(wallet.m_signal_rbf)) {
            for (auto& input : mtx.vin) {
                if (input.nSequence < 0xfffffffe) input.nSequence = 0xfffffffe;
            }
        }

        return Result::OK;
        */
}

/**
  | Sign the new transaction,
  | 
  | -----------
  | @return
  | 
  | false if the tx couldn't be found or if
  | it was impossible to create the signature(s)
  |
  */
pub fn feebumper_sign_transaction(
        wallet: &mut Wallet,
        mtx:    &mut MutableTransaction) -> bool {
    
    todo!();
        /*
            LOCK(wallet.cs_wallet);
        return wallet.SignTransaction(mtx);
        */
}

/**
  | Commit the bumpfee transaction.
  |
  | @return success in case of
  | CWallet::CommitTransaction was successful, but
  | sets errors if the tx could not be added to
  | the mempool (will try later) or if the old
  | transaction could not be marked as replaced.
  */
pub fn feebumper_commit_transaction(
        wallet:      &mut Wallet,
        txid:        &u256,
        mtx:         MutableTransaction,
        errors:      &mut Vec<BilingualStr>,
        bumped_txid: &mut u256) -> FeeBumperResult {
    
    todo!();
        /*
            LOCK(wallet.cs_wallet);
        if (!errors.empty()) {
            return Result::MISC_ERROR;
        }
        auto it = txid.IsNull() ? wallet.mapWallet.end() : wallet.mapWallet.find(txid);
        if (it == wallet.mapWallet.end()) {
            errors.push_back(Untranslated("Invalid or non-wallet transaction id"));
            return Result::MISC_ERROR;
        }
        const CWalletTx& oldWtx = it->second;

        // make sure the transaction still has no descendants and hasn't been mined in the meantime
        Result result = PreconditionChecks(wallet, oldWtx, errors);
        if (result != Result::OK) {
            return result;
        }

        // commit/broadcast the tx
        CTransactionRef tx = MakeTransactionRef(std::move(mtx));
        mapValue_t mapValue = oldWtx.mapValue;
        mapValue["replaces_txid"] = oldWtx.GetHash().ToString();

        wallet.CommitTransaction(tx, std::move(mapValue), oldWtx.vOrderForm);

        // mark the original tx as bumped
        bumped_txid = tx->GetHash();
        if (!wallet.MarkReplaced(oldWtx.GetHash(), bumped_txid)) {
            // TODO: see if JSON-RPC has a standard way of returning a response
            // along with an exception. It would be good to return information about
            // wtxBumped to the caller even if marking the original transaction
            // replaced does not succeed for some reason.
            errors.push_back(Untranslated("Created new bumpfee transaction but could not mark the original transaction as replaced"));
        }
        return Result::OK;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/feebumper.cpp]

/**
  | Check whether transaction has descendant in
  | wallet or mempool, or has been mined, or
  | conflicts with a mined transaction. Return
  | a feebumper::Result.
  */
#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn precondition_checks(
        wallet: &Wallet,
        wtx:    &WalletTx,
        errors: &mut Vec<BilingualStr>) -> FeeBumperResult {
    
    todo!();
        /*
            if (wallet.HasWalletSpend(wtx.GetHash())) {
            errors.push_back(Untranslated("Transaction has descendants in the wallet"));
            return feebumper::Result::INVALID_PARAMETER;
        }

        {
            if (wallet.chain().hasDescendantsInMempool(wtx.GetHash())) {
                errors.push_back(Untranslated("Transaction has descendants in the mempool"));
                return feebumper::Result::INVALID_PARAMETER;
            }
        }

        if (wallet.GetTxDepthInMainChain(wtx) != 0) {
            errors.push_back(Untranslated("Transaction has been mined, or is conflicted with a mined transaction"));
            return feebumper::Result::WALLET_ERROR;
        }

        if (!SignalsOptInRBF(*wtx.tx)) {
            errors.push_back(Untranslated("Transaction is not BIP 125 replaceable"));
            return feebumper::Result::WALLET_ERROR;
        }

        if (wtx.mapValue.count("replaced_by_txid")) {
            errors.push_back(strprintf(Untranslated("Cannot bump transaction %s which was already bumped by transaction %s"), wtx.GetHash().ToString(), wtx.mapValue.at("replaced_by_txid")));
            return feebumper::Result::WALLET_ERROR;
        }

        // check that original tx consists entirely of our inputs
        // if not, we can't bump the fee, because the wallet has no way of knowing the value of the other inputs (thus the fee)
        isminefilter filter = wallet.GetLegacyScriptPubKeyMan() && wallet.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS) ? ISMINE_WATCH_ONLY : ISMINE_SPENDABLE;
        if (!AllInputsMine(wallet, *wtx.tx, filter)) {
            errors.push_back(Untranslated("Transaction contains inputs that don't belong to this wallet"));
            return feebumper::Result::WALLET_ERROR;
        }

        return feebumper::Result::OK;
        */
}

/**
  | Check if the user provided a valid feeRate
  |
  */
pub fn check_fee_rate(
        wallet:      &Wallet,
        wtx:         &WalletTx,
        new_feerate: &FeeRate,
        max_tx_size: i64,
        errors:      &mut Vec<BilingualStr>) -> FeeBumperResult {
    
    todo!();
        /*
            // check that fee rate is higher than mempool's minimum fee
        // (no point in bumping fee if we know that the new tx won't be accepted to the mempool)
        // This may occur if the user set fee_rate or paytxfee too low, if fallbackfee is too low, or, perhaps,
        // in a rare situation where the mempool minimum fee increased significantly since the fee estimation just a
        // moment earlier. In this case, we report an error to the user, who may adjust the fee.
        CFeeRate minMempoolFeeRate = wallet.chain().mempoolMinFee();

        if (newFeerate.GetFeePerK() < minMempoolFeeRate.GetFeePerK()) {
            errors.push_back(strprintf(
                Untranslated("New fee rate (%s) is lower than the minimum fee rate (%s) to get into the mempool -- "),
                FormatMoney(newFeerate.GetFeePerK()),
                FormatMoney(minMempoolFeeRate.GetFeePerK())));
            return feebumper::Result::WALLET_ERROR;
        }

        CAmount new_total_fee = newFeerate.GetFee(maxTxSize);

        CFeeRate incrementalRelayFee = std::max(wallet.chain().relayIncrementalFee(), CFeeRate(WALLET_INCREMENTAL_RELAY_FEE));

        // Given old total fee and transaction size, calculate the old feeRate
        isminefilter filter = wallet.GetLegacyScriptPubKeyMan() && wallet.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS) ? ISMINE_WATCH_ONLY : ISMINE_SPENDABLE;
        CAmount old_fee = CachedTxGetDebit(wallet, wtx, filter) - wtx.tx->GetValueOut();
        const int64_t txSize = GetVirtualTransactionSize(*(wtx.tx));
        CFeeRate nOldFeeRate(old_fee, txSize);
        // Min total fee is old fee + relay fee
        CAmount minTotalFee = nOldFeeRate.GetFee(maxTxSize) + incrementalRelayFee.GetFee(maxTxSize);

        if (new_total_fee < minTotalFee) {
            errors.push_back(strprintf(Untranslated("Insufficient total fee %s, must be at least %s (oldFee %s + incrementalFee %s)"),
                FormatMoney(new_total_fee), FormatMoney(minTotalFee), FormatMoney(nOldFeeRate.GetFee(maxTxSize)), FormatMoney(incrementalRelayFee.GetFee(maxTxSize))));
            return feebumper::Result::INVALID_PARAMETER;
        }

        CAmount requiredFee = GetRequiredFee(wallet, maxTxSize);
        if (new_total_fee < requiredFee) {
            errors.push_back(strprintf(Untranslated("Insufficient total fee (cannot be less than required fee %s)"),
                FormatMoney(requiredFee)));
            return feebumper::Result::INVALID_PARAMETER;
        }

        // Check that in all cases the new fee doesn't violate maxTxFee
        const CAmount max_tx_fee = wallet.m_default_max_tx_fee;
        if (new_total_fee > max_tx_fee) {
            errors.push_back(strprintf(Untranslated("Specified or calculated fee %s is too high (cannot be higher than -maxtxfee %s)"),
                FormatMoney(new_total_fee), FormatMoney(max_tx_fee)));
            return feebumper::Result::WALLET_ERROR;
        }

        return feebumper::Result::OK;
        */
}

pub fn estimate_fee_rate(
        wallet:       &Wallet,
        wtx:          &WalletTx,
        old_fee:      Amount,
        coin_control: &CoinControl) -> FeeRate {
    
    todo!();
        /*
            // Get the fee rate of the original transaction. This is calculated from
        // the tx fee/vsize, so it may have been rounded down. Add 1 satoshi to the
        // result.
        int64_t txSize = GetVirtualTransactionSize(*(wtx.tx));
        CFeeRate feerate(old_fee, txSize);
        feerate += CFeeRate(1);

        // The node has a configurable incremental relay fee. Increment the fee by
        // the minimum of that and the wallet's conservative
        // WALLET_INCREMENTAL_RELAY_FEE value to future proof against changes to
        // network wide policy for incremental relay fee that our node may not be
        // aware of. This ensures we're over the required relay fee rate
        // (BIP 125 rule 4).  The replacement tx will be at least as large as the
        // original tx, so the total fee will be greater (BIP 125 rule 3)
        CFeeRate node_incremental_relay_fee = wallet.chain().relayIncrementalFee();
        CFeeRate wallet_incremental_relay_fee = CFeeRate(WALLET_INCREMENTAL_RELAY_FEE);
        feerate += std::max(node_incremental_relay_fee, wallet_incremental_relay_fee);

        // Fee rate must also be at least the wallet's GetMinimumFeeRate
        CFeeRate min_feerate(GetMinimumFeeRate(wallet, coin_control, /* feeCalc */ nullptr));

        // Set the required fee rate for the replacement transaction in coin control.
        return std::max(feerate, min_feerate);
        */
}
