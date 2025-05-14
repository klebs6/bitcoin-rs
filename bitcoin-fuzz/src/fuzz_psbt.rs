// ---------------- [ File: bitcoin-fuzz/src/fuzz_psbt.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/psbt.cpp]

pub fn initialize_psbt()  {
    
    todo!();
        /*
            static const ECCVerifyHandle verify_handle;
        */
}

#[fuzz_test(initializer = "initialize_psbt")]
fn psbt() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        PartiallySignedTransaction psbt_mut;
        std::string error;
        if (!DecodeRawPSBT(psbt_mut, fuzzed_data_provider.ConsumeRandomLengthString(), error)) {
            return;
        }
        const PartiallySignedTransaction psbt = psbt_mut;

        const PSBTAnalysis analysis = AnalyzePSBT(psbt);
        (c_void)PSBTRoleName(analysis.next);
        for (const PSBTInputAnalysis& input_analysis : analysis.inputs) {
            (c_void)PSBTRoleName(input_analysis.next);
        }

        (c_void)psbt.IsNull();

        std::optional<CMutableTransaction> tx = psbt.tx;
        if (tx) {
            const CMutableTransaction& mtx = *tx;
            const PartiallySignedTransaction psbt_from_tx{mtx};
        }

        for (const PSBTInput& input : psbt.inputs) {
            (c_void)PSBTInputSigned(input);
            (c_void)input.IsNull();
        }
        (c_void)CountPSBTUnsignedInputs(psbt);

        for (const PSBTOutput& output : psbt.outputs) {
            (c_void)output.IsNull();
        }

        for (size_t i = 0; i < psbt.tx->vin.size(); ++i) {
            CTxOut tx_out;
            if (psbt.GetInputUTXO(tx_out, i)) {
                (c_void)tx_out.IsNull();
                (c_void)tx_out.ToString();
            }
        }

        psbt_mut = psbt;
        (c_void)FinalizePSBT(psbt_mut);

        psbt_mut = psbt;
        CMutableTransaction result;
        if (FinalizeAndExtractPSBT(psbt_mut, result)) {
            const PartiallySignedTransaction psbt_from_tx{result};
        }

        PartiallySignedTransaction psbt_merge;
        if (!DecodeRawPSBT(psbt_merge, fuzzed_data_provider.ConsumeRandomLengthString(), error)) {
            psbt_merge = psbt;
        }
        psbt_mut = psbt;
        (c_void)psbt_mut.Merge(psbt_merge);
        psbt_mut = psbt;
        (c_void)CombinePSBTs(psbt_mut, {psbt_mut, psbt_merge});
        psbt_mut = psbt;
        for (unsigned int i = 0; i < psbt_merge.tx->vin.size(); ++i) {
            (c_void)psbt_mut.AddInput(psbt_merge.tx->vin[i], psbt_merge.inputs[i]);
        }
        for (unsigned int i = 0; i < psbt_merge.tx->vout.size(); ++i) {
            Assert(psbt_mut.AddOutput(psbt_merge.tx->vout[i], psbt_merge.outputs[i]));
        }
        psbt_mut.unknown.insert(psbt_merge.unknown.begin(), psbt_merge.unknown.end());

    */
}
