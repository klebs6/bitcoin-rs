// ---------------- [ File: bitcoin-fuzz/src/fuzz_integer.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/integer.cpp]

pub fn initialize_integer()  {
    
    todo!();
        /*
            SelectParams(CBaseChainParams::REGTEST);
        */
}

#[fuzz_test(initializer = "initialize_integer")]
fn integer() {
    todo!();
    /*
    
        if (buffer.size() < sizeof(uint256) + sizeof(u160)) {
            return;
        }
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const uint256 u256(fuzzed_data_provider.ConsumeBytes<unsigned char>(sizeof(uint256)));
        const u160 u160(fuzzed_data_provider.ConsumeBytes<unsigned char>(sizeof(u160)));
        const uint64_t u64 = fuzzed_data_provider.ConsumeIntegral<uint64_t>();
        const int64_t i64 = fuzzed_data_provider.ConsumeIntegral<int64_t>();
        const uint32_t u32 = fuzzed_data_provider.ConsumeIntegral<uint32_t>();
        const int32_t i32 = fuzzed_data_provider.ConsumeIntegral<int32_t>();
        const uint16_t u16 = fuzzed_data_provider.ConsumeIntegral<uint16_t>();
        const int16_t i16 = fuzzed_data_provider.ConsumeIntegral<int16_t>();
        const uint8_t u8 = fuzzed_data_provider.ConsumeIntegral<uint8_t>();
        const int8_t i8 = fuzzed_data_provider.ConsumeIntegral<int8_t>();
        // We cannot assume a specific value of std::is_signed<char>::value:
        // ConsumeIntegral<char>() instead of casting from {u,}int8_t.
        const char ch = fuzzed_data_provider.ConsumeIntegral<char>();
        const bool b = fuzzed_data_provider.ConsumeBool();

        const ChainConsensusParams& consensus_params = Params().GetConsensus();
        (c_void)CheckProofOfWork(u256, u32, consensus_params);
        if (u64 <= MAX_MONEY) {
            const uint64_t compressed_money_amount = CompressAmount(u64);
            assert(u64 == DecompressAmount(compressed_money_amount));
            static const uint64_t compressed_money_amount_max = CompressAmount(MAX_MONEY - 1);
            assert(compressed_money_amount <= compressed_money_amount_max);
        } else {
            (c_void)CompressAmount(u64);
        }
        static const uint256 u256_min(uint256S("0000000000000000000000000000000000000000000000000000000000000000"));
        static const uint256 u256_max(uint256S("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
        const std::vector<uint256> v256{u256, u256_min, u256_max};
        (c_void)ComputeMerkleRoot(v256);
        (c_void)CountBits(u64);
        (c_void)DecompressAmount(u64);
        (c_void)FormatISO8601Date(i64);
        (c_void)FormatISO8601DateTime(i64);
        {
            if (std::optional<CAmount> parsed = ParseMoney(FormatMoney(i64))) {
                assert(parsed.value() == i64);
            }
        }
        (c_void)GetSizeOfCompactSize(u64);
        (c_void)GetSpecialScriptSize(u32);
        if (!MultiplicationOverflow(i64, static_cast<int64_t>(::nBytesPerSigOp)) && !AdditionOverflow(i64 * ::nBytesPerSigOp, static_cast<int64_t>(4))) {
            (c_void)GetVirtualTransactionSize(i64, i64);
        }
        if (!MultiplicationOverflow(i64, static_cast<int64_t>(u32)) && !AdditionOverflow(i64, static_cast<int64_t>(4)) && !AdditionOverflow(i64 * u32, static_cast<int64_t>(4))) {
            (c_void)GetVirtualTransactionSize(i64, i64, u32);
        }
        (c_void)HexDigit(ch);
        (c_void)MoneyRange(i64);
        (c_void)ToString(i64);
        (c_void)IsDigit(ch);
        (c_void)IsSpace(ch);
        (c_void)IsSwitchChar(ch);
        (c_void)memusage::DynamicUsage(ch);
        (c_void)memusage::DynamicUsage(i16);
        (c_void)memusage::DynamicUsage(i32);
        (c_void)memusage::DynamicUsage(i64);
        (c_void)memusage::DynamicUsage(i8);
        (c_void)memusage::DynamicUsage(u16);
        (c_void)memusage::DynamicUsage(u32);
        (c_void)memusage::DynamicUsage(u64);
        (c_void)memusage::DynamicUsage(u8);
        const unsigned char uch = static_cast<unsigned char>(u8);
        (c_void)memusage::DynamicUsage(uch);
        {
            const std::set<int64_t> i64s{i64, static_cast<int64_t>(u64)};
            const size_t dynamic_usage = memusage::DynamicUsage(i64s);
            const size_t incremental_dynamic_usage = memusage::IncrementalDynamicUsage(i64s);
            assert(dynamic_usage == incremental_dynamic_usage * i64s.size());
        }
        (c_void)MillisToTimeval(i64);
        (c_void)SighashToStr(uch);
        (c_void)SipHashUint256(u64, u64, u256);
        (c_void)SipHashUint256Extra(u64, u64, u256, u32);
        (c_void)ToLower(ch);
        (c_void)ToUpper(ch);
        {
            if (std::optional<CAmount> parsed = ParseMoney(ValueFromAmount(i64).getValStr())) {
                assert(parsed.value() == i64);
            }
        }
        if (i32 >= 0 && i32 <= 16) {
            assert(i32 == CScript::DecodeOP_N(CScript::EncodeOP_N(i32)));
        }

        const std::chrono::seconds seconds{i64};
        assert(count_seconds(seconds) == i64);

        const CScriptNum script_num{i64};
        (c_void)script_num.getint();
        (c_void)script_num.getvch();

        const arith_uint256 au256 = UintToArith256(u256);
        assert(ArithToUint256(au256) == u256);
        assert(uint256S(au256.GetHex()) == u256);
        (c_void)au256.bits();
        (c_void)au256.GetCompact(/* fNegative= */ false);
        (c_void)au256.GetCompact(/* fNegative= */ true);
        (c_void)au256.getdouble();
        (c_void)au256.GetHex();
        (c_void)au256.GetLow64();
        (c_void)au256.size();
        (c_void)au256.ToString();

        const CKeyID key_id{u160};
        const CScriptID script_id{u160};

        {
            DataStream stream(SER_NETWORK, INIT_PROTO_VERSION);

            uint256 deserialized_u256;
            stream << u256;
            stream >> deserialized_u256;
            assert(u256 == deserialized_u256 && stream.empty());

            u160 deserialized_u160;
            stream << u160;
            stream >> deserialized_u160;
            assert(u160 == deserialized_u160 && stream.empty());

            uint64_t deserialized_u64;
            stream << u64;
            stream >> deserialized_u64;
            assert(u64 == deserialized_u64 && stream.empty());

            int64_t deserialized_i64;
            stream << i64;
            stream >> deserialized_i64;
            assert(i64 == deserialized_i64 && stream.empty());

            uint32_t deserialized_u32;
            stream << u32;
            stream >> deserialized_u32;
            assert(u32 == deserialized_u32 && stream.empty());

            int32_t deserialized_i32;
            stream << i32;
            stream >> deserialized_i32;
            assert(i32 == deserialized_i32 && stream.empty());

            uint16_t deserialized_u16;
            stream << u16;
            stream >> deserialized_u16;
            assert(u16 == deserialized_u16 && stream.empty());

            int16_t deserialized_i16;
            stream << i16;
            stream >> deserialized_i16;
            assert(i16 == deserialized_i16 && stream.empty());

            uint8_t deserialized_u8;
            stream << u8;
            stream >> deserialized_u8;
            assert(u8 == deserialized_u8 && stream.empty());

            int8_t deserialized_i8;
            stream << i8;
            stream >> deserialized_i8;
            assert(i8 == deserialized_i8 && stream.empty());

            char deserialized_ch;
            stream << ch;
            stream >> deserialized_ch;
            assert(ch == deserialized_ch && stream.empty());

            bool deserialized_b;
            stream << b;
            stream >> deserialized_b;
            assert(b == deserialized_b && stream.empty());
        }

        {
            const ServiceFlags service_flags = (ServiceFlags)u64;
            (c_void)HasAllDesirableServiceFlags(service_flags);
            (c_void)MayHaveUsefulAddressDB(service_flags);
        }

        {
            DataStream stream(SER_NETWORK, INIT_PROTO_VERSION);

            ser_writedata64(stream, u64);
            const uint64_t deserialized_u64 = ser_readdata64(stream);
            assert(u64 == deserialized_u64 && stream.empty());

            ser_writedata32(stream, u32);
            const uint32_t deserialized_u32 = ser_readdata32(stream);
            assert(u32 == deserialized_u32 && stream.empty());

            ser_writedata32be(stream, u32);
            const uint32_t deserialized_u32be = ser_readdata32be(stream);
            assert(u32 == deserialized_u32be && stream.empty());

            ser_writedata16(stream, u16);
            const uint16_t deserialized_u16 = ser_readdata16(stream);
            assert(u16 == deserialized_u16 && stream.empty());

            ser_writedata16be(stream, u16);
            const uint16_t deserialized_u16be = ser_readdata16be(stream);
            assert(u16 == deserialized_u16be && stream.empty());

            ser_writedata8(stream, u8);
            const uint8_t deserialized_u8 = ser_readdata8(stream);
            assert(u8 == deserialized_u8 && stream.empty());
        }

        {
            DataStream stream(SER_NETWORK, INIT_PROTO_VERSION);

            WriteCompactSize(stream, u64);
            try {
                const uint64_t deserialized_u64 = ReadCompactSize(stream);
                assert(u64 == deserialized_u64 && stream.empty());
            } catch (const std::ios_base::failure&) {
            }
        }

        try {
            CHECK_NONFATAL(b);
        } catch (const NonFatalCheckError&) {
        }

    */
}
