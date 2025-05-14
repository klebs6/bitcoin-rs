// ---------------- [ File: bitcoin-fuzz/src/fuzz_kitchen_sink.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/kitchen_sink.cpp]

lazy_static!{
    /*
    constexpr TransactionError ALL_TRANSACTION_ERROR[] = {
        TransactionError::OK,
        TransactionError::MISSING_INPUTS,
        TransactionError::ALREADY_IN_CHAIN,
        TransactionError::P2P_DISABLED,
        TransactionError::MEMPOOL_REJECTED,
        TransactionError::MEMPOOL_ERROR,
        TransactionError::INVALID_PSBT,
        TransactionError::PSBT_MISMATCH,
        TransactionError::SIGHASH_MISMATCH,
        TransactionError::MAX_FEE_EXCEEDED,
    };
    */
}

/**
  | The fuzzing kitchen sink: Fuzzing harness for
  | functions that need to be fuzzed but a.) don't
  | belong in any existing fuzzing harness file,
  | and b.) are not important enough to warrant
  | their own fuzzing harness file.
  */
#[fuzz] fn kitchen_sink() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        const TransactionError transaction_error = fuzzed_data_provider.PickValueInArray(ALL_TRANSACTION_ERROR);
        (c_void)JSONRPCTransactionError(transaction_error);
        (c_void)RPCErrorFromTransactionError(transaction_error);
        (c_void)TransactionErrorString(transaction_error);

        (c_void)StringForFeeEstimateHorizon(fuzzed_data_provider.PickValueInArray(ALL_FEE_ESTIMATE_HORIZONS));

        const OutputType output_type = fuzzed_data_provider.PickValueInArray(OUTPUT_TYPES);
        const std::string& output_type_string = FormatOutputType(output_type);
        const std::optional<OutputType> parsed = ParseOutputType(output_type_string);
        assert(parsed);
        assert(output_type == parsed.value());
        (c_void)ParseOutputType(fuzzed_data_provider.ConsumeRandomLengthString(64));

        const std::vector<uint8_t> bytes = ConsumeRandomLengthByteVector(fuzzed_data_provider);
        const std::vector<bool> bits = BytesToBits(bytes);
        const std::vector<uint8_t> bytes_decoded = BitsToBytes(bits);
        assert(bytes == bytes_decoded);

    */
}
