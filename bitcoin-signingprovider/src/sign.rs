// ---------------- [ File: bitcoin-signingprovider/src/sign.rs ]
crate::ix!();

/**
  | Extract signature data from a transaction
  | input, and insert it.
  |
  -------------------------
  | Extracts signatures and scripts from
  | incomplete scriptSigs. Please do not
  | extend this, use PSBT instead
  |
  */
pub fn data_from_transaction(
        tx:    &MutableTransaction,
        n_in:  u32,
        txout: &TxOut) -> SignatureData {
    
    todo!();
        /*
            SignatureData data;
        assert(tx.vin.size() > nIn);
        data.scriptSig = tx.vin[nIn].scriptSig;
        data.scriptWitness = tx.vin[nIn].scriptWitness;
        Stacks stack(data);

        // Get signatures
        MutableTransactionSignatureChecker tx_checker(&tx, nIn, txout.nValue, MissingDataBehavior::FAIL);
        SignatureExtractorChecker extractor_checker(data, tx_checker);
        if (VerifyScript(data.scriptSig, txout.scriptPubKey, &data.scriptWitness, STANDARD_SCRIPT_VERIFY_FLAGS, extractor_checker)) {
            data.complete = true;
            return data;
        }

        // Get scripts
        std::vector<std::vector<unsigned char>> solutions;
        TxoutType script_type = Solver(txout.scriptPubKey, solutions);
        SigVersion sigversion = SigVersion::BASE;
        CScript next_script = txout.scriptPubKey;

        if (script_type == TxoutType::SCRIPTHASH && !stack.script.empty() && !stack.script.back().empty()) {
            // Get the redeemScript
            CScript redeem_script(stack.script.back().begin(), stack.script.back().end());
            data.redeem_script = redeem_script;
            next_script = std::move(redeem_script);

            // Get redeemScript type
            script_type = Solver(next_script, solutions);
            stack.script.pop_back();
        }
        if (script_type == TxoutType::WITNESS_V0_SCRIPTHASH && !stack.witness.empty() && !stack.witness.back().empty()) {
            // Get the witnessScript
            CScript witness_script(stack.witness.back().begin(), stack.witness.back().end());
            data.witness_script = witness_script;
            next_script = std::move(witness_script);

            // Get witnessScript type
            script_type = Solver(next_script, solutions);
            stack.witness.pop_back();
            stack.script = std::move(stack.witness);
            stack.witness.clear();
            sigversion = SigVersion::WITNESS_V0;
        }
        if (script_type == TxoutType::MULTISIG && !stack.script.empty()) {
            // Build a map of pubkey -> signature by matching sigs to pubkeys:
            assert(solutions.size() > 1);
            unsigned int num_pubkeys = solutions.size()-2;
            unsigned int last_success_key = 0;
            for (const valtype& sig : stack.script) {
                for (unsigned int i = last_success_key; i < num_pubkeys; ++i) {
                    const valtype& pubkey = solutions[i+1];
                    // We either have a signature for this pubkey, or we have found a signature and it is valid
                    if (data.signatures.count(CPubKey(pubkey).GetID()) || extractor_checker.CheckECDSASignature(sig, pubkey, next_script, sigversion)) {
                        last_success_key = i + 1;
                        break;
                    }
                }
            }
        }

        return data;
        */
}

pub fn update_input(
        input: &mut TxIn,
        data:  &SignatureData)  {
    
    todo!();
        /*
            input.scriptSig = data.scriptSig;
        input.scriptWitness = data.scriptWitness;
        */
}

/**
  | Produce a script signature for a transaction.
  |
  */
pub fn sign_signature_with_amount(
        provider:     &SigningProvider,
        from_pub_key: &Script,
        tx_to:        &mut MutableTransaction,
        n_in:         u32,
        amount:       &Amount,
        n_hash_type:  i32) -> bool {
    
    todo!();
        /*
            assert(nIn < txTo.vin.size());

        MutableTransactionSignatureCreator creator(&txTo, nIn, amount, nHashType);

        SignatureData sigdata;
        bool ret = ProduceSignature(provider, creator, fromPubKey, sigdata);
        UpdateInput(txTo.vin.at(nIn), sigdata);
        return ret;
        */
}

pub fn sign_signature(
        provider:    &SigningProvider,
        tx_from:     &Transaction,
        tx_to:       &mut MutableTransaction,
        n_in:        u32,
        n_hash_type: i32) -> bool {
    
    todo!();
        /*
            assert(nIn < txTo.vin.size());
        const CTxIn& txin = txTo.vin[nIn];
        assert(txin.prevout.n < txFrom.vout.size());
        const CTxOut& txout = txFrom.vout[txin.prevout.n];

        return SignSignature(provider, txout.scriptPubKey, txTo, nIn, txout.nValue, nHashType);
        */
}

/**
  | Sign the CMutableTransaction
  |
  */
pub fn sign_transaction(
        mtx:          &mut MutableTransaction,
        keystore:     *const SigningProvider,
        coins:        &HashMap<OutPoint,Coin>,
        n_hash_type:  i32,
        input_errors: &mut HashMap<i32,BilingualStr>) -> bool {
    
    todo!();
        /*
            bool fHashSingle = ((nHashType & ~SIGHASH_ANYONECANPAY) == SIGHASH_SINGLE);

        // Use CTransaction for the constant parts of the
        // transaction to avoid rehashing.
        const CTransaction txConst(mtx);

        PrecomputedTransactionData txdata;
        std::vector<CTxOut> spent_outputs;
        for (unsigned int i = 0; i < mtx.vin.size(); ++i) {
            CTxIn& txin = mtx.vin[i];
            auto coin = coins.find(txin.prevout);
            if (coin == coins.end() || coin->second.IsSpent()) {
                txdata.Init(txConst, /* spent_outputs */ {}, /* force */ true);
                break;
            } else {
                spent_outputs.emplace_back(coin->second.out.nValue, coin->second.out.scriptPubKey);
            }
        }
        if (spent_outputs.size() == mtx.vin.size()) {
            txdata.Init(txConst, std::move(spent_outputs), true);
        }

        // Sign what we can:
        for (unsigned int i = 0; i < mtx.vin.size(); ++i) {
            CTxIn& txin = mtx.vin[i];
            auto coin = coins.find(txin.prevout);
            if (coin == coins.end() || coin->second.IsSpent()) {
                input_errors[i] = _("Input not found or already spent");
                continue;
            }
            const CScript& prevPubKey = coin->second.out.scriptPubKey;
            const CAmount& amount = coin->second.out.nValue;

            SignatureData sigdata = DataFromTransaction(mtx, i, coin->second.out);
            // Only sign SIGHASH_SINGLE if there's a corresponding output:
            if (!fHashSingle || (i < mtx.vout.size())) {
                ProduceSignature(*keystore, MutableTransactionSignatureCreator(&mtx, i, amount, &txdata, nHashType), prevPubKey, sigdata);
            }

            UpdateInput(txin, sigdata);

            // amount must be specified for valid segwit signature
            if (amount == MAX_MONEY && !txin.scriptWitness.IsNull()) {
                input_errors[i] = _("Missing amount");
                continue;
            }

            ScriptError serror = SCRIPT_ERR_OK;
            if (!VerifyScript(txin.scriptSig, prevPubKey, &txin.scriptWitness, STANDARD_SCRIPT_VERIFY_FLAGS, TransactionSignatureChecker(&txConst, i, amount, txdata, MissingDataBehavior::FAIL), &serror)) {
                if (serror == SCRIPT_ERR_INVALID_STACK_OPERATION) {
                    // Unable to sign input and verification failed (possible attempt to partially sign).
                    input_errors[i] = Untranslated("Unable to sign input, invalid stack size (possibly missing key)");
                } else if (serror == SCRIPT_ERR_SIG_NULLFAIL) {
                    // Verification failed (possibly due to insufficient signatures).
                    input_errors[i] = Untranslated("CHECK(MULTI)SIG failing with non-zero signature (possibly need more signatures)");
                } else {
                    input_errors[i] = Untranslated(ScriptErrorString(serror));
                }
            } else {
                // If this input succeeds, make sure there is no error set for it
                input_errors.erase(i);
            }
        }
        return input_errors.empty();
        */
}
