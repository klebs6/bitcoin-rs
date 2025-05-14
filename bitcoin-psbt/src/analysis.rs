// ---------------- [ File: bitcoin-psbt/src/analysis.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/node/psbt.h]
//-------------------------------------------[.cpp/bitcoin/src/node/psbt.cpp]

/**
  | Holds an analysis of one input from a
  | PSBT
  |
  */
pub struct PSBTInputAnalysis {

    /**
      | Whether we have UTXO information for
      | this input
      |
      */
    has_utxo:               bool,

    /**
      | Whether the input has all required information
      | including signatures
      |
      */
    is_final:               bool,

    /**
      | Which of the BIP 174 roles needs to handle
      | this input next
      |
      */
    next:                   PSBTRole,

    /**
      | Pubkeys whose BIP32 derivation path
      | is missing
      |
      */
    missing_pubkeys:        Vec<KeyID>,

    /**
      | Pubkeys whose signatures are missing
      |
      */
    missing_sigs:           Vec<KeyID>,

    /**
      | Hash160 of redeem script, if missing
      |
      */
    missing_redeem_script:  u160,

    /**
      | SHA256 of witness script, if missing
      |
      */
    missing_witness_script: u256,
}

/**
  | Holds the results of AnalyzePSBT (miscellaneous
  | information about a PSBT)
  |
  */
pub struct PSBTAnalysis {

    /**
      | Estimated weight of the transaction
      |
      */
    estimated_vsize:   Option<usize>,

    /**
      | Estimated feerate (fee / weight) of
      | the transaction
      |
      */
    estimated_feerate: Option<FeeRate>,

    /**
      | Amount of fee being paid by the transaction
      |
      */
    fee:               Option<Amount>,

    /**
      | More information about the individual
      | inputs of the transaction
      |
      */
    inputs:            Vec<PSBTInputAnalysis>,

    /**
      | Which of the BIP 174 roles needs to handle
      | the transaction next
      |
      */
    next:              PSBTRole,

    /**
      | Error message
      |
      */
    error:             String,
}

impl PSBTAnalysis {
    
    pub fn set_invalid(&mut self, err_msg: String)  {
        
        todo!();
        /*
            estimated_vsize = std::nullopt;
            estimated_feerate = std::nullopt;
            fee = std::nullopt;
            inputs.clear();
            next = PSBTRole::CREATOR;
            error = err_msg;
        */
    }
}

/**
  | Provides helpful miscellaneous information
  | about where a PSBT is in the signing workflow.
  | 
  | -----------
  | @param[in] psbtx
  | 
  | the PSBT to analyze
  | 
  | -----------
  | @return
  | 
  | A PSBTAnalysis with information about
  | the provided PSBT.
  |
  */
pub fn analyzepsbt(psbtx: PartiallySignedTransaction) -> PSBTAnalysis {
    
    todo!();
        /*
            // Go through each input and build status
        PSBTAnalysis result;

        bool calc_fee = true;

        CAmount in_amt = 0;

        result.inputs.resize(psbtx.tx->vin.size());

        const PrecomputedTransactionData txdata = PrecomputePSBTData(psbtx);

        for (unsigned int i = 0; i < psbtx.tx->vin.size(); ++i) {
            PSBTInput& input = psbtx.inputs[i];
            PSBTInputAnalysis& input_analysis = result.inputs[i];

            // We set next role here and ratchet backwards as required
            input_analysis.next = PSBTRole::EXTRACTOR;

            // Check for a UTXO
            CTxOut utxo;
            if (psbtx.GetInputUTXO(utxo, i)) {
                if (!MoneyRange(utxo.nValue) || !MoneyRange(in_amt + utxo.nValue)) {
                    result.SetInvalid(strprintf("PSBT is not valid. Input %u has invalid value", i));
                    return result;
                }
                in_amt += utxo.nValue;
                input_analysis.has_utxo = true;
            } else {
                if (input.non_witness_utxo && psbtx.tx->vin[i].prevout.n >= input.non_witness_utxo->vout.size()) {
                    result.SetInvalid(strprintf("PSBT is not valid. Input %u specifies invalid prevout", i));
                    return result;
                }
                input_analysis.has_utxo = false;
                input_analysis.is_final = false;
                input_analysis.next = PSBTRole::UPDATER;
                calc_fee = false;
            }

            if (!utxo.IsNull() && utxo.scriptPubKey.IsUnspendable()) {
                result.SetInvalid(strprintf("PSBT is not valid. Input %u spends unspendable output", i));
                return result;
            }

            // Check if it is final
            if (!utxo.IsNull() && !PSBTInputSigned(input)) {
                input_analysis.is_final = false;

                // Figure out what is missing
                SignatureData outdata;
                bool complete = SignPSBTInput(DUMMY_SIGNING_PROVIDER, psbtx, i, &txdata, 1, &outdata);

                // Things are missing
                if (!complete) {
                    input_analysis.missing_pubkeys = outdata.missing_pubkeys;
                    input_analysis.missing_redeem_script = outdata.missing_redeem_script;
                    input_analysis.missing_witness_script = outdata.missing_witness_script;
                    input_analysis.missing_sigs = outdata.missing_sigs;

                    // If we are only missing signatures and nothing else, then next is signer
                    if (outdata.missing_pubkeys.empty() && outdata.missing_redeem_script.IsNull() && outdata.missing_witness_script.IsNull() && !outdata.missing_sigs.empty()) {
                        input_analysis.next = PSBTRole::SIGNER;
                    } else {
                        input_analysis.next = PSBTRole::UPDATER;
                    }
                } else {
                    input_analysis.next = PSBTRole::FINALIZER;
                }
            } else if (!utxo.IsNull()){
                input_analysis.is_final = true;
            }
        }

        // Calculate next role for PSBT by grabbing "minimum" PSBTInput next role
        result.next = PSBTRole::EXTRACTOR;
        for (unsigned int i = 0; i < psbtx.tx->vin.size(); ++i) {
            PSBTInputAnalysis& input_analysis = result.inputs[i];
            result.next = std::min(result.next, input_analysis.next);
        }
        assert(result.next > PSBTRole::CREATOR);

        if (calc_fee) {
            // Get the output amount
            CAmount out_amt = std::accumulate(psbtx.tx->vout.begin(), psbtx.tx->vout.end(), CAmount(0),
                [](CAmount a, const CTxOut& b) {
                    if (!MoneyRange(a) || !MoneyRange(b.nValue) || !MoneyRange(a + b.nValue)) {
                        return CAmount(-1);
                    }
                    return a += b.nValue;
                }
            );
            if (!MoneyRange(out_amt)) {
                result.SetInvalid(strprintf("PSBT is not valid. Output amount invalid"));
                return result;
            }

            // Get the fee
            CAmount fee = in_amt - out_amt;
            result.fee = fee;

            // Estimate the size
            CMutableTransaction mtx(*psbtx.tx);
            CCoinsView view_dummy;
            CCoinsViewCache view(&view_dummy);
            bool success = true;

            for (unsigned int i = 0; i < psbtx.tx->vin.size(); ++i) {
                PSBTInput& input = psbtx.inputs[i];
                Coin newcoin;

                if (!SignPSBTInput(DUMMY_SIGNING_PROVIDER, psbtx, i, nullptr, 1) || !psbtx.GetInputUTXO(newcoin.out, i)) {
                    success = false;
                    break;
                } else {
                    mtx.vin[i].scriptSig = input.final_script_sig;
                    mtx.vin[i].scriptWitness = input.final_script_witness;
                    newcoin.nHeight = 1;
                    view.AddCoin(psbtx.tx->vin[i].prevout, std::move(newcoin), true);
                }
            }

            if (success) {
                CTransaction ctx = CTransaction(mtx);
                size_t size = GetVirtualTransactionSize(ctx, GetTransactionSigOpCost(ctx, view, STANDARD_SCRIPT_VERIFY_FLAGS));
                result.estimated_vsize = size;
                // Estimate fee rate
                CFeeRate feerate(fee, size);
                result.estimated_feerate = feerate;
            }

        }

        return result;
        */
}
