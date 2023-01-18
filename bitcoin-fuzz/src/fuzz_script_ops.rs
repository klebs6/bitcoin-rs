crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/script_ops.cpp]

#[fuzz_test] fn script_ops() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        CScript script_mut = ConsumeScript(fuzzed_data_provider);
        while (fuzzed_data_provider.remaining_bytes() > 0) {
            CallOneOf(
                fuzzed_data_provider,
                [&] {
                    CScript s = ConsumeScript(fuzzed_data_provider);
                    script_mut = std::move(s);
                },
                [&] {
                    const CScript& s = ConsumeScript(fuzzed_data_provider);
                    script_mut = s;
                },
                [&] {
                    script_mut << fuzzed_data_provider.ConsumeIntegral<int64_t>();
                },
                [&] {
                    script_mut << ConsumeOpcodeType(fuzzed_data_provider);
                },
                [&] {
                    script_mut << ConsumeScriptNum(fuzzed_data_provider);
                },
                [&] {
                    script_mut << ConsumeRandomLengthByteVector(fuzzed_data_provider);
                },
                [&] {
                    script_mut.clear();
                });
        }
        const CScript& script = script_mut;
        (c_void)script.GetSigOpCount(false);
        (c_void)script.GetSigOpCount(true);
        (c_void)script.GetSigOpCount(script);
        (c_void)script.HasValidOps();
        (c_void)script.IsPayToScriptHash();
        (c_void)script.IsPayToWitnessScriptHash();
        (c_void)script.IsPushOnly();
        (c_void)script.IsUnspendable();
        {
            CScript::const_iterator pc = script.begin();
            opcodetype opcode;
            (c_void)script.GetOp(pc, opcode);
            std::vector<uint8_t> data;
            (c_void)script.GetOp(pc, opcode, data);
            (c_void)script.IsPushOnly(pc);
        }
        {
            int version;
            std::vector<uint8_t> program;
            (c_void)script.IsWitnessProgram(version, program);
        }

    */
}
