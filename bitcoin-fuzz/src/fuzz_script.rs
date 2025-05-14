// ---------------- [ File: bitcoin-fuzz/src/fuzz_script.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/script.cpp]

pub fn initialize_script()  {
    
    todo!();
        /*
            // Fuzzers using pubkey must hold an ECCVerifyHandle.
        static const ECCVerifyHandle verify_handle;

        SelectParams(CBaseChainParams::REGTEST);
        */
}

#[fuzz_test(initializer = "initialize_script")]
fn script() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::optional<CScript> script_opt = ConsumeDeserializable<CScript>(fuzzed_data_provider);
        if (!script_opt) return;
        const CScript script{*script_opt};

        CompressedScript compressed;
        if (CompressScript(script, compressed)) {
            const unsigned int size = compressed[0];
            compressed.erase(compressed.begin());
            assert(size <= 5);
            CScript decompressed_script;
            const bool ok = DecompressScript(decompressed_script, size, compressed);
            assert(ok);
            assert(script == decompressed_script);
        }

        TxoutType which_type;
        bool is_standard_ret = IsStandard(script, which_type);
        if (!is_standard_ret) {
            assert(which_type == TxoutType::NONSTANDARD ||
                   which_type == TxoutType::NULL_DATA ||
                   which_type == TxoutType::MULTISIG);
        }
        if (which_type == TxoutType::NONSTANDARD) {
            assert(!is_standard_ret);
        }
        if (which_type == TxoutType::NULL_DATA) {
            assert(script.IsUnspendable());
        }
        if (script.IsUnspendable()) {
            assert(which_type == TxoutType::NULL_DATA ||
                   which_type == TxoutType::NONSTANDARD);
        }

        TxDestination address;
        bool extract_destination_ret = ExtractDestination(script, address);
        if (!extract_destination_ret) {
            assert(which_type == TxoutType::PUBKEY ||
                   which_type == TxoutType::NONSTANDARD ||
                   which_type == TxoutType::NULL_DATA ||
                   which_type == TxoutType::MULTISIG);
        }
        if (which_type == TxoutType::NONSTANDARD ||
            which_type == TxoutType::NULL_DATA ||
            which_type == TxoutType::MULTISIG) {
            assert(!extract_destination_ret);
        }

        const FlatSigningProvider signing_provider;
        (c_void)InferDescriptor(script, signing_provider);
        (c_void)IsSegWitOutput(signing_provider, script);
        (c_void)IsSolvable(signing_provider, script);

        (c_void)RecursiveDynamicUsage(script);

        std::vector<std::vector<unsigned char>> solutions;
        (c_void)Solver(script, solutions);

        (c_void)script.HasValidOps();
        (c_void)script.IsPayToScriptHash();
        (c_void)script.IsPayToWitnessScriptHash();
        (c_void)script.IsPushOnly();
        (c_void)script.GetSigOpCount(/* fAccurate= */ false);

        (c_void)FormatScript(script);
        (c_void)ScriptToAsmStr(script, false);
        (c_void)ScriptToAsmStr(script, true);

        UniValue o1(UniValue::VOBJ);
        ScriptPubKeyToUniv(script, o1, true);
        UniValue o2(UniValue::VOBJ);
        ScriptPubKeyToUniv(script, o2, false);
        UniValue o3(UniValue::VOBJ);
        ScriptToUniv(script, o3);

        {
            const std::vector<uint8_t> bytes = ConsumeRandomLengthByteVector(fuzzed_data_provider);
            CompressedScript compressed_script;
            compressed_script.assign(bytes.begin(), bytes.end());
            // DecompressScript(..., ..., bytes) is not guaranteed to be defined if the bytes vector is too short
            if (compressed_script.size() >= 32) {
                CScript decompressed_script;
                DecompressScript(decompressed_script, fuzzed_data_provider.ConsumeIntegral<unsigned int>(), compressed_script);
            }
        }

        const std::optional<CScript> other_script = ConsumeDeserializable<CScript>(fuzzed_data_provider);
        if (other_script) {
            {
                CScript script_mut{script};
                (c_void)FindAndDelete(script_mut, *other_script);
            }
            const std::vector<std::string> random_string_vector = ConsumeRandomLengthStringVector(fuzzed_data_provider);
            const uint32_t u32{fuzzed_data_provider.ConsumeIntegral<uint32_t>()};
            const uint32_t flags{u32 | SCRIPT_VERIFY_P2SH};
            {
                CScriptWitness wit;
                for (const auto& s : random_string_vector) {
                    wit.stack.emplace_back(s.begin(), s.end());
                }
                (c_void)CountWitnessSigOps(script, *other_script, &wit, flags);
                wit.SetNull();
            }
        }

        (c_void)GetOpName(ConsumeOpcodeType(fuzzed_data_provider));
        (c_void)ScriptErrorString(static_cast<ScriptError>(fuzzed_data_provider.ConsumeIntegralInRange<int>(0, SCRIPT_ERR_ERROR_COUNT)));

        {
            const std::vector<uint8_t> bytes = ConsumeRandomLengthByteVector(fuzzed_data_provider);
            CScript append_script{bytes.begin(), bytes.end()};
            append_script << fuzzed_data_provider.ConsumeIntegral<int64_t>();
            append_script << ConsumeOpcodeType(fuzzed_data_provider);
            append_script << CScriptNum{fuzzed_data_provider.ConsumeIntegral<int64_t>()};
            append_script << ConsumeRandomLengthByteVector(fuzzed_data_provider);
        }

        {
            const TxDestination tx_destination_1{
                fuzzed_data_provider.ConsumeBool() ?
                    DecodeDestination(fuzzed_data_provider.ConsumeRandomLengthString()) :
                    ConsumeTxDestination(fuzzed_data_provider)};
            const TxDestination tx_destination_2{ConsumeTxDestination(fuzzed_data_provider)};
            const std::string encoded_dest{EncodeDestination(tx_destination_1)};
            const UniValue json_dest{DescribeAddress(tx_destination_1)};
            Assert(tx_destination_1 == DecodeDestination(encoded_dest));
            (c_void)GetKeyForDestination(/* store */ {}, tx_destination_1);
            const CScript dest{GetScriptForDestination(tx_destination_1)};
            const bool valid{IsValidDestination(tx_destination_1)};
            Assert(dest.empty() != valid);

            Assert(valid == IsValidDestinationString(encoded_dest));

            (c_void)(tx_destination_1 < tx_destination_2);
            if (tx_destination_1 == tx_destination_2) {
                Assert(encoded_dest == EncodeDestination(tx_destination_2));
                Assert(json_dest.write() == DescribeAddress(tx_destination_2).write());
                Assert(dest == GetScriptForDestination(tx_destination_2));
            }
        }

    */
}
